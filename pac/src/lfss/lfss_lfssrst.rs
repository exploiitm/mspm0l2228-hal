#[doc = "Register `LFSS_LFSSRST` reader"]
pub type R = crate::R<LfssLfssrstSpec>;
#[doc = "Register `LFSS_LFSSRST` writer"]
pub type W = crate::W<LfssLfssrstSpec>;
#[doc = "If set, the register bit will request a power on reset to the PMU of the LFSS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatpor {
    #[doc = "0: Writing this value has no effect."]
    NoEffect = 0,
    #[doc = "1: Request power on reset to the LFSS."]
    Set = 1,
}
impl From<Vbatpor> for bool {
    #[inline(always)]
    fn from(variant: Vbatpor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPOR` reader - If set, the register bit will request a power on reset to the PMU of the LFSS."]
pub type VbatporR = crate::BitReader<Vbatpor>;
impl VbatporR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatpor {
        match self.bits {
            false => Vbatpor::NoEffect,
            true => Vbatpor::Set,
        }
    }
    #[doc = "Writing this value has no effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Vbatpor::NoEffect
    }
    #[doc = "Request power on reset to the LFSS."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Vbatpor::Set
    }
}
#[doc = "Field `VBATPOR` writer - If set, the register bit will request a power on reset to the PMU of the LFSS."]
pub type VbatporW<'a, REG> = crate::BitWriter<'a, REG, Vbatpor>;
impl<'a, REG> VbatporW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing this value has no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpor::NoEffect)
    }
    #[doc = "Request power on reset to the LFSS."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpor::Set)
    }
}
impl R {
    #[doc = "Bit 0 - If set, the register bit will request a power on reset to the PMU of the LFSS."]
    #[inline(always)]
    pub fn vbatpor(&self) -> VbatporR {
        VbatporR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, the register bit will request a power on reset to the PMU of the LFSS."]
    #[inline(always)]
    pub fn vbatpor(&mut self) -> VbatporW<'_, LfssLfssrstSpec> {
        VbatporW::new(self, 0)
    }
}
#[doc = "Low frequency sub-system reset request\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_lfssrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_lfssrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssLfssrstSpec;
impl crate::RegisterSpec for LfssLfssrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_lfssrst::R`](R) reader structure"]
impl crate::Readable for LfssLfssrstSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_lfssrst::W`](W) writer structure"]
impl crate::Writable for LfssLfssrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_LFSSRST to value 0"]
impl crate::Resettable for LfssLfssrstSpec {}
