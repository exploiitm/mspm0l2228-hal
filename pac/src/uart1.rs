#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    uart1_gprcm: [Uart1Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    uart1_clkdiv: Uart1Clkdiv,
    _reserved2: [u8; 0x04],
    uart1_clksel: Uart1Clksel,
    _reserved3: [u8; 0x0c],
    uart1_pdbgctl: Uart1Pdbgctl,
    _reserved4: [u8; 0x04],
    uart1_int_event0: [Uart1IntEvent0; 1],
    _reserved5: [u8; 0x04],
    uart1_int_event1: [Uart1IntEvent1; 1],
    _reserved6: [u8; 0x04],
    uart1_int_event2: [Uart1IntEvent2; 1],
    _reserved7: [u8; 0x34],
    uart1_evt_mode: Uart1EvtMode,
    uart1_intctl: Uart1Intctl,
    _reserved9: [u8; 0x18],
    uart1_ctl0: Uart1Ctl0,
    uart1_lcrh: Uart1Lcrh,
    uart1_stat: Uart1Stat,
    uart1_ifls: Uart1Ifls,
    uart1_ibrd: Uart1Ibrd,
    uart1_fbrd: Uart1Fbrd,
    uart1_gfctl: Uart1Gfctl,
    _reserved16: [u8; 0x04],
    uart1_txdata: Uart1Txdata,
    uart1_rxdata: Uart1Rxdata,
    _reserved18: [u8; 0x08],
    uart1_lincnt: Uart1Lincnt,
    uart1_linctl: Uart1Linctl,
    uart1_linc0: Uart1Linc0,
    uart1_linc1: Uart1Linc1,
    uart1_irctl: Uart1Irctl,
    _reserved23: [u8; 0x04],
    uart1_amask: Uart1Amask,
    uart1_addr: Uart1Addr,
    _reserved25: [u8; 0x10],
    uart1_clkdiv2: Uart1Clkdiv2,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - UART1_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn uart1_gprcm(&self, n: usize) -> &Uart1Gprcm {
        &self.uart1_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - UART1_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn uart1_gprcm_iter(&self) -> impl Iterator<Item = &Uart1Gprcm> {
        self.uart1_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn uart1_clkdiv(&self) -> &Uart1Clkdiv {
        &self.uart1_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn uart1_clksel(&self) -> &Uart1Clksel {
        &self.uart1_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn uart1_pdbgctl(&self) -> &Uart1Pdbgctl {
        &self.uart1_pdbgctl
    }
    #[doc = "0x1020..0x104c - UART1_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn uart1_int_event0(&self, n: usize) -> &Uart1IntEvent0 {
        &self.uart1_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - UART1_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn uart1_int_event0_iter(&self) -> impl Iterator<Item = &Uart1IntEvent0> {
        self.uart1_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - UART1_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn uart1_int_event1(&self, n: usize) -> &Uart1IntEvent1 {
        &self.uart1_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - UART1_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn uart1_int_event1_iter(&self) -> impl Iterator<Item = &Uart1IntEvent1> {
        self.uart1_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - UART1_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn uart1_int_event2(&self, n: usize) -> &Uart1IntEvent2 {
        &self.uart1_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - UART1_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn uart1_int_event2_iter(&self) -> impl Iterator<Item = &Uart1IntEvent2> {
        self.uart1_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn uart1_evt_mode(&self) -> &Uart1EvtMode {
        &self.uart1_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn uart1_intctl(&self) -> &Uart1Intctl {
        &self.uart1_intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn uart1_ctl0(&self) -> &Uart1Ctl0 {
        &self.uart1_ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn uart1_lcrh(&self) -> &Uart1Lcrh {
        &self.uart1_lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn uart1_stat(&self) -> &Uart1Stat {
        &self.uart1_stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn uart1_ifls(&self) -> &Uart1Ifls {
        &self.uart1_ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart1_ibrd(&self) -> &Uart1Ibrd {
        &self.uart1_ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart1_fbrd(&self) -> &Uart1Fbrd {
        &self.uart1_fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn uart1_gfctl(&self) -> &Uart1Gfctl {
        &self.uart1_gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn uart1_txdata(&self) -> &Uart1Txdata {
        &self.uart1_txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn uart1_rxdata(&self) -> &Uart1Rxdata {
        &self.uart1_rxdata
    }
    #[doc = "0x1130 - UART LIN Mode Counter Register"]
    #[inline(always)]
    pub const fn uart1_lincnt(&self) -> &Uart1Lincnt {
        &self.uart1_lincnt
    }
    #[doc = "0x1134 - UART LIN Mode Control Register"]
    #[inline(always)]
    pub const fn uart1_linctl(&self) -> &Uart1Linctl {
        &self.uart1_linctl
    }
    #[doc = "0x1138 - UART LIN Mode Capture 0 Register"]
    #[inline(always)]
    pub const fn uart1_linc0(&self) -> &Uart1Linc0 {
        &self.uart1_linc0
    }
    #[doc = "0x113c - UART LIN Mode Capture 1 Register"]
    #[inline(always)]
    pub const fn uart1_linc1(&self) -> &Uart1Linc1 {
        &self.uart1_linc1
    }
    #[doc = "0x1140 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uart1_irctl(&self) -> &Uart1Irctl {
        &self.uart1_irctl
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn uart1_amask(&self) -> &Uart1Amask {
        &self.uart1_amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn uart1_addr(&self) -> &Uart1Addr {
        &self.uart1_addr
    }
    #[doc = "0x1160 - Clock Divider"]
    #[inline(always)]
    pub const fn uart1_clkdiv2(&self) -> &Uart1Clkdiv2 {
        &self.uart1_clkdiv2
    }
}
#[doc = "UART1_GPRCM\\[%s\\]"]
pub use self::uart1_gprcm::Uart1Gprcm;
#[doc = r"Cluster"]
#[doc = "UART1_GPRCM\\[%s\\]"]
pub mod uart1_gprcm;
#[doc = "UART1_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_clkdiv`] module"]
#[doc(alias = "UART1_CLKDIV")]
pub type Uart1Clkdiv = crate::Reg<uart1_clkdiv::Uart1ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod uart1_clkdiv;
#[doc = "UART1_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_clksel`] module"]
#[doc(alias = "UART1_CLKSEL")]
pub type Uart1Clksel = crate::Reg<uart1_clksel::Uart1ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod uart1_clksel;
#[doc = "UART1_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_pdbgctl`] module"]
#[doc(alias = "UART1_PDBGCTL")]
pub type Uart1Pdbgctl = crate::Reg<uart1_pdbgctl::Uart1PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod uart1_pdbgctl;
#[doc = "UART1_INT_EVENT0\\[%s\\]"]
pub use self::uart1_int_event0::Uart1IntEvent0;
#[doc = r"Cluster"]
#[doc = "UART1_INT_EVENT0\\[%s\\]"]
pub mod uart1_int_event0;
#[doc = "UART1_INT_EVENT1\\[%s\\]"]
pub use self::uart1_int_event1::Uart1IntEvent1;
#[doc = r"Cluster"]
#[doc = "UART1_INT_EVENT1\\[%s\\]"]
pub mod uart1_int_event1;
#[doc = "UART1_INT_EVENT2\\[%s\\]"]
pub use self::uart1_int_event2::Uart1IntEvent2;
#[doc = r"Cluster"]
#[doc = "UART1_INT_EVENT2\\[%s\\]"]
pub mod uart1_int_event2;
#[doc = "UART1_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_evt_mode`] module"]
#[doc(alias = "UART1_EVT_MODE")]
pub type Uart1EvtMode = crate::Reg<uart1_evt_mode::Uart1EvtModeSpec>;
#[doc = "Event Mode"]
pub mod uart1_evt_mode;
#[doc = "UART1_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_intctl`] module"]
#[doc(alias = "UART1_INTCTL")]
pub type Uart1Intctl = crate::Reg<uart1_intctl::Uart1IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod uart1_intctl;
#[doc = "UART1_CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_ctl0`] module"]
#[doc(alias = "UART1_CTL0")]
pub type Uart1Ctl0 = crate::Reg<uart1_ctl0::Uart1Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod uart1_ctl0;
#[doc = "UART1_LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_lcrh`] module"]
#[doc(alias = "UART1_LCRH")]
pub type Uart1Lcrh = crate::Reg<uart1_lcrh::Uart1LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod uart1_lcrh;
#[doc = "UART1_STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_stat`] module"]
#[doc(alias = "UART1_STAT")]
pub type Uart1Stat = crate::Reg<uart1_stat::Uart1StatSpec>;
#[doc = "UART Status Register"]
pub mod uart1_stat;
#[doc = "UART1_IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_ifls`] module"]
#[doc(alias = "UART1_IFLS")]
pub type Uart1Ifls = crate::Reg<uart1_ifls::Uart1IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod uart1_ifls;
#[doc = "UART1_IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_ibrd`] module"]
#[doc(alias = "UART1_IBRD")]
pub type Uart1Ibrd = crate::Reg<uart1_ibrd::Uart1IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod uart1_ibrd;
#[doc = "UART1_FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_fbrd`] module"]
#[doc(alias = "UART1_FBRD")]
pub type Uart1Fbrd = crate::Reg<uart1_fbrd::Uart1FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod uart1_fbrd;
#[doc = "UART1_GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_gfctl`] module"]
#[doc(alias = "UART1_GFCTL")]
pub type Uart1Gfctl = crate::Reg<uart1_gfctl::Uart1GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod uart1_gfctl;
#[doc = "UART1_TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_txdata`] module"]
#[doc(alias = "UART1_TXDATA")]
pub type Uart1Txdata = crate::Reg<uart1_txdata::Uart1TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod uart1_txdata;
#[doc = "UART1_RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_rxdata`] module"]
#[doc(alias = "UART1_RXDATA")]
pub type Uart1Rxdata = crate::Reg<uart1_rxdata::Uart1RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod uart1_rxdata;
#[doc = "UART1_LINCNT (rw) register accessor: UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_lincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_lincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_lincnt`] module"]
#[doc(alias = "UART1_LINCNT")]
pub type Uart1Lincnt = crate::Reg<uart1_lincnt::Uart1LincntSpec>;
#[doc = "UART LIN Mode Counter Register"]
pub mod uart1_lincnt;
#[doc = "UART1_LINCTL (rw) register accessor: UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_linctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_linctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_linctl`] module"]
#[doc(alias = "UART1_LINCTL")]
pub type Uart1Linctl = crate::Reg<uart1_linctl::Uart1LinctlSpec>;
#[doc = "UART LIN Mode Control Register"]
pub mod uart1_linctl;
#[doc = "UART1_LINC0 (rw) register accessor: UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_linc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_linc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_linc0`] module"]
#[doc(alias = "UART1_LINC0")]
pub type Uart1Linc0 = crate::Reg<uart1_linc0::Uart1Linc0Spec>;
#[doc = "UART LIN Mode Capture 0 Register"]
pub mod uart1_linc0;
#[doc = "UART1_LINC1 (rw) register accessor: UART LIN Mode Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_linc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_linc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_linc1`] module"]
#[doc(alias = "UART1_LINC1")]
pub type Uart1Linc1 = crate::Reg<uart1_linc1::Uart1Linc1Spec>;
#[doc = "UART LIN Mode Capture 1 Register"]
pub mod uart1_linc1;
#[doc = "UART1_IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_irctl`] module"]
#[doc(alias = "UART1_IRCTL")]
pub type Uart1Irctl = crate::Reg<uart1_irctl::Uart1IrctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uart1_irctl;
#[doc = "UART1_AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_amask`] module"]
#[doc(alias = "UART1_AMASK")]
pub type Uart1Amask = crate::Reg<uart1_amask::Uart1AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod uart1_amask;
#[doc = "UART1_ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_addr`] module"]
#[doc(alias = "UART1_ADDR")]
pub type Uart1Addr = crate::Reg<uart1_addr::Uart1AddrSpec>;
#[doc = "Self Address Register"]
pub mod uart1_addr;
#[doc = "UART1_CLKDIV2 (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_clkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_clkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_clkdiv2`] module"]
#[doc(alias = "UART1_CLKDIV2")]
pub type Uart1Clkdiv2 = crate::Reg<uart1_clkdiv2::Uart1Clkdiv2Spec>;
#[doc = "Clock Divider"]
pub mod uart1_clkdiv2;
