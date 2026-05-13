#[doc = "Register `SPI0_DMA_TRIG_RX_IIDX` reader"]
pub type R = crate::R<Spi0DmaTrigRxIidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "3: SPI receive time-out interrupt"]
    RtoutEvt = 3,
    #[doc = "4: Receive Event/interrupt pending"]
    RxEvt = 4,
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
            3 => Some(Stat::RtoutEvt),
            4 => Some(Stat::RxEvt),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
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
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_dma_trig_rx_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0DmaTrigRxIidxSpec;
impl crate::RegisterSpec for Spi0DmaTrigRxIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_dma_trig_rx_iidx::R`](R) reader structure"]
impl crate::Readable for Spi0DmaTrigRxIidxSpec {}
#[doc = "`reset()` method sets SPI0_DMA_TRIG_RX_IIDX to value 0"]
impl crate::Resettable for Spi0DmaTrigRxIidxSpec {}
