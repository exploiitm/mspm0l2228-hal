#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    uart0_gprcm: [Uart0Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    uart0_clkdiv: Uart0Clkdiv,
    _reserved2: [u8; 0x04],
    uart0_clksel: Uart0Clksel,
    _reserved3: [u8; 0x0c],
    uart0_pdbgctl: Uart0Pdbgctl,
    _reserved4: [u8; 0x04],
    uart0_int_event0: [Uart0IntEvent0; 1],
    _reserved5: [u8; 0x04],
    uart0_int_event1: [Uart0IntEvent1; 1],
    _reserved6: [u8; 0x04],
    uart0_int_event2: [Uart0IntEvent2; 1],
    _reserved7: [u8; 0x34],
    uart0_evt_mode: Uart0EvtMode,
    uart0_intctl: Uart0Intctl,
    _reserved9: [u8; 0x18],
    uart0_ctl0: Uart0Ctl0,
    uart0_lcrh: Uart0Lcrh,
    uart0_stat: Uart0Stat,
    uart0_ifls: Uart0Ifls,
    uart0_ibrd: Uart0Ibrd,
    uart0_fbrd: Uart0Fbrd,
    uart0_gfctl: Uart0Gfctl,
    _reserved16: [u8; 0x04],
    uart0_txdata: Uart0Txdata,
    uart0_rxdata: Uart0Rxdata,
    _reserved18: [u8; 0x08],
    uart0_lincnt: Uart0Lincnt,
    uart0_linctl: Uart0Linctl,
    uart0_linc0: Uart0Linc0,
    uart0_linc1: Uart0Linc1,
    uart0_irctl: Uart0Irctl,
    _reserved23: [u8; 0x04],
    uart0_amask: Uart0Amask,
    uart0_addr: Uart0Addr,
    _reserved25: [u8; 0x10],
    uart0_clkdiv2: Uart0Clkdiv2,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - UART0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn uart0_gprcm(&self, n: usize) -> &Uart0Gprcm {
        &self.uart0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - UART0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn uart0_gprcm_iter(&self) -> impl Iterator<Item = &Uart0Gprcm> {
        self.uart0_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn uart0_clkdiv(&self) -> &Uart0Clkdiv {
        &self.uart0_clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn uart0_clksel(&self) -> &Uart0Clksel {
        &self.uart0_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn uart0_pdbgctl(&self) -> &Uart0Pdbgctl {
        &self.uart0_pdbgctl
    }
    #[doc = "0x1020..0x104c - UART0_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn uart0_int_event0(&self, n: usize) -> &Uart0IntEvent0 {
        &self.uart0_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - UART0_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn uart0_int_event0_iter(&self) -> impl Iterator<Item = &Uart0IntEvent0> {
        self.uart0_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - UART0_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn uart0_int_event1(&self, n: usize) -> &Uart0IntEvent1 {
        &self.uart0_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - UART0_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn uart0_int_event1_iter(&self) -> impl Iterator<Item = &Uart0IntEvent1> {
        self.uart0_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - UART0_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn uart0_int_event2(&self, n: usize) -> &Uart0IntEvent2 {
        &self.uart0_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - UART0_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn uart0_int_event2_iter(&self) -> impl Iterator<Item = &Uart0IntEvent2> {
        self.uart0_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn uart0_evt_mode(&self) -> &Uart0EvtMode {
        &self.uart0_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn uart0_intctl(&self) -> &Uart0Intctl {
        &self.uart0_intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn uart0_ctl0(&self) -> &Uart0Ctl0 {
        &self.uart0_ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn uart0_lcrh(&self) -> &Uart0Lcrh {
        &self.uart0_lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn uart0_stat(&self) -> &Uart0Stat {
        &self.uart0_stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn uart0_ifls(&self) -> &Uart0Ifls {
        &self.uart0_ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart0_ibrd(&self) -> &Uart0Ibrd {
        &self.uart0_ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn uart0_fbrd(&self) -> &Uart0Fbrd {
        &self.uart0_fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn uart0_gfctl(&self) -> &Uart0Gfctl {
        &self.uart0_gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn uart0_txdata(&self) -> &Uart0Txdata {
        &self.uart0_txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn uart0_rxdata(&self) -> &Uart0Rxdata {
        &self.uart0_rxdata
    }
    #[doc = "0x1130 - UART LIN Mode Counter Register"]
    #[inline(always)]
    pub const fn uart0_lincnt(&self) -> &Uart0Lincnt {
        &self.uart0_lincnt
    }
    #[doc = "0x1134 - UART LIN Mode Control Register"]
    #[inline(always)]
    pub const fn uart0_linctl(&self) -> &Uart0Linctl {
        &self.uart0_linctl
    }
    #[doc = "0x1138 - UART LIN Mode Capture 0 Register"]
    #[inline(always)]
    pub const fn uart0_linc0(&self) -> &Uart0Linc0 {
        &self.uart0_linc0
    }
    #[doc = "0x113c - UART LIN Mode Capture 1 Register"]
    #[inline(always)]
    pub const fn uart0_linc1(&self) -> &Uart0Linc1 {
        &self.uart0_linc1
    }
    #[doc = "0x1140 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uart0_irctl(&self) -> &Uart0Irctl {
        &self.uart0_irctl
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn uart0_amask(&self) -> &Uart0Amask {
        &self.uart0_amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn uart0_addr(&self) -> &Uart0Addr {
        &self.uart0_addr
    }
    #[doc = "0x1160 - Clock Divider"]
    #[inline(always)]
    pub const fn uart0_clkdiv2(&self) -> &Uart0Clkdiv2 {
        &self.uart0_clkdiv2
    }
}
#[doc = "UART0_GPRCM\\[%s\\]"]
pub use self::uart0_gprcm::Uart0Gprcm;
#[doc = r"Cluster"]
#[doc = "UART0_GPRCM\\[%s\\]"]
pub mod uart0_gprcm;
#[doc = "UART0_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_clkdiv`] module"]
#[doc(alias = "UART0_CLKDIV")]
pub type Uart0Clkdiv = crate::Reg<uart0_clkdiv::Uart0ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod uart0_clkdiv;
#[doc = "UART0_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_clksel`] module"]
#[doc(alias = "UART0_CLKSEL")]
pub type Uart0Clksel = crate::Reg<uart0_clksel::Uart0ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod uart0_clksel;
#[doc = "UART0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_pdbgctl`] module"]
#[doc(alias = "UART0_PDBGCTL")]
pub type Uart0Pdbgctl = crate::Reg<uart0_pdbgctl::Uart0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod uart0_pdbgctl;
#[doc = "UART0_INT_EVENT0\\[%s\\]"]
pub use self::uart0_int_event0::Uart0IntEvent0;
#[doc = r"Cluster"]
#[doc = "UART0_INT_EVENT0\\[%s\\]"]
pub mod uart0_int_event0;
#[doc = "UART0_INT_EVENT1\\[%s\\]"]
pub use self::uart0_int_event1::Uart0IntEvent1;
#[doc = r"Cluster"]
#[doc = "UART0_INT_EVENT1\\[%s\\]"]
pub mod uart0_int_event1;
#[doc = "UART0_INT_EVENT2\\[%s\\]"]
pub use self::uart0_int_event2::Uart0IntEvent2;
#[doc = r"Cluster"]
#[doc = "UART0_INT_EVENT2\\[%s\\]"]
pub mod uart0_int_event2;
#[doc = "UART0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_evt_mode`] module"]
#[doc(alias = "UART0_EVT_MODE")]
pub type Uart0EvtMode = crate::Reg<uart0_evt_mode::Uart0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod uart0_evt_mode;
#[doc = "UART0_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_intctl`] module"]
#[doc(alias = "UART0_INTCTL")]
pub type Uart0Intctl = crate::Reg<uart0_intctl::Uart0IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod uart0_intctl;
#[doc = "UART0_CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_ctl0`] module"]
#[doc(alias = "UART0_CTL0")]
pub type Uart0Ctl0 = crate::Reg<uart0_ctl0::Uart0Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod uart0_ctl0;
#[doc = "UART0_LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_lcrh`] module"]
#[doc(alias = "UART0_LCRH")]
pub type Uart0Lcrh = crate::Reg<uart0_lcrh::Uart0LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod uart0_lcrh;
#[doc = "UART0_STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_stat`] module"]
#[doc(alias = "UART0_STAT")]
pub type Uart0Stat = crate::Reg<uart0_stat::Uart0StatSpec>;
#[doc = "UART Status Register"]
pub mod uart0_stat;
#[doc = "UART0_IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_ifls`] module"]
#[doc(alias = "UART0_IFLS")]
pub type Uart0Ifls = crate::Reg<uart0_ifls::Uart0IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod uart0_ifls;
#[doc = "UART0_IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_ibrd`] module"]
#[doc(alias = "UART0_IBRD")]
pub type Uart0Ibrd = crate::Reg<uart0_ibrd::Uart0IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod uart0_ibrd;
#[doc = "UART0_FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_fbrd`] module"]
#[doc(alias = "UART0_FBRD")]
pub type Uart0Fbrd = crate::Reg<uart0_fbrd::Uart0FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod uart0_fbrd;
#[doc = "UART0_GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_gfctl`] module"]
#[doc(alias = "UART0_GFCTL")]
pub type Uart0Gfctl = crate::Reg<uart0_gfctl::Uart0GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod uart0_gfctl;
#[doc = "UART0_TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_txdata`] module"]
#[doc(alias = "UART0_TXDATA")]
pub type Uart0Txdata = crate::Reg<uart0_txdata::Uart0TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod uart0_txdata;
#[doc = "UART0_RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_rxdata`] module"]
#[doc(alias = "UART0_RXDATA")]
pub type Uart0Rxdata = crate::Reg<uart0_rxdata::Uart0RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod uart0_rxdata;
#[doc = "UART0_LINCNT (rw) register accessor: UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_lincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_lincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_lincnt`] module"]
#[doc(alias = "UART0_LINCNT")]
pub type Uart0Lincnt = crate::Reg<uart0_lincnt::Uart0LincntSpec>;
#[doc = "UART LIN Mode Counter Register"]
pub mod uart0_lincnt;
#[doc = "UART0_LINCTL (rw) register accessor: UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_linctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_linctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_linctl`] module"]
#[doc(alias = "UART0_LINCTL")]
pub type Uart0Linctl = crate::Reg<uart0_linctl::Uart0LinctlSpec>;
#[doc = "UART LIN Mode Control Register"]
pub mod uart0_linctl;
#[doc = "UART0_LINC0 (rw) register accessor: UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_linc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_linc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_linc0`] module"]
#[doc(alias = "UART0_LINC0")]
pub type Uart0Linc0 = crate::Reg<uart0_linc0::Uart0Linc0Spec>;
#[doc = "UART LIN Mode Capture 0 Register"]
pub mod uart0_linc0;
#[doc = "UART0_LINC1 (rw) register accessor: UART LIN Mode Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_linc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_linc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_linc1`] module"]
#[doc(alias = "UART0_LINC1")]
pub type Uart0Linc1 = crate::Reg<uart0_linc1::Uart0Linc1Spec>;
#[doc = "UART LIN Mode Capture 1 Register"]
pub mod uart0_linc1;
#[doc = "UART0_IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_irctl`] module"]
#[doc(alias = "UART0_IRCTL")]
pub type Uart0Irctl = crate::Reg<uart0_irctl::Uart0IrctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uart0_irctl;
#[doc = "UART0_AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_amask`] module"]
#[doc(alias = "UART0_AMASK")]
pub type Uart0Amask = crate::Reg<uart0_amask::Uart0AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod uart0_amask;
#[doc = "UART0_ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_addr`] module"]
#[doc(alias = "UART0_ADDR")]
pub type Uart0Addr = crate::Reg<uart0_addr::Uart0AddrSpec>;
#[doc = "Self Address Register"]
pub mod uart0_addr;
#[doc = "UART0_CLKDIV2 (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_clkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_clkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_clkdiv2`] module"]
#[doc(alias = "UART0_CLKDIV2")]
pub type Uart0Clkdiv2 = crate::Reg<uart0_clkdiv2::Uart0Clkdiv2Spec>;
#[doc = "Clock Divider"]
pub mod uart0_clkdiv2;
