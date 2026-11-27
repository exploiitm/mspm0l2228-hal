#[doc = "Register `SPI0_CPU_INT_IIDX` reader"]
pub type R = crate::R<Spi0CpuIntIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: RX FIFO Overflow Event/interrupt pending"]
    RxfifoOfvEvt = 1,
    #[doc = "2: Transmit Parity Event/interrupt pending"]
    PerEvt = 2,
    #[doc = "3: SPI receive time-out interrupt"]
    RtoutEvt = 3,
    #[doc = "4: Receive Event/interrupt pending"]
    RxEvt = 4,
    #[doc = "5: Transmit Event/interrupt pending"]
    TxEvt = 5,
    #[doc = "6: Transmit Buffer Empty Event/interrupt pending"]
    TxEmpty = 6,
    #[doc = "7: End of Transmit Event/interrupt pending"]
    IdleEvt = 7,
    #[doc = "8: DMA Done for Receive Event/interrupt pending"]
    DmaDoneRxEvt = 8,
    #[doc = "9: DMA Done for Transmit Event/interrupt pending"]
    DmaDoneTxEvt = 9,
    #[doc = "10: TX FIFO underflow interrupt"]
    TxfifoUnfEvt = 10,
    #[doc = "11: RX FIFO Full Interrupt"]
    RxfullEvt = 11,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::RxfifoOfvEvt),
            2 => Some(Stat::PerEvt),
            3 => Some(Stat::RtoutEvt),
            4 => Some(Stat::RxEvt),
            5 => Some(Stat::TxEvt),
            6 => Some(Stat::TxEmpty),
            7 => Some(Stat::IdleEvt),
            8 => Some(Stat::DmaDoneRxEvt),
            9 => Some(Stat::DmaDoneTxEvt),
            10 => Some(Stat::TxfifoUnfEvt),
            11 => Some(Stat::RxfullEvt),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "RX FIFO Overflow Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rxfifo_ofv_evt(&self) -> bool {
        *self == Stat::RxfifoOfvEvt
    }
    #[doc = "Transmit Parity Event/interrupt pending"]
    #[inline(always)]
    pub fn is_per_evt(&self) -> bool {
        *self == Stat::PerEvt
    }
    #[doc = "SPI receive time-out interrupt"]
    #[inline(always)]
    pub fn is_rtout_evt(&self) -> bool {
        *self == Stat::RtoutEvt
    }
    #[doc = "Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn is_rx_evt(&self) -> bool {
        *self == Stat::RxEvt
    }
    #[doc = "Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_evt(&self) -> bool {
        *self == Stat::TxEvt
    }
    #[doc = "Transmit Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_tx_empty(&self) -> bool {
        *self == Stat::TxEmpty
    }
    #[doc = "End of Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_idle_evt(&self) -> bool {
        *self == Stat::IdleEvt
    }
    #[doc = "DMA Done for Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn is_dma_done_rx_evt(&self) -> bool {
        *self == Stat::DmaDoneRxEvt
    }
    #[doc = "DMA Done for Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn is_dma_done_tx_evt(&self) -> bool {
        *self == Stat::DmaDoneTxEvt
    }
    #[doc = "TX FIFO underflow interrupt"]
    #[inline(always)]
    pub fn is_txfifo_unf_evt(&self) -> bool {
        *self == Stat::TxfifoUnfEvt
    }
    #[doc = "RX FIFO Full Interrupt"]
    #[inline(always)]
    pub fn is_rxfull_evt(&self) -> bool {
        *self == Stat::RxfullEvt
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_cpu_int_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0CpuIntIidxSpec;
impl crate::RegisterSpec for Spi0CpuIntIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_cpu_int_iidx::R`](R) reader structure"]
impl crate::Readable for Spi0CpuIntIidxSpec {}
#[doc = "`reset()` method sets SPI0_CPU_INT_IIDX to value 0"]
impl crate::Resettable for Spi0CpuIntIidxSpec {}
