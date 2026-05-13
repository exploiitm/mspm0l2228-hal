#[doc = "Register `I2C0_DMA_TRIG0_IMASK` reader"]
pub type R = crate::R<I2c0DmaTrig0ImaskSpec>;
#[doc = "Register `I2C0_DMA_TRIG0_IMASK` writer"]
pub type W = crate::W<I2c0DmaTrig0ImaskSpec>;
#[doc = "Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crxfifotrg {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Crxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Crxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRXFIFOTRG` reader - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
pub type CrxfifotrgR = crate::BitReader<Crxfifotrg>;
impl CrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crxfifotrg {
        match self.bits {
            false => Crxfifotrg::Clr,
            true => Crxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Crxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Crxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Crxfifotrg::Clr)
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
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ctxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ctxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTXFIFOTRG` reader - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
pub type CtxfifotrgR = crate::BitReader<Ctxfifotrg>;
impl CtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctxfifotrg {
        match self.bits {
            false => Ctxfifotrg::Clr,
            true => Ctxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ctxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ctxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctxfifotrg::Clr)
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
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Trxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Trxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRXFIFOTRG` reader - Target Receive FIFO Trigger"]
pub type TrxfifotrgR = crate::BitReader<Trxfifotrg>;
impl TrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trxfifotrg {
        match self.bits {
            false => Trxfifotrg::Clr,
            true => Trxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trxfifotrg::Clr)
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
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Ttxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: Ttxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTXFIFOTRG` reader - Target Transmit FIFO Trigger"]
pub type TtxfifotrgR = crate::BitReader<Ttxfifotrg>;
impl TtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttxfifotrg {
        match self.bits {
            false => Ttxfifotrg::Clr,
            true => Ttxfifotrg::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ttxfifotrg::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ttxfifotrg::Set
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
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ttxfifotrg::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&self) -> CrxfifotrgR {
        CrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&self) -> CtxfifotrgR {
        CtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&self) -> TrxfifotrgR {
        TrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&self) -> TtxfifotrgR {
        TtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controller Receive FIFO Trigger Trigger when RX FIFO contains >= defined bytes"]
    #[inline(always)]
    pub fn crxfifotrg(&mut self) -> CrxfifotrgW<'_, I2c0DmaTrig0ImaskSpec> {
        CrxfifotrgW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller Transmit FIFO Trigger Trigger when Transmit FIFO contains <= defined bytes"]
    #[inline(always)]
    pub fn ctxfifotrg(&mut self) -> CtxfifotrgW<'_, I2c0DmaTrig0ImaskSpec> {
        CtxfifotrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Target Receive FIFO Trigger"]
    #[inline(always)]
    pub fn trxfifotrg(&mut self) -> TrxfifotrgW<'_, I2c0DmaTrig0ImaskSpec> {
        TrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Target Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn ttxfifotrg(&mut self) -> TtxfifotrgW<'_, I2c0DmaTrig0ImaskSpec> {
        TtxfifotrgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_dma_trig0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0DmaTrig0ImaskSpec;
impl crate::RegisterSpec for I2c0DmaTrig0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_dma_trig0_imask::R`](R) reader structure"]
impl crate::Readable for I2c0DmaTrig0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_dma_trig0_imask::W`](W) writer structure"]
impl crate::Writable for I2c0DmaTrig0ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_DMA_TRIG0_IMASK to value 0"]
impl crate::Resettable for I2c0DmaTrig0ImaskSpec {}
