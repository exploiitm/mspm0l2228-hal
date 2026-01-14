#[doc = "Register `AESADV_INT_EVENT2_IMASK` reader"]
pub type R = crate::R<AesadvIntEvent2ImaskSpec>;
#[doc = "Register `AESADV_INT_EVENT2_IMASK` writer"]
pub type W = crate::W<AesadvIntEvent2ImaskSpec>;
#[doc = "TRIG1 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig1 {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Trig1> for bool {
    #[inline(always)]
    fn from(variant: Trig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG1` reader - TRIG1 event mask."]
pub type Trig1R = crate::BitReader<Trig1>;
impl Trig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig1 {
        match self.bits {
            false => Trig1::Clr,
            true => Trig1::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Trig1::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Trig1::Set
    }
}
#[doc = "Field `TRIG1` writer - TRIG1 event mask."]
pub type Trig1W<'a, REG> = crate::BitWriter<'a, REG, Trig1>;
impl<'a, REG> Trig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Trig1::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Trig1::Set)
    }
}
impl R {
    #[doc = "Bit 0 - TRIG1 event mask."]
    #[inline(always)]
    pub fn trig1(&self) -> Trig1R {
        Trig1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG1 event mask."]
    #[inline(always)]
    pub fn trig1(&mut self) -> Trig1W<'_, AesadvIntEvent2ImaskSpec> {
        Trig1W::new(self, 0)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIntEvent2ImaskSpec;
impl crate::RegisterSpec for AesadvIntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_int_event2_imask::R`](R) reader structure"]
impl crate::Readable for AesadvIntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_int_event2_imask::W`](W) writer structure"]
impl crate::Writable for AesadvIntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for AesadvIntEvent2ImaskSpec {}
