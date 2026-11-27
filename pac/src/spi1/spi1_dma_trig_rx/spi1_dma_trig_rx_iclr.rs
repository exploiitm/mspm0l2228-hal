#[doc = "Register `SPI1_DMA_TRIG_RX_ICLR` writer"]
pub type W = crate::W<Spi1DmaTrigRxIclrSpec>;
#[doc = "Clear SPI Receive Time-Out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Clr = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` writer - Clear SPI Receive Time-Out event."]
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
}
#[doc = "Clear Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` writer - Clear Receive FIFO event."]
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
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
}
impl W {
    #[doc = "Bit 2 - Clear SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi1DmaTrigRxIclrSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi1DmaTrigRxIclrSpec> {
        RxW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_rx_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigRxIclrSpec;
impl crate::RegisterSpec for Spi1DmaTrigRxIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi1_dma_trig_rx_iclr::W`](W) writer structure"]
impl crate::Writable for Spi1DmaTrigRxIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_RX_ICLR to value 0"]
impl crate::Resettable for Spi1DmaTrigRxIclrSpec {}
