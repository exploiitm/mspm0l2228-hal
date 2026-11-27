#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    spi0_gprcm: [Spi0Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    spi0_clkdiv: Spi0Clkdiv,
    spi0_clksel: Spi0Clksel,
    _reserved3: [u8; 0x10],
    spi0_pdbgctl: Spi0Pdbgctl,
    _reserved4: [u8; 0x04],
    spi0_cpu_int: [Spi0CpuInt; 1],
    _reserved5: [u8; 0x04],
    spi0_dma_trig_rx: [Spi0DmaTrigRx; 1],
    _reserved6: [u8; 0x04],
    spi0_dma_trig_tx: [Spi0DmaTrigTx; 1],
    _reserved7: [u8; 0x34],
    spi0_evt_mode: Spi0EvtMode,
    spi0_intctl: Spi0Intctl,
    _reserved9: [u8; 0x18],
    spi0_ctl0: Spi0Ctl0,
    spi0_ctl1: Spi0Ctl1,
    spi0_clkctl: Spi0Clkctl,
    spi0_ifls: Spi0Ifls,
    spi0_stat: Spi0Stat,
    _reserved14: [u8; 0x1c],
    spi0_rxdata: Spi0Rxdata,
    _reserved15: [u8; 0x0c],
    spi0_txdata: Spi0Txdata,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - SPI0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn spi0_gprcm(&self, n: usize) -> &Spi0Gprcm {
        &self.spi0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - SPI0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn spi0_gprcm_iter(&self) -> impl Iterator<Item = &Spi0Gprcm> {
        self.spi0_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn spi0_clkdiv(&self) -> &Spi0Clkdiv {
        &self.spi0_clkdiv
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn spi0_clksel(&self) -> &Spi0Clksel {
        &self.spi0_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn spi0_pdbgctl(&self) -> &Spi0Pdbgctl {
        &self.spi0_pdbgctl
    }
    #[doc = "0x1020..0x104c - SPI0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn spi0_cpu_int(&self, n: usize) -> &Spi0CpuInt {
        &self.spi0_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - SPI0_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn spi0_cpu_int_iter(&self) -> impl Iterator<Item = &Spi0CpuInt> {
        self.spi0_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - SPI0_DMA_TRIG_RX\\[%s\\]"]
    #[inline(always)]
    pub const fn spi0_dma_trig_rx(&self, n: usize) -> &Spi0DmaTrigRx {
        &self.spi0_dma_trig_rx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - SPI0_DMA_TRIG_RX\\[%s\\]"]
    #[inline(always)]
    pub fn spi0_dma_trig_rx_iter(&self) -> impl Iterator<Item = &Spi0DmaTrigRx> {
        self.spi0_dma_trig_rx.iter()
    }
    #[doc = "0x1080..0x10ac - SPI0_DMA_TRIG_TX\\[%s\\]"]
    #[inline(always)]
    pub const fn spi0_dma_trig_tx(&self, n: usize) -> &Spi0DmaTrigTx {
        &self.spi0_dma_trig_tx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - SPI0_DMA_TRIG_TX\\[%s\\]"]
    #[inline(always)]
    pub fn spi0_dma_trig_tx_iter(&self) -> impl Iterator<Item = &Spi0DmaTrigTx> {
        self.spi0_dma_trig_tx.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn spi0_evt_mode(&self) -> &Spi0EvtMode {
        &self.spi0_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn spi0_intctl(&self) -> &Spi0Intctl {
        &self.spi0_intctl
    }
    #[doc = "0x1100 - SPI control register 0"]
    #[inline(always)]
    pub const fn spi0_ctl0(&self) -> &Spi0Ctl0 {
        &self.spi0_ctl0
    }
    #[doc = "0x1104 - SPI control register 1"]
    #[inline(always)]
    pub const fn spi0_ctl1(&self) -> &Spi0Ctl1 {
        &self.spi0_ctl1
    }
    #[doc = "0x1108 - Clock prescaler and divider register."]
    #[inline(always)]
    pub const fn spi0_clkctl(&self) -> &Spi0Clkctl {
        &self.spi0_clkctl
    }
    #[doc = "0x110c - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn spi0_ifls(&self) -> &Spi0Ifls {
        &self.spi0_ifls
    }
    #[doc = "0x1110 - Status Register"]
    #[inline(always)]
    pub const fn spi0_stat(&self) -> &Spi0Stat {
        &self.spi0_stat
    }
    #[doc = "0x1130 - RXDATA Register"]
    #[inline(always)]
    pub const fn spi0_rxdata(&self) -> &Spi0Rxdata {
        &self.spi0_rxdata
    }
    #[doc = "0x1140 - TXDATA Register"]
    #[inline(always)]
    pub const fn spi0_txdata(&self) -> &Spi0Txdata {
        &self.spi0_txdata
    }
}
#[doc = "SPI0_GPRCM\\[%s\\]"]
pub use self::spi0_gprcm::Spi0Gprcm;
#[doc = r"Cluster"]
#[doc = "SPI0_GPRCM\\[%s\\]"]
pub mod spi0_gprcm;
#[doc = "SPI0_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_clkdiv`] module"]
#[doc(alias = "SPI0_CLKDIV")]
pub type Spi0Clkdiv = crate::Reg<spi0_clkdiv::Spi0ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod spi0_clkdiv;
#[doc = "SPI0_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_clksel`] module"]
#[doc(alias = "SPI0_CLKSEL")]
pub type Spi0Clksel = crate::Reg<spi0_clksel::Spi0ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod spi0_clksel;
#[doc = "SPI0_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_pdbgctl`] module"]
#[doc(alias = "SPI0_PDBGCTL")]
pub type Spi0Pdbgctl = crate::Reg<spi0_pdbgctl::Spi0PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod spi0_pdbgctl;
#[doc = "SPI0_CPU_INT\\[%s\\]"]
pub use self::spi0_cpu_int::Spi0CpuInt;
#[doc = r"Cluster"]
#[doc = "SPI0_CPU_INT\\[%s\\]"]
pub mod spi0_cpu_int;
#[doc = "SPI0_DMA_TRIG_RX\\[%s\\]"]
pub use self::spi0_dma_trig_rx::Spi0DmaTrigRx;
#[doc = r"Cluster"]
#[doc = "SPI0_DMA_TRIG_RX\\[%s\\]"]
pub mod spi0_dma_trig_rx;
#[doc = "SPI0_DMA_TRIG_TX\\[%s\\]"]
pub use self::spi0_dma_trig_tx::Spi0DmaTrigTx;
#[doc = r"Cluster"]
#[doc = "SPI0_DMA_TRIG_TX\\[%s\\]"]
pub mod spi0_dma_trig_tx;
#[doc = "SPI0_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_evt_mode`] module"]
#[doc(alias = "SPI0_EVT_MODE")]
pub type Spi0EvtMode = crate::Reg<spi0_evt_mode::Spi0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod spi0_evt_mode;
#[doc = "SPI0_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_intctl`] module"]
#[doc(alias = "SPI0_INTCTL")]
pub type Spi0Intctl = crate::Reg<spi0_intctl::Spi0IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod spi0_intctl;
#[doc = "SPI0_CTL0 (rw) register accessor: SPI control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_ctl0`] module"]
#[doc(alias = "SPI0_CTL0")]
pub type Spi0Ctl0 = crate::Reg<spi0_ctl0::Spi0Ctl0Spec>;
#[doc = "SPI control register 0"]
pub mod spi0_ctl0;
#[doc = "SPI0_CTL1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_ctl1`] module"]
#[doc(alias = "SPI0_CTL1")]
pub type Spi0Ctl1 = crate::Reg<spi0_ctl1::Spi0Ctl1Spec>;
#[doc = "SPI control register 1"]
pub mod spi0_ctl1;
#[doc = "SPI0_CLKCTL (rw) register accessor: Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_clkctl`] module"]
#[doc(alias = "SPI0_CLKCTL")]
pub type Spi0Clkctl = crate::Reg<spi0_clkctl::Spi0ClkctlSpec>;
#[doc = "Clock prescaler and divider register."]
pub mod spi0_clkctl;
#[doc = "SPI0_IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_ifls`] module"]
#[doc(alias = "SPI0_IFLS")]
pub type Spi0Ifls = crate::Reg<spi0_ifls::Spi0IflsSpec>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod spi0_ifls;
#[doc = "SPI0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_stat`] module"]
#[doc(alias = "SPI0_STAT")]
pub type Spi0Stat = crate::Reg<spi0_stat::Spi0StatSpec>;
#[doc = "Status Register"]
pub mod spi0_stat;
#[doc = "SPI0_RXDATA (r) register accessor: RXDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_rxdata`] module"]
#[doc(alias = "SPI0_RXDATA")]
pub type Spi0Rxdata = crate::Reg<spi0_rxdata::Spi0RxdataSpec>;
#[doc = "RXDATA Register"]
pub mod spi0_rxdata;
#[doc = "SPI0_TXDATA (rw) register accessor: TXDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_txdata`] module"]
#[doc(alias = "SPI0_TXDATA")]
pub type Spi0Txdata = crate::Reg<spi0_txdata::Spi0TxdataSpec>;
#[doc = "TXDATA Register"]
pub mod spi0_txdata;
