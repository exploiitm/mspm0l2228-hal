#[doc = "Register `AESADV_INT_EVENT0_ISET` writer"]
pub type W = crate::W<AesadvIntEvent0IsetSpec>;
#[doc = "This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outputrdy {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set Interrupt"]
    Set = 1,
}
impl From<Outputrdy> for bool {
    #[inline(always)]
    fn from(variant: Outputrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUTRDY` writer - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type OutputrdyW<'a, REG> = crate::BitWriter<'a, REG, Outputrdy>;
impl<'a, REG> OutputrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Outputrdy::NoEffect)
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Outputrdy::Set)
    }
}
#[doc = "This indicates that the engine can take new input.This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inputrdy {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Inputrdy> for bool {
    #[inline(always)]
    fn from(variant: Inputrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUTRDY` writer - This indicates that the engine can take new input.This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type InputrdyW<'a, REG> = crate::BitWriter<'a, REG, Inputrdy>;
impl<'a, REG> InputrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Inputrdy::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Inputrdy::Set)
    }
}
#[doc = "This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Savedcntxtrdy {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Savedcntxtrdy> for bool {
    #[inline(always)]
    fn from(variant: Savedcntxtrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAVEDCNTXTRDY` writer - This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
pub type SavedcntxtrdyW<'a, REG> = crate::BitWriter<'a, REG, Savedcntxtrdy>;
impl<'a, REG> SavedcntxtrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Savedcntxtrdy::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Savedcntxtrdy::Set)
    }
}
#[doc = "This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write next context.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntxtrdy {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Cntxtrdy> for bool {
    #[inline(always)]
    fn from(variant: Cntxtrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTXTRDY` writer - This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write next context."]
pub type CntxtrdyW<'a, REG> = crate::BitWriter<'a, REG, Cntxtrdy>;
impl<'a, REG> CntxtrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cntxtrdy::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cntxtrdy::Set)
    }
}
impl W {
    #[doc = "Bit 0 - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn outputrdy(&mut self) -> OutputrdyW<'_, AesadvIntEvent0IsetSpec> {
        OutputrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - This indicates that the engine can take new input.This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn inputrdy(&mut self) -> InputrdyW<'_, AesadvIntEvent0IsetSpec> {
        InputrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
    #[inline(always)]
    pub fn savedcntxtrdy(&mut self) -> SavedcntxtrdyW<'_, AesadvIntEvent0IsetSpec> {
        SavedcntxtrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write next context."]
    #[inline(always)]
    pub fn cntxtrdy(&mut self) -> CntxtrdyW<'_, AesadvIntEvent0IsetSpec> {
        CntxtrdyW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent0IsetSpec;
impl crate::RegisterSpec for AesadvIntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event0_iset::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent0IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT0_ISET to value 0"]
impl crate::Resettable for AesadvIntEvent0IsetSpec {}
