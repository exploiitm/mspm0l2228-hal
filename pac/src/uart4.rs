#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    uart4_gprcm: [Uart4Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    uart4_clkdiv: Uart4Clkdiv,
    _reserved2: [u8; 0x04],
    uart4_clksel: Uart4Clksel,
    _reserved3: [u8; 0x0c],
    uart4_pdbgctl: Uart4Pdbgctl,
    _reserved4: [u8; 0x04],
    uart4_int_event0: [Uart4IntEvent0; 1],
    _reserved5: [u8; 0x04],
    uart4_int_event1: [Uart4IntEvent1; 1],
    _reserved6: [u8; 0x04],
    uart4_int_event2: [Uart4IntEvent2; 1],
    _reserved7: [u8; 0x34],
    uart4_evt_mode: Uart4EvtMode,
    uart4_intctl: Uart4Intctl,
    _reserved9: [u8; 0x18],
    uart4_ctl0: Uart4Ctl0,
    uart4_lcrh: Uart4Lcrh,
    uart4_stat: Uart4Stat,
    uart4_ifls: Uart4Ifls,
    uart4_ibrd: Uart4Ibrd,
    uart4_fbrd: Uart4Fbrd,
    uart4_gfctl: Uart4Gfctl,
    _reserved16: [u8; 0x04],
    uart4_txdata: Uart4Txdata,
    uart4_rxdata: Uart4Rxdata,
    _reserved18: [u8; 0x20],
    uart4_amask: Uart4Amask,
    uart4_addr: Uart4Addr,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - UART4_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn uart4_gprcm(&self, n: usize) -> &Uart4Gprcm {
        &self.uart4_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - UART4_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn uart4_gprcm_iter(&self) -> impl Iterator<Item = &Uart4Gprcm> {
        self.uart4_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn uart4_clkdiv(&self) -> &Uart4Clkdiv {
        &self.uart4_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn uart4_clksel(&self) -> &Uart4Clksel {
        &self.uart4_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn uart4_pdbgctl(&self) -> &Uart4Pdbgctl {
        &self.uart4_pdbgctl
    }
    #[doc = "0x1020..0x104c - UART4_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn uart4_int_event0(&self, n: usize) -> &Uart4IntEvent0 {
        &self.uart4_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - UART4_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn uart4_int_event0_iter(&self) -> impl Iterator<Item = &Uart4IntEvent0> {
        self.uart4_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - UART4_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn uart4_int_event1(&self, n: usize) -> &Uart4IntEvent1 {
        &self.uart4_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - UART4_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn uart4_int_event1_iter(&self) -> impl Iterator<Item = &Uart4IntEvent1> {
        self.uart4_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - UART4_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn uart4_int_event2(&self, n: usize) -> &Uart4IntEvent2 {
        &self.uart4_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - UART4_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn uart4_int_event2_iter(&self) -> impl Iterator<Item = &Uart4IntEvent2> {
        self.uart4_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn uart4_evt_mode(&self) -> &Uart4EvtMode {
        &self.uart4_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn uart4_intctl(&self) -> &Uart4Intctl {
        &self.uart4_intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn uart4_ctl0(&self) -> &Uart4Ctl0 {
        &self.uart4_ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn uart4_lcrh(&self) -> &Uart4Lcrh {
        &self.uart4_lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn uart4_stat(&self) -> &Uart4Stat {
        &self.uart4_stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn uart4_ifls(&self) -> &Uart4Ifls {
        &self.uart4_ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart4_ibrd(&self) -> &Uart4Ibrd {
        &self.uart4_ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart4_fbrd(&self) -> &Uart4Fbrd {
        &self.uart4_fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn uart4_gfctl(&self) -> &Uart4Gfctl {
        &self.uart4_gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn uart4_txdata(&self) -> &Uart4Txdata {
        &self.uart4_txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn uart4_rxdata(&self) -> &Uart4Rxdata {
        &self.uart4_rxdata
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn uart4_amask(&self) -> &Uart4Amask {
        &self.uart4_amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn uart4_addr(&self) -> &Uart4Addr {
        &self.uart4_addr
    }
}
#[doc = "UART4_GPRCM\\[%s\\]"]
pub use self::uart4_gprcm::Uart4Gprcm;
#[doc = r"Cluster"]
#[doc = "UART4_GPRCM\\[%s\\]"]
pub mod uart4_gprcm;
#[doc = "UART4_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_clkdiv`] module"]
#[doc(alias = "UART4_CLKDIV")]
pub type Uart4Clkdiv = crate::Reg<uart4_clkdiv::Uart4ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod uart4_clkdiv;
#[doc = "UART4_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_clksel`] module"]
#[doc(alias = "UART4_CLKSEL")]
pub type Uart4Clksel = crate::Reg<uart4_clksel::Uart4ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod uart4_clksel;
#[doc = "UART4_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_pdbgctl`] module"]
#[doc(alias = "UART4_PDBGCTL")]
pub type Uart4Pdbgctl = crate::Reg<uart4_pdbgctl::Uart4PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod uart4_pdbgctl;
#[doc = "UART4_INT_EVENT0\\[%s\\]"]
pub use self::uart4_int_event0::Uart4IntEvent0;
#[doc = r"Cluster"]
#[doc = "UART4_INT_EVENT0\\[%s\\]"]
pub mod uart4_int_event0;
#[doc = "UART4_INT_EVENT1\\[%s\\]"]
pub use self::uart4_int_event1::Uart4IntEvent1;
#[doc = r"Cluster"]
#[doc = "UART4_INT_EVENT1\\[%s\\]"]
pub mod uart4_int_event1;
#[doc = "UART4_INT_EVENT2\\[%s\\]"]
pub use self::uart4_int_event2::Uart4IntEvent2;
#[doc = r"Cluster"]
#[doc = "UART4_INT_EVENT2\\[%s\\]"]
pub mod uart4_int_event2;
#[doc = "UART4_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_evt_mode`] module"]
#[doc(alias = "UART4_EVT_MODE")]
pub type Uart4EvtMode = crate::Reg<uart4_evt_mode::Uart4EvtModeSpec>;
#[doc = "Event Mode"]
pub mod uart4_evt_mode;
#[doc = "UART4_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_intctl`] module"]
#[doc(alias = "UART4_INTCTL")]
pub type Uart4Intctl = crate::Reg<uart4_intctl::Uart4IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod uart4_intctl;
#[doc = "UART4_CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_ctl0`] module"]
#[doc(alias = "UART4_CTL0")]
pub type Uart4Ctl0 = crate::Reg<uart4_ctl0::Uart4Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod uart4_ctl0;
#[doc = "UART4_LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_lcrh`] module"]
#[doc(alias = "UART4_LCRH")]
pub type Uart4Lcrh = crate::Reg<uart4_lcrh::Uart4LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod uart4_lcrh;
#[doc = "UART4_STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_stat`] module"]
#[doc(alias = "UART4_STAT")]
pub type Uart4Stat = crate::Reg<uart4_stat::Uart4StatSpec>;
#[doc = "UART Status Register"]
pub mod uart4_stat;
#[doc = "UART4_IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_ifls`] module"]
#[doc(alias = "UART4_IFLS")]
pub type Uart4Ifls = crate::Reg<uart4_ifls::Uart4IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod uart4_ifls;
#[doc = "UART4_IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_ibrd`] module"]
#[doc(alias = "UART4_IBRD")]
pub type Uart4Ibrd = crate::Reg<uart4_ibrd::Uart4IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod uart4_ibrd;
#[doc = "UART4_FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_fbrd`] module"]
#[doc(alias = "UART4_FBRD")]
pub type Uart4Fbrd = crate::Reg<uart4_fbrd::Uart4FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod uart4_fbrd;
#[doc = "UART4_GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_gfctl`] module"]
#[doc(alias = "UART4_GFCTL")]
pub type Uart4Gfctl = crate::Reg<uart4_gfctl::Uart4GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod uart4_gfctl;
#[doc = "UART4_TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_txdata`] module"]
#[doc(alias = "UART4_TXDATA")]
pub type Uart4Txdata = crate::Reg<uart4_txdata::Uart4TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod uart4_txdata;
#[doc = "UART4_RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_rxdata`] module"]
#[doc(alias = "UART4_RXDATA")]
pub type Uart4Rxdata = crate::Reg<uart4_rxdata::Uart4RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod uart4_rxdata;
#[doc = "UART4_AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_amask`] module"]
#[doc(alias = "UART4_AMASK")]
pub type Uart4Amask = crate::Reg<uart4_amask::Uart4AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod uart4_amask;
#[doc = "UART4_ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart4_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart4_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_addr`] module"]
#[doc(alias = "UART4_ADDR")]
pub type Uart4Addr = crate::Reg<uart4_addr::Uart4AddrSpec>;
#[doc = "Self Address Register"]
pub mod uart4_addr;
