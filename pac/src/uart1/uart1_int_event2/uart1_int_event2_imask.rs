#[doc = "Register `UART1_INT_EVENT2_IMASK` reader"]
pub type R = crate::R<Uart1IntEvent2ImaskSpec>;
#[doc = "Register `UART1_INT_EVENT2_IMASK` writer"]
pub type W = crate::W<Uart1IntEvent2ImaskSpec>;
#[doc = "Enable UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txint {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Txint> for bool {
    #[inline(always)]
    fn from(variant: Txint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINT` reader - Enable UART Transmit Interrupt."]
pub type TxintR = crate::BitReader<Txint>;
impl TxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txint {
        match self.bits {
            false => Txint::Clr,
            true => Txint::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txint::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txint::Set
    }
}
#[doc = "Field `TXINT` writer - Enable UART Transmit Interrupt."]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG, Txint>;
impl<'a, REG> TxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txint::Set)
    }
}
impl R {
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, Uart1IntEvent2ImaskSpec> {
        TxintW::new(self, 11)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IntEvent2ImaskSpec;
impl crate::RegisterSpec for Uart1IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_int_event2_imask::R`](R) reader structure"]
impl crate::Readable for Uart1IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_int_event2_imask::W`](W) writer structure"]
impl crate::Writable for Uart1IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for Uart1IntEvent2ImaskSpec {}
