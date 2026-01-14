#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    uart3_gprcm: [Uart3Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    uart3_clkdiv: Uart3Clkdiv,
    _reserved2: [u8; 0x04],
    uart3_clksel: Uart3Clksel,
    _reserved3: [u8; 0x0c],
    uart3_pdbgctl: Uart3Pdbgctl,
    _reserved4: [u8; 0x04],
    uart3_int_event0: [Uart3IntEvent0; 1],
    _reserved5: [u8; 0x04],
    uart3_int_event1: [Uart3IntEvent1; 1],
    _reserved6: [u8; 0x04],
    uart3_int_event2: [Uart3IntEvent2; 1],
    _reserved7: [u8; 0x34],
    uart3_evt_mode: Uart3EvtMode,
    uart3_intctl: Uart3Intctl,
    _reserved9: [u8; 0x18],
    uart3_ctl0: Uart3Ctl0,
    uart3_lcrh: Uart3Lcrh,
    uart3_stat: Uart3Stat,
    uart3_ifls: Uart3Ifls,
    uart3_ibrd: Uart3Ibrd,
    uart3_fbrd: Uart3Fbrd,
    uart3_gfctl: Uart3Gfctl,
    _reserved16: [u8; 0x04],
    uart3_txdata: Uart3Txdata,
    uart3_rxdata: Uart3Rxdata,
    _reserved18: [u8; 0x20],
    uart3_amask: Uart3Amask,
    uart3_addr: Uart3Addr,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - UART3_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn uart3_gprcm(&self, n: usize) -> &Uart3Gprcm {
        &self.uart3_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - UART3_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn uart3_gprcm_iter(&self) -> impl Iterator<Item = &Uart3Gprcm> {
        self.uart3_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn uart3_clkdiv(&self) -> &Uart3Clkdiv {
        &self.uart3_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn uart3_clksel(&self) -> &Uart3Clksel {
        &self.uart3_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn uart3_pdbgctl(&self) -> &Uart3Pdbgctl {
        &self.uart3_pdbgctl
    }
    #[doc = "0x1020..0x104c - UART3_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn uart3_int_event0(&self, n: usize) -> &Uart3IntEvent0 {
        &self.uart3_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - UART3_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn uart3_int_event0_iter(&self) -> impl Iterator<Item = &Uart3IntEvent0> {
        self.uart3_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - UART3_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn uart3_int_event1(&self, n: usize) -> &Uart3IntEvent1 {
        &self.uart3_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - UART3_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn uart3_int_event1_iter(&self) -> impl Iterator<Item = &Uart3IntEvent1> {
        self.uart3_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - UART3_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn uart3_int_event2(&self, n: usize) -> &Uart3IntEvent2 {
        &self.uart3_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - UART3_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn uart3_int_event2_iter(&self) -> impl Iterator<Item = &Uart3IntEvent2> {
        self.uart3_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn uart3_evt_mode(&self) -> &Uart3EvtMode {
        &self.uart3_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn uart3_intctl(&self) -> &Uart3Intctl {
        &self.uart3_intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn uart3_ctl0(&self) -> &Uart3Ctl0 {
        &self.uart3_ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn uart3_lcrh(&self) -> &Uart3Lcrh {
        &self.uart3_lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn uart3_stat(&self) -> &Uart3Stat {
        &self.uart3_stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn uart3_ifls(&self) -> &Uart3Ifls {
        &self.uart3_ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart3_ibrd(&self) -> &Uart3Ibrd {
        &self.uart3_ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart3_fbrd(&self) -> &Uart3Fbrd {
        &self.uart3_fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn uart3_gfctl(&self) -> &Uart3Gfctl {
        &self.uart3_gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn uart3_txdata(&self) -> &Uart3Txdata {
        &self.uart3_txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn uart3_rxdata(&self) -> &Uart3Rxdata {
        &self.uart3_rxdata
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn uart3_amask(&self) -> &Uart3Amask {
        &self.uart3_amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn uart3_addr(&self) -> &Uart3Addr {
        &self.uart3_addr
    }
}
#[doc = "UART3_GPRCM\\[%s\\]"]
pub use self::uart3_gprcm::Uart3Gprcm;
#[doc = r"Cluster"]
#[doc = "UART3_GPRCM\\[%s\\]"]
pub mod uart3_gprcm;
#[doc = "UART3_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_clkdiv`] module"]
#[doc(alias = "UART3_CLKDIV")]
pub type Uart3Clkdiv = crate::Reg<uart3_clkdiv::Uart3ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod uart3_clkdiv;
#[doc = "UART3_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_clksel`] module"]
#[doc(alias = "UART3_CLKSEL")]
pub type Uart3Clksel = crate::Reg<uart3_clksel::Uart3ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod uart3_clksel;
#[doc = "UART3_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_pdbgctl`] module"]
#[doc(alias = "UART3_PDBGCTL")]
pub type Uart3Pdbgctl = crate::Reg<uart3_pdbgctl::Uart3PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod uart3_pdbgctl;
#[doc = "UART3_INT_EVENT0\\[%s\\]"]
pub use self::uart3_int_event0::Uart3IntEvent0;
#[doc = r"Cluster"]
#[doc = "UART3_INT_EVENT0\\[%s\\]"]
pub mod uart3_int_event0;
#[doc = "UART3_INT_EVENT1\\[%s\\]"]
pub use self::uart3_int_event1::Uart3IntEvent1;
#[doc = r"Cluster"]
#[doc = "UART3_INT_EVENT1\\[%s\\]"]
pub mod uart3_int_event1;
#[doc = "UART3_INT_EVENT2\\[%s\\]"]
pub use self::uart3_int_event2::Uart3IntEvent2;
#[doc = r"Cluster"]
#[doc = "UART3_INT_EVENT2\\[%s\\]"]
pub mod uart3_int_event2;
#[doc = "UART3_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_evt_mode`] module"]
#[doc(alias = "UART3_EVT_MODE")]
pub type Uart3EvtMode = crate::Reg<uart3_evt_mode::Uart3EvtModeSpec>;
#[doc = "Event Mode"]
pub mod uart3_evt_mode;
#[doc = "UART3_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_intctl`] module"]
#[doc(alias = "UART3_INTCTL")]
pub type Uart3Intctl = crate::Reg<uart3_intctl::Uart3IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod uart3_intctl;
#[doc = "UART3_CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_ctl0`] module"]
#[doc(alias = "UART3_CTL0")]
pub type Uart3Ctl0 = crate::Reg<uart3_ctl0::Uart3Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod uart3_ctl0;
#[doc = "UART3_LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_lcrh`] module"]
#[doc(alias = "UART3_LCRH")]
pub type Uart3Lcrh = crate::Reg<uart3_lcrh::Uart3LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod uart3_lcrh;
#[doc = "UART3_STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_stat`] module"]
#[doc(alias = "UART3_STAT")]
pub type Uart3Stat = crate::Reg<uart3_stat::Uart3StatSpec>;
#[doc = "UART Status Register"]
pub mod uart3_stat;
#[doc = "UART3_IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_ifls`] module"]
#[doc(alias = "UART3_IFLS")]
pub type Uart3Ifls = crate::Reg<uart3_ifls::Uart3IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod uart3_ifls;
#[doc = "UART3_IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_ibrd`] module"]
#[doc(alias = "UART3_IBRD")]
pub type Uart3Ibrd = crate::Reg<uart3_ibrd::Uart3IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod uart3_ibrd;
#[doc = "UART3_FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_fbrd`] module"]
#[doc(alias = "UART3_FBRD")]
pub type Uart3Fbrd = crate::Reg<uart3_fbrd::Uart3FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod uart3_fbrd;
#[doc = "UART3_GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_gfctl`] module"]
#[doc(alias = "UART3_GFCTL")]
pub type Uart3Gfctl = crate::Reg<uart3_gfctl::Uart3GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod uart3_gfctl;
#[doc = "UART3_TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_txdata`] module"]
#[doc(alias = "UART3_TXDATA")]
pub type Uart3Txdata = crate::Reg<uart3_txdata::Uart3TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod uart3_txdata;
#[doc = "UART3_RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_rxdata`] module"]
#[doc(alias = "UART3_RXDATA")]
pub type Uart3Rxdata = crate::Reg<uart3_rxdata::Uart3RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod uart3_rxdata;
#[doc = "UART3_AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_amask`] module"]
#[doc(alias = "UART3_AMASK")]
pub type Uart3Amask = crate::Reg<uart3_amask::Uart3AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod uart3_amask;
#[doc = "UART3_ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_addr`] module"]
#[doc(alias = "UART3_ADDR")]
pub type Uart3Addr = crate::Reg<uart3_addr::Uart3AddrSpec>;
#[doc = "Self Address Register"]
pub mod uart3_addr;
