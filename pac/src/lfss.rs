#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0444],
    lfss_fpub_0: LfssFpub0,
    _reserved1: [u8; 0x0bbc],
    lfss_clksel: LfssClksel,
    _reserved2: [u8; 0x18],
    lfss_cpu_int: [LfssCpuInt; 1],
    _reserved3: [u8; 0x04],
    lfss_gen_event: [LfssGenEvent; 1],
    _reserved4: [u8; 0x64],
    lfss_evt_mode: LfssEvtMode,
    _reserved5: [u8; 0x18],
    lfss_desc: LfssDesc,
    lfss_clkctl: LfssClkctl,
    lfss_dbgctl: LfssDbgctl,
    lfss_ctl: LfssCtl,
    lfss_sta: LfssSta,
    lfss_cal: LfssCal,
    lfss_tcmp: LfssTcmp,
    lfss_sec: LfssSec,
    lfss_min: LfssMin,
    lfss_hour: LfssHour,
    lfss_day: LfssDay,
    lfss_mon: LfssMon,
    lfss_year: LfssYear,
    lfss_a1min: LfssA1min,
    lfss_a1hour: LfssA1hour,
    lfss_a1day: LfssA1day,
    lfss_a2min: LfssA2min,
    lfss_a2hour: LfssA2hour,
    lfss_a2day: LfssA2day,
    lfss_psctl: LfssPsctl,
    lfss_extpsctl: LfssExtpsctl,
    lfss_tssec: LfssTssec,
    lfss_tsmin: LfssTsmin,
    lfss_tshour: LfssTshour,
    lfss_tsday: LfssTsday,
    lfss_tsmon: LfssTsmon,
    lfss_tsyear: LfssTsyear,
    lfss_tsstat: LfssTsstat,
    lfss_tsctl: LfssTsctl,
    lfss_tsclr: LfssTsclr,
    _reserved35: [u8; 0x7c],
    lfss_lfssrst: LfssLfssrst,
    _reserved36: [u8; 0x08],
    lfss_rtclock: LfssRtclock,
    lfss_tioctl: [LfssTioctl; 16],
    _reserved38: [u8; 0x40],
    lfss_tout3_0: LfssTout3_0,
    lfss_tout7_4: LfssTout7_4,
    lfss_tout11_8: LfssTout11_8,
    lfss_tout15_12: LfssTout15_12,
    lfss_toe3_0: LfssToe3_0,
    lfss_toe7_4: LfssToe7_4,
    lfss_toe11_8: LfssToe11_8,
    lfss_toe15_12: LfssToe15_12,
    lfss_tin3_0: LfssTin3_0,
    lfss_tin7_4: LfssTin7_4,
    lfss_tin11_8: LfssTin11_8,
    lfss_tin15_12: LfssTin15_12,
    _reserved50: [u8; 0x10],
    lfss_heartbeat: LfssHeartbeat,
    _reserved51: [u8; 0x38],
    lfss_tiolock: LfssTiolock,
    lfss_wdten: LfssWdten,
    lfss_wdtdbgctl: LfssWdtdbgctl,
    lfss_wdtctl: LfssWdtctl,
    lfss_wdtcntrst: LfssWdtcntrst,
    lfss_wdtstat: LfssWdtstat,
    _reserved57: [u8; 0xe8],
    lfss_wdtlock: LfssWdtlock,
    lfss_spmem: [LfssSpmem; 32],
    _reserved59: [u8; 0x80],
    lfss_spmwprot0: LfssSpmwprot0,
    lfss_spmwprot1: LfssSpmwprot1,
    lfss_spmwprot2: LfssSpmwprot2,
    lfss_spmwprot3: LfssSpmwprot3,
    lfss_spmwprot4: LfssSpmwprot4,
    lfss_spmwprot5: LfssSpmwprot5,
    lfss_spmwprot6: LfssSpmwprot6,
    lfss_spmwprot7: LfssSpmwprot7,
    _reserved67: [u8; 0x20],
    lfss_spmterase0: LfssSpmterase0,
    lfss_spmterase1: LfssSpmterase1,
    lfss_spmterase2: LfssSpmterase2,
    lfss_spmterase3: LfssSpmterase3,
    lfss_spmterase4: LfssSpmterase4,
    lfss_spmterase5: LfssSpmterase5,
    lfss_spmterase6: LfssSpmterase6,
    lfss_spmterase7: LfssSpmterase7,
}
impl RegisterBlock {
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn lfss_fpub_0(&self) -> &LfssFpub0 {
        &self.lfss_fpub_0
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn lfss_clksel(&self) -> &LfssClksel {
        &self.lfss_clksel
    }
    #[doc = "0x1020..0x104c - LFSS_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn lfss_cpu_int(&self, n: usize) -> &LfssCpuInt {
        &self.lfss_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - LFSS_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn lfss_cpu_int_iter(&self) -> impl Iterator<Item = &LfssCpuInt> {
        self.lfss_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - LFSS_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub const fn lfss_gen_event(&self, n: usize) -> &LfssGenEvent {
        &self.lfss_gen_event[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - LFSS_GEN_EVENT\\[%s\\]"]
    #[inline(always)]
    pub fn lfss_gen_event_iter(&self) -> impl Iterator<Item = &LfssGenEvent> {
        self.lfss_gen_event.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn lfss_evt_mode(&self) -> &LfssEvtMode {
        &self.lfss_evt_mode
    }
    #[doc = "0x10fc - LFSS Descriptor Register"]
    #[inline(always)]
    pub const fn lfss_desc(&self) -> &LfssDesc {
        &self.lfss_desc
    }
    #[doc = "0x1100 - RTC Clock Control Register"]
    #[inline(always)]
    pub const fn lfss_clkctl(&self) -> &LfssClkctl {
        &self.lfss_clkctl
    }
    #[doc = "0x1104 - RTC Module Debug Control Register"]
    #[inline(always)]
    pub const fn lfss_dbgctl(&self) -> &LfssDbgctl {
        &self.lfss_dbgctl
    }
    #[doc = "0x1108 - RTC Control Register"]
    #[inline(always)]
    pub const fn lfss_ctl(&self) -> &LfssCtl {
        &self.lfss_ctl
    }
    #[doc = "0x110c - RTC Status Register"]
    #[inline(always)]
    pub const fn lfss_sta(&self) -> &LfssSta {
        &self.lfss_sta
    }
    #[doc = "0x1110 - RTC Clock Offset Calibration Register"]
    #[inline(always)]
    pub const fn lfss_cal(&self) -> &LfssCal {
        &self.lfss_cal
    }
    #[doc = "0x1114 - RTC Temperature Compensation Register"]
    #[inline(always)]
    pub const fn lfss_tcmp(&self) -> &LfssTcmp {
        &self.lfss_tcmp
    }
    #[doc = "0x1118 - RTC Seconds Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_sec(&self) -> &LfssSec {
        &self.lfss_sec
    }
    #[doc = "0x111c - RTC Minutes Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_min(&self) -> &LfssMin {
        &self.lfss_min
    }
    #[doc = "0x1120 - RTC Hours Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_hour(&self) -> &LfssHour {
        &self.lfss_hour
    }
    #[doc = "0x1124 - RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_day(&self) -> &LfssDay {
        &self.lfss_day
    }
    #[doc = "0x1128 - RTC Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_mon(&self) -> &LfssMon {
        &self.lfss_mon
    }
    #[doc = "0x112c - RTC Year Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_year(&self) -> &LfssYear {
        &self.lfss_year
    }
    #[doc = "0x1130 - RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a1min(&self) -> &LfssA1min {
        &self.lfss_a1min
    }
    #[doc = "0x1134 - RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a1hour(&self) -> &LfssA1hour {
        &self.lfss_a1hour
    }
    #[doc = "0x1138 - RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a1day(&self) -> &LfssA1day {
        &self.lfss_a1day
    }
    #[doc = "0x113c - RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a2min(&self) -> &LfssA2min {
        &self.lfss_a2min
    }
    #[doc = "0x1140 - RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a2hour(&self) -> &LfssA2hour {
        &self.lfss_a2hour
    }
    #[doc = "0x1144 - RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_a2day(&self) -> &LfssA2day {
        &self.lfss_a2day
    }
    #[doc = "0x1148 - RTC Prescale Timer 0/1 Control Register"]
    #[inline(always)]
    pub const fn lfss_psctl(&self) -> &LfssPsctl {
        &self.lfss_psctl
    }
    #[doc = "0x114c - RTC Prescale Timer 2 Control Register"]
    #[inline(always)]
    pub const fn lfss_extpsctl(&self) -> &LfssExtpsctl {
        &self.lfss_extpsctl
    }
    #[doc = "0x1150 - Time Stamp Seconds Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tssec(&self) -> &LfssTssec {
        &self.lfss_tssec
    }
    #[doc = "0x1154 - Time Stamp Minutes Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tsmin(&self) -> &LfssTsmin {
        &self.lfss_tsmin
    }
    #[doc = "0x1158 - Time Stamp Hours Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tshour(&self) -> &LfssTshour {
        &self.lfss_tshour
    }
    #[doc = "0x115c - Time Stamp Day Of Week / MonthRegister - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tsday(&self) -> &LfssTsday {
        &self.lfss_tsday
    }
    #[doc = "0x1160 - Time Stamp Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tsmon(&self) -> &LfssTsmon {
        &self.lfss_tsmon
    }
    #[doc = "0x1164 - Time Stamp Years Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn lfss_tsyear(&self) -> &LfssTsyear {
        &self.lfss_tsyear
    }
    #[doc = "0x1168 - Time Stamp Status Register"]
    #[inline(always)]
    pub const fn lfss_tsstat(&self) -> &LfssTsstat {
        &self.lfss_tsstat
    }
    #[doc = "0x116c - Time Stamp Control Register"]
    #[inline(always)]
    pub const fn lfss_tsctl(&self) -> &LfssTsctl {
        &self.lfss_tsctl
    }
    #[doc = "0x1170 - Time Stamp Clear Register"]
    #[inline(always)]
    pub const fn lfss_tsclr(&self) -> &LfssTsclr {
        &self.lfss_tsclr
    }
    #[doc = "0x11f0 - Low frequency sub-system reset request"]
    #[inline(always)]
    pub const fn lfss_lfssrst(&self) -> &LfssLfssrst {
        &self.lfss_lfssrst
    }
    #[doc = "0x11fc - Real time clock lock register"]
    #[inline(always)]
    pub const fn lfss_rtclock(&self) -> &LfssRtclock {
        &self.lfss_rtclock
    }
    #[doc = "0x1200..0x1240 - Tamper I/O Control Register"]
    #[inline(always)]
    pub const fn lfss_tioctl(&self, n: usize) -> &LfssTioctl {
        &self.lfss_tioctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1200..0x1240 - Tamper I/O Control Register"]
    #[inline(always)]
    pub fn lfss_tioctl_iter(&self) -> impl Iterator<Item = &LfssTioctl> {
        self.lfss_tioctl.iter()
    }
    #[doc = "0x1280 - Tamper Output 3 to 0"]
    #[inline(always)]
    pub const fn lfss_tout3_0(&self) -> &LfssTout3_0 {
        &self.lfss_tout3_0
    }
    #[doc = "0x1284 - Tamper Output 7 to 4"]
    #[inline(always)]
    pub const fn lfss_tout7_4(&self) -> &LfssTout7_4 {
        &self.lfss_tout7_4
    }
    #[doc = "0x1288 - Tamper Output 11 to 8"]
    #[inline(always)]
    pub const fn lfss_tout11_8(&self) -> &LfssTout11_8 {
        &self.lfss_tout11_8
    }
    #[doc = "0x128c - Tamper Output 15 to 12"]
    #[inline(always)]
    pub const fn lfss_tout15_12(&self) -> &LfssTout15_12 {
        &self.lfss_tout15_12
    }
    #[doc = "0x1290 - Tamper Output Enable 3 to 0"]
    #[inline(always)]
    pub const fn lfss_toe3_0(&self) -> &LfssToe3_0 {
        &self.lfss_toe3_0
    }
    #[doc = "0x1294 - Tamper Output Enable 7 to 4"]
    #[inline(always)]
    pub const fn lfss_toe7_4(&self) -> &LfssToe7_4 {
        &self.lfss_toe7_4
    }
    #[doc = "0x1298 - Tamper Output Enable 7 to 4"]
    #[inline(always)]
    pub const fn lfss_toe11_8(&self) -> &LfssToe11_8 {
        &self.lfss_toe11_8
    }
    #[doc = "0x129c - Tamper Output Enable 7 to 4"]
    #[inline(always)]
    pub const fn lfss_toe15_12(&self) -> &LfssToe15_12 {
        &self.lfss_toe15_12
    }
    #[doc = "0x12a0 - Tamper Input Register"]
    #[inline(always)]
    pub const fn lfss_tin3_0(&self) -> &LfssTin3_0 {
        &self.lfss_tin3_0
    }
    #[doc = "0x12a4 - Tamper Input Register"]
    #[inline(always)]
    pub const fn lfss_tin7_4(&self) -> &LfssTin7_4 {
        &self.lfss_tin7_4
    }
    #[doc = "0x12a8 - Tamper Input Register"]
    #[inline(always)]
    pub const fn lfss_tin11_8(&self) -> &LfssTin11_8 {
        &self.lfss_tin11_8
    }
    #[doc = "0x12ac - Tamper Input Register"]
    #[inline(always)]
    pub const fn lfss_tin15_12(&self) -> &LfssTin15_12 {
        &self.lfss_tin15_12
    }
    #[doc = "0x12c0 - Heartbeat Register"]
    #[inline(always)]
    pub const fn lfss_heartbeat(&self) -> &LfssHeartbeat {
        &self.lfss_heartbeat
    }
    #[doc = "0x12fc - Tamper I/O lock register"]
    #[inline(always)]
    pub const fn lfss_tiolock(&self) -> &LfssTiolock {
        &self.lfss_tiolock
    }
    #[doc = "0x1300 - Watchdog Timer Enable Register"]
    #[inline(always)]
    pub const fn lfss_wdten(&self) -> &LfssWdten {
        &self.lfss_wdten
    }
    #[doc = "0x1304 - Watchdog Timer Debug Control Register"]
    #[inline(always)]
    pub const fn lfss_wdtdbgctl(&self) -> &LfssWdtdbgctl {
        &self.lfss_wdtdbgctl
    }
    #[doc = "0x1308 - Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn lfss_wdtctl(&self) -> &LfssWdtctl {
        &self.lfss_wdtctl
    }
    #[doc = "0x130c - Watchdog Timer Counter Reset Register"]
    #[inline(always)]
    pub const fn lfss_wdtcntrst(&self) -> &LfssWdtcntrst {
        &self.lfss_wdtcntrst
    }
    #[doc = "0x1310 - Watchdog Timer Status Register"]
    #[inline(always)]
    pub const fn lfss_wdtstat(&self) -> &LfssWdtstat {
        &self.lfss_wdtstat
    }
    #[doc = "0x13fc - Watchdog timer lock register"]
    #[inline(always)]
    pub const fn lfss_wdtlock(&self) -> &LfssWdtlock {
        &self.lfss_wdtlock
    }
    #[doc = "0x1400..0x1480 - Scratch Pad Memory Data Register"]
    #[inline(always)]
    pub const fn lfss_spmem(&self, n: usize) -> &LfssSpmem {
        &self.lfss_spmem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x1480 - Scratch Pad Memory Data Register"]
    #[inline(always)]
    pub fn lfss_spmem_iter(&self) -> impl Iterator<Item = &LfssSpmem> {
        self.lfss_spmem.iter()
    }
    #[doc = "0x1500 - Scratch Pad Memory Write Protect Register 0"]
    #[inline(always)]
    pub const fn lfss_spmwprot0(&self) -> &LfssSpmwprot0 {
        &self.lfss_spmwprot0
    }
    #[doc = "0x1504 - Scratch Pad Memory Write Protect Register 1"]
    #[inline(always)]
    pub const fn lfss_spmwprot1(&self) -> &LfssSpmwprot1 {
        &self.lfss_spmwprot1
    }
    #[doc = "0x1508 - Scratch Pad Memory Write Protect Register 2"]
    #[inline(always)]
    pub const fn lfss_spmwprot2(&self) -> &LfssSpmwprot2 {
        &self.lfss_spmwprot2
    }
    #[doc = "0x150c - Scratch Pad Memory Write Protect Register 3"]
    #[inline(always)]
    pub const fn lfss_spmwprot3(&self) -> &LfssSpmwprot3 {
        &self.lfss_spmwprot3
    }
    #[doc = "0x1510 - Scratch Pad Memory Write Protect Register 4"]
    #[inline(always)]
    pub const fn lfss_spmwprot4(&self) -> &LfssSpmwprot4 {
        &self.lfss_spmwprot4
    }
    #[doc = "0x1514 - Scratch Pad Memory Write Protect Register 5"]
    #[inline(always)]
    pub const fn lfss_spmwprot5(&self) -> &LfssSpmwprot5 {
        &self.lfss_spmwprot5
    }
    #[doc = "0x1518 - Scratch Pad Memory Write Protect Register 6"]
    #[inline(always)]
    pub const fn lfss_spmwprot6(&self) -> &LfssSpmwprot6 {
        &self.lfss_spmwprot6
    }
    #[doc = "0x151c - Scratch Pad Memory Write Protect Register 7"]
    #[inline(always)]
    pub const fn lfss_spmwprot7(&self) -> &LfssSpmwprot7 {
        &self.lfss_spmwprot7
    }
    #[doc = "0x1540 - Scratch Pad Memory Tamper Erase Register 0"]
    #[inline(always)]
    pub const fn lfss_spmterase0(&self) -> &LfssSpmterase0 {
        &self.lfss_spmterase0
    }
    #[doc = "0x1544 - Scratch Pad Memory Tamper Erase Register 1"]
    #[inline(always)]
    pub const fn lfss_spmterase1(&self) -> &LfssSpmterase1 {
        &self.lfss_spmterase1
    }
    #[doc = "0x1548 - Scratch Pad Memory Tamper Erase Register 2"]
    #[inline(always)]
    pub const fn lfss_spmterase2(&self) -> &LfssSpmterase2 {
        &self.lfss_spmterase2
    }
    #[doc = "0x154c - Scratch Pad Memory Tamper Erase Register 3"]
    #[inline(always)]
    pub const fn lfss_spmterase3(&self) -> &LfssSpmterase3 {
        &self.lfss_spmterase3
    }
    #[doc = "0x1550 - Scratch Pad Memory Tamper Erase Register 4"]
    #[inline(always)]
    pub const fn lfss_spmterase4(&self) -> &LfssSpmterase4 {
        &self.lfss_spmterase4
    }
    #[doc = "0x1554 - Scratch Pad Memory Tamper Erase Register 5"]
    #[inline(always)]
    pub const fn lfss_spmterase5(&self) -> &LfssSpmterase5 {
        &self.lfss_spmterase5
    }
    #[doc = "0x1558 - Scratch Pad Memory Tamper Erase Register 6"]
    #[inline(always)]
    pub const fn lfss_spmterase6(&self) -> &LfssSpmterase6 {
        &self.lfss_spmterase6
    }
    #[doc = "0x155c - Scratch Pad Memory Tamper Erase Register 7"]
    #[inline(always)]
    pub const fn lfss_spmterase7(&self) -> &LfssSpmterase7 {
        &self.lfss_spmterase7
    }
}
#[doc = "LFSS_FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_fpub_0`] module"]
#[doc(alias = "LFSS_FPUB_0")]
pub type LfssFpub0 = crate::Reg<lfss_fpub_0::LfssFpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod lfss_fpub_0;
#[doc = "LFSS_CLKSEL (r) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_clksel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_clksel`] module"]
#[doc(alias = "LFSS_CLKSEL")]
pub type LfssClksel = crate::Reg<lfss_clksel::LfssClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod lfss_clksel;
#[doc = "LFSS_CPU_INT\\[%s\\]"]
pub use self::lfss_cpu_int::LfssCpuInt;
#[doc = r"Cluster"]
#[doc = "LFSS_CPU_INT\\[%s\\]"]
pub mod lfss_cpu_int;
#[doc = "LFSS_GEN_EVENT\\[%s\\]"]
pub use self::lfss_gen_event::LfssGenEvent;
#[doc = r"Cluster"]
#[doc = "LFSS_GEN_EVENT\\[%s\\]"]
pub mod lfss_gen_event;
#[doc = "LFSS_EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_evt_mode`] module"]
#[doc(alias = "LFSS_EVT_MODE")]
pub type LfssEvtMode = crate::Reg<lfss_evt_mode::LfssEvtModeSpec>;
#[doc = "Event Mode"]
pub mod lfss_evt_mode;
#[doc = "LFSS_DESC (r) register accessor: LFSS Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_desc`] module"]
#[doc(alias = "LFSS_DESC")]
pub type LfssDesc = crate::Reg<lfss_desc::LfssDescSpec>;
#[doc = "LFSS Descriptor Register"]
pub mod lfss_desc;
#[doc = "LFSS_CLKCTL (rw) register accessor: RTC Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_clkctl`] module"]
#[doc(alias = "LFSS_CLKCTL")]
pub type LfssClkctl = crate::Reg<lfss_clkctl::LfssClkctlSpec>;
#[doc = "RTC Clock Control Register"]
pub mod lfss_clkctl;
#[doc = "LFSS_DBGCTL (rw) register accessor: RTC Module Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_dbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_dbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_dbgctl`] module"]
#[doc(alias = "LFSS_DBGCTL")]
pub type LfssDbgctl = crate::Reg<lfss_dbgctl::LfssDbgctlSpec>;
#[doc = "RTC Module Debug Control Register"]
pub mod lfss_dbgctl;
#[doc = "LFSS_CTL (rw) register accessor: RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_ctl`] module"]
#[doc(alias = "LFSS_CTL")]
pub type LfssCtl = crate::Reg<lfss_ctl::LfssCtlSpec>;
#[doc = "RTC Control Register"]
pub mod lfss_ctl;
#[doc = "LFSS_STA (r) register accessor: RTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_sta`] module"]
#[doc(alias = "LFSS_STA")]
pub type LfssSta = crate::Reg<lfss_sta::LfssStaSpec>;
#[doc = "RTC Status Register"]
pub mod lfss_sta;
#[doc = "LFSS_CAL (rw) register accessor: RTC Clock Offset Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_cal`] module"]
#[doc(alias = "LFSS_CAL")]
pub type LfssCal = crate::Reg<lfss_cal::LfssCalSpec>;
#[doc = "RTC Clock Offset Calibration Register"]
pub mod lfss_cal;
#[doc = "LFSS_TCMP (rw) register accessor: RTC Temperature Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tcmp`] module"]
#[doc(alias = "LFSS_TCMP")]
pub type LfssTcmp = crate::Reg<lfss_tcmp::LfssTcmpSpec>;
#[doc = "RTC Temperature Compensation Register"]
pub mod lfss_tcmp;
#[doc = "LFSS_SEC (rw) register accessor: RTC Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_sec`] module"]
#[doc(alias = "LFSS_SEC")]
pub type LfssSec = crate::Reg<lfss_sec::LfssSecSpec>;
#[doc = "RTC Seconds Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_sec;
#[doc = "LFSS_MIN (rw) register accessor: RTC Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_min`] module"]
#[doc(alias = "LFSS_MIN")]
pub type LfssMin = crate::Reg<lfss_min::LfssMinSpec>;
#[doc = "RTC Minutes Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_min;
#[doc = "LFSS_HOUR (rw) register accessor: RTC Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_hour`] module"]
#[doc(alias = "LFSS_HOUR")]
pub type LfssHour = crate::Reg<lfss_hour::LfssHourSpec>;
#[doc = "RTC Hours Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_hour;
#[doc = "LFSS_DAY (rw) register accessor: RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_day`] module"]
#[doc(alias = "LFSS_DAY")]
pub type LfssDay = crate::Reg<lfss_day::LfssDaySpec>;
#[doc = "RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_day;
#[doc = "LFSS_MON (rw) register accessor: RTC Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_mon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_mon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_mon`] module"]
#[doc(alias = "LFSS_MON")]
pub type LfssMon = crate::Reg<lfss_mon::LfssMonSpec>;
#[doc = "RTC Month Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_mon;
#[doc = "LFSS_YEAR (rw) register accessor: RTC Year Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_year::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_year::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_year`] module"]
#[doc(alias = "LFSS_YEAR")]
pub type LfssYear = crate::Reg<lfss_year::LfssYearSpec>;
#[doc = "RTC Year Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_year;
#[doc = "LFSS_A1MIN (rw) register accessor: RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a1min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a1min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a1min`] module"]
#[doc(alias = "LFSS_A1MIN")]
pub type LfssA1min = crate::Reg<lfss_a1min::LfssA1minSpec>;
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a1min;
#[doc = "LFSS_A1HOUR (rw) register accessor: RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a1hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a1hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a1hour`] module"]
#[doc(alias = "LFSS_A1HOUR")]
pub type LfssA1hour = crate::Reg<lfss_a1hour::LfssA1hourSpec>;
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a1hour;
#[doc = "LFSS_A1DAY (rw) register accessor: RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a1day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a1day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a1day`] module"]
#[doc(alias = "LFSS_A1DAY")]
pub type LfssA1day = crate::Reg<lfss_a1day::LfssA1daySpec>;
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a1day;
#[doc = "LFSS_A2MIN (rw) register accessor: RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a2min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a2min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a2min`] module"]
#[doc(alias = "LFSS_A2MIN")]
pub type LfssA2min = crate::Reg<lfss_a2min::LfssA2minSpec>;
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a2min;
#[doc = "LFSS_A2HOUR (rw) register accessor: RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a2hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a2hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a2hour`] module"]
#[doc(alias = "LFSS_A2HOUR")]
pub type LfssA2hour = crate::Reg<lfss_a2hour::LfssA2hourSpec>;
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a2hour;
#[doc = "LFSS_A2DAY (rw) register accessor: RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_a2day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_a2day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_a2day`] module"]
#[doc(alias = "LFSS_A2DAY")]
pub type LfssA2day = crate::Reg<lfss_a2day::LfssA2daySpec>;
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_a2day;
#[doc = "LFSS_PSCTL (rw) register accessor: RTC Prescale Timer 0/1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_psctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_psctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_psctl`] module"]
#[doc(alias = "LFSS_PSCTL")]
pub type LfssPsctl = crate::Reg<lfss_psctl::LfssPsctlSpec>;
#[doc = "RTC Prescale Timer 0/1 Control Register"]
pub mod lfss_psctl;
#[doc = "LFSS_EXTPSCTL (rw) register accessor: RTC Prescale Timer 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_extpsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_extpsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_extpsctl`] module"]
#[doc(alias = "LFSS_EXTPSCTL")]
pub type LfssExtpsctl = crate::Reg<lfss_extpsctl::LfssExtpsctlSpec>;
#[doc = "RTC Prescale Timer 2 Control Register"]
pub mod lfss_extpsctl;
#[doc = "LFSS_TSSEC (r) register accessor: Time Stamp Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tssec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tssec`] module"]
#[doc(alias = "LFSS_TSSEC")]
pub type LfssTssec = crate::Reg<lfss_tssec::LfssTssecSpec>;
#[doc = "Time Stamp Seconds Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tssec;
#[doc = "LFSS_TSMIN (r) register accessor: Time Stamp Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsmin`] module"]
#[doc(alias = "LFSS_TSMIN")]
pub type LfssTsmin = crate::Reg<lfss_tsmin::LfssTsminSpec>;
#[doc = "Time Stamp Minutes Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tsmin;
#[doc = "LFSS_TSHOUR (r) register accessor: Time Stamp Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tshour::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tshour`] module"]
#[doc(alias = "LFSS_TSHOUR")]
pub type LfssTshour = crate::Reg<lfss_tshour::LfssTshourSpec>;
#[doc = "Time Stamp Hours Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tshour;
#[doc = "LFSS_TSDAY (r) register accessor: Time Stamp Day Of Week / MonthRegister - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsday::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsday`] module"]
#[doc(alias = "LFSS_TSDAY")]
pub type LfssTsday = crate::Reg<lfss_tsday::LfssTsdaySpec>;
#[doc = "Time Stamp Day Of Week / MonthRegister - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tsday;
#[doc = "LFSS_TSMON (r) register accessor: Time Stamp Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsmon`] module"]
#[doc(alias = "LFSS_TSMON")]
pub type LfssTsmon = crate::Reg<lfss_tsmon::LfssTsmonSpec>;
#[doc = "Time Stamp Month Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tsmon;
#[doc = "LFSS_TSYEAR (r) register accessor: Time Stamp Years Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsyear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsyear`] module"]
#[doc(alias = "LFSS_TSYEAR")]
pub type LfssTsyear = crate::Reg<lfss_tsyear::LfssTsyearSpec>;
#[doc = "Time Stamp Years Register - Calendar Mode With Binary / BCD Format"]
pub mod lfss_tsyear;
#[doc = "LFSS_TSSTAT (r) register accessor: Time Stamp Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsstat`] module"]
#[doc(alias = "LFSS_TSSTAT")]
pub type LfssTsstat = crate::Reg<lfss_tsstat::LfssTsstatSpec>;
#[doc = "Time Stamp Status Register"]
pub mod lfss_tsstat;
#[doc = "LFSS_TSCTL (rw) register accessor: Time Stamp Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsctl`] module"]
#[doc(alias = "LFSS_TSCTL")]
pub type LfssTsctl = crate::Reg<lfss_tsctl::LfssTsctlSpec>;
#[doc = "Time Stamp Control Register"]
pub mod lfss_tsctl;
#[doc = "LFSS_TSCLR (w) register accessor: Time Stamp Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tsclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tsclr`] module"]
#[doc(alias = "LFSS_TSCLR")]
pub type LfssTsclr = crate::Reg<lfss_tsclr::LfssTsclrSpec>;
#[doc = "Time Stamp Clear Register"]
pub mod lfss_tsclr;
#[doc = "LFSS_LFSSRST (rw) register accessor: Low frequency sub-system reset request\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_lfssrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_lfssrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_lfssrst`] module"]
#[doc(alias = "LFSS_LFSSRST")]
pub type LfssLfssrst = crate::Reg<lfss_lfssrst::LfssLfssrstSpec>;
#[doc = "Low frequency sub-system reset request"]
pub mod lfss_lfssrst;
#[doc = "LFSS_RTCLOCK (rw) register accessor: Real time clock lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_rtclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_rtclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_rtclock`] module"]
#[doc(alias = "LFSS_RTCLOCK")]
pub type LfssRtclock = crate::Reg<lfss_rtclock::LfssRtclockSpec>;
#[doc = "Real time clock lock register"]
pub mod lfss_rtclock;
#[doc = "LFSS_TIOCTL (rw) register accessor: Tamper I/O Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tioctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tioctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tioctl`] module"]
#[doc(alias = "LFSS_TIOCTL")]
pub type LfssTioctl = crate::Reg<lfss_tioctl::LfssTioctlSpec>;
#[doc = "Tamper I/O Control Register"]
pub mod lfss_tioctl;
#[doc = "LFSS_TOUT3_0 (rw) register accessor: Tamper Output 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tout3_0`] module"]
#[doc(alias = "LFSS_TOUT3_0")]
pub type LfssTout3_0 = crate::Reg<lfss_tout3_0::LfssTout3_0Spec>;
#[doc = "Tamper Output 3 to 0"]
pub mod lfss_tout3_0;
#[doc = "LFSS_TOUT7_4 (rw) register accessor: Tamper Output 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout7_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout7_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tout7_4`] module"]
#[doc(alias = "LFSS_TOUT7_4")]
pub type LfssTout7_4 = crate::Reg<lfss_tout7_4::LfssTout7_4Spec>;
#[doc = "Tamper Output 7 to 4"]
pub mod lfss_tout7_4;
#[doc = "LFSS_TOUT11_8 (rw) register accessor: Tamper Output 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout11_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout11_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tout11_8`] module"]
#[doc(alias = "LFSS_TOUT11_8")]
pub type LfssTout11_8 = crate::Reg<lfss_tout11_8::LfssTout11_8Spec>;
#[doc = "Tamper Output 11 to 8"]
pub mod lfss_tout11_8;
#[doc = "LFSS_TOUT15_12 (rw) register accessor: Tamper Output 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tout15_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tout15_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tout15_12`] module"]
#[doc(alias = "LFSS_TOUT15_12")]
pub type LfssTout15_12 = crate::Reg<lfss_tout15_12::LfssTout15_12Spec>;
#[doc = "Tamper Output 15 to 12"]
pub mod lfss_tout15_12;
#[doc = "LFSS_TOE3_0 (rw) register accessor: Tamper Output Enable 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_toe3_0`] module"]
#[doc(alias = "LFSS_TOE3_0")]
pub type LfssToe3_0 = crate::Reg<lfss_toe3_0::LfssToe3_0Spec>;
#[doc = "Tamper Output Enable 3 to 0"]
pub mod lfss_toe3_0;
#[doc = "LFSS_TOE7_4 (rw) register accessor: Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe7_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe7_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_toe7_4`] module"]
#[doc(alias = "LFSS_TOE7_4")]
pub type LfssToe7_4 = crate::Reg<lfss_toe7_4::LfssToe7_4Spec>;
#[doc = "Tamper Output Enable 7 to 4"]
pub mod lfss_toe7_4;
#[doc = "LFSS_TOE11_8 (rw) register accessor: Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe11_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe11_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_toe11_8`] module"]
#[doc(alias = "LFSS_TOE11_8")]
pub type LfssToe11_8 = crate::Reg<lfss_toe11_8::LfssToe11_8Spec>;
#[doc = "Tamper Output Enable 7 to 4"]
pub mod lfss_toe11_8;
#[doc = "LFSS_TOE15_12 (rw) register accessor: Tamper Output Enable 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_toe15_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_toe15_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_toe15_12`] module"]
#[doc(alias = "LFSS_TOE15_12")]
pub type LfssToe15_12 = crate::Reg<lfss_toe15_12::LfssToe15_12Spec>;
#[doc = "Tamper Output Enable 7 to 4"]
pub mod lfss_toe15_12;
#[doc = "LFSS_TIN3_0 (r) register accessor: Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tin3_0`] module"]
#[doc(alias = "LFSS_TIN3_0")]
pub type LfssTin3_0 = crate::Reg<lfss_tin3_0::LfssTin3_0Spec>;
#[doc = "Tamper Input Register"]
pub mod lfss_tin3_0;
#[doc = "LFSS_TIN7_4 (r) register accessor: Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin7_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tin7_4`] module"]
#[doc(alias = "LFSS_TIN7_4")]
pub type LfssTin7_4 = crate::Reg<lfss_tin7_4::LfssTin7_4Spec>;
#[doc = "Tamper Input Register"]
pub mod lfss_tin7_4;
#[doc = "LFSS_TIN11_8 (r) register accessor: Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin11_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tin11_8`] module"]
#[doc(alias = "LFSS_TIN11_8")]
pub type LfssTin11_8 = crate::Reg<lfss_tin11_8::LfssTin11_8Spec>;
#[doc = "Tamper Input Register"]
pub mod lfss_tin11_8;
#[doc = "LFSS_TIN15_12 (r) register accessor: Tamper Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tin15_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tin15_12`] module"]
#[doc(alias = "LFSS_TIN15_12")]
pub type LfssTin15_12 = crate::Reg<lfss_tin15_12::LfssTin15_12Spec>;
#[doc = "Tamper Input Register"]
pub mod lfss_tin15_12;
#[doc = "LFSS_HEARTBEAT (rw) register accessor: Heartbeat Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_heartbeat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_heartbeat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_heartbeat`] module"]
#[doc(alias = "LFSS_HEARTBEAT")]
pub type LfssHeartbeat = crate::Reg<lfss_heartbeat::LfssHeartbeatSpec>;
#[doc = "Heartbeat Register"]
pub mod lfss_heartbeat;
#[doc = "LFSS_TIOLOCK (rw) register accessor: Tamper I/O lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tiolock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tiolock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_tiolock`] module"]
#[doc(alias = "LFSS_TIOLOCK")]
pub type LfssTiolock = crate::Reg<lfss_tiolock::LfssTiolockSpec>;
#[doc = "Tamper I/O lock register"]
pub mod lfss_tiolock;
#[doc = "LFSS_WDTEN (rw) register accessor: Watchdog Timer Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdten`] module"]
#[doc(alias = "LFSS_WDTEN")]
pub type LfssWdten = crate::Reg<lfss_wdten::LfssWdtenSpec>;
#[doc = "Watchdog Timer Enable Register"]
pub mod lfss_wdten;
#[doc = "LFSS_WDTDBGCTL (rw) register accessor: Watchdog Timer Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdtdbgctl`] module"]
#[doc(alias = "LFSS_WDTDBGCTL")]
pub type LfssWdtdbgctl = crate::Reg<lfss_wdtdbgctl::LfssWdtdbgctlSpec>;
#[doc = "Watchdog Timer Debug Control Register"]
pub mod lfss_wdtdbgctl;
#[doc = "LFSS_WDTCTL (rw) register accessor: Watchdog Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdtctl`] module"]
#[doc(alias = "LFSS_WDTCTL")]
pub type LfssWdtctl = crate::Reg<lfss_wdtctl::LfssWdtctlSpec>;
#[doc = "Watchdog Timer Control Register"]
pub mod lfss_wdtctl;
#[doc = "LFSS_WDTCNTRST (w) register accessor: Watchdog Timer Counter Reset Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtcntrst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdtcntrst`] module"]
#[doc(alias = "LFSS_WDTCNTRST")]
pub type LfssWdtcntrst = crate::Reg<lfss_wdtcntrst::LfssWdtcntrstSpec>;
#[doc = "Watchdog Timer Counter Reset Register"]
pub mod lfss_wdtcntrst;
#[doc = "LFSS_WDTSTAT (r) register accessor: Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdtstat`] module"]
#[doc(alias = "LFSS_WDTSTAT")]
pub type LfssWdtstat = crate::Reg<lfss_wdtstat::LfssWdtstatSpec>;
#[doc = "Watchdog Timer Status Register"]
pub mod lfss_wdtstat;
#[doc = "LFSS_WDTLOCK (rw) register accessor: Watchdog timer lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_wdtlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_wdtlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_wdtlock`] module"]
#[doc(alias = "LFSS_WDTLOCK")]
pub type LfssWdtlock = crate::Reg<lfss_wdtlock::LfssWdtlockSpec>;
#[doc = "Watchdog timer lock register"]
pub mod lfss_wdtlock;
#[doc = "LFSS_SPMEM (rw) register accessor: Scratch Pad Memory Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmem`] module"]
#[doc(alias = "LFSS_SPMEM")]
pub type LfssSpmem = crate::Reg<lfss_spmem::LfssSpmemSpec>;
#[doc = "Scratch Pad Memory Data Register"]
pub mod lfss_spmem;
#[doc = "LFSS_SPMWPROT0 (rw) register accessor: Scratch Pad Memory Write Protect Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot0`] module"]
#[doc(alias = "LFSS_SPMWPROT0")]
pub type LfssSpmwprot0 = crate::Reg<lfss_spmwprot0::LfssSpmwprot0Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 0"]
pub mod lfss_spmwprot0;
#[doc = "LFSS_SPMWPROT1 (rw) register accessor: Scratch Pad Memory Write Protect Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot1`] module"]
#[doc(alias = "LFSS_SPMWPROT1")]
pub type LfssSpmwprot1 = crate::Reg<lfss_spmwprot1::LfssSpmwprot1Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 1"]
pub mod lfss_spmwprot1;
#[doc = "LFSS_SPMWPROT2 (rw) register accessor: Scratch Pad Memory Write Protect Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot2`] module"]
#[doc(alias = "LFSS_SPMWPROT2")]
pub type LfssSpmwprot2 = crate::Reg<lfss_spmwprot2::LfssSpmwprot2Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 2"]
pub mod lfss_spmwprot2;
#[doc = "LFSS_SPMWPROT3 (rw) register accessor: Scratch Pad Memory Write Protect Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot3`] module"]
#[doc(alias = "LFSS_SPMWPROT3")]
pub type LfssSpmwprot3 = crate::Reg<lfss_spmwprot3::LfssSpmwprot3Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 3"]
pub mod lfss_spmwprot3;
#[doc = "LFSS_SPMWPROT4 (rw) register accessor: Scratch Pad Memory Write Protect Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot4`] module"]
#[doc(alias = "LFSS_SPMWPROT4")]
pub type LfssSpmwprot4 = crate::Reg<lfss_spmwprot4::LfssSpmwprot4Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 4"]
pub mod lfss_spmwprot4;
#[doc = "LFSS_SPMWPROT5 (rw) register accessor: Scratch Pad Memory Write Protect Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot5`] module"]
#[doc(alias = "LFSS_SPMWPROT5")]
pub type LfssSpmwprot5 = crate::Reg<lfss_spmwprot5::LfssSpmwprot5Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 5"]
pub mod lfss_spmwprot5;
#[doc = "LFSS_SPMWPROT6 (rw) register accessor: Scratch Pad Memory Write Protect Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot6`] module"]
#[doc(alias = "LFSS_SPMWPROT6")]
pub type LfssSpmwprot6 = crate::Reg<lfss_spmwprot6::LfssSpmwprot6Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 6"]
pub mod lfss_spmwprot6;
#[doc = "LFSS_SPMWPROT7 (rw) register accessor: Scratch Pad Memory Write Protect Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmwprot7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmwprot7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmwprot7`] module"]
#[doc(alias = "LFSS_SPMWPROT7")]
pub type LfssSpmwprot7 = crate::Reg<lfss_spmwprot7::LfssSpmwprot7Spec>;
#[doc = "Scratch Pad Memory Write Protect Register 7"]
pub mod lfss_spmwprot7;
#[doc = "LFSS_SPMTERASE0 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase0`] module"]
#[doc(alias = "LFSS_SPMTERASE0")]
pub type LfssSpmterase0 = crate::Reg<lfss_spmterase0::LfssSpmterase0Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 0"]
pub mod lfss_spmterase0;
#[doc = "LFSS_SPMTERASE1 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase1`] module"]
#[doc(alias = "LFSS_SPMTERASE1")]
pub type LfssSpmterase1 = crate::Reg<lfss_spmterase1::LfssSpmterase1Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 1"]
pub mod lfss_spmterase1;
#[doc = "LFSS_SPMTERASE2 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase2`] module"]
#[doc(alias = "LFSS_SPMTERASE2")]
pub type LfssSpmterase2 = crate::Reg<lfss_spmterase2::LfssSpmterase2Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 2"]
pub mod lfss_spmterase2;
#[doc = "LFSS_SPMTERASE3 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase3`] module"]
#[doc(alias = "LFSS_SPMTERASE3")]
pub type LfssSpmterase3 = crate::Reg<lfss_spmterase3::LfssSpmterase3Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 3"]
pub mod lfss_spmterase3;
#[doc = "LFSS_SPMTERASE4 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase4`] module"]
#[doc(alias = "LFSS_SPMTERASE4")]
pub type LfssSpmterase4 = crate::Reg<lfss_spmterase4::LfssSpmterase4Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 4"]
pub mod lfss_spmterase4;
#[doc = "LFSS_SPMTERASE5 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase5`] module"]
#[doc(alias = "LFSS_SPMTERASE5")]
pub type LfssSpmterase5 = crate::Reg<lfss_spmterase5::LfssSpmterase5Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 5"]
pub mod lfss_spmterase5;
#[doc = "LFSS_SPMTERASE6 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase6`] module"]
#[doc(alias = "LFSS_SPMTERASE6")]
pub type LfssSpmterase6 = crate::Reg<lfss_spmterase6::LfssSpmterase6Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 6"]
pub mod lfss_spmterase6;
#[doc = "LFSS_SPMTERASE7 (rw) register accessor: Scratch Pad Memory Tamper Erase Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmterase7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmterase7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfss_spmterase7`] module"]
#[doc(alias = "LFSS_SPMTERASE7")]
pub type LfssSpmterase7 = crate::Reg<lfss_spmterase7::LfssSpmterase7Spec>;
#[doc = "Scratch Pad Memory Tamper Erase Register 7"]
pub mod lfss_spmterase7;
