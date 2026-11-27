#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    spi1_gprcm: [Spi1Gprcm; 1],
    _reserved1: [u8; 0x07e8],
    spi1_clkdiv: Spi1Clkdiv,
    spi1_clksel: Spi1Clksel,
    _reserved3: [u8; 0x10],
    spi1_pdbgctl: Spi1Pdbgctl,
    _reserved4: [u8; 0x04],
    spi1_cpu_int: [Spi1CpuInt; 1],
    _reserved5: [u8; 0x04],
    spi1_dma_trig_rx: [Spi1DmaTrigRx; 1],
    _reserved6: [u8; 0x04],
    spi1_dma_trig_tx: [Spi1DmaTrigTx; 1],
    _reserved7: [u8; 0x34],
    spi1_evt_mode: Spi1EvtMode,
    spi1_intctl: Spi1Intctl,
    _reserved9: [u8; 0x18],
    spi1_ctl0: Spi1Ctl0,
    spi1_ctl1: Spi1Ctl1,
    spi1_clkctl: Spi1Clkctl,
    spi1_ifls: Spi1Ifls,
    spi1_stat: Spi1Stat,
    _reserved14: [u8; 0x1c],
    spi1_rxdata: Spi1Rxdata,
    _reserved15: [u8; 0x0c],
    spi1_txdata: Spi1Txdata,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - SPI1_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn spi1_gprcm(&self, n: usize) -> &Spi1Gprcm {
        &self.spi1_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - SPI1_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn spi1_gprcm_iter(&self) -> impl Iterator<Item = &Spi1Gprcm> {
        self.spi1_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn spi1_clkdiv(&self) -> &Spi1Clkdiv {
        &self.spi1_clkdiv
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn spi1_clksel(&self) -> &Spi1Clksel {
        &self.spi1_clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn spi1_pdbgctl(&self) -> &Spi1Pdbgctl {
        &self.spi1_pdbgctl
    }
    #[doc = "0x1020..0x104c - SPI1_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub const fn spi1_cpu_int(&self, n: usize) -> &Spi1CpuInt {
        &self.spi1_cpu_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - SPI1_CPU_INT\\[%s\\]"]
    #[inline(always)]
    pub fn spi1_cpu_int_iter(&self) -> impl Iterator<Item = &Spi1CpuInt> {
        self.spi1_cpu_int.iter()
    }
    #[doc = "0x1050..0x107c - SPI1_DMA_TRIG_RX\\[%s\\]"]
    #[inline(always)]
    pub const fn spi1_dma_trig_rx(&self, n: usize) -> &Spi1DmaTrigRx {
        &self.spi1_dma_trig_rx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - SPI1_DMA_TRIG_RX\\[%s\\]"]
    #[inline(always)]
    pub fn spi1_dma_trig_rx_iter(&self) -> impl Iterator<Item = &Spi1DmaTrigRx> {
        self.spi1_dma_trig_rx.iter()
    }
    #[doc = "0x1080..0x10ac - SPI1_DMA_TRIG_TX\\[%s\\]"]
    #[inline(always)]
    pub const fn spi1_dma_trig_tx(&self, n: usize) -> &Spi1DmaTrigTx {
        &self.spi1_dma_trig_tx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - SPI1_DMA_TRIG_TX\\[%s\\]"]
    #[inline(always)]
    pub fn spi1_dma_trig_tx_iter(&self) -> impl Iterator<Item = &Spi1DmaTrigTx> {
        self.spi1_dma_trig_tx.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn spi1_evt_mode(&self) -> &Spi1EvtMode {
        &self.spi1_evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn spi1_intctl(&self) -> &Spi1Intctl {
        &self.spi1_intctl
    }
    #[doc = "0x1100 - SPI control register 0"]
    #[inline(always)]
    pub const fn spi1_ctl0(&self) -> &Spi1Ctl0 {
        &self.spi1_ctl0
    }
    #[doc = "0x1104 - SPI control register 1"]
    #[inline(always)]
    pub const fn spi1_ctl1(&self) -> &Spi1Ctl1 {
        &self.spi1_ctl1
    }
    #[doc = "0x1108 - Clock prescaler and divider register."]
    #[inline(always)]
    pub const fn spi1_clkctl(&self) -> &Spi1Clkctl {
        &self.spi1_clkctl
    }
    #[doc = "0x110c - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn spi1_ifls(&self) -> &Spi1Ifls {
        &self.spi1_ifls
    }
    #[doc = "0x1110 - Status Register"]
    #[inline(always)]
    pub const fn spi1_stat(&self) -> &Spi1Stat {
        &self.spi1_stat
    }
    #[doc = "0x1130 - RXDATA Register"]
    #[inline(always)]
    pub const fn spi1_rxdata(&self) -> &Spi1Rxdata {
        &self.spi1_rxdata
    }
    #[doc = "0x1140 - TXDATA Register"]
    #[inline(always)]
    pub const fn spi1_txdata(&self) -> &Spi1Txdata {
        &self.spi1_txdata
    }
}
#[doc = "SPI1_GPRCM\\[%s\\]"]
pub use self::spi1_gprcm::Spi1Gprcm;
#[doc = r"Cluster"]
#[doc = "SPI1_GPRCM\\[%s\\]"]
pub mod spi1_gprcm;
#[doc = "SPI1_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_clkdiv`] module"]
#[doc(alias = "SPI1_CLKDIV")]
pub type Spi1Clkdiv = crate::Reg<spi1_clkdiv::Spi1ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod spi1_clkdiv;
#[doc = "SPI1_CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_clksel`] module"]
#[doc(alias = "SPI1_CLKSEL")]
pub type Spi1Clksel = crate::Reg<spi1_clksel::Spi1ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod spi1_clksel;
#[doc = "SPI1_PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_pdbgctl`] module"]
#[doc(alias = "SPI1_PDBGCTL")]
pub type Spi1Pdbgctl = crate::Reg<spi1_pdbgctl::Spi1PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod spi1_pdbgctl;
#[doc = "SPI1_CPU_INT\\[%s\\]"]
pub use self::spi1_cpu_int::Spi1CpuInt;
#[doc = r"Cluster"]
#[doc = "SPI1_CPU_INT\\[%s\\]"]
pub mod spi1_cpu_int;
#[doc = "SPI1_DMA_TRIG_RX\\[%s\\]"]
pub use self::spi1_dma_trig_rx::Spi1DmaTrigRx;
#[doc = r"Cluster"]
#[doc = "SPI1_DMA_TRIG_RX\\[%s\\]"]
pub mod spi1_dma_trig_rx;
#[doc = "SPI1_DMA_TRIG_TX\\[%s\\]"]
pub use self::spi1_dma_trig_tx::Spi1DmaTrigTx;
#[doc = r"Cluster"]
#[doc = "SPI1_DMA_TRIG_TX\\[%s\\]"]
pub mod spi1_dma_trig_tx;
#[doc = "SPI1_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_evt_mode`] module"]
#[doc(alias = "SPI1_EVT_MODE")]
pub type Spi1EvtMode = crate::Reg<spi1_evt_mode::Spi1EvtModeSpec>;
#[doc = "Event Mode"]
pub mod spi1_evt_mode;
#[doc = "SPI1_INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_intctl`] module"]
#[doc(alias = "SPI1_INTCTL")]
pub type Spi1Intctl = crate::Reg<spi1_intctl::Spi1IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod spi1_intctl;
#[doc = "SPI1_CTL0 (rw) register accessor: SPI control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_ctl0`] module"]
#[doc(alias = "SPI1_CTL0")]
pub type Spi1Ctl0 = crate::Reg<spi1_ctl0::Spi1Ctl0Spec>;
#[doc = "SPI control register 0"]
pub mod spi1_ctl0;
#[doc = "SPI1_CTL1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_ctl1`] module"]
#[doc(alias = "SPI1_CTL1")]
pub type Spi1Ctl1 = crate::Reg<spi1_ctl1::Spi1Ctl1Spec>;
#[doc = "SPI control register 1"]
pub mod spi1_ctl1;
#[doc = "SPI1_CLKCTL (rw) register accessor: Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_clkctl`] module"]
#[doc(alias = "SPI1_CLKCTL")]
pub type Spi1Clkctl = crate::Reg<spi1_clkctl::Spi1ClkctlSpec>;
#[doc = "Clock prescaler and divider register."]
pub mod spi1_clkctl;
#[doc = "SPI1_IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_ifls`] module"]
#[doc(alias = "SPI1_IFLS")]
pub type Spi1Ifls = crate::Reg<spi1_ifls::Spi1IflsSpec>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod spi1_ifls;
#[doc = "SPI1_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_stat`] module"]
#[doc(alias = "SPI1_STAT")]
pub type Spi1Stat = crate::Reg<spi1_stat::Spi1StatSpec>;
#[doc = "Status Register"]
pub mod spi1_stat;
#[doc = "SPI1_RXDATA (r) register accessor: RXDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_rxdata`] module"]
#[doc(alias = "SPI1_RXDATA")]
pub type Spi1Rxdata = crate::Reg<spi1_rxdata::Spi1RxdataSpec>;
#[doc = "RXDATA Register"]
pub mod spi1_rxdata;
#[doc = "SPI1_TXDATA (rw) register accessor: TXDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_txdata`] module"]
#[doc(alias = "SPI1_TXDATA")]
pub type Spi1Txdata = crate::Reg<spi1_txdata::Spi1TxdataSpec>;
#[doc = "TXDATA Register"]
pub mod spi1_txdata;
