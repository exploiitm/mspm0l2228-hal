#[doc = "Register `COMP0_PWREN` reader"]
pub type R = crate::R<Comp0PwrenSpec>;
#[doc = "Register `COMP0_PWREN` writer"]
pub type W = crate::W<Comp0PwrenSpec>;
#[doc = "Enable the power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable Power"]
    Disable = 0,
    #[doc = "1: Enable Power"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable the power"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable the power"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Power"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable Power"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the power"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the power"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Comp0PwrenSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0PwrenSpec;
impl crate::RegisterSpec for Comp0PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_pwren::R`](R) reader structure"]
impl crate::Readable for Comp0PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`comp0_pwren::W`](W) writer structure"]
impl crate::Writable for Comp0PwrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_PWREN to value 0"]
impl crate::Resettable for Comp0PwrenSpec {}
