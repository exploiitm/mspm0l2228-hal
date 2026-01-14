#[doc = "Register `SPI0_DMA_TRIG_RX_ISET` writer"]
pub type W = crate::W<Spi0DmaTrigRxIsetSpec>;
#[doc = "Set SPI Receive Time-Out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Set SPI Receive Time-Out event."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
}
#[doc = "Set Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` writer - Set Receive FIFO event."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
}
impl W {
    #[doc = "Bit 2 - Set SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi0DmaTrigRxIsetSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Set Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi0DmaTrigRxIsetSpec> {
        RxW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_dma_trig_rx_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0DmaTrigRxIsetSpec;
impl crate::RegisterSpec for Spi0DmaTrigRxIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi0_dma_trig_rx_iset::W`](W) writer structure"]
impl crate::Writable for Spi0DmaTrigRxIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_DMA_TRIG_RX_ISET to value 0"]
impl crate::Resettable for Spi0DmaTrigRxIsetSpec {}
