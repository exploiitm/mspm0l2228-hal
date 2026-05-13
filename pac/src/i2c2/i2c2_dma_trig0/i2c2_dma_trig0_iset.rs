#[doc = "Register `I2C2_DMA_TRIG0_ISET` writer"]
pub type W = crate::W<I2c2DmaTrig0IsetSpec>;
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Crxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Crxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOTRG` writer - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
pub type CrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Crxfifotrg>;
impl<'a, REG> CrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::Set)
    }
}
#[doc = "Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ctxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ctxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXFIFOTRG` writer - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
pub type CtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Ctxfifotrg>;
impl<'a, REG> CtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::Set)
    }
}
#[doc = "Target Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Trxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Trxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOTRG` writer - Target Receive FIFO Trigger"]
pub type TrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Trxfifotrg>;
impl<'a, REG> TrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::Set)
    }
}
#[doc = "Target Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ttxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ttxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXFIFOTRG` writer - Target Transmit FIFO Trigger"]
pub type TtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, Ttxfifotrg>;
impl<'a, REG> TtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::NoEffect)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&mut self) -> CrxfifotrgW<'_, I2c2DmaTrig0IsetSpec> {
        CrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&mut self) -> CtxfifotrgW<'_, I2c2DmaTrig0IsetSpec> {
        CtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&mut self) -> TrxfifotrgW<'_, I2c2DmaTrig0IsetSpec> {
        TrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&mut self) -> TtxfifotrgW<'_, I2c2DmaTrig0IsetSpec> {
        TtxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_dma_trig0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2DmaTrig0IsetSpec;
impl crate::RegisterSpec for I2c2DmaTrig0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c2_dma_trig0_iset::W`](W) writer structure"]
impl crate::Writable for I2c2DmaTrig0IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_DMA_TRIG0_ISET to value 0"]
impl crate::Resettable for I2c2DmaTrig0IsetSpec {}
