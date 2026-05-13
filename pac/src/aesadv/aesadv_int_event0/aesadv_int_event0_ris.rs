#[doc = "Register `AESADV_INT_EVENT0_RIS` reader"]
pub type R = crate::R<AesadvIntEvent0RisSpec>;
#[doc = "This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outputrdy {
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Interrupt occured"]
    Set = 1,
}
impl From<Outputrdy> for bool {
    #[inline(always)]
    fn from(variant: Outputrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUTRDY` reader - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type OutputrdyR = crate::BitReader<Outputrdy>;
impl OutputrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outputrdy {
        match self.bits {
            false => Outputrdy::Clr,
            true => Outputrdy::Set,
        }
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Outputrdy::Clr
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Outputrdy::Set
    }
}
#[doc = "This indicates that the engine can take new input. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)\n\nValue on reset: 0"]
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
#[doc = "Field `INPUTRDY` reader - This indicates that the engine can take new input. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
pub type InputrdyR = crate::BitReader<Inputrdy>;
impl InputrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inputrdy {
        match self.bits {
            false => Inputrdy::Clr,
            true => Inputrdy::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Inputrdy::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Inputrdy::Set
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
#[doc = "Field `SAVEDCNTXTRDY` reader - This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
pub type SavedcntxtrdyR = crate::BitReader<Savedcntxtrdy>;
impl SavedcntxtrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Savedcntxtrdy {
        match self.bits {
            false => Savedcntxtrdy::Clr,
            true => Savedcntxtrdy::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Savedcntxtrdy::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Savedcntxtrdy::Set
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
#[doc = "Field `CNTXTRDY` reader - This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write next context."]
pub type CntxtrdyR = crate::BitReader<Cntxtrdy>;
impl CntxtrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntxtrdy {
        match self.bits {
            false => Cntxtrdy::Clr,
            true => Cntxtrdy::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cntxtrdy::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cntxtrdy::Set
    }
}
impl R {
    #[doc = "Bit 0 - This indicates that the core has an output available to be read out. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OutputrdyR {
        OutputrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This indicates that the engine can take new input. This should not be used if DMA handshake is used (AES_DMA_HS.DMA_DATA_ACK set to 1)"]
    #[inline(always)]
    pub fn inputrdy(&self) -> InputrdyR {
        InputrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the CPU to retrieve. This bit is only asserted if the save_context bit is set to 1b. The bit is mutually exclusive with the context_ready bit."]
    #[inline(always)]
    pub fn savedcntxtrdy(&self) -> SavedcntxtrdyR {
        SavedcntxtrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the context data registers can be overwritten, and the CPU is permitted to write next context."]
    #[inline(always)]
    pub fn cntxtrdy(&self) -> CntxtrdyR {
        CntxtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent0RisSpec;
impl crate::RegisterSpec for AesadvIntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event0_ris::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent0RisSpec {}
#[doc = "`reset()` method sets AESADV_INT_EVENT0_RIS to value 0"]
impl crate::Resettable for AesadvIntEvent0RisSpec {}
