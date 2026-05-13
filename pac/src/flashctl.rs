#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    flashctl_iidx: FlashctlIidx,
    _reserved1: [u8; 0x04],
    flashctl_imask: FlashctlImask,
    _reserved2: [u8; 0x04],
    flashctl_ris: FlashctlRis,
    _reserved3: [u8; 0x04],
    flashctl_mis: FlashctlMis,
    _reserved4: [u8; 0x04],
    flashctl_iset: FlashctlIset,
    _reserved5: [u8; 0x04],
    flashctl_iclr: FlashctlIclr,
    _reserved6: [u8; 0x94],
    flashctl_evt_mode: FlashctlEvtMode,
    _reserved7: [u8; 0x18],
    flashctl_desc: FlashctlDesc,
    flashctl_cmdexec: FlashctlCmdexec,
    flashctl_cmdtype: FlashctlCmdtype,
    flashctl_cmdctl: FlashctlCmdctl,
    _reserved11: [u8; 0x14],
    flashctl_cmdaddr: FlashctlCmdaddr,
    flashctl_cmdbyten: FlashctlCmdbyten,
    _reserved13: [u8; 0x04],
    flashctl_cmddataindex: FlashctlCmddataindex,
    flashctl_cmddata0: FlashctlCmddata0,
    flashctl_cmddata1: FlashctlCmddata1,
    flashctl_cmddata2: FlashctlCmddata2,
    flashctl_cmddata3: FlashctlCmddata3,
    flashctl_cmddata4: FlashctlCmddata4,
    flashctl_cmddata5: FlashctlCmddata5,
    flashctl_cmddata6: FlashctlCmddata6,
    flashctl_cmddata7: FlashctlCmddata7,
    flashctl_cmddata8: FlashctlCmddata8,
    flashctl_cmddata9: FlashctlCmddata9,
    flashctl_cmddata10: FlashctlCmddata10,
    flashctl_cmddata11: FlashctlCmddata11,
    flashctl_cmddata12: FlashctlCmddata12,
    flashctl_cmddata13: FlashctlCmddata13,
    flashctl_cmddata14: FlashctlCmddata14,
    flashctl_cmddata15: FlashctlCmddata15,
    flashctl_cmddata16: FlashctlCmddata16,
    flashctl_cmddata17: FlashctlCmddata17,
    flashctl_cmddata18: FlashctlCmddata18,
    flashctl_cmddata19: FlashctlCmddata19,
    flashctl_cmddata20: FlashctlCmddata20,
    flashctl_cmddata21: FlashctlCmddata21,
    flashctl_cmddata22: FlashctlCmddata22,
    flashctl_cmddata23: FlashctlCmddata23,
    flashctl_cmddata24: FlashctlCmddata24,
    flashctl_cmddata25: FlashctlCmddata25,
    flashctl_cmddata26: FlashctlCmddata26,
    flashctl_cmddata27: FlashctlCmddata27,
    flashctl_cmddata28: FlashctlCmddata28,
    flashctl_cmddata29: FlashctlCmddata29,
    flashctl_cmddata30: FlashctlCmddata30,
    flashctl_cmddata31: FlashctlCmddata31,
    flashctl_cmddataecc0: FlashctlCmddataecc0,
    flashctl_cmddataecc1: FlashctlCmddataecc1,
    flashctl_cmddataecc2: FlashctlCmddataecc2,
    flashctl_cmddataecc3: FlashctlCmddataecc3,
    flashctl_cmddataecc4: FlashctlCmddataecc4,
    flashctl_cmddataecc5: FlashctlCmddataecc5,
    flashctl_cmddataecc6: FlashctlCmddataecc6,
    flashctl_cmddataecc7: FlashctlCmddataecc7,
    flashctl_cmdweprota: FlashctlCmdweprota,
    flashctl_cmdweprotb: FlashctlCmdweprotb,
    flashctl_cmdweprotc: FlashctlCmdweprotc,
    _reserved57: [u8; 0x34],
    flashctl_cmdweprotnm: FlashctlCmdweprotnm,
    flashctl_cmdweprottr: FlashctlCmdweprottr,
    flashctl_cmdweproten: FlashctlCmdweproten,
    _reserved60: [u8; 0x0194],
    flashctl_cfgcmd: FlashctlCfgcmd,
    flashctl_cfgpcnt: FlashctlCfgpcnt,
    _reserved62: [u8; 0x18],
    flashctl_statcmd: FlashctlStatcmd,
    flashctl_stataddr: FlashctlStataddr,
    flashctl_statpcnt: FlashctlStatpcnt,
    flashctl_statmode: FlashctlStatmode,
    _reserved66: [u8; 0x10],
    flashctl_gblinfo0: FlashctlGblinfo0,
    flashctl_gblinfo1: FlashctlGblinfo1,
    flashctl_gblinfo2: FlashctlGblinfo2,
    _reserved69: [u8; 0x04],
    flashctl_bank0info0: FlashctlBank0info0,
    flashctl_bank0info1: FlashctlBank0info1,
    _reserved71: [u8; 0x08],
    flashctl_bank1info0: FlashctlBank1info0,
    flashctl_bank1info1: FlashctlBank1info1,
    _reserved73: [u8; 0x08],
    flashctl_bank2info0: FlashctlBank2info0,
    flashctl_bank2info1: FlashctlBank2info1,
    _reserved75: [u8; 0x08],
    flashctl_bank3info0: FlashctlBank3info0,
    flashctl_bank3info1: FlashctlBank3info1,
    _reserved77: [u8; 0x08],
    flashctl_bank4info0: FlashctlBank4info0,
    flashctl_bank4info1: FlashctlBank4info1,
}
impl RegisterBlock {
    #[doc = "0x1020 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn flashctl_iidx(&self) -> &FlashctlIidx {
        &self.flashctl_iidx
    }
    #[doc = "0x1028 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flashctl_imask(&self) -> &FlashctlImask {
        &self.flashctl_imask
    }
    #[doc = "0x1030 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn flashctl_ris(&self) -> &FlashctlRis {
        &self.flashctl_ris
    }
    #[doc = "0x1038 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn flashctl_mis(&self) -> &FlashctlMis {
        &self.flashctl_mis
    }
    #[doc = "0x1040 - Interrupt Set Register"]
    #[inline(always)]
    pub const fn flashctl_iset(&self) -> &FlashctlIset {
        &self.flashctl_iset
    }
    #[doc = "0x1048 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn flashctl_iclr(&self) -> &FlashctlIclr {
        &self.flashctl_iclr
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn flashctl_evt_mode(&self) -> &FlashctlEvtMode {
        &self.flashctl_evt_mode
    }
    #[doc = "0x10fc - Hardware Version Description Register"]
    #[inline(always)]
    pub const fn flashctl_desc(&self) -> &FlashctlDesc {
        &self.flashctl_desc
    }
    #[doc = "0x1100 - Command Execute Register"]
    #[inline(always)]
    pub const fn flashctl_cmdexec(&self) -> &FlashctlCmdexec {
        &self.flashctl_cmdexec
    }
    #[doc = "0x1104 - Command Type Register"]
    #[inline(always)]
    pub const fn flashctl_cmdtype(&self) -> &FlashctlCmdtype {
        &self.flashctl_cmdtype
    }
    #[doc = "0x1108 - Command Control Register"]
    #[inline(always)]
    pub const fn flashctl_cmdctl(&self) -> &FlashctlCmdctl {
        &self.flashctl_cmdctl
    }
    #[doc = "0x1120 - Command Address Register"]
    #[inline(always)]
    pub const fn flashctl_cmdaddr(&self) -> &FlashctlCmdaddr {
        &self.flashctl_cmdaddr
    }
    #[doc = "0x1124 - Command Program Byte Enable Register"]
    #[inline(always)]
    pub const fn flashctl_cmdbyten(&self) -> &FlashctlCmdbyten {
        &self.flashctl_cmdbyten
    }
    #[doc = "0x112c - Command Data Index Register"]
    #[inline(always)]
    pub const fn flashctl_cmddataindex(&self) -> &FlashctlCmddataindex {
        &self.flashctl_cmddataindex
    }
    #[doc = "0x1130 - Command Data Register 0"]
    #[inline(always)]
    pub const fn flashctl_cmddata0(&self) -> &FlashctlCmddata0 {
        &self.flashctl_cmddata0
    }
    #[doc = "0x1134 - Command Data Register 1"]
    #[inline(always)]
    pub const fn flashctl_cmddata1(&self) -> &FlashctlCmddata1 {
        &self.flashctl_cmddata1
    }
    #[doc = "0x1138 - Command Data Register 2"]
    #[inline(always)]
    pub const fn flashctl_cmddata2(&self) -> &FlashctlCmddata2 {
        &self.flashctl_cmddata2
    }
    #[doc = "0x113c - Command Data Register Bits 127:96"]
    #[inline(always)]
    pub const fn flashctl_cmddata3(&self) -> &FlashctlCmddata3 {
        &self.flashctl_cmddata3
    }
    #[doc = "0x1140 - Command Data Register 4"]
    #[inline(always)]
    pub const fn flashctl_cmddata4(&self) -> &FlashctlCmddata4 {
        &self.flashctl_cmddata4
    }
    #[doc = "0x1144 - Command Data Register 5"]
    #[inline(always)]
    pub const fn flashctl_cmddata5(&self) -> &FlashctlCmddata5 {
        &self.flashctl_cmddata5
    }
    #[doc = "0x1148 - Command Data Register 6"]
    #[inline(always)]
    pub const fn flashctl_cmddata6(&self) -> &FlashctlCmddata6 {
        &self.flashctl_cmddata6
    }
    #[doc = "0x114c - Command Data Register 7"]
    #[inline(always)]
    pub const fn flashctl_cmddata7(&self) -> &FlashctlCmddata7 {
        &self.flashctl_cmddata7
    }
    #[doc = "0x1150 - Command Data Register 8"]
    #[inline(always)]
    pub const fn flashctl_cmddata8(&self) -> &FlashctlCmddata8 {
        &self.flashctl_cmddata8
    }
    #[doc = "0x1154 - Command Data Register 9"]
    #[inline(always)]
    pub const fn flashctl_cmddata9(&self) -> &FlashctlCmddata9 {
        &self.flashctl_cmddata9
    }
    #[doc = "0x1158 - Command Data Register 10"]
    #[inline(always)]
    pub const fn flashctl_cmddata10(&self) -> &FlashctlCmddata10 {
        &self.flashctl_cmddata10
    }
    #[doc = "0x115c - Command Data Register 11"]
    #[inline(always)]
    pub const fn flashctl_cmddata11(&self) -> &FlashctlCmddata11 {
        &self.flashctl_cmddata11
    }
    #[doc = "0x1160 - Command Data Register 12"]
    #[inline(always)]
    pub const fn flashctl_cmddata12(&self) -> &FlashctlCmddata12 {
        &self.flashctl_cmddata12
    }
    #[doc = "0x1164 - Command Data Register 13"]
    #[inline(always)]
    pub const fn flashctl_cmddata13(&self) -> &FlashctlCmddata13 {
        &self.flashctl_cmddata13
    }
    #[doc = "0x1168 - Command Data Register 14"]
    #[inline(always)]
    pub const fn flashctl_cmddata14(&self) -> &FlashctlCmddata14 {
        &self.flashctl_cmddata14
    }
    #[doc = "0x116c - Command Data Register 15"]
    #[inline(always)]
    pub const fn flashctl_cmddata15(&self) -> &FlashctlCmddata15 {
        &self.flashctl_cmddata15
    }
    #[doc = "0x1170 - Command Data Register 16"]
    #[inline(always)]
    pub const fn flashctl_cmddata16(&self) -> &FlashctlCmddata16 {
        &self.flashctl_cmddata16
    }
    #[doc = "0x1174 - Command Data Register 17"]
    #[inline(always)]
    pub const fn flashctl_cmddata17(&self) -> &FlashctlCmddata17 {
        &self.flashctl_cmddata17
    }
    #[doc = "0x1178 - Command Data Register 18"]
    #[inline(always)]
    pub const fn flashctl_cmddata18(&self) -> &FlashctlCmddata18 {
        &self.flashctl_cmddata18
    }
    #[doc = "0x117c - Command Data Register 19"]
    #[inline(always)]
    pub const fn flashctl_cmddata19(&self) -> &FlashctlCmddata19 {
        &self.flashctl_cmddata19
    }
    #[doc = "0x1180 - Command Data Register 20"]
    #[inline(always)]
    pub const fn flashctl_cmddata20(&self) -> &FlashctlCmddata20 {
        &self.flashctl_cmddata20
    }
    #[doc = "0x1184 - Command Data Register 21"]
    #[inline(always)]
    pub const fn flashctl_cmddata21(&self) -> &FlashctlCmddata21 {
        &self.flashctl_cmddata21
    }
    #[doc = "0x1188 - Command Data Register 22"]
    #[inline(always)]
    pub const fn flashctl_cmddata22(&self) -> &FlashctlCmddata22 {
        &self.flashctl_cmddata22
    }
    #[doc = "0x118c - Command Data Register 23"]
    #[inline(always)]
    pub const fn flashctl_cmddata23(&self) -> &FlashctlCmddata23 {
        &self.flashctl_cmddata23
    }
    #[doc = "0x1190 - Command Data Register 24"]
    #[inline(always)]
    pub const fn flashctl_cmddata24(&self) -> &FlashctlCmddata24 {
        &self.flashctl_cmddata24
    }
    #[doc = "0x1194 - Command Data Register 25"]
    #[inline(always)]
    pub const fn flashctl_cmddata25(&self) -> &FlashctlCmddata25 {
        &self.flashctl_cmddata25
    }
    #[doc = "0x1198 - Command Data Register 26"]
    #[inline(always)]
    pub const fn flashctl_cmddata26(&self) -> &FlashctlCmddata26 {
        &self.flashctl_cmddata26
    }
    #[doc = "0x119c - Command Data Register 27"]
    #[inline(always)]
    pub const fn flashctl_cmddata27(&self) -> &FlashctlCmddata27 {
        &self.flashctl_cmddata27
    }
    #[doc = "0x11a0 - Command Data Register 28"]
    #[inline(always)]
    pub const fn flashctl_cmddata28(&self) -> &FlashctlCmddata28 {
        &self.flashctl_cmddata28
    }
    #[doc = "0x11a4 - Command Data Register 29"]
    #[inline(always)]
    pub const fn flashctl_cmddata29(&self) -> &FlashctlCmddata29 {
        &self.flashctl_cmddata29
    }
    #[doc = "0x11a8 - Command Data Register 30"]
    #[inline(always)]
    pub const fn flashctl_cmddata30(&self) -> &FlashctlCmddata30 {
        &self.flashctl_cmddata30
    }
    #[doc = "0x11ac - Command Data Register 31"]
    #[inline(always)]
    pub const fn flashctl_cmddata31(&self) -> &FlashctlCmddata31 {
        &self.flashctl_cmddata31
    }
    #[doc = "0x11b0 - Command Data Register ECC 0"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc0(&self) -> &FlashctlCmddataecc0 {
        &self.flashctl_cmddataecc0
    }
    #[doc = "0x11b4 - Command Data Register ECC 1"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc1(&self) -> &FlashctlCmddataecc1 {
        &self.flashctl_cmddataecc1
    }
    #[doc = "0x11b8 - Command Data Register ECC 2"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc2(&self) -> &FlashctlCmddataecc2 {
        &self.flashctl_cmddataecc2
    }
    #[doc = "0x11bc - Command Data Register ECC 3"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc3(&self) -> &FlashctlCmddataecc3 {
        &self.flashctl_cmddataecc3
    }
    #[doc = "0x11c0 - Command Data Register ECC 4"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc4(&self) -> &FlashctlCmddataecc4 {
        &self.flashctl_cmddataecc4
    }
    #[doc = "0x11c4 - Command Data Register ECC 5"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc5(&self) -> &FlashctlCmddataecc5 {
        &self.flashctl_cmddataecc5
    }
    #[doc = "0x11c8 - Command Data Register ECC 6"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc6(&self) -> &FlashctlCmddataecc6 {
        &self.flashctl_cmddataecc6
    }
    #[doc = "0x11cc - Command Data Register ECC 7"]
    #[inline(always)]
    pub const fn flashctl_cmddataecc7(&self) -> &FlashctlCmddataecc7 {
        &self.flashctl_cmddataecc7
    }
    #[doc = "0x11d0 - Command Write Erase Protect A Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweprota(&self) -> &FlashctlCmdweprota {
        &self.flashctl_cmdweprota
    }
    #[doc = "0x11d4 - Command Write Erase Protect B Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweprotb(&self) -> &FlashctlCmdweprotb {
        &self.flashctl_cmdweprotb
    }
    #[doc = "0x11d8 - Command Write Erase Protect C Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweprotc(&self) -> &FlashctlCmdweprotc {
        &self.flashctl_cmdweprotc
    }
    #[doc = "0x1210 - Command Write Erase Protect Non-Main Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweprotnm(&self) -> &FlashctlCmdweprotnm {
        &self.flashctl_cmdweprotnm
    }
    #[doc = "0x1214 - Command Write Erase Protect Trim Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweprottr(&self) -> &FlashctlCmdweprottr {
        &self.flashctl_cmdweprottr
    }
    #[doc = "0x1218 - Command Write Erase Protect Engr Register"]
    #[inline(always)]
    pub const fn flashctl_cmdweproten(&self) -> &FlashctlCmdweproten {
        &self.flashctl_cmdweproten
    }
    #[doc = "0x13b0 - Command Configuration Register"]
    #[inline(always)]
    pub const fn flashctl_cfgcmd(&self) -> &FlashctlCfgcmd {
        &self.flashctl_cfgcmd
    }
    #[doc = "0x13b4 - Pulse Counter Configuration Register"]
    #[inline(always)]
    pub const fn flashctl_cfgpcnt(&self) -> &FlashctlCfgpcnt {
        &self.flashctl_cfgpcnt
    }
    #[doc = "0x13d0 - Command Status Register"]
    #[inline(always)]
    pub const fn flashctl_statcmd(&self) -> &FlashctlStatcmd {
        &self.flashctl_statcmd
    }
    #[doc = "0x13d4 - Address Status Register"]
    #[inline(always)]
    pub const fn flashctl_stataddr(&self) -> &FlashctlStataddr {
        &self.flashctl_stataddr
    }
    #[doc = "0x13d8 - Pulse Count Status Register"]
    #[inline(always)]
    pub const fn flashctl_statpcnt(&self) -> &FlashctlStatpcnt {
        &self.flashctl_statpcnt
    }
    #[doc = "0x13dc - Mode Status Register"]
    #[inline(always)]
    pub const fn flashctl_statmode(&self) -> &FlashctlStatmode {
        &self.flashctl_statmode
    }
    #[doc = "0x13f0 - Global Information Register 0"]
    #[inline(always)]
    pub const fn flashctl_gblinfo0(&self) -> &FlashctlGblinfo0 {
        &self.flashctl_gblinfo0
    }
    #[doc = "0x13f4 - Global Information Register 1"]
    #[inline(always)]
    pub const fn flashctl_gblinfo1(&self) -> &FlashctlGblinfo1 {
        &self.flashctl_gblinfo1
    }
    #[doc = "0x13f8 - Global Information Register 2"]
    #[inline(always)]
    pub const fn flashctl_gblinfo2(&self) -> &FlashctlGblinfo2 {
        &self.flashctl_gblinfo2
    }
    #[doc = "0x1400 - Bank Information Register 0 for Bank 0"]
    #[inline(always)]
    pub const fn flashctl_bank0info0(&self) -> &FlashctlBank0info0 {
        &self.flashctl_bank0info0
    }
    #[doc = "0x1404 - Bank Information Register 1 for Bank 0"]
    #[inline(always)]
    pub const fn flashctl_bank0info1(&self) -> &FlashctlBank0info1 {
        &self.flashctl_bank0info1
    }
    #[doc = "0x1410 - Bank Information Register 0 for Bank 1"]
    #[inline(always)]
    pub const fn flashctl_bank1info0(&self) -> &FlashctlBank1info0 {
        &self.flashctl_bank1info0
    }
    #[doc = "0x1414 - Bank Information Register 1 for Bank 1"]
    #[inline(always)]
    pub const fn flashctl_bank1info1(&self) -> &FlashctlBank1info1 {
        &self.flashctl_bank1info1
    }
    #[doc = "0x1420 - Bank Information Register 0 for Bank 2"]
    #[inline(always)]
    pub const fn flashctl_bank2info0(&self) -> &FlashctlBank2info0 {
        &self.flashctl_bank2info0
    }
    #[doc = "0x1424 - Bank Information Register 1 for Bank 2"]
    #[inline(always)]
    pub const fn flashctl_bank2info1(&self) -> &FlashctlBank2info1 {
        &self.flashctl_bank2info1
    }
    #[doc = "0x1430 - Bank Information Register 0 for Bank 3"]
    #[inline(always)]
    pub const fn flashctl_bank3info0(&self) -> &FlashctlBank3info0 {
        &self.flashctl_bank3info0
    }
    #[doc = "0x1434 - Bank Information Register 1 for Bank 3"]
    #[inline(always)]
    pub const fn flashctl_bank3info1(&self) -> &FlashctlBank3info1 {
        &self.flashctl_bank3info1
    }
    #[doc = "0x1440 - Bank Information Register 0 for Bank 4"]
    #[inline(always)]
    pub const fn flashctl_bank4info0(&self) -> &FlashctlBank4info0 {
        &self.flashctl_bank4info0
    }
    #[doc = "0x1444 - Bank Information Register 1 for Bank 4"]
    #[inline(always)]
    pub const fn flashctl_bank4info1(&self) -> &FlashctlBank4info1 {
        &self.flashctl_bank4info1
    }
}
#[doc = "FLASHCTL_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_iidx`] module"]
#[doc(alias = "FLASHCTL_IIDX")]
pub type FlashctlIidx = crate::Reg<flashctl_iidx::FlashctlIidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod flashctl_iidx;
#[doc = "FLASHCTL_IMASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_imask`] module"]
#[doc(alias = "FLASHCTL_IMASK")]
pub type FlashctlImask = crate::Reg<flashctl_imask::FlashctlImaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod flashctl_imask;
#[doc = "FLASHCTL_RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_ris`] module"]
#[doc(alias = "FLASHCTL_RIS")]
pub type FlashctlRis = crate::Reg<flashctl_ris::FlashctlRisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod flashctl_ris;
#[doc = "FLASHCTL_MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_mis`] module"]
#[doc(alias = "FLASHCTL_MIS")]
pub type FlashctlMis = crate::Reg<flashctl_mis::FlashctlMisSpec>;
#[doc = "Masked Interrupt Status Register"]
pub mod flashctl_mis;
#[doc = "FLASHCTL_ISET (w) register accessor: Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_iset`] module"]
#[doc(alias = "FLASHCTL_ISET")]
pub type FlashctlIset = crate::Reg<flashctl_iset::FlashctlIsetSpec>;
#[doc = "Interrupt Set Register"]
pub mod flashctl_iset;
#[doc = "FLASHCTL_ICLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_iclr`] module"]
#[doc(alias = "FLASHCTL_ICLR")]
pub type FlashctlIclr = crate::Reg<flashctl_iclr::FlashctlIclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod flashctl_iclr;
#[doc = "FLASHCTL_EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_evt_mode`] module"]
#[doc(alias = "FLASHCTL_EVT_MODE")]
pub type FlashctlEvtMode = crate::Reg<flashctl_evt_mode::FlashctlEvtModeSpec>;
#[doc = "Event Mode"]
pub mod flashctl_evt_mode;
#[doc = "FLASHCTL_DESC (r) register accessor: Hardware Version Description Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_desc`] module"]
#[doc(alias = "FLASHCTL_DESC")]
pub type FlashctlDesc = crate::Reg<flashctl_desc::FlashctlDescSpec>;
#[doc = "Hardware Version Description Register"]
pub mod flashctl_desc;
#[doc = "FLASHCTL_CMDEXEC (rw) register accessor: Command Execute Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdexec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdexec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdexec`] module"]
#[doc(alias = "FLASHCTL_CMDEXEC")]
pub type FlashctlCmdexec = crate::Reg<flashctl_cmdexec::FlashctlCmdexecSpec>;
#[doc = "Command Execute Register"]
pub mod flashctl_cmdexec;
#[doc = "FLASHCTL_CMDTYPE (rw) register accessor: Command Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdtype`] module"]
#[doc(alias = "FLASHCTL_CMDTYPE")]
pub type FlashctlCmdtype = crate::Reg<flashctl_cmdtype::FlashctlCmdtypeSpec>;
#[doc = "Command Type Register"]
pub mod flashctl_cmdtype;
#[doc = "FLASHCTL_CMDCTL (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdctl`] module"]
#[doc(alias = "FLASHCTL_CMDCTL")]
pub type FlashctlCmdctl = crate::Reg<flashctl_cmdctl::FlashctlCmdctlSpec>;
#[doc = "Command Control Register"]
pub mod flashctl_cmdctl;
#[doc = "FLASHCTL_CMDADDR (rw) register accessor: Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdaddr`] module"]
#[doc(alias = "FLASHCTL_CMDADDR")]
pub type FlashctlCmdaddr = crate::Reg<flashctl_cmdaddr::FlashctlCmdaddrSpec>;
#[doc = "Command Address Register"]
pub mod flashctl_cmdaddr;
#[doc = "FLASHCTL_CMDBYTEN (rw) register accessor: Command Program Byte Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdbyten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdbyten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdbyten`] module"]
#[doc(alias = "FLASHCTL_CMDBYTEN")]
pub type FlashctlCmdbyten = crate::Reg<flashctl_cmdbyten::FlashctlCmdbytenSpec>;
#[doc = "Command Program Byte Enable Register"]
pub mod flashctl_cmdbyten;
#[doc = "FLASHCTL_CMDDATAINDEX (rw) register accessor: Command Data Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataindex`] module"]
#[doc(alias = "FLASHCTL_CMDDATAINDEX")]
pub type FlashctlCmddataindex = crate::Reg<flashctl_cmddataindex::FlashctlCmddataindexSpec>;
#[doc = "Command Data Index Register"]
pub mod flashctl_cmddataindex;
#[doc = "FLASHCTL_CMDDATA0 (rw) register accessor: Command Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata0`] module"]
#[doc(alias = "FLASHCTL_CMDDATA0")]
pub type FlashctlCmddata0 = crate::Reg<flashctl_cmddata0::FlashctlCmddata0Spec>;
#[doc = "Command Data Register 0"]
pub mod flashctl_cmddata0;
#[doc = "FLASHCTL_CMDDATA1 (rw) register accessor: Command Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata1`] module"]
#[doc(alias = "FLASHCTL_CMDDATA1")]
pub type FlashctlCmddata1 = crate::Reg<flashctl_cmddata1::FlashctlCmddata1Spec>;
#[doc = "Command Data Register 1"]
pub mod flashctl_cmddata1;
#[doc = "FLASHCTL_CMDDATA2 (rw) register accessor: Command Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata2`] module"]
#[doc(alias = "FLASHCTL_CMDDATA2")]
pub type FlashctlCmddata2 = crate::Reg<flashctl_cmddata2::FlashctlCmddata2Spec>;
#[doc = "Command Data Register 2"]
pub mod flashctl_cmddata2;
#[doc = "FLASHCTL_CMDDATA3 (rw) register accessor: Command Data Register Bits 127:96\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata3`] module"]
#[doc(alias = "FLASHCTL_CMDDATA3")]
pub type FlashctlCmddata3 = crate::Reg<flashctl_cmddata3::FlashctlCmddata3Spec>;
#[doc = "Command Data Register Bits 127:96"]
pub mod flashctl_cmddata3;
#[doc = "FLASHCTL_CMDDATA4 (rw) register accessor: Command Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata4`] module"]
#[doc(alias = "FLASHCTL_CMDDATA4")]
pub type FlashctlCmddata4 = crate::Reg<flashctl_cmddata4::FlashctlCmddata4Spec>;
#[doc = "Command Data Register 4"]
pub mod flashctl_cmddata4;
#[doc = "FLASHCTL_CMDDATA5 (rw) register accessor: Command Data Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata5`] module"]
#[doc(alias = "FLASHCTL_CMDDATA5")]
pub type FlashctlCmddata5 = crate::Reg<flashctl_cmddata5::FlashctlCmddata5Spec>;
#[doc = "Command Data Register 5"]
pub mod flashctl_cmddata5;
#[doc = "FLASHCTL_CMDDATA6 (rw) register accessor: Command Data Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata6`] module"]
#[doc(alias = "FLASHCTL_CMDDATA6")]
pub type FlashctlCmddata6 = crate::Reg<flashctl_cmddata6::FlashctlCmddata6Spec>;
#[doc = "Command Data Register 6"]
pub mod flashctl_cmddata6;
#[doc = "FLASHCTL_CMDDATA7 (rw) register accessor: Command Data Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata7`] module"]
#[doc(alias = "FLASHCTL_CMDDATA7")]
pub type FlashctlCmddata7 = crate::Reg<flashctl_cmddata7::FlashctlCmddata7Spec>;
#[doc = "Command Data Register 7"]
pub mod flashctl_cmddata7;
#[doc = "FLASHCTL_CMDDATA8 (rw) register accessor: Command Data Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata8`] module"]
#[doc(alias = "FLASHCTL_CMDDATA8")]
pub type FlashctlCmddata8 = crate::Reg<flashctl_cmddata8::FlashctlCmddata8Spec>;
#[doc = "Command Data Register 8"]
pub mod flashctl_cmddata8;
#[doc = "FLASHCTL_CMDDATA9 (rw) register accessor: Command Data Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata9`] module"]
#[doc(alias = "FLASHCTL_CMDDATA9")]
pub type FlashctlCmddata9 = crate::Reg<flashctl_cmddata9::FlashctlCmddata9Spec>;
#[doc = "Command Data Register 9"]
pub mod flashctl_cmddata9;
#[doc = "FLASHCTL_CMDDATA10 (rw) register accessor: Command Data Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata10`] module"]
#[doc(alias = "FLASHCTL_CMDDATA10")]
pub type FlashctlCmddata10 = crate::Reg<flashctl_cmddata10::FlashctlCmddata10Spec>;
#[doc = "Command Data Register 10"]
pub mod flashctl_cmddata10;
#[doc = "FLASHCTL_CMDDATA11 (rw) register accessor: Command Data Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata11`] module"]
#[doc(alias = "FLASHCTL_CMDDATA11")]
pub type FlashctlCmddata11 = crate::Reg<flashctl_cmddata11::FlashctlCmddata11Spec>;
#[doc = "Command Data Register 11"]
pub mod flashctl_cmddata11;
#[doc = "FLASHCTL_CMDDATA12 (rw) register accessor: Command Data Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata12`] module"]
#[doc(alias = "FLASHCTL_CMDDATA12")]
pub type FlashctlCmddata12 = crate::Reg<flashctl_cmddata12::FlashctlCmddata12Spec>;
#[doc = "Command Data Register 12"]
pub mod flashctl_cmddata12;
#[doc = "FLASHCTL_CMDDATA13 (rw) register accessor: Command Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata13`] module"]
#[doc(alias = "FLASHCTL_CMDDATA13")]
pub type FlashctlCmddata13 = crate::Reg<flashctl_cmddata13::FlashctlCmddata13Spec>;
#[doc = "Command Data Register 13"]
pub mod flashctl_cmddata13;
#[doc = "FLASHCTL_CMDDATA14 (rw) register accessor: Command Data Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata14`] module"]
#[doc(alias = "FLASHCTL_CMDDATA14")]
pub type FlashctlCmddata14 = crate::Reg<flashctl_cmddata14::FlashctlCmddata14Spec>;
#[doc = "Command Data Register 14"]
pub mod flashctl_cmddata14;
#[doc = "FLASHCTL_CMDDATA15 (rw) register accessor: Command Data Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata15`] module"]
#[doc(alias = "FLASHCTL_CMDDATA15")]
pub type FlashctlCmddata15 = crate::Reg<flashctl_cmddata15::FlashctlCmddata15Spec>;
#[doc = "Command Data Register 15"]
pub mod flashctl_cmddata15;
#[doc = "FLASHCTL_CMDDATA16 (rw) register accessor: Command Data Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata16`] module"]
#[doc(alias = "FLASHCTL_CMDDATA16")]
pub type FlashctlCmddata16 = crate::Reg<flashctl_cmddata16::FlashctlCmddata16Spec>;
#[doc = "Command Data Register 16"]
pub mod flashctl_cmddata16;
#[doc = "FLASHCTL_CMDDATA17 (rw) register accessor: Command Data Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata17`] module"]
#[doc(alias = "FLASHCTL_CMDDATA17")]
pub type FlashctlCmddata17 = crate::Reg<flashctl_cmddata17::FlashctlCmddata17Spec>;
#[doc = "Command Data Register 17"]
pub mod flashctl_cmddata17;
#[doc = "FLASHCTL_CMDDATA18 (rw) register accessor: Command Data Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata18`] module"]
#[doc(alias = "FLASHCTL_CMDDATA18")]
pub type FlashctlCmddata18 = crate::Reg<flashctl_cmddata18::FlashctlCmddata18Spec>;
#[doc = "Command Data Register 18"]
pub mod flashctl_cmddata18;
#[doc = "FLASHCTL_CMDDATA19 (rw) register accessor: Command Data Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata19`] module"]
#[doc(alias = "FLASHCTL_CMDDATA19")]
pub type FlashctlCmddata19 = crate::Reg<flashctl_cmddata19::FlashctlCmddata19Spec>;
#[doc = "Command Data Register 19"]
pub mod flashctl_cmddata19;
#[doc = "FLASHCTL_CMDDATA20 (rw) register accessor: Command Data Register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata20`] module"]
#[doc(alias = "FLASHCTL_CMDDATA20")]
pub type FlashctlCmddata20 = crate::Reg<flashctl_cmddata20::FlashctlCmddata20Spec>;
#[doc = "Command Data Register 20"]
pub mod flashctl_cmddata20;
#[doc = "FLASHCTL_CMDDATA21 (rw) register accessor: Command Data Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata21`] module"]
#[doc(alias = "FLASHCTL_CMDDATA21")]
pub type FlashctlCmddata21 = crate::Reg<flashctl_cmddata21::FlashctlCmddata21Spec>;
#[doc = "Command Data Register 21"]
pub mod flashctl_cmddata21;
#[doc = "FLASHCTL_CMDDATA22 (rw) register accessor: Command Data Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata22`] module"]
#[doc(alias = "FLASHCTL_CMDDATA22")]
pub type FlashctlCmddata22 = crate::Reg<flashctl_cmddata22::FlashctlCmddata22Spec>;
#[doc = "Command Data Register 22"]
pub mod flashctl_cmddata22;
#[doc = "FLASHCTL_CMDDATA23 (rw) register accessor: Command Data Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata23`] module"]
#[doc(alias = "FLASHCTL_CMDDATA23")]
pub type FlashctlCmddata23 = crate::Reg<flashctl_cmddata23::FlashctlCmddata23Spec>;
#[doc = "Command Data Register 23"]
pub mod flashctl_cmddata23;
#[doc = "FLASHCTL_CMDDATA24 (rw) register accessor: Command Data Register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata24`] module"]
#[doc(alias = "FLASHCTL_CMDDATA24")]
pub type FlashctlCmddata24 = crate::Reg<flashctl_cmddata24::FlashctlCmddata24Spec>;
#[doc = "Command Data Register 24"]
pub mod flashctl_cmddata24;
#[doc = "FLASHCTL_CMDDATA25 (rw) register accessor: Command Data Register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata25`] module"]
#[doc(alias = "FLASHCTL_CMDDATA25")]
pub type FlashctlCmddata25 = crate::Reg<flashctl_cmddata25::FlashctlCmddata25Spec>;
#[doc = "Command Data Register 25"]
pub mod flashctl_cmddata25;
#[doc = "FLASHCTL_CMDDATA26 (rw) register accessor: Command Data Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata26`] module"]
#[doc(alias = "FLASHCTL_CMDDATA26")]
pub type FlashctlCmddata26 = crate::Reg<flashctl_cmddata26::FlashctlCmddata26Spec>;
#[doc = "Command Data Register 26"]
pub mod flashctl_cmddata26;
#[doc = "FLASHCTL_CMDDATA27 (rw) register accessor: Command Data Register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata27`] module"]
#[doc(alias = "FLASHCTL_CMDDATA27")]
pub type FlashctlCmddata27 = crate::Reg<flashctl_cmddata27::FlashctlCmddata27Spec>;
#[doc = "Command Data Register 27"]
pub mod flashctl_cmddata27;
#[doc = "FLASHCTL_CMDDATA28 (rw) register accessor: Command Data Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata28`] module"]
#[doc(alias = "FLASHCTL_CMDDATA28")]
pub type FlashctlCmddata28 = crate::Reg<flashctl_cmddata28::FlashctlCmddata28Spec>;
#[doc = "Command Data Register 28"]
pub mod flashctl_cmddata28;
#[doc = "FLASHCTL_CMDDATA29 (rw) register accessor: Command Data Register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata29`] module"]
#[doc(alias = "FLASHCTL_CMDDATA29")]
pub type FlashctlCmddata29 = crate::Reg<flashctl_cmddata29::FlashctlCmddata29Spec>;
#[doc = "Command Data Register 29"]
pub mod flashctl_cmddata29;
#[doc = "FLASHCTL_CMDDATA30 (rw) register accessor: Command Data Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata30`] module"]
#[doc(alias = "FLASHCTL_CMDDATA30")]
pub type FlashctlCmddata30 = crate::Reg<flashctl_cmddata30::FlashctlCmddata30Spec>;
#[doc = "Command Data Register 30"]
pub mod flashctl_cmddata30;
#[doc = "FLASHCTL_CMDDATA31 (rw) register accessor: Command Data Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddata31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddata31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddata31`] module"]
#[doc(alias = "FLASHCTL_CMDDATA31")]
pub type FlashctlCmddata31 = crate::Reg<flashctl_cmddata31::FlashctlCmddata31Spec>;
#[doc = "Command Data Register 31"]
pub mod flashctl_cmddata31;
#[doc = "FLASHCTL_CMDDATAECC0 (rw) register accessor: Command Data Register ECC 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc0`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC0")]
pub type FlashctlCmddataecc0 = crate::Reg<flashctl_cmddataecc0::FlashctlCmddataecc0Spec>;
#[doc = "Command Data Register ECC 0"]
pub mod flashctl_cmddataecc0;
#[doc = "FLASHCTL_CMDDATAECC1 (rw) register accessor: Command Data Register ECC 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc1`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC1")]
pub type FlashctlCmddataecc1 = crate::Reg<flashctl_cmddataecc1::FlashctlCmddataecc1Spec>;
#[doc = "Command Data Register ECC 1"]
pub mod flashctl_cmddataecc1;
#[doc = "FLASHCTL_CMDDATAECC2 (rw) register accessor: Command Data Register ECC 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc2`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC2")]
pub type FlashctlCmddataecc2 = crate::Reg<flashctl_cmddataecc2::FlashctlCmddataecc2Spec>;
#[doc = "Command Data Register ECC 2"]
pub mod flashctl_cmddataecc2;
#[doc = "FLASHCTL_CMDDATAECC3 (rw) register accessor: Command Data Register ECC 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc3`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC3")]
pub type FlashctlCmddataecc3 = crate::Reg<flashctl_cmddataecc3::FlashctlCmddataecc3Spec>;
#[doc = "Command Data Register ECC 3"]
pub mod flashctl_cmddataecc3;
#[doc = "FLASHCTL_CMDDATAECC4 (rw) register accessor: Command Data Register ECC 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc4`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC4")]
pub type FlashctlCmddataecc4 = crate::Reg<flashctl_cmddataecc4::FlashctlCmddataecc4Spec>;
#[doc = "Command Data Register ECC 4"]
pub mod flashctl_cmddataecc4;
#[doc = "FLASHCTL_CMDDATAECC5 (rw) register accessor: Command Data Register ECC 5\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc5`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC5")]
pub type FlashctlCmddataecc5 = crate::Reg<flashctl_cmddataecc5::FlashctlCmddataecc5Spec>;
#[doc = "Command Data Register ECC 5"]
pub mod flashctl_cmddataecc5;
#[doc = "FLASHCTL_CMDDATAECC6 (rw) register accessor: Command Data Register ECC 6\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc6`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC6")]
pub type FlashctlCmddataecc6 = crate::Reg<flashctl_cmddataecc6::FlashctlCmddataecc6Spec>;
#[doc = "Command Data Register ECC 6"]
pub mod flashctl_cmddataecc6;
#[doc = "FLASHCTL_CMDDATAECC7 (rw) register accessor: Command Data Register ECC 7\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmddataecc7`] module"]
#[doc(alias = "FLASHCTL_CMDDATAECC7")]
pub type FlashctlCmddataecc7 = crate::Reg<flashctl_cmddataecc7::FlashctlCmddataecc7Spec>;
#[doc = "Command Data Register ECC 7"]
pub mod flashctl_cmddataecc7;
#[doc = "FLASHCTL_CMDWEPROTA (rw) register accessor: Command Write Erase Protect A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprota::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprota::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweprota`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTA")]
pub type FlashctlCmdweprota = crate::Reg<flashctl_cmdweprota::FlashctlCmdweprotaSpec>;
#[doc = "Command Write Erase Protect A Register"]
pub mod flashctl_cmdweprota;
#[doc = "FLASHCTL_CMDWEPROTB (rw) register accessor: Command Write Erase Protect B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprotb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprotb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweprotb`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTB")]
pub type FlashctlCmdweprotb = crate::Reg<flashctl_cmdweprotb::FlashctlCmdweprotbSpec>;
#[doc = "Command Write Erase Protect B Register"]
pub mod flashctl_cmdweprotb;
#[doc = "FLASHCTL_CMDWEPROTC (rw) register accessor: Command Write Erase Protect C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprotc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprotc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweprotc`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTC")]
pub type FlashctlCmdweprotc = crate::Reg<flashctl_cmdweprotc::FlashctlCmdweprotcSpec>;
#[doc = "Command Write Erase Protect C Register"]
pub mod flashctl_cmdweprotc;
#[doc = "FLASHCTL_CMDWEPROTNM (rw) register accessor: Command Write Erase Protect Non-Main Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprotnm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprotnm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweprotnm`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTNM")]
pub type FlashctlCmdweprotnm = crate::Reg<flashctl_cmdweprotnm::FlashctlCmdweprotnmSpec>;
#[doc = "Command Write Erase Protect Non-Main Register"]
pub mod flashctl_cmdweprotnm;
#[doc = "FLASHCTL_CMDWEPROTTR (rw) register accessor: Command Write Erase Protect Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprottr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprottr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweprottr`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTTR")]
pub type FlashctlCmdweprottr = crate::Reg<flashctl_cmdweprottr::FlashctlCmdweprottrSpec>;
#[doc = "Command Write Erase Protect Trim Register"]
pub mod flashctl_cmdweprottr;
#[doc = "FLASHCTL_CMDWEPROTEN (rw) register accessor: Command Write Erase Protect Engr Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweproten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweproten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cmdweproten`] module"]
#[doc(alias = "FLASHCTL_CMDWEPROTEN")]
pub type FlashctlCmdweproten = crate::Reg<flashctl_cmdweproten::FlashctlCmdweprotenSpec>;
#[doc = "Command Write Erase Protect Engr Register"]
pub mod flashctl_cmdweproten;
#[doc = "FLASHCTL_CFGCMD (rw) register accessor: Command Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cfgcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cfgcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cfgcmd`] module"]
#[doc(alias = "FLASHCTL_CFGCMD")]
pub type FlashctlCfgcmd = crate::Reg<flashctl_cfgcmd::FlashctlCfgcmdSpec>;
#[doc = "Command Configuration Register"]
pub mod flashctl_cfgcmd;
#[doc = "FLASHCTL_CFGPCNT (rw) register accessor: Pulse Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cfgpcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cfgpcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_cfgpcnt`] module"]
#[doc(alias = "FLASHCTL_CFGPCNT")]
pub type FlashctlCfgpcnt = crate::Reg<flashctl_cfgpcnt::FlashctlCfgpcntSpec>;
#[doc = "Pulse Counter Configuration Register"]
pub mod flashctl_cfgpcnt;
#[doc = "FLASHCTL_STATCMD (r) register accessor: Command Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_statcmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_statcmd`] module"]
#[doc(alias = "FLASHCTL_STATCMD")]
pub type FlashctlStatcmd = crate::Reg<flashctl_statcmd::FlashctlStatcmdSpec>;
#[doc = "Command Status Register"]
pub mod flashctl_statcmd;
#[doc = "FLASHCTL_STATADDR (r) register accessor: Address Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_stataddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_stataddr`] module"]
#[doc(alias = "FLASHCTL_STATADDR")]
pub type FlashctlStataddr = crate::Reg<flashctl_stataddr::FlashctlStataddrSpec>;
#[doc = "Address Status Register"]
pub mod flashctl_stataddr;
#[doc = "FLASHCTL_STATPCNT (r) register accessor: Pulse Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_statpcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_statpcnt`] module"]
#[doc(alias = "FLASHCTL_STATPCNT")]
pub type FlashctlStatpcnt = crate::Reg<flashctl_statpcnt::FlashctlStatpcntSpec>;
#[doc = "Pulse Count Status Register"]
pub mod flashctl_statpcnt;
#[doc = "FLASHCTL_STATMODE (r) register accessor: Mode Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_statmode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_statmode`] module"]
#[doc(alias = "FLASHCTL_STATMODE")]
pub type FlashctlStatmode = crate::Reg<flashctl_statmode::FlashctlStatmodeSpec>;
#[doc = "Mode Status Register"]
pub mod flashctl_statmode;
#[doc = "FLASHCTL_GBLINFO0 (r) register accessor: Global Information Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_gblinfo0`] module"]
#[doc(alias = "FLASHCTL_GBLINFO0")]
pub type FlashctlGblinfo0 = crate::Reg<flashctl_gblinfo0::FlashctlGblinfo0Spec>;
#[doc = "Global Information Register 0"]
pub mod flashctl_gblinfo0;
#[doc = "FLASHCTL_GBLINFO1 (r) register accessor: Global Information Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_gblinfo1`] module"]
#[doc(alias = "FLASHCTL_GBLINFO1")]
pub type FlashctlGblinfo1 = crate::Reg<flashctl_gblinfo1::FlashctlGblinfo1Spec>;
#[doc = "Global Information Register 1"]
pub mod flashctl_gblinfo1;
#[doc = "FLASHCTL_GBLINFO2 (r) register accessor: Global Information Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_gblinfo2`] module"]
#[doc(alias = "FLASHCTL_GBLINFO2")]
pub type FlashctlGblinfo2 = crate::Reg<flashctl_gblinfo2::FlashctlGblinfo2Spec>;
#[doc = "Global Information Register 2"]
pub mod flashctl_gblinfo2;
#[doc = "FLASHCTL_BANK0INFO0 (r) register accessor: Bank Information Register 0 for Bank 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank0info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank0info0`] module"]
#[doc(alias = "FLASHCTL_BANK0INFO0")]
pub type FlashctlBank0info0 = crate::Reg<flashctl_bank0info0::FlashctlBank0info0Spec>;
#[doc = "Bank Information Register 0 for Bank 0"]
pub mod flashctl_bank0info0;
#[doc = "FLASHCTL_BANK0INFO1 (r) register accessor: Bank Information Register 1 for Bank 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank0info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank0info1`] module"]
#[doc(alias = "FLASHCTL_BANK0INFO1")]
pub type FlashctlBank0info1 = crate::Reg<flashctl_bank0info1::FlashctlBank0info1Spec>;
#[doc = "Bank Information Register 1 for Bank 0"]
pub mod flashctl_bank0info1;
#[doc = "FLASHCTL_BANK1INFO0 (r) register accessor: Bank Information Register 0 for Bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank1info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank1info0`] module"]
#[doc(alias = "FLASHCTL_BANK1INFO0")]
pub type FlashctlBank1info0 = crate::Reg<flashctl_bank1info0::FlashctlBank1info0Spec>;
#[doc = "Bank Information Register 0 for Bank 1"]
pub mod flashctl_bank1info0;
#[doc = "FLASHCTL_BANK1INFO1 (r) register accessor: Bank Information Register 1 for Bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank1info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank1info1`] module"]
#[doc(alias = "FLASHCTL_BANK1INFO1")]
pub type FlashctlBank1info1 = crate::Reg<flashctl_bank1info1::FlashctlBank1info1Spec>;
#[doc = "Bank Information Register 1 for Bank 1"]
pub mod flashctl_bank1info1;
#[doc = "FLASHCTL_BANK2INFO0 (r) register accessor: Bank Information Register 0 for Bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank2info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank2info0`] module"]
#[doc(alias = "FLASHCTL_BANK2INFO0")]
pub type FlashctlBank2info0 = crate::Reg<flashctl_bank2info0::FlashctlBank2info0Spec>;
#[doc = "Bank Information Register 0 for Bank 2"]
pub mod flashctl_bank2info0;
#[doc = "FLASHCTL_BANK2INFO1 (r) register accessor: Bank Information Register 1 for Bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank2info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank2info1`] module"]
#[doc(alias = "FLASHCTL_BANK2INFO1")]
pub type FlashctlBank2info1 = crate::Reg<flashctl_bank2info1::FlashctlBank2info1Spec>;
#[doc = "Bank Information Register 1 for Bank 2"]
pub mod flashctl_bank2info1;
#[doc = "FLASHCTL_BANK3INFO0 (r) register accessor: Bank Information Register 0 for Bank 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank3info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank3info0`] module"]
#[doc(alias = "FLASHCTL_BANK3INFO0")]
pub type FlashctlBank3info0 = crate::Reg<flashctl_bank3info0::FlashctlBank3info0Spec>;
#[doc = "Bank Information Register 0 for Bank 3"]
pub mod flashctl_bank3info0;
#[doc = "FLASHCTL_BANK3INFO1 (r) register accessor: Bank Information Register 1 for Bank 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank3info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank3info1`] module"]
#[doc(alias = "FLASHCTL_BANK3INFO1")]
pub type FlashctlBank3info1 = crate::Reg<flashctl_bank3info1::FlashctlBank3info1Spec>;
#[doc = "Bank Information Register 1 for Bank 3"]
pub mod flashctl_bank3info1;
#[doc = "FLASHCTL_BANK4INFO0 (r) register accessor: Bank Information Register 0 for Bank 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank4info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank4info0`] module"]
#[doc(alias = "FLASHCTL_BANK4INFO0")]
pub type FlashctlBank4info0 = crate::Reg<flashctl_bank4info0::FlashctlBank4info0Spec>;
#[doc = "Bank Information Register 0 for Bank 4"]
pub mod flashctl_bank4info0;
#[doc = "FLASHCTL_BANK4INFO1 (r) register accessor: Bank Information Register 1 for Bank 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank4info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl_bank4info1`] module"]
#[doc(alias = "FLASHCTL_BANK4INFO1")]
pub type FlashctlBank4info1 = crate::Reg<flashctl_bank4info1::FlashctlBank4info1Spec>;
#[doc = "Bank Information Register 1 for Bank 4"]
pub mod flashctl_bank4info1;
