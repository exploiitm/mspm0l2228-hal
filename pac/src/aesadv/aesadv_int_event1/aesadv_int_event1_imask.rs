#[doc = "Register `AESADV_INT_EVENT1_IMASK` reader"]
pub type R = crate::R<AesadvIntEvent1ImaskSpec>;
#[doc = "Register `AESADV_INT_EVENT1_IMASK` writer"]
pub type W = crate::W<AesadvIntEvent1ImaskSpec>;
#[doc = "TRIG0 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig0 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Trig0> for bool {
    #[inline(always)]
    fn from(variant: Trig0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG0` reader - TRIG0 event mask."]
pub type Trig0R = crate::BitReader<Trig0>;
impl Trig0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig0 {
        match self.bits {
            false => Trig0::Clr,
            true => Trig0::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trig0::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trig0::Set
    }
}
#[doc = "Field `TRIG0` writer - TRIG0 event mask."]
pub type Trig0W<'a, REG> = crate::BitWriter<'a, REG, Trig0>;
impl<'a, REG> Trig0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trig0::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trig0::Set)
    }
}
impl R {
    #[doc = "Bit 0 - TRIG0 event mask."]
    #[inline(always)]
    pub fn trig0(&self) -> Trig0R {
        Trig0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0 event mask."]
    #[inline(always)]
    pub fn trig0(&mut self) -> Trig0W<'_, AesadvIntEvent1ImaskSpec> {
        Trig0W::new(self, 0)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent1ImaskSpec;
impl crate::RegisterSpec for AesadvIntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event1_imask::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event1_imask::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for AesadvIntEvent1ImaskSpec {}
