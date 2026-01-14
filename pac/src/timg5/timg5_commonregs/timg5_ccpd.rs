#[doc = "Register `TIMG5_CCPD` reader"]
pub type R = crate::R<Timg5CcpdSpec>;
#[doc = "Register `TIMG5_CCPD` writer"]
pub type W = crate::W<Timg5CcpdSpec>;
#[doc = "Counter CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp0 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp0> for bool {
    #[inline(always)]
    fn from(variant: C0ccp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP0` reader - Counter CCP0"]
pub type C0ccp0R = crate::BitReader<C0ccp0>;
impl C0ccp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp0 {
        match self.bits {
            false => C0ccp0::Input,
            true => C0ccp0::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp0::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp0::Output
    }
}
#[doc = "Field `C0CCP0` writer - Counter CCP0"]
pub type C0ccp0W<'a, REG> = crate::BitWriter<'a, REG, C0ccp0>;
impl<'a, REG> C0ccp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::Output)
    }
}
#[doc = "Counter CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp1 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp1> for bool {
    #[inline(always)]
    fn from(variant: C0ccp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP1` reader - Counter CCP1"]
pub type C0ccp1R = crate::BitReader<C0ccp1>;
impl C0ccp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp1 {
        match self.bits {
            false => C0ccp1::Input,
            true => C0ccp1::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp1::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp1::Output
    }
}
#[doc = "Field `C0CCP1` writer - Counter CCP1"]
pub type C0ccp1W<'a, REG> = crate::BitWriter<'a, REG, C0ccp1>;
impl<'a, REG> C0ccp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::Output)
    }
}
impl R {
    #[doc = "Bit 0 - Counter CCP0"]
    #[inline(always)]
    pub fn c0ccp0(&self) -> C0ccp0R {
        C0ccp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter CCP1"]
    #[inline(always)]
    pub fn c0ccp1(&self) -> C0ccp1R {
        C0ccp1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter CCP0"]
    #[inline(always)]
    pub fn c0ccp0(&mut self) -> C0ccp0W<'_, Timg5CcpdSpec> {
        C0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter CCP1"]
    #[inline(always)]
    pub fn c0ccp1(&mut self) -> C0ccp1W<'_, Timg5CcpdSpec> {
        C0ccp1W::new(self, 1)
    }
}
#[doc = "CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`timg5_ccpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg5_ccpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg5CcpdSpec;
impl crate::RegisterSpec for Timg5CcpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg5_ccpd::R`](R) reader structure"]
impl crate::Readable for Timg5CcpdSpec {}
#[doc = "`write(|w| ..)` method takes [`timg5_ccpd::W`](W) writer structure"]
impl crate::Writable for Timg5CcpdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG5_CCPD to value 0"]
impl crate::Resettable for Timg5CcpdSpec {}
