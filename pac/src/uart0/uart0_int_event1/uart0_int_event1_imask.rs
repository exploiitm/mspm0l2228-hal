#[doc = "Register `UART0_INT_EVENT1_IMASK` reader"]
pub type R = crate::R<Uart0IntEvent1ImaskSpec>;
#[doc = "Register `UART0_INT_EVENT1_IMASK` writer"]
pub type W = crate::W<Uart0IntEvent1ImaskSpec>;
#[doc = "Enable UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `RTOUT` reader - Enable UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Field `RTOUT` writer - Enable UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Enable UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxint {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Rxint> for bool {
    #[inline(always)]
    fn from(variant: Rxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINT` reader - Enable UART Receive Interrupt."]
pub type RxintR = crate::BitReader<Rxint>;
impl RxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxint {
        match self.bits {
            false => Rxint::Clr,
            true => Rxint::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxint::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxint::Set
    }
}
#[doc = "Field `RXINT` writer - Enable UART Receive Interrupt."]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG, Rxint>;
impl<'a, REG> RxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxint::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn rtout(&mut self) -> RtoutW<'_, Uart0IntEvent1ImaskSpec> {
        RtoutW::new(self, 0)
    }
    #[doc = "Bit 10 - Enable UART Receive Interrupt."]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, Uart0IntEvent1ImaskSpec> {
        RxintW::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart0IntEvent1ImaskSpec;
impl crate::RegisterSpec for Uart0IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_int_event1_imask::R`](R) reader structure"]
impl crate::Readable for Uart0IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uart0_int_event1_imask::W`](W) writer structure"]
impl crate::Writable for Uart0IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART0_INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for Uart0IntEvent1ImaskSpec {}
