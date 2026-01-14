#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    uart2_gprcm: [Uart2Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    uart2_clkdiv: Uart2Clkdiv,
    _reserved2: [u8; 0x04],
    uart2_clksel: Uart2Clksel,
    _reserved3: [u8; 0x0c],
    uart2_pdbgctl: Uart2Pdbgctl,
    _reserved4: [u8; 0x04],
    uart2_int_event0: [Uart2IntEvent0; 1],
    _reserved5: [u8; 0x04],
    uart2_int_event1: [Uart2IntEvent1; 1],
    _reserved6: [u8; 0x04],
    uart2_int_event2: [Uart2IntEvent2; 1],
    _reserved7: [u8; 0x34],
    uart2_evt_mode: Uart2EvtMode,
    uart2_intctl: Uart2Intctl,
    _reserved9: [u8; 0x18],
    uart2_ctl0: Uart2Ctl0,
    uart2_lcrh: Uart2Lcrh,
    uart2_stat: Uart2Stat,
    uart2_ifls: Uart2Ifls,
    uart2_ibrd: Uart2Ibrd,
    uart2_fbrd: Uart2Fbrd,
    uart2_gfctl: Uart2Gfctl,
    _reserved16: [u8; 0x04],
    uart2_txdata: Uart2Txdata,
    uart2_rxdata: Uart2Rxdata,
    _reserved18: [u8; 0x20],
    uart2_amask: Uart2Amask,
    uart2_addr: Uart2Addr,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - UART2_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn uart2_gprcm(&self, n: usize) -> &Uart2Gprcm {
        &self.uart2_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - UART2_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn uart2_gprcm_iter(&self) -> impl Iterator<Item = &Uart2Gprcm> {
        self.uart2_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn uart2_clkdiv(&self) -> &Uart2Clkdiv {
        &self.uart2_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn uart2_clksel(&self) -> &Uart2Clksel {
        &self.uart2_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn uart2_pdbgctl(&self) -> &Uart2Pdbgctl {
        &self.uart2_pdbgctl
    }
    #[doc = "0x1020..0x104c - UART2_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn uart2_int_event0(&self, n: usize) -> &Uart2IntEvent0 {
        &self.uart2_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - UART2_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn uart2_int_event0_iter(&self) -> impl Iterator<Item = &Uart2IntEvent0> {
        self.uart2_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - UART2_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn uart2_int_event1(&self, n: usize) -> &Uart2IntEvent1 {
        &self.uart2_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - UART2_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn uart2_int_event1_iter(&self) -> impl Iterator<Item = &Uart2IntEvent1> {
        self.uart2_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - UART2_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn uart2_int_event2(&self, n: usize) -> &Uart2IntEvent2 {
        &self.uart2_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - UART2_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn uart2_int_event2_iter(&self) -> impl Iterator<Item = &Uart2IntEvent2> {
        self.uart2_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn uart2_evt_mode(&self) -> &Uart2EvtMode {
        &self.uart2_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn uart2_intctl(&self) -> &Uart2Intctl {
        &self.uart2_intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn uart2_ctl0(&self) -> &Uart2Ctl0 {
        &self.uart2_ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn uart2_lcrh(&self) -> &Uart2Lcrh {
        &self.uart2_lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn uart2_stat(&self) -> &Uart2Stat {
        &self.uart2_stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn uart2_ifls(&self) -> &Uart2Ifls {
        &self.uart2_ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart2_ibrd(&self) -> &Uart2Ibrd {
        &self.uart2_ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart2_fbrd(&self) -> &Uart2Fbrd {
        &self.uart2_fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn uart2_gfctl(&self) -> &Uart2Gfctl {
        &self.uart2_gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn uart2_txdata(&self) -> &Uart2Txdata {
        &self.uart2_txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn uart2_rxdata(&self) -> &Uart2Rxdata {
        &self.uart2_rxdata
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn uart2_amask(&self) -> &Uart2Amask {
        &self.uart2_amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn uart2_addr(&self) -> &Uart2Addr {
        &self.uart2_addr
    }
}
#[doc = "UART2_GPRCM\\[%s\\]"]
pub use self::uart2_gprcm::Uart2Gprcm;
#[doc = r"Cluster"]
#[doc = "UART2_GPRCM\\[%s\\]"]
pub mod uart2_gprcm;
#[doc = "UART2_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_clkdiv`] module"]
#[doc(alias = "UART2_CLKDIV")]
pub type Uart2Clkdiv = crate::Reg<uart2_clkdiv::Uart2ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod uart2_clkdiv;
#[doc = "UART2_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_clksel`] module"]
#[doc(alias = "UART2_CLKSEL")]
pub type Uart2Clksel = crate::Reg<uart2_clksel::Uart2ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod uart2_clksel;
#[doc = "UART2_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_pdbgctl`] module"]
#[doc(alias = "UART2_PDBGCTL")]
pub type Uart2Pdbgctl = crate::Reg<uart2_pdbgctl::Uart2PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod uart2_pdbgctl;
#[doc = "UART2_INT_EVENT0\\[%s\\]"]
pub use self::uart2_int_event0::Uart2IntEvent0;
#[doc = r"Cluster"]
#[doc = "UART2_INT_EVENT0\\[%s\\]"]
pub mod uart2_int_event0;
#[doc = "UART2_INT_EVENT1\\[%s\\]"]
pub use self::uart2_int_event1::Uart2IntEvent1;
#[doc = r"Cluster"]
#[doc = "UART2_INT_EVENT1\\[%s\\]"]
pub mod uart2_int_event1;
#[doc = "UART2_INT_EVENT2\\[%s\\]"]
pub use self::uart2_int_event2::Uart2IntEvent2;
#[doc = r"Cluster"]
#[doc = "UART2_INT_EVENT2\\[%s\\]"]
pub mod uart2_int_event2;
#[doc = "UART2_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_evt_mode`] module"]
#[doc(alias = "UART2_EVT_MODE")]
pub type Uart2EvtMode = crate::Reg<uart2_evt_mode::Uart2EvtModeSpec>;
#[doc = "Event Mode"]
pub mod uart2_evt_mode;
#[doc = "UART2_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_intctl`] module"]
#[doc(alias = "UART2_INTCTL")]
pub type Uart2Intctl = crate::Reg<uart2_intctl::Uart2IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod uart2_intctl;
#[doc = "UART2_CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_ctl0`] module"]
#[doc(alias = "UART2_CTL0")]
pub type Uart2Ctl0 = crate::Reg<uart2_ctl0::Uart2Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod uart2_ctl0;
#[doc = "UART2_LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_lcrh`] module"]
#[doc(alias = "UART2_LCRH")]
pub type Uart2Lcrh = crate::Reg<uart2_lcrh::Uart2LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod uart2_lcrh;
#[doc = "UART2_STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_stat`] module"]
#[doc(alias = "UART2_STAT")]
pub type Uart2Stat = crate::Reg<uart2_stat::Uart2StatSpec>;
#[doc = "UART Status Register"]
pub mod uart2_stat;
#[doc = "UART2_IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_ifls`] module"]
#[doc(alias = "UART2_IFLS")]
pub type Uart2Ifls = crate::Reg<uart2_ifls::Uart2IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod uart2_ifls;
#[doc = "UART2_IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_ibrd`] module"]
#[doc(alias = "UART2_IBRD")]
pub type Uart2Ibrd = crate::Reg<uart2_ibrd::Uart2IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod uart2_ibrd;
#[doc = "UART2_FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_fbrd`] module"]
#[doc(alias = "UART2_FBRD")]
pub type Uart2Fbrd = crate::Reg<uart2_fbrd::Uart2FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod uart2_fbrd;
#[doc = "UART2_GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_gfctl`] module"]
#[doc(alias = "UART2_GFCTL")]
pub type Uart2Gfctl = crate::Reg<uart2_gfctl::Uart2GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod uart2_gfctl;
#[doc = "UART2_TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_txdata`] module"]
#[doc(alias = "UART2_TXDATA")]
pub type Uart2Txdata = crate::Reg<uart2_txdata::Uart2TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod uart2_txdata;
#[doc = "UART2_RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_rxdata`] module"]
#[doc(alias = "UART2_RXDATA")]
pub type Uart2Rxdata = crate::Reg<uart2_rxdata::Uart2RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod uart2_rxdata;
#[doc = "UART2_AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_amask`] module"]
#[doc(alias = "UART2_AMASK")]
pub type Uart2Amask = crate::Reg<uart2_amask::Uart2AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod uart2_amask;
#[doc = "UART2_ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_addr`] module"]
#[doc(alias = "UART2_ADDR")]
pub type Uart2Addr = crate::Reg<uart2_addr::Uart2AddrSpec>;
#[doc = "Self Address Register"]
pub mod uart2_addr;
