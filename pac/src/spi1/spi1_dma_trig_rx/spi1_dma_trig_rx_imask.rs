#[doc = "Register `SPI1_DMA_TRIG_RX_IMASK` reader"]
pub type R = crate::R<Spi1DmaTrigRxImaskSpec>;
#[doc = "Register `SPI1_DMA_TRIG_RX_IMASK` writer"]
pub type W = crate::W<Spi1DmaTrigRxImaskSpec>;
#[doc = "SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - SPI Receive Time-Out event mask."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            false => Rtout::Clr,
            true => Rtout::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
}
#[doc = "Field `RTOUT` writer - SPI Receive Time-Out event mask."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
}
#[doc = "Receive FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Receive FIFO event mask."]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::Clr,
            true => Rx::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rx::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rx::Set
    }
}
#[doc = "Field `RX` writer - Receive FIFO event mask."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
}
impl R {
    #[doc = "Bit 2 - SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event mask."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Spi1DmaTrigRxImaskSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO event mask."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, Spi1DmaTrigRxImaskSpec> {
        RxW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_dma_trig_rx_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_dma_trig_rx_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1DmaTrigRxImaskSpec;
impl crate::RegisterSpec for Spi1DmaTrigRxImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_dma_trig_rx_imask::R`](R) reader structure"]
impl crate::Readable for Spi1DmaTrigRxImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`spi1_dma_trig_rx_imask::W`](W) writer structure"]
impl crate::Writable for Spi1DmaTrigRxImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_DMA_TRIG_RX_IMASK to value 0"]
impl crate::Resettable for Spi1DmaTrigRxImaskSpec {}
