#[doc = "Register `WWDT0_IMASK` reader"]
pub type R = crate::R<Wwdt0ImaskSpec>;
#[doc = "Register `WWDT0_IMASK` writer"]
pub type W = crate::W<Wwdt0ImaskSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inttim {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
}
impl From<Inttim> for bool {
    #[inline(always)]
    fn from(variant: Inttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTTIM` reader - Interval Timer Interrupt."]
pub type InttimR = crate::BitReader<Inttim>;
impl InttimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttim {
        match self.bits {
            false => Inttim::Clr,
            true => Inttim::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Inttim::Clr
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Inttim::Set
    }
}
#[doc = "Field `INTTIM` writer - Interval Timer Interrupt."]
pub type InttimW<'a, REG> = crate::BitWriter<'a, REG, Inttim>;
impl<'a, REG> InttimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Inttim::Clr)
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Inttim::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn inttim(&self) -> InttimR {
        InttimR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn inttim(&mut self) -> InttimW<'_, Wwdt0ImaskSpec> {
        InttimW::new(self, 0)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0ImaskSpec;
impl crate::RegisterSpec for Wwdt0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_imask::R`](R) reader structure"]
impl crate::Readable for Wwdt0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdt0_imask::W`](W) writer structure"]
impl crate::Writable for Wwdt0ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WWDT0_IMASK to value 0"]
impl crate::Resettable for Wwdt0ImaskSpec {}
