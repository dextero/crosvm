// Copyright 2020 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

use acpi_tables::aml;
use acpi_tables::aml::Aml;
use anyhow::bail;
use anyhow::Context;
use base::error;
use base::warn;
use base::AsRawDescriptor;
use base::Event;
use base::EventToken;
use base::RawDescriptor;
use base::Tube;
use base::WaitContext;
use base::WorkerThread;
use power_monitor::BatteryStatus;
use power_monitor::CreatePowerClientFn;
use power_monitor::CreatePowerMonitorFn;
use power_monitor::PowerClient;
use remain::sorted;
use serde::Deserialize;
use serde::Serialize;
use snapshot::AnySnapshot;
use sync::Mutex;
use thiserror::Error;
use vm_control::BatControlCommand;
use vm_control::BatControlResult;

use crate::pci::CrosvmDeviceId;
use crate::BusAccessInfo;
use crate::BusDevice;
use crate::DeviceId;
use crate::IrqLevelEvent;
use crate::Suspendable;

/// Errors for battery devices.
#[sorted]
#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("Non 32-bit mmio address space")]
    Non32BitMmioAddress,
}

type Result<T> = std::result::Result<T, BatteryError>;

/// the GoldFish Battery MMIO length.
pub const GOLDFISHBAT_MMIO_LEN: u64 = 0x1000;

/// Configuration of fake battery status information.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Copy)]
pub enum BatConfig {
    /// Propagates host's battery status
    #[default]
    Real,
    /// Fake on battery status. Simulates a disconnected AC adapter.
    /// This forces ac_online to false and sets the battery status
    /// to DISCHARGING
    Fake {
        // Sets the maximum battery capacity reported to the guest
        max_capacity: u32,
    },
}

#[derive(Clone, Serialize, Deserialize)]
struct GoldfishBatteryState {
    // interrupt state
    int_status: u32,
    int_enable: u32,
    // AC state
    ac_online: u32,
    // Battery state
    status: u32,
    health: u32,
    present: u32,
    capacity: u32,
    voltage: u32,
    current: u32,
    charge_counter: u32,
    charge_full: u32,
    // bat_config is used for goldfish battery to report fake battery to the guest.
    bat_config: BatConfig,
}

macro_rules! create_battery_func {
    // $property: the battery property which is going to be modified.
    // $ty: the type annotation of value argument
    // $int: the interrupt status which is going to be set to notify the guest.
    ($fn:ident, $property:ident, $ty:ty, $int:ident) => {
        pub(crate) fn $fn(&mut self, value: $ty) -> bool {
            let old = std::mem::replace(&mut self.$property, value);
            old != self.$property && self.set_int_status($int)
        }
    };
}

impl GoldfishBatteryState {
    fn set_int_status(&mut self, mask: u32) -> bool {
        let old = self.int_status;
        self.int_status |= self.int_enable & mask;
        old != self.int_status
    }

    fn int_status(&self) -> u32 {
        self.int_status
    }

    create_battery_func!(set_ac_online, ac_online, u32, AC_STATUS_CHANGED);

    create_battery_func!(set_status, status, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_health, health, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_present, present, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_capacity, capacity, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_voltage, voltage, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_current, current, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(
        set_charge_counter,
        charge_counter,
        u32,
        BATTERY_STATUS_CHANGED
    );

    create_battery_func!(set_charge_full, charge_full, u32, BATTERY_STATUS_CHANGED);

    create_battery_func!(set_bat_config, bat_config, BatConfig, BATTERY_INT_MASK);
}

enum BatInitializationState {
    NotYet,
    Pending(Box<dyn PowerClient>),
    Done,
}

/// GoldFish Battery state
pub struct GoldfishBattery {
    state: Arc<Mutex<GoldfishBatteryState>>,
    mmio_base: u32,
    irq_num: u32,
    irq_evt: IrqLevelEvent,
    activated: bool,
    monitor_thread: Option<WorkerThread<()>>,
    tube: Option<Tube>,
    create_power_monitor: Option<Box<dyn CreatePowerMonitorFn>>,
    create_powerd_client: Option<Box<dyn CreatePowerClientFn>>,
    init_state: Arc<Mutex<BatInitializationState>>,
}

#[derive(Serialize, Deserialize)]
struct GoldfishBatterySnapshot {
    state: GoldfishBatteryState,
    mmio_base: u32,
    irq_num: u32,
    activated: bool,
}

/// Goldfish Battery MMIO offset
const BATTERY_INT_STATUS: u32 = 0;
const BATTERY_INT_ENABLE: u32 = 0x4;
const BATTERY_AC_ONLINE: u32 = 0x8;
const BATTERY_STATUS: u32 = 0xC;
const BATTERY_HEALTH: u32 = 0x10;
const BATTERY_PRESENT: u32 = 0x14;
const BATTERY_CAPACITY: u32 = 0x18;
const BATTERY_VOLTAGE: u32 = 0x1C;
const BATTERY_TEMP: u32 = 0x20;
const BATTERY_CHARGE_COUNTER: u32 = 0x24;
const BATTERY_VOLTAGE_MAX: u32 = 0x28;
const BATTERY_CURRENT_MAX: u32 = 0x2C;
const BATTERY_CURRENT_NOW: u32 = 0x30;
const BATTERY_CURRENT_AVG: u32 = 0x34;
const BATTERY_CHARGE_FULL_UAH: u32 = 0x38;
const BATTERY_CYCLE_COUNT: u32 = 0x40;

/// Goldfish Battery interrupt bits
const BATTERY_STATUS_CHANGED: u32 = 1 << 0;
const AC_STATUS_CHANGED: u32 = 1 << 1;
const BATTERY_INT_MASK: u32 = BATTERY_STATUS_CHANGED | AC_STATUS_CHANGED;

/// Goldfish Battery status
const BATTERY_STATUS_VAL_UNKNOWN: u32 = 0;
const BATTERY_STATUS_VAL_CHARGING: u32 = 1;
const BATTERY_STATUS_VAL_DISCHARGING: u32 = 2;
const BATTERY_STATUS_VAL_NOT_CHARGING: u32 = 3;

/// Goldfish Battery health
const BATTERY_HEALTH_VAL_UNKNOWN: u32 = 0;

// Goldfish ac online status
const AC_ONLINE_VAL_OFFLINE: u32 = 0;

#[derive(EventToken)]
pub(crate) enum Token {
    Commands,
    Resample,
    Kill,
    Monitor,
}

fn command_monitor(
    tube: Tube,
    irq_evt: IrqLevelEvent,
    kill_evt: Event,
    state: Arc<Mutex<GoldfishBatteryState>>,
    create_power_monitor: Option<Box<dyn CreatePowerMonitorFn>>,
    init_state: Arc<Mutex<BatInitializationState>>,
) {
    let wait_ctx: WaitContext<Token> = match WaitContext::build_with(&[
        (&tube, Token::Commands),
        (irq_evt.get_resample(), Token::Resample),
        (&kill_evt, Token::Kill),
    ]) {
        Ok(pc) => pc,
        Err(e) => {
            error!("failed to build WaitContext: {}", e);
            return;
        }
    };

    let mut power_monitor = match create_power_monitor {
        Some(f) => match f() {
            Ok(p) => match wait_ctx.add(p.get_read_notifier(), Token::Monitor) {
                Ok(()) => Some(p),
                Err(e) => {
                    error!("failed to add power monitor to poll context: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("failed to create power monitor: {}", e);
                None
            }
        },
        None => None,
    };

    'poll: loop {
        let events = match wait_ctx.wait() {
            Ok(v) => v,
            Err(e) => {
                error!("error while polling for events: {}", e);
                break;
            }
        };

        for event in events.iter().filter(|e| e.is_readable) {
            match event.token {
                Token::Commands => {
                    let req = match tube.recv() {
                        Ok(req) => req,
                        Err(e) => {
                            error!("failed to receive request: {}", e);
                            continue;
                        }
                    };

                    let mut bat_state = state.lock();
                    let inject_irq = match req {
                        BatControlCommand::SetStatus(status) => bat_state.set_status(status.into()),
                        BatControlCommand::SetHealth(health) => bat_state.set_health(health.into()),
                        BatControlCommand::SetPresent(present) => {
                            let v = present != 0;
                            bat_state.set_present(v.into())
                        }
                        BatControlCommand::SetCapacity(capacity) => {
                            let v = std::cmp::min(capacity, 100);
                            bat_state.set_capacity(v)
                        }
                        BatControlCommand::SetACOnline(ac_online) => {
                            let v = ac_online != 0;
                            bat_state.set_ac_online(v.into())
                        }
                        BatControlCommand::SetFakeBatConfig(max_capacity) => {
                            let max_capacity = std::cmp::min(max_capacity, 100);
                            bat_state.set_bat_config(BatConfig::Fake { max_capacity })
                        }
                        BatControlCommand::CancelFakeConfig => {
                            bat_state.set_bat_config(BatConfig::Real)
                        }
                    };

                    if inject_irq {
                        let _ = irq_evt.trigger();
                    }

                    if let Err(e) = tube.send(&BatControlResult::Ok) {
                        error!("failed to send response: {}", e);
                    }
                }

                Token::Monitor => {
                    // Safe because power_monitor must be populated if Token::Monitor is triggered.
                    let power_monitor = power_monitor.as_mut().unwrap();

                    let data = match power_monitor.read_message() {
                        Ok(Some(d)) => d,
                        Ok(None) => continue,
                        Err(e) => {
                            error!("failed to read new power data: {}", e);
                            continue;
                        }
                    };

                    let mut bat_state = state.lock();

                    // Each set_* function called below returns true when interrupt bits
                    // (*_STATUS_CHANGED) changed. If `inject_irq` is true after we attempt to
                    // update each field, inject an interrupt.
                    let mut inject_irq = bat_state.set_ac_online(data.ac_online.into());

                    match data.battery {
                        Some(battery_data) => {
                            inject_irq |= bat_state.set_capacity(battery_data.percent);
                            let battery_status = match battery_data.status {
                                BatteryStatus::Unknown => BATTERY_STATUS_VAL_UNKNOWN,
                                BatteryStatus::Charging => BATTERY_STATUS_VAL_CHARGING,
                                BatteryStatus::Discharging => BATTERY_STATUS_VAL_DISCHARGING,
                                BatteryStatus::NotCharging => BATTERY_STATUS_VAL_NOT_CHARGING,
                            };
                            inject_irq |= bat_state.set_status(battery_status);
                            inject_irq |= bat_state.set_voltage(battery_data.voltage);
                            inject_irq |= bat_state.set_current(battery_data.current);
                            inject_irq |= bat_state.set_charge_counter(battery_data.charge_counter);
                            inject_irq |= bat_state.set_charge_full(battery_data.charge_full);
                        }
                        None => {
                            inject_irq |= bat_state.set_present(0);
                        }
                    }
                    *init_state.lock() = BatInitializationState::Done;

                    if inject_irq {
                        let _ = irq_evt.trigger();
                    }
                }

                Token::Resample => {
                    irq_evt.clear_resample();
                    if state.lock().int_status() != 0 {
                        let _ = irq_evt.trigger();
                    }
                }

                Token::Kill => break 'poll,
            }
        }
    }
}

impl GoldfishBattery {
    /// The interval in milli seconds between DBus requests to powerd.  This is used to rate-limit
    /// requests to avoid overwhelming the power daemon.
    pub(crate) const POWERD_REQ_INTERVAL_MS: u64 = 1000;

    /// Create GoldfishBattery device model
    ///
    /// * `mmio_base` - The 32-bit mmio base address.
    /// * `irq_num` - The corresponding interrupt number of the irq_evt which will be put into the
    ///   ACPI DSDT.
    /// * `irq_evt` - The interrupt event used to notify driver about the battery properties
    ///   changing.
    /// * `socket` - Battery control socket
    pub fn new(
        mmio_base: u64,
        irq_num: u32,
        irq_evt: IrqLevelEvent,
        tube: Tube,
        create_power_monitor: Option<Box<dyn CreatePowerMonitorFn>>,
        create_powerd_client: Option<Box<dyn CreatePowerClientFn>>,
    ) -> Result<Self> {
        if mmio_base + GOLDFISHBAT_MMIO_LEN - 1 > u32::MAX as u64 {
            return Err(BatteryError::Non32BitMmioAddress);
        }
        let state = Arc::new(Mutex::new(GoldfishBatteryState {
            capacity: 50,
            health: BATTERY_HEALTH_VAL_UNKNOWN,
            present: 1,
            status: BATTERY_STATUS_VAL_UNKNOWN,
            ac_online: 1,
            int_enable: 0,
            int_status: 0,
            voltage: 0,
            current: 0,
            charge_counter: 0,
            charge_full: 0,
            bat_config: BatConfig::Real,
        }));

        Ok(GoldfishBattery {
            state,
            mmio_base: mmio_base as u32,
            irq_num,
            irq_evt,
            activated: false,
            monitor_thread: None,
            tube: Some(tube),
            create_power_monitor,
            create_powerd_client,
            init_state: Arc::new(Mutex::new(BatInitializationState::NotYet)),
        })
    }

    /// return the descriptors used by this device
    pub fn keep_rds(&self) -> Vec<RawDescriptor> {
        let mut rds = vec![
            self.irq_evt.get_trigger().as_raw_descriptor(),
            self.irq_evt.get_resample().as_raw_descriptor(),
        ];

        if let Some(tube) = &self.tube {
            rds.push(tube.as_raw_descriptor());
        }

        rds
    }

    /// start a monitor thread to monitor the events from host
    fn start_monitor(&mut self) {
        if self.activated {
            return;
        }

        if let Some(tube) = self.tube.take() {
            let irq_evt = self.irq_evt.try_clone().unwrap();
            let bat_state = self.state.clone();
            let create_monitor_fn = self.create_power_monitor.take();
            let init_state = self.init_state.clone();
            self.monitor_thread = Some(WorkerThread::start(self.debug_label(), move |kill_evt| {
                command_monitor(
                    tube,
                    irq_evt,
                    kill_evt,
                    bat_state,
                    create_monitor_fn,
                    init_state,
                )
            }));
            self.activated = true;
        }
    }

    fn initialize_battery_state(&mut self) -> anyhow::Result<()> {
        let mut init_state = self.init_state.lock();
        let power_client = match (init_state.deref_mut(), &self.create_powerd_client) {
            (BatInitializationState::NotYet, None) => {
                // No need to initialize the state via DBus.
                return Ok(());
            }
            (BatInitializationState::NotYet, Some(f)) => {
                // Initialize power_client
                let client = match f() {
                    Ok(c) => c,
                    Err(e) => bail!("failed to connect to the powerd: {:#}", e),
                };
                // Save power_client to init_state
                *init_state = BatInitializationState::Pending(client);

                let power_client = match init_state.deref_mut() {
                    BatInitializationState::Pending(ref mut power_client) => power_client.as_mut(),
                    _ => unreachable!("init_state should be Pending"),
                };
                power_client
            }
            (BatInitializationState::Pending(ref mut power_client), _) => power_client.as_mut(),
            (BatInitializationState::Done, _) => bail!("battery status already intialized"),
        };

        if let Some(prev_call) = power_client.last_request_timestamp() {
            // Fail if the last request was sent within 1 second.
            let now = SystemTime::now();
            let duration = now
                .duration_since(prev_call)
                .context("failed to calculate time for dbus request")?;
            if duration < Duration::from_millis(Self::POWERD_REQ_INTERVAL_MS) {
                return Ok(());
            }
        }

        match power_client.get_power_data() {
            Ok(data) => {
                let mut bat_state = self.state.lock();
                bat_state.set_ac_online(data.ac_online.into());

                match data.battery {
                    Some(battery_data) => {
                        bat_state.set_capacity(battery_data.percent);
                        let battery_status = match battery_data.status {
                            BatteryStatus::Unknown => BATTERY_STATUS_VAL_UNKNOWN,
                            BatteryStatus::Charging => BATTERY_STATUS_VAL_CHARGING,
                            BatteryStatus::Discharging => BATTERY_STATUS_VAL_DISCHARGING,
                            BatteryStatus::NotCharging => BATTERY_STATUS_VAL_NOT_CHARGING,
                        };
                        bat_state.set_status(battery_status);
                        bat_state.set_voltage(battery_data.voltage);
                        bat_state.set_current(battery_data.current);
                        bat_state.set_charge_counter(battery_data.charge_counter);
                        bat_state.set_charge_full(battery_data.charge_full);
                    }
                    None => {
                        bat_state.set_present(0);
                    }
                }
            }
            Err(e) => {
                bail!("failed to get response from powerd: {:#}", e);
            }
        };
        // Release powerd_client if the initialization data is obtained.
        *init_state = BatInitializationState::Done;
        Ok(())
    }

    fn battery_init_done(&self) -> bool {
        matches!(*self.init_state.lock(), BatInitializationState::Done)
    }
}

impl Drop for GoldfishBattery {
    fn drop(&mut self) {
        if let Err(e) = self.sleep() {
            error!("{}", e);
        };
    }
}

impl BusDevice for GoldfishBattery {
    fn device_id(&self) -> DeviceId {
        CrosvmDeviceId::GoldfishBattery.into()
    }

    fn debug_label(&self) -> String {
        "GoldfishBattery".to_owned()
    }

    fn read(&mut self, info: BusAccessInfo, data: &mut [u8]) {
        if data.len() != std::mem::size_of::<u32>() {
            warn!(
                "{}: unsupported read length {}, only support 4bytes read",
                self.debug_label(),
                data.len()
            );
            return;
        }

        // Before first read, we try to ask powerd the actual power data to initialize `self.state`.
        if !self.battery_init_done() {
            if let Err(e) = self.initialize_battery_state() {
                error!(
                    "{}: failed to initialize bettery state (info={:?}): {:#}",
                    self.debug_label(),
                    info,
                    e
                );
            }
        }

        let val = match info.offset as u32 {
            BATTERY_INT_STATUS => {
                // read to clear the interrupt status
                std::mem::replace(&mut self.state.lock().int_status, 0)
            }
            BATTERY_INT_ENABLE => self.state.lock().int_enable,
            BATTERY_AC_ONLINE => {
                let bat_config = self.state.lock().bat_config;
                match bat_config {
                    BatConfig::Real => self.state.lock().ac_online,
                    BatConfig::Fake { max_capacity: _ } => AC_ONLINE_VAL_OFFLINE,
                }
            }
            BATTERY_STATUS => {
                let bat_config = self.state.lock().bat_config;
                match bat_config {
                    BatConfig::Real => self.state.lock().status,
                    BatConfig::Fake { max_capacity: _ } => BATTERY_STATUS_VAL_DISCHARGING,
                }
            }
            BATTERY_HEALTH => self.state.lock().health,
            BATTERY_PRESENT => self.state.lock().present,
            BATTERY_CAPACITY => {
                let max_capacity = match self.state.lock().bat_config {
                    BatConfig::Real => 100,
                    BatConfig::Fake { max_capacity } => max_capacity,
                };
                std::cmp::min(max_capacity, self.state.lock().capacity)
            }
            BATTERY_VOLTAGE => self.state.lock().voltage,
            BATTERY_TEMP => 0,
            BATTERY_CHARGE_COUNTER => self.state.lock().charge_counter,
            BATTERY_VOLTAGE_MAX => 0,
            BATTERY_CURRENT_MAX => 0,
            BATTERY_CURRENT_NOW => self.state.lock().current,
            BATTERY_CURRENT_AVG => 0,
            BATTERY_CHARGE_FULL_UAH => self.state.lock().charge_full,
            BATTERY_CYCLE_COUNT => 0,
            _ => {
                warn!("{}: unsupported read address {}", self.debug_label(), info);
                return;
            }
        };

        let val_arr = val.to_ne_bytes();
        data.copy_from_slice(&val_arr);
    }

    fn write(&mut self, info: BusAccessInfo, data: &[u8]) {
        if data.len() != std::mem::size_of::<u32>() {
            warn!(
                "{}: unsupported write length {}, only support 4bytes write",
                self.debug_label(),
                data.len()
            );
            return;
        }

        let mut val_arr = u32::to_ne_bytes(0u32);
        val_arr.copy_from_slice(data);
        let val = u32::from_ne_bytes(val_arr);

        match info.offset as u32 {
            BATTERY_INT_ENABLE => {
                self.state.lock().int_enable = val;
                if (val & BATTERY_INT_MASK) != 0 && !self.activated {
                    self.start_monitor();
                }
            }
            _ => {
                warn!("{}: Bad write to address {}", self.debug_label(), info);
            }
        };
    }
}

impl Aml for GoldfishBattery {
    fn to_aml_bytes(&self, bytes: &mut Vec<u8>) {
        aml::Device::new(
            "GFBY".into(),
            vec![
                &aml::Name::new("_HID".into(), &"GFSH0001"),
                &aml::Name::new(
                    "_CRS".into(),
                    &aml::ResourceTemplate::new(vec![
                        &aml::Memory32Fixed::new(true, self.mmio_base, GOLDFISHBAT_MMIO_LEN as u32),
                        &aml::Interrupt::new(true, false, false, true, self.irq_num),
                    ]),
                ),
            ],
        )
        .to_aml_bytes(bytes);
    }
}

impl Suspendable for GoldfishBattery {
    fn sleep(&mut self) -> anyhow::Result<()> {
        if let Some(thread) = self.monitor_thread.take() {
            thread.stop();
        }
        Ok(())
    }

    fn wake(&mut self) -> anyhow::Result<()> {
        if self.activated {
            // Set activated to false for start_monitor to start monitoring again.
            self.activated = false;
            self.start_monitor();
        }
        Ok(())
    }

    fn snapshot(&mut self) -> anyhow::Result<AnySnapshot> {
        AnySnapshot::to_any(GoldfishBatterySnapshot {
            state: self.state.lock().clone(),
            mmio_base: self.mmio_base,
            irq_num: self.irq_num,
            activated: self.activated,
        })
        .context("failed to snapshot GoldfishBattery")
    }

    fn restore(&mut self, data: AnySnapshot) -> anyhow::Result<()> {
        let deser: GoldfishBatterySnapshot =
            AnySnapshot::from_any(data).context("failed to deserialize GoldfishBattery")?;
        {
            let mut locked_state = self.state.lock();
            *locked_state = deser.state;
        }
        self.mmio_base = deser.mmio_base;
        self.irq_num = deser.irq_num;
        self.activated = deser.activated;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use crate::suspendable_tests;

    fn modify_device(battery: &mut GoldfishBattery) {
        let mut state = battery.state.lock();
        state.set_capacity(70);
    }

    suspendable_tests! {
        battery, GoldfishBattery::new(
            0,
            0,
            IrqLevelEvent::new().unwrap(),
            Tube::pair().unwrap().1,
            None,
            None,
        ).unwrap(),
        modify_device
    }

    // Mock power client for testing rate limiting.
    struct MockPowerClient {
        last_request_time: Option<SystemTime>,
    }

    impl MockPowerClient {
        fn new(last_request_time: Option<SystemTime>) -> Self {
            Self { last_request_time }
        }
    }

    impl power_monitor::PowerClient for MockPowerClient {
        fn last_request_timestamp(&self) -> Option<SystemTime> {
            self.last_request_time
        }

        fn get_power_data(
            &mut self,
        ) -> std::result::Result<power_monitor::PowerData, Box<dyn std::error::Error>> {
            self.last_request_time = Some(SystemTime::now());
            Ok(power_monitor::PowerData {
                ac_online: true,
                battery: Some(power_monitor::BatteryData {
                    percent: 50,
                    status: power_monitor::BatteryStatus::Unknown,
                    voltage: 0,
                    current: 0,
                    charge_counter: 0,
                    charge_full: 0,
                }),
            })
        }
    }

    fn create_mock_power_client(last_request_time: Option<SystemTime>) -> MockPowerClient {
        MockPowerClient::new(last_request_time)
    }

    #[test]
    fn test_initialize_battery_state_rate_limiting() {
        let mmio_base = 0;
        let irq_num = 0;
        let irq_evt = IrqLevelEvent::new().unwrap();
        let tube = Tube::pair().unwrap().1;

        let now = SystemTime::now();
        let recent_time =
            now - Duration::from_millis(GoldfishBattery::POWERD_REQ_INTERVAL_MS - 500);

        let mut battery = GoldfishBattery::new(
            mmio_base,
            irq_num,
            irq_evt,
            tube,
            None,
            Some(Box::new(move || {
                Ok(Box::new(create_mock_power_client(Some(recent_time))))
            })),
        )
        .unwrap();

        assert!(matches!(battery.initialize_battery_state(), Ok(())));

        // First initialization status should be pending due to rate limiting
        assert!(matches!(
            *battery.init_state.lock(),
            BatInitializationState::Pending(_)
        ));

        *battery.init_state.lock() = BatInitializationState::NotYet;
        let old_time = now - Duration::from_millis(GoldfishBattery::POWERD_REQ_INTERVAL_MS + 500);
        // Replace the factory function to simulate an older last_request_time
        battery.create_powerd_client = Some(Box::new(move || {
            Ok(Box::new(create_mock_power_client(Some(old_time))))
        }));

        // Second call with old time should succeed.
        assert!(matches!(battery.initialize_battery_state(), Ok(())));

        assert!(matches!(
            *battery.init_state.lock(),
            BatInitializationState::Done,
        ));

        // Check if values are correctly updated.
        let state = battery.state.lock();
        assert_eq!(state.ac_online, 1);
        assert_eq!(state.capacity, 50);
    }

    #[test]
    fn test_initialize_battery_state_no_powerd_client() {
        let mmio_base = 0;
        let irq_num = 0;
        let irq_evt = IrqLevelEvent::new().unwrap();
        let tube = Tube::pair().unwrap().1;

        let mut battery =
            GoldfishBattery::new(mmio_base, irq_num, irq_evt, tube, None, None).unwrap();

        assert!(matches!(battery.initialize_battery_state(), Ok(())));
        assert!(matches!(
            *battery.init_state.lock(),
            BatInitializationState::NotYet,
        ));
    }
}
