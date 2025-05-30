// Copyright 2025 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! AArch64 system register range constants.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_snake_case, non_upper_case_globals)]

use crate::AArch64SysRegId;
use crate::funcs::*;

pub const AMEVCNTR00_EL0: AArch64SysRegId = AMEVCNTR0n_EL0(0);
pub const AMEVCNTR01_EL0: AArch64SysRegId = AMEVCNTR0n_EL0(1);
pub const AMEVCNTR02_EL0: AArch64SysRegId = AMEVCNTR0n_EL0(2);
pub const AMEVCNTR03_EL0: AArch64SysRegId = AMEVCNTR0n_EL0(3);

pub const AMEVCNTR10_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(0);
pub const AMEVCNTR11_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(1);
pub const AMEVCNTR12_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(2);
pub const AMEVCNTR13_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(3);
pub const AMEVCNTR14_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(4);
pub const AMEVCNTR15_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(5);
pub const AMEVCNTR16_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(6);
pub const AMEVCNTR17_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(7);
pub const AMEVCNTR18_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(8);
pub const AMEVCNTR19_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(9);
pub const AMEVCNTR1A_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(10);
pub const AMEVCNTR1B_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(11);
pub const AMEVCNTR1C_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(12);
pub const AMEVCNTR1D_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(13);
pub const AMEVCNTR1E_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(14);
pub const AMEVCNTR1F_EL0: AArch64SysRegId = AMEVCNTR1n_EL0(15);

pub const AMEVCNTVOFF00_EL2: AArch64SysRegId = AMEVCNTVOFF0n_EL2(0);
pub const AMEVCNTVOFF02_EL2: AArch64SysRegId = AMEVCNTVOFF0n_EL2(2);
pub const AMEVCNTVOFF03_EL2: AArch64SysRegId = AMEVCNTVOFF0n_EL2(3);

pub const AMEVCNTVOFF10_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(0);
pub const AMEVCNTVOFF11_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(1);
pub const AMEVCNTVOFF12_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(2);
pub const AMEVCNTVOFF13_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(3);
pub const AMEVCNTVOFF14_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(4);
pub const AMEVCNTVOFF15_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(5);
pub const AMEVCNTVOFF16_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(6);
pub const AMEVCNTVOFF17_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(7);
pub const AMEVCNTVOFF18_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(8);
pub const AMEVCNTVOFF19_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(9);
pub const AMEVCNTVOFF1A_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(10);
pub const AMEVCNTVOFF1B_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(11);
pub const AMEVCNTVOFF1C_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(12);
pub const AMEVCNTVOFF1D_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(13);
pub const AMEVCNTVOFF1E_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(14);
pub const AMEVCNTVOFF1F_EL2: AArch64SysRegId = AMEVCNTVOFF1n_EL2(15);

pub const AMEVTYPER00_EL0: AArch64SysRegId = AMEVTYPER0n_EL0(0);
pub const AMEVTYPER01_EL0: AArch64SysRegId = AMEVTYPER0n_EL0(1);
pub const AMEVTYPER02_EL0: AArch64SysRegId = AMEVTYPER0n_EL0(2);
pub const AMEVTYPER03_EL0: AArch64SysRegId = AMEVTYPER0n_EL0(3);

pub const AMEVTYPER10_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(0);
pub const AMEVTYPER11_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(1);
pub const AMEVTYPER12_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(2);
pub const AMEVTYPER13_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(3);
pub const AMEVTYPER14_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(4);
pub const AMEVTYPER15_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(5);
pub const AMEVTYPER16_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(6);
pub const AMEVTYPER17_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(7);
pub const AMEVTYPER18_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(8);
pub const AMEVTYPER19_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(9);
pub const AMEVTYPER1A_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(10);
pub const AMEVTYPER1B_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(11);
pub const AMEVTYPER1C_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(12);
pub const AMEVTYPER1D_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(13);
pub const AMEVTYPER1E_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(14);
pub const AMEVTYPER1F_EL0: AArch64SysRegId = AMEVTYPER1n_EL0(15);

pub const BRBINF0_EL1: AArch64SysRegId = BRBINFn_EL1(0);
pub const BRBINF1_EL1: AArch64SysRegId = BRBINFn_EL1(1);
pub const BRBINF2_EL1: AArch64SysRegId = BRBINFn_EL1(2);
pub const BRBINF3_EL1: AArch64SysRegId = BRBINFn_EL1(3);
pub const BRBINF4_EL1: AArch64SysRegId = BRBINFn_EL1(4);
pub const BRBINF5_EL1: AArch64SysRegId = BRBINFn_EL1(5);
pub const BRBINF6_EL1: AArch64SysRegId = BRBINFn_EL1(6);
pub const BRBINF7_EL1: AArch64SysRegId = BRBINFn_EL1(7);
pub const BRBINF8_EL1: AArch64SysRegId = BRBINFn_EL1(8);
pub const BRBINF9_EL1: AArch64SysRegId = BRBINFn_EL1(9);
pub const BRBINF10_EL1: AArch64SysRegId = BRBINFn_EL1(10);
pub const BRBINF11_EL1: AArch64SysRegId = BRBINFn_EL1(11);
pub const BRBINF12_EL1: AArch64SysRegId = BRBINFn_EL1(12);
pub const BRBINF13_EL1: AArch64SysRegId = BRBINFn_EL1(13);
pub const BRBINF14_EL1: AArch64SysRegId = BRBINFn_EL1(14);
pub const BRBINF15_EL1: AArch64SysRegId = BRBINFn_EL1(15);
pub const BRBINF16_EL1: AArch64SysRegId = BRBINFn_EL1(16);
pub const BRBINF17_EL1: AArch64SysRegId = BRBINFn_EL1(17);
pub const BRBINF18_EL1: AArch64SysRegId = BRBINFn_EL1(18);
pub const BRBINF19_EL1: AArch64SysRegId = BRBINFn_EL1(19);
pub const BRBINF20_EL1: AArch64SysRegId = BRBINFn_EL1(20);
pub const BRBINF21_EL1: AArch64SysRegId = BRBINFn_EL1(21);
pub const BRBINF22_EL1: AArch64SysRegId = BRBINFn_EL1(22);
pub const BRBINF23_EL1: AArch64SysRegId = BRBINFn_EL1(23);
pub const BRBINF24_EL1: AArch64SysRegId = BRBINFn_EL1(24);
pub const BRBINF25_EL1: AArch64SysRegId = BRBINFn_EL1(25);
pub const BRBINF26_EL1: AArch64SysRegId = BRBINFn_EL1(26);
pub const BRBINF27_EL1: AArch64SysRegId = BRBINFn_EL1(27);
pub const BRBINF28_EL1: AArch64SysRegId = BRBINFn_EL1(28);
pub const BRBINF29_EL1: AArch64SysRegId = BRBINFn_EL1(29);
pub const BRBINF30_EL1: AArch64SysRegId = BRBINFn_EL1(30);
pub const BRBINF31_EL1: AArch64SysRegId = BRBINFn_EL1(31);

pub const BRBSRC0_EL1: AArch64SysRegId = BRBSRCn_EL1(0);
pub const BRBSRC1_EL1: AArch64SysRegId = BRBSRCn_EL1(1);
pub const BRBSRC2_EL1: AArch64SysRegId = BRBSRCn_EL1(2);
pub const BRBSRC3_EL1: AArch64SysRegId = BRBSRCn_EL1(3);
pub const BRBSRC4_EL1: AArch64SysRegId = BRBSRCn_EL1(4);
pub const BRBSRC5_EL1: AArch64SysRegId = BRBSRCn_EL1(5);
pub const BRBSRC6_EL1: AArch64SysRegId = BRBSRCn_EL1(6);
pub const BRBSRC7_EL1: AArch64SysRegId = BRBSRCn_EL1(7);
pub const BRBSRC8_EL1: AArch64SysRegId = BRBSRCn_EL1(8);
pub const BRBSRC9_EL1: AArch64SysRegId = BRBSRCn_EL1(9);
pub const BRBSRC10_EL1: AArch64SysRegId = BRBSRCn_EL1(10);
pub const BRBSRC11_EL1: AArch64SysRegId = BRBSRCn_EL1(11);
pub const BRBSRC12_EL1: AArch64SysRegId = BRBSRCn_EL1(12);
pub const BRBSRC13_EL1: AArch64SysRegId = BRBSRCn_EL1(13);
pub const BRBSRC14_EL1: AArch64SysRegId = BRBSRCn_EL1(14);
pub const BRBSRC15_EL1: AArch64SysRegId = BRBSRCn_EL1(15);
pub const BRBSRC16_EL1: AArch64SysRegId = BRBSRCn_EL1(16);
pub const BRBSRC17_EL1: AArch64SysRegId = BRBSRCn_EL1(17);
pub const BRBSRC18_EL1: AArch64SysRegId = BRBSRCn_EL1(18);
pub const BRBSRC19_EL1: AArch64SysRegId = BRBSRCn_EL1(19);
pub const BRBSRC20_EL1: AArch64SysRegId = BRBSRCn_EL1(20);
pub const BRBSRC21_EL1: AArch64SysRegId = BRBSRCn_EL1(21);
pub const BRBSRC22_EL1: AArch64SysRegId = BRBSRCn_EL1(22);
pub const BRBSRC23_EL1: AArch64SysRegId = BRBSRCn_EL1(23);
pub const BRBSRC24_EL1: AArch64SysRegId = BRBSRCn_EL1(24);
pub const BRBSRC25_EL1: AArch64SysRegId = BRBSRCn_EL1(25);
pub const BRBSRC26_EL1: AArch64SysRegId = BRBSRCn_EL1(26);
pub const BRBSRC27_EL1: AArch64SysRegId = BRBSRCn_EL1(27);
pub const BRBSRC28_EL1: AArch64SysRegId = BRBSRCn_EL1(28);
pub const BRBSRC29_EL1: AArch64SysRegId = BRBSRCn_EL1(29);
pub const BRBSRC30_EL1: AArch64SysRegId = BRBSRCn_EL1(30);
pub const BRBSRC31_EL1: AArch64SysRegId = BRBSRCn_EL1(31);

pub const BRBTGT0_EL1: AArch64SysRegId = BRBTGTn_EL1(0);
pub const BRBTGT1_EL1: AArch64SysRegId = BRBTGTn_EL1(1);
pub const BRBTGT2_EL1: AArch64SysRegId = BRBTGTn_EL1(2);
pub const BRBTGT3_EL1: AArch64SysRegId = BRBTGTn_EL1(3);
pub const BRBTGT4_EL1: AArch64SysRegId = BRBTGTn_EL1(4);
pub const BRBTGT5_EL1: AArch64SysRegId = BRBTGTn_EL1(5);
pub const BRBTGT6_EL1: AArch64SysRegId = BRBTGTn_EL1(6);
pub const BRBTGT7_EL1: AArch64SysRegId = BRBTGTn_EL1(7);
pub const BRBTGT8_EL1: AArch64SysRegId = BRBTGTn_EL1(8);
pub const BRBTGT9_EL1: AArch64SysRegId = BRBTGTn_EL1(9);
pub const BRBTGT10_EL1: AArch64SysRegId = BRBTGTn_EL1(10);
pub const BRBTGT11_EL1: AArch64SysRegId = BRBTGTn_EL1(11);
pub const BRBTGT12_EL1: AArch64SysRegId = BRBTGTn_EL1(12);
pub const BRBTGT13_EL1: AArch64SysRegId = BRBTGTn_EL1(13);
pub const BRBTGT14_EL1: AArch64SysRegId = BRBTGTn_EL1(14);
pub const BRBTGT15_EL1: AArch64SysRegId = BRBTGTn_EL1(15);
pub const BRBTGT16_EL1: AArch64SysRegId = BRBTGTn_EL1(16);
pub const BRBTGT17_EL1: AArch64SysRegId = BRBTGTn_EL1(17);
pub const BRBTGT18_EL1: AArch64SysRegId = BRBTGTn_EL1(18);
pub const BRBTGT19_EL1: AArch64SysRegId = BRBTGTn_EL1(19);
pub const BRBTGT20_EL1: AArch64SysRegId = BRBTGTn_EL1(20);
pub const BRBTGT21_EL1: AArch64SysRegId = BRBTGTn_EL1(21);
pub const BRBTGT22_EL1: AArch64SysRegId = BRBTGTn_EL1(22);
pub const BRBTGT23_EL1: AArch64SysRegId = BRBTGTn_EL1(23);
pub const BRBTGT24_EL1: AArch64SysRegId = BRBTGTn_EL1(24);
pub const BRBTGT25_EL1: AArch64SysRegId = BRBTGTn_EL1(25);
pub const BRBTGT26_EL1: AArch64SysRegId = BRBTGTn_EL1(26);
pub const BRBTGT27_EL1: AArch64SysRegId = BRBTGTn_EL1(27);
pub const BRBTGT28_EL1: AArch64SysRegId = BRBTGTn_EL1(28);
pub const BRBTGT29_EL1: AArch64SysRegId = BRBTGTn_EL1(29);
pub const BRBTGT30_EL1: AArch64SysRegId = BRBTGTn_EL1(30);
pub const BRBTGT31_EL1: AArch64SysRegId = BRBTGTn_EL1(31);

pub const DBGBCR0_EL1: AArch64SysRegId = DBGBCRn_EL1(0);
pub const DBGBCR1_EL1: AArch64SysRegId = DBGBCRn_EL1(1);
pub const DBGBCR2_EL1: AArch64SysRegId = DBGBCRn_EL1(2);
pub const DBGBCR3_EL1: AArch64SysRegId = DBGBCRn_EL1(3);
pub const DBGBCR4_EL1: AArch64SysRegId = DBGBCRn_EL1(4);
pub const DBGBCR5_EL1: AArch64SysRegId = DBGBCRn_EL1(5);
pub const DBGBCR6_EL1: AArch64SysRegId = DBGBCRn_EL1(6);
pub const DBGBCR7_EL1: AArch64SysRegId = DBGBCRn_EL1(7);
pub const DBGBCR8_EL1: AArch64SysRegId = DBGBCRn_EL1(8);
pub const DBGBCR9_EL1: AArch64SysRegId = DBGBCRn_EL1(9);
pub const DBGBCRA_EL1: AArch64SysRegId = DBGBCRn_EL1(10);
pub const DBGBCRB_EL1: AArch64SysRegId = DBGBCRn_EL1(11);
pub const DBGBCRC_EL1: AArch64SysRegId = DBGBCRn_EL1(12);
pub const DBGBCRD_EL1: AArch64SysRegId = DBGBCRn_EL1(13);
pub const DBGBCRE_EL1: AArch64SysRegId = DBGBCRn_EL1(14);
pub const DBGBCRF_EL1: AArch64SysRegId = DBGBCRn_EL1(15);

pub const DBGBVR0_EL1: AArch64SysRegId = DBGBVRn_EL1(0);
pub const DBGBVR1_EL1: AArch64SysRegId = DBGBVRn_EL1(1);
pub const DBGBVR2_EL1: AArch64SysRegId = DBGBVRn_EL1(2);
pub const DBGBVR3_EL1: AArch64SysRegId = DBGBVRn_EL1(3);
pub const DBGBVR4_EL1: AArch64SysRegId = DBGBVRn_EL1(4);
pub const DBGBVR5_EL1: AArch64SysRegId = DBGBVRn_EL1(5);
pub const DBGBVR6_EL1: AArch64SysRegId = DBGBVRn_EL1(6);
pub const DBGBVR7_EL1: AArch64SysRegId = DBGBVRn_EL1(7);
pub const DBGBVR8_EL1: AArch64SysRegId = DBGBVRn_EL1(8);
pub const DBGBVR9_EL1: AArch64SysRegId = DBGBVRn_EL1(9);
pub const DBGBVRA_EL1: AArch64SysRegId = DBGBVRn_EL1(10);
pub const DBGBVRB_EL1: AArch64SysRegId = DBGBVRn_EL1(11);
pub const DBGBVRC_EL1: AArch64SysRegId = DBGBVRn_EL1(12);
pub const DBGBVRD_EL1: AArch64SysRegId = DBGBVRn_EL1(13);
pub const DBGBVRE_EL1: AArch64SysRegId = DBGBVRn_EL1(14);
pub const DBGBVRF_EL1: AArch64SysRegId = DBGBVRn_EL1(15);

pub const DBGWCR0_EL1: AArch64SysRegId = DBGWCRn_EL1(0);
pub const DBGWCR1_EL1: AArch64SysRegId = DBGWCRn_EL1(1);
pub const DBGWCR2_EL1: AArch64SysRegId = DBGWCRn_EL1(2);
pub const DBGWCR3_EL1: AArch64SysRegId = DBGWCRn_EL1(3);
pub const DBGWCR4_EL1: AArch64SysRegId = DBGWCRn_EL1(4);
pub const DBGWCR5_EL1: AArch64SysRegId = DBGWCRn_EL1(5);
pub const DBGWCR6_EL1: AArch64SysRegId = DBGWCRn_EL1(6);
pub const DBGWCR7_EL1: AArch64SysRegId = DBGWCRn_EL1(7);
pub const DBGWCR8_EL1: AArch64SysRegId = DBGWCRn_EL1(8);
pub const DBGWCR9_EL1: AArch64SysRegId = DBGWCRn_EL1(9);
pub const DBGWCRA_EL1: AArch64SysRegId = DBGWCRn_EL1(10);
pub const DBGWCRB_EL1: AArch64SysRegId = DBGWCRn_EL1(11);
pub const DBGWCRC_EL1: AArch64SysRegId = DBGWCRn_EL1(12);
pub const DBGWCRD_EL1: AArch64SysRegId = DBGWCRn_EL1(13);
pub const DBGWCRE_EL1: AArch64SysRegId = DBGWCRn_EL1(14);
pub const DBGWCRF_EL1: AArch64SysRegId = DBGWCRn_EL1(15);

pub const DBGWVR0_EL1: AArch64SysRegId = DBGWVRn_EL1(0);
pub const DBGWVR1_EL1: AArch64SysRegId = DBGWVRn_EL1(1);
pub const DBGWVR2_EL1: AArch64SysRegId = DBGWVRn_EL1(2);
pub const DBGWVR3_EL1: AArch64SysRegId = DBGWVRn_EL1(3);
pub const DBGWVR4_EL1: AArch64SysRegId = DBGWVRn_EL1(4);
pub const DBGWVR5_EL1: AArch64SysRegId = DBGWVRn_EL1(5);
pub const DBGWVR6_EL1: AArch64SysRegId = DBGWVRn_EL1(6);
pub const DBGWVR7_EL1: AArch64SysRegId = DBGWVRn_EL1(7);
pub const DBGWVR8_EL1: AArch64SysRegId = DBGWVRn_EL1(8);
pub const DBGWVR9_EL1: AArch64SysRegId = DBGWVRn_EL1(9);
pub const DBGWVRA_EL1: AArch64SysRegId = DBGWVRn_EL1(10);
pub const DBGWVRB_EL1: AArch64SysRegId = DBGWVRn_EL1(11);
pub const DBGWVRC_EL1: AArch64SysRegId = DBGWVRn_EL1(12);
pub const DBGWVRD_EL1: AArch64SysRegId = DBGWVRn_EL1(13);
pub const DBGWVRE_EL1: AArch64SysRegId = DBGWVRn_EL1(14);
pub const DBGWVRF_EL1: AArch64SysRegId = DBGWVRn_EL1(15);

pub const ICC_AP0R0_EL1: AArch64SysRegId = ICC_AP0Rn_EL1(0);
pub const ICC_AP0R1_EL1: AArch64SysRegId = ICC_AP0Rn_EL1(1);
pub const ICC_AP0R2_EL1: AArch64SysRegId = ICC_AP0Rn_EL1(2);
pub const ICC_AP0R3_EL1: AArch64SysRegId = ICC_AP0Rn_EL1(3);

pub const ICC_AP1R0_EL1: AArch64SysRegId = ICC_AP1Rn_EL1(0);
pub const ICC_AP1R1_EL1: AArch64SysRegId = ICC_AP1Rn_EL1(1);
pub const ICC_AP1R2_EL1: AArch64SysRegId = ICC_AP1Rn_EL1(2);
pub const ICC_AP1R3_EL1: AArch64SysRegId = ICC_AP1Rn_EL1(3);

pub const ICH_AP0R0_EL2: AArch64SysRegId = ICH_AP0Rn_EL2(0);
pub const ICH_AP0R1_EL2: AArch64SysRegId = ICH_AP0Rn_EL2(1);
pub const ICH_AP0R2_EL2: AArch64SysRegId = ICH_AP0Rn_EL2(2);
pub const ICH_AP0R3_EL2: AArch64SysRegId = ICH_AP0Rn_EL2(3);

pub const ICH_AP1R0_EL2: AArch64SysRegId = ICH_AP1Rn_EL2(0);
pub const ICH_AP1R1_EL2: AArch64SysRegId = ICH_AP1Rn_EL2(1);
pub const ICH_AP1R2_EL2: AArch64SysRegId = ICH_AP1Rn_EL2(2);
pub const ICH_AP1R3_EL2: AArch64SysRegId = ICH_AP1Rn_EL2(3);

pub const ICH_LR0_EL2: AArch64SysRegId = ICH_LRn_EL2(0);
pub const ICH_LR1_EL2: AArch64SysRegId = ICH_LRn_EL2(1);
pub const ICH_LR2_EL2: AArch64SysRegId = ICH_LRn_EL2(2);
pub const ICH_LR3_EL2: AArch64SysRegId = ICH_LRn_EL2(3);
pub const ICH_LR4_EL2: AArch64SysRegId = ICH_LRn_EL2(4);
pub const ICH_LR5_EL2: AArch64SysRegId = ICH_LRn_EL2(5);
pub const ICH_LR6_EL2: AArch64SysRegId = ICH_LRn_EL2(6);
pub const ICH_LR7_EL2: AArch64SysRegId = ICH_LRn_EL2(7);
pub const ICH_LR8_EL2: AArch64SysRegId = ICH_LRn_EL2(8);
pub const ICH_LR9_EL2: AArch64SysRegId = ICH_LRn_EL2(9);
pub const ICH_LRA_EL2: AArch64SysRegId = ICH_LRn_EL2(10);
pub const ICH_LRB_EL2: AArch64SysRegId = ICH_LRn_EL2(11);
pub const ICH_LRC_EL2: AArch64SysRegId = ICH_LRn_EL2(12);
pub const ICH_LRD_EL2: AArch64SysRegId = ICH_LRn_EL2(13);
pub const ICH_LRE_EL2: AArch64SysRegId = ICH_LRn_EL2(14);
pub const ICH_LRF_EL2: AArch64SysRegId = ICH_LRn_EL2(15);

pub const PMEVCNTR0_EL0: AArch64SysRegId = PMEVCNTRn_EL0(0);
pub const PMEVCNTR1_EL0: AArch64SysRegId = PMEVCNTRn_EL0(1);
pub const PMEVCNTR2_EL0: AArch64SysRegId = PMEVCNTRn_EL0(2);
pub const PMEVCNTR3_EL0: AArch64SysRegId = PMEVCNTRn_EL0(3);
pub const PMEVCNTR4_EL0: AArch64SysRegId = PMEVCNTRn_EL0(4);
pub const PMEVCNTR5_EL0: AArch64SysRegId = PMEVCNTRn_EL0(5);
pub const PMEVCNTR6_EL0: AArch64SysRegId = PMEVCNTRn_EL0(6);
pub const PMEVCNTR7_EL0: AArch64SysRegId = PMEVCNTRn_EL0(7);
pub const PMEVCNTR8_EL0: AArch64SysRegId = PMEVCNTRn_EL0(8);
pub const PMEVCNTR9_EL0: AArch64SysRegId = PMEVCNTRn_EL0(9);
pub const PMEVCNTR10_EL0: AArch64SysRegId = PMEVCNTRn_EL0(10);
pub const PMEVCNTR11_EL0: AArch64SysRegId = PMEVCNTRn_EL0(11);
pub const PMEVCNTR12_EL0: AArch64SysRegId = PMEVCNTRn_EL0(12);
pub const PMEVCNTR13_EL0: AArch64SysRegId = PMEVCNTRn_EL0(13);
pub const PMEVCNTR14_EL0: AArch64SysRegId = PMEVCNTRn_EL0(14);
pub const PMEVCNTR15_EL0: AArch64SysRegId = PMEVCNTRn_EL0(15);
pub const PMEVCNTR16_EL0: AArch64SysRegId = PMEVCNTRn_EL0(16);
pub const PMEVCNTR17_EL0: AArch64SysRegId = PMEVCNTRn_EL0(17);
pub const PMEVCNTR18_EL0: AArch64SysRegId = PMEVCNTRn_EL0(18);
pub const PMEVCNTR19_EL0: AArch64SysRegId = PMEVCNTRn_EL0(19);
pub const PMEVCNTR20_EL0: AArch64SysRegId = PMEVCNTRn_EL0(20);
pub const PMEVCNTR21_EL0: AArch64SysRegId = PMEVCNTRn_EL0(21);
pub const PMEVCNTR22_EL0: AArch64SysRegId = PMEVCNTRn_EL0(22);
pub const PMEVCNTR23_EL0: AArch64SysRegId = PMEVCNTRn_EL0(23);
pub const PMEVCNTR24_EL0: AArch64SysRegId = PMEVCNTRn_EL0(24);
pub const PMEVCNTR25_EL0: AArch64SysRegId = PMEVCNTRn_EL0(25);
pub const PMEVCNTR26_EL0: AArch64SysRegId = PMEVCNTRn_EL0(26);
pub const PMEVCNTR27_EL0: AArch64SysRegId = PMEVCNTRn_EL0(27);
pub const PMEVCNTR28_EL0: AArch64SysRegId = PMEVCNTRn_EL0(28);
pub const PMEVCNTR29_EL0: AArch64SysRegId = PMEVCNTRn_EL0(29);
pub const PMEVCNTR30_EL0: AArch64SysRegId = PMEVCNTRn_EL0(30);

pub const PMEVCNTSVR0_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(0);
pub const PMEVCNTSVR1_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(1);
pub const PMEVCNTSVR2_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(2);
pub const PMEVCNTSVR3_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(3);
pub const PMEVCNTSVR4_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(4);
pub const PMEVCNTSVR5_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(5);
pub const PMEVCNTSVR6_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(6);
pub const PMEVCNTSVR7_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(7);
pub const PMEVCNTSVR8_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(8);
pub const PMEVCNTSVR9_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(9);
pub const PMEVCNTSVR10_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(10);
pub const PMEVCNTSVR11_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(11);
pub const PMEVCNTSVR12_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(12);
pub const PMEVCNTSVR13_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(13);
pub const PMEVCNTSVR14_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(14);
pub const PMEVCNTSVR15_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(15);
pub const PMEVCNTSVR16_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(16);
pub const PMEVCNTSVR17_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(17);
pub const PMEVCNTSVR18_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(18);
pub const PMEVCNTSVR19_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(19);
pub const PMEVCNTSVR20_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(20);
pub const PMEVCNTSVR21_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(21);
pub const PMEVCNTSVR22_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(22);
pub const PMEVCNTSVR23_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(23);
pub const PMEVCNTSVR24_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(24);
pub const PMEVCNTSVR25_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(25);
pub const PMEVCNTSVR26_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(26);
pub const PMEVCNTSVR27_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(27);
pub const PMEVCNTSVR28_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(28);
pub const PMEVCNTSVR29_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(29);
pub const PMEVCNTSVR30_EL1: AArch64SysRegId = PMEVCNTSVRn_EL1(30);

pub const PMEVTYPER0_EL0: AArch64SysRegId = PMEVTYPERn_EL0(0);
pub const PMEVTYPER1_EL0: AArch64SysRegId = PMEVTYPERn_EL0(1);
pub const PMEVTYPER2_EL0: AArch64SysRegId = PMEVTYPERn_EL0(2);
pub const PMEVTYPER3_EL0: AArch64SysRegId = PMEVTYPERn_EL0(3);
pub const PMEVTYPER4_EL0: AArch64SysRegId = PMEVTYPERn_EL0(4);
pub const PMEVTYPER5_EL0: AArch64SysRegId = PMEVTYPERn_EL0(5);
pub const PMEVTYPER6_EL0: AArch64SysRegId = PMEVTYPERn_EL0(6);
pub const PMEVTYPER7_EL0: AArch64SysRegId = PMEVTYPERn_EL0(7);
pub const PMEVTYPER8_EL0: AArch64SysRegId = PMEVTYPERn_EL0(8);
pub const PMEVTYPER9_EL0: AArch64SysRegId = PMEVTYPERn_EL0(9);
pub const PMEVTYPER10_EL0: AArch64SysRegId = PMEVTYPERn_EL0(10);
pub const PMEVTYPER11_EL0: AArch64SysRegId = PMEVTYPERn_EL0(11);
pub const PMEVTYPER12_EL0: AArch64SysRegId = PMEVTYPERn_EL0(12);
pub const PMEVTYPER13_EL0: AArch64SysRegId = PMEVTYPERn_EL0(13);
pub const PMEVTYPER14_EL0: AArch64SysRegId = PMEVTYPERn_EL0(14);
pub const PMEVTYPER15_EL0: AArch64SysRegId = PMEVTYPERn_EL0(15);
pub const PMEVTYPER16_EL0: AArch64SysRegId = PMEVTYPERn_EL0(16);
pub const PMEVTYPER17_EL0: AArch64SysRegId = PMEVTYPERn_EL0(17);
pub const PMEVTYPER18_EL0: AArch64SysRegId = PMEVTYPERn_EL0(18);
pub const PMEVTYPER19_EL0: AArch64SysRegId = PMEVTYPERn_EL0(19);
pub const PMEVTYPER20_EL0: AArch64SysRegId = PMEVTYPERn_EL0(20);
pub const PMEVTYPER21_EL0: AArch64SysRegId = PMEVTYPERn_EL0(21);
pub const PMEVTYPER22_EL0: AArch64SysRegId = PMEVTYPERn_EL0(22);
pub const PMEVTYPER23_EL0: AArch64SysRegId = PMEVTYPERn_EL0(23);
pub const PMEVTYPER24_EL0: AArch64SysRegId = PMEVTYPERn_EL0(24);
pub const PMEVTYPER25_EL0: AArch64SysRegId = PMEVTYPERn_EL0(25);
pub const PMEVTYPER26_EL0: AArch64SysRegId = PMEVTYPERn_EL0(26);
pub const PMEVTYPER27_EL0: AArch64SysRegId = PMEVTYPERn_EL0(27);
pub const PMEVTYPER28_EL0: AArch64SysRegId = PMEVTYPERn_EL0(28);
pub const PMEVTYPER29_EL0: AArch64SysRegId = PMEVTYPERn_EL0(29);
pub const PMEVTYPER30_EL0: AArch64SysRegId = PMEVTYPERn_EL0(30);

pub const SPMCGCR0_EL1: AArch64SysRegId = SPMCGCRn_EL1(0);
pub const SPMCGCR1_EL1: AArch64SysRegId = SPMCGCRn_EL1(1);

pub const SPMEVCNTR0_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(0);
pub const SPMEVCNTR1_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(1);
pub const SPMEVCNTR2_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(2);
pub const SPMEVCNTR3_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(3);
pub const SPMEVCNTR4_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(4);
pub const SPMEVCNTR5_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(5);
pub const SPMEVCNTR6_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(6);
pub const SPMEVCNTR7_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(7);
pub const SPMEVCNTR8_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(8);
pub const SPMEVCNTR9_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(9);
pub const SPMEVCNTRA_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(10);
pub const SPMEVCNTRB_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(11);
pub const SPMEVCNTRC_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(12);
pub const SPMEVCNTRD_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(13);
pub const SPMEVCNTRE_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(14);
pub const SPMEVCNTRF_EL0: AArch64SysRegId = SPMEVCNTRn_EL0(15);

pub const SPMEVFILT2R0_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(0);
pub const SPMEVFILT2R1_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(1);
pub const SPMEVFILT2R2_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(2);
pub const SPMEVFILT2R3_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(3);
pub const SPMEVFILT2R4_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(4);
pub const SPMEVFILT2R5_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(5);
pub const SPMEVFILT2R6_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(6);
pub const SPMEVFILT2R7_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(7);
pub const SPMEVFILT2R8_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(8);
pub const SPMEVFILT2R9_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(9);
pub const SPMEVFILT2RA_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(10);
pub const SPMEVFILT2RB_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(11);
pub const SPMEVFILT2RC_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(12);
pub const SPMEVFILT2RD_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(13);
pub const SPMEVFILT2RE_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(14);
pub const SPMEVFILT2RF_EL0: AArch64SysRegId = SPMEVFILT2Rn_EL0(15);

pub const SPMEVFILTR0_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(0);
pub const SPMEVFILTR1_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(1);
pub const SPMEVFILTR2_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(2);
pub const SPMEVFILTR3_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(3);
pub const SPMEVFILTR4_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(4);
pub const SPMEVFILTR5_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(5);
pub const SPMEVFILTR6_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(6);
pub const SPMEVFILTR7_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(7);
pub const SPMEVFILTR8_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(8);
pub const SPMEVFILTR9_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(9);
pub const SPMEVFILTRA_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(10);
pub const SPMEVFILTRB_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(11);
pub const SPMEVFILTRC_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(12);
pub const SPMEVFILTRD_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(13);
pub const SPMEVFILTRE_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(14);
pub const SPMEVFILTRF_EL0: AArch64SysRegId = SPMEVFILTRn_EL0(15);

pub const SPMEVTYPER0_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(0);
pub const SPMEVTYPER1_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(1);
pub const SPMEVTYPER2_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(2);
pub const SPMEVTYPER3_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(3);
pub const SPMEVTYPER4_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(4);
pub const SPMEVTYPER5_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(5);
pub const SPMEVTYPER6_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(6);
pub const SPMEVTYPER7_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(7);
pub const SPMEVTYPER8_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(8);
pub const SPMEVTYPER9_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(9);
pub const SPMEVTYPERA_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(10);
pub const SPMEVTYPERB_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(11);
pub const SPMEVTYPERC_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(12);
pub const SPMEVTYPERD_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(13);
pub const SPMEVTYPERE_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(14);
pub const SPMEVTYPERF_EL0: AArch64SysRegId = SPMEVTYPERn_EL0(15);

pub const TRCACATR0: AArch64SysRegId = TRCACATRn(0);
pub const TRCACATR1: AArch64SysRegId = TRCACATRn(1);
pub const TRCACATR2: AArch64SysRegId = TRCACATRn(2);
pub const TRCACATR3: AArch64SysRegId = TRCACATRn(3);
pub const TRCACATR4: AArch64SysRegId = TRCACATRn(4);
pub const TRCACATR5: AArch64SysRegId = TRCACATRn(5);
pub const TRCACATR6: AArch64SysRegId = TRCACATRn(6);
pub const TRCACATR7: AArch64SysRegId = TRCACATRn(7);
pub const TRCACATR8: AArch64SysRegId = TRCACATRn(8);
pub const TRCACATR9: AArch64SysRegId = TRCACATRn(9);
pub const TRCACATRA: AArch64SysRegId = TRCACATRn(10);
pub const TRCACATRB: AArch64SysRegId = TRCACATRn(11);
pub const TRCACATRC: AArch64SysRegId = TRCACATRn(12);
pub const TRCACATRD: AArch64SysRegId = TRCACATRn(13);
pub const TRCACATRE: AArch64SysRegId = TRCACATRn(14);
pub const TRCACATRF: AArch64SysRegId = TRCACATRn(15);

pub const TRCACVR0: AArch64SysRegId = TRCACVRn(0);
pub const TRCACVR1: AArch64SysRegId = TRCACVRn(1);
pub const TRCACVR2: AArch64SysRegId = TRCACVRn(2);
pub const TRCACVR3: AArch64SysRegId = TRCACVRn(3);
pub const TRCACVR4: AArch64SysRegId = TRCACVRn(4);
pub const TRCACVR5: AArch64SysRegId = TRCACVRn(5);
pub const TRCACVR6: AArch64SysRegId = TRCACVRn(6);
pub const TRCACVR7: AArch64SysRegId = TRCACVRn(7);
pub const TRCACVR8: AArch64SysRegId = TRCACVRn(8);
pub const TRCACVR9: AArch64SysRegId = TRCACVRn(9);
pub const TRCACVRA: AArch64SysRegId = TRCACVRn(10);
pub const TRCACVRB: AArch64SysRegId = TRCACVRn(11);
pub const TRCACVRC: AArch64SysRegId = TRCACVRn(12);
pub const TRCACVRD: AArch64SysRegId = TRCACVRn(13);
pub const TRCACVRE: AArch64SysRegId = TRCACVRn(14);
pub const TRCACVRF: AArch64SysRegId = TRCACVRn(15);

pub const TRCCIDCVR0: AArch64SysRegId = TRCCIDCVRn(0);
pub const TRCCIDCVR1: AArch64SysRegId = TRCCIDCVRn(1);
pub const TRCCIDCVR2: AArch64SysRegId = TRCCIDCVRn(2);
pub const TRCCIDCVR3: AArch64SysRegId = TRCCIDCVRn(3);
pub const TRCCIDCVR4: AArch64SysRegId = TRCCIDCVRn(4);
pub const TRCCIDCVR5: AArch64SysRegId = TRCCIDCVRn(5);
pub const TRCCIDCVR6: AArch64SysRegId = TRCCIDCVRn(6);
pub const TRCCIDCVR7: AArch64SysRegId = TRCCIDCVRn(7);

pub const TRCCNTCTLR0: AArch64SysRegId = TRCCNTCTLRn(0);
pub const TRCCNTCTLR1: AArch64SysRegId = TRCCNTCTLRn(1);
pub const TRCCNTCTLR2: AArch64SysRegId = TRCCNTCTLRn(2);
pub const TRCCNTCTLR3: AArch64SysRegId = TRCCNTCTLRn(3);

pub const TRCCNTRLDVR0: AArch64SysRegId = TRCCNTRLDVRn(0);
pub const TRCCNTRLDVR1: AArch64SysRegId = TRCCNTRLDVRn(1);
pub const TRCCNTRLDVR2: AArch64SysRegId = TRCCNTRLDVRn(2);
pub const TRCCNTRLDVR3: AArch64SysRegId = TRCCNTRLDVRn(3);

pub const TRCCNTVR0: AArch64SysRegId = TRCCNTVRn(0);
pub const TRCCNTVR1: AArch64SysRegId = TRCCNTVRn(1);
pub const TRCCNTVR2: AArch64SysRegId = TRCCNTVRn(2);
pub const TRCCNTVR3: AArch64SysRegId = TRCCNTVRn(3);

pub const TRCEXTINSELR0: AArch64SysRegId = TRCEXTINSELRn(0);
pub const TRCEXTINSELR1: AArch64SysRegId = TRCEXTINSELRn(1);
pub const TRCEXTINSELR2: AArch64SysRegId = TRCEXTINSELRn(2);
pub const TRCEXTINSELR3: AArch64SysRegId = TRCEXTINSELRn(3);

pub const TRCIMSPEC1: AArch64SysRegId = TRCIMSPECn(1);
pub const TRCIMSPEC2: AArch64SysRegId = TRCIMSPECn(2);
pub const TRCIMSPEC3: AArch64SysRegId = TRCIMSPECn(3);
pub const TRCIMSPEC4: AArch64SysRegId = TRCIMSPECn(4);
pub const TRCIMSPEC5: AArch64SysRegId = TRCIMSPECn(5);
pub const TRCIMSPEC6: AArch64SysRegId = TRCIMSPECn(6);
pub const TRCIMSPEC7: AArch64SysRegId = TRCIMSPECn(7);

pub const TRCRSCTLR2: AArch64SysRegId = TRCRSCTLRn(2);
pub const TRCRSCTLR3: AArch64SysRegId = TRCRSCTLRn(3);
pub const TRCRSCTLR4: AArch64SysRegId = TRCRSCTLRn(4);
pub const TRCRSCTLR5: AArch64SysRegId = TRCRSCTLRn(5);
pub const TRCRSCTLR6: AArch64SysRegId = TRCRSCTLRn(6);
pub const TRCRSCTLR7: AArch64SysRegId = TRCRSCTLRn(7);
pub const TRCRSCTLR8: AArch64SysRegId = TRCRSCTLRn(8);
pub const TRCRSCTLR9: AArch64SysRegId = TRCRSCTLRn(9);
pub const TRCRSCTLR10: AArch64SysRegId = TRCRSCTLRn(10);
pub const TRCRSCTLR11: AArch64SysRegId = TRCRSCTLRn(11);
pub const TRCRSCTLR12: AArch64SysRegId = TRCRSCTLRn(12);
pub const TRCRSCTLR13: AArch64SysRegId = TRCRSCTLRn(13);
pub const TRCRSCTLR14: AArch64SysRegId = TRCRSCTLRn(14);
pub const TRCRSCTLR15: AArch64SysRegId = TRCRSCTLRn(15);
pub const TRCRSCTLR16: AArch64SysRegId = TRCRSCTLRn(16);
pub const TRCRSCTLR17: AArch64SysRegId = TRCRSCTLRn(17);
pub const TRCRSCTLR18: AArch64SysRegId = TRCRSCTLRn(18);
pub const TRCRSCTLR19: AArch64SysRegId = TRCRSCTLRn(19);
pub const TRCRSCTLR20: AArch64SysRegId = TRCRSCTLRn(20);
pub const TRCRSCTLR21: AArch64SysRegId = TRCRSCTLRn(21);
pub const TRCRSCTLR22: AArch64SysRegId = TRCRSCTLRn(22);
pub const TRCRSCTLR23: AArch64SysRegId = TRCRSCTLRn(23);
pub const TRCRSCTLR24: AArch64SysRegId = TRCRSCTLRn(24);
pub const TRCRSCTLR25: AArch64SysRegId = TRCRSCTLRn(25);
pub const TRCRSCTLR26: AArch64SysRegId = TRCRSCTLRn(26);
pub const TRCRSCTLR27: AArch64SysRegId = TRCRSCTLRn(27);
pub const TRCRSCTLR28: AArch64SysRegId = TRCRSCTLRn(28);
pub const TRCRSCTLR29: AArch64SysRegId = TRCRSCTLRn(29);
pub const TRCRSCTLR30: AArch64SysRegId = TRCRSCTLRn(30);
pub const TRCRSCTLR31: AArch64SysRegId = TRCRSCTLRn(31);

pub const TRCSEQEVR0: AArch64SysRegId = TRCSEQEVRn(0);
pub const TRCSEQEVR1: AArch64SysRegId = TRCSEQEVRn(1);
pub const TRCSEQEVR2: AArch64SysRegId = TRCSEQEVRn(2);

pub const TRCSSCCR0: AArch64SysRegId = TRCSSCCRn(0);
pub const TRCSSCCR1: AArch64SysRegId = TRCSSCCRn(1);
pub const TRCSSCCR2: AArch64SysRegId = TRCSSCCRn(2);
pub const TRCSSCCR3: AArch64SysRegId = TRCSSCCRn(3);
pub const TRCSSCCR4: AArch64SysRegId = TRCSSCCRn(4);
pub const TRCSSCCR5: AArch64SysRegId = TRCSSCCRn(5);
pub const TRCSSCCR6: AArch64SysRegId = TRCSSCCRn(6);
pub const TRCSSCCR7: AArch64SysRegId = TRCSSCCRn(7);

pub const TRCSSCSR0: AArch64SysRegId = TRCSSCSRn(0);
pub const TRCSSCSR1: AArch64SysRegId = TRCSSCSRn(1);
pub const TRCSSCSR2: AArch64SysRegId = TRCSSCSRn(2);
pub const TRCSSCSR3: AArch64SysRegId = TRCSSCSRn(3);
pub const TRCSSCSR4: AArch64SysRegId = TRCSSCSRn(4);
pub const TRCSSCSR5: AArch64SysRegId = TRCSSCSRn(5);
pub const TRCSSCSR6: AArch64SysRegId = TRCSSCSRn(6);
pub const TRCSSCSR7: AArch64SysRegId = TRCSSCSRn(7);

pub const TRCSSPCICR0: AArch64SysRegId = TRCSSPCICRn(0);
pub const TRCSSPCICR1: AArch64SysRegId = TRCSSPCICRn(1);
pub const TRCSSPCICR2: AArch64SysRegId = TRCSSPCICRn(2);
pub const TRCSSPCICR3: AArch64SysRegId = TRCSSPCICRn(3);
pub const TRCSSPCICR4: AArch64SysRegId = TRCSSPCICRn(4);
pub const TRCSSPCICR5: AArch64SysRegId = TRCSSPCICRn(5);
pub const TRCSSPCICR6: AArch64SysRegId = TRCSSPCICRn(6);
pub const TRCSSPCICR7: AArch64SysRegId = TRCSSPCICRn(7);

pub const TRCVMIDCVR0: AArch64SysRegId = TRCVMIDCVRn(0);
pub const TRCVMIDCVR1: AArch64SysRegId = TRCVMIDCVRn(1);
pub const TRCVMIDCVR2: AArch64SysRegId = TRCVMIDCVRn(2);
pub const TRCVMIDCVR3: AArch64SysRegId = TRCVMIDCVRn(3);
pub const TRCVMIDCVR4: AArch64SysRegId = TRCVMIDCVRn(4);
pub const TRCVMIDCVR5: AArch64SysRegId = TRCVMIDCVRn(5);
pub const TRCVMIDCVR6: AArch64SysRegId = TRCVMIDCVRn(6);
pub const TRCVMIDCVR7: AArch64SysRegId = TRCVMIDCVRn(7);
