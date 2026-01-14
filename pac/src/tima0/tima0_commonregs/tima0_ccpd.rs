#[doc = "Register `TIMA0_CCPD` reader"]
pub type R = crate::R<Tima0CcpdSpec>;
#[doc = "Register `TIMA0_CCPD` writer"]
pub type W = crate::W<Tima0CcpdSpec>;
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
#[doc = "Counter CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp2 {
    #[doc = "0: input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp2> for bool {
    #[inline(always)]
    fn from(variant: C0ccp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP2` reader - Counter CCP2"]
pub type C0ccp2R = crate::BitReader<C0ccp2>;
impl C0ccp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp2 {
        match self.bits {
            false => C0ccp2::Input,
            true => C0ccp2::Output,
        }
    }
    #[doc = "input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp2::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp2::Output
    }
}
#[doc = "Field `C0CCP2` writer - Counter CCP2"]
pub type C0ccp2W<'a, REG> = crate::BitWriter<'a, REG, C0ccp2>;
impl<'a, REG> C0ccp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp2::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp2::Output)
    }
}
#[doc = "Counter CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp3 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<C0ccp3> for bool {
    #[inline(always)]
    fn from(variant: C0ccp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP3` reader - Counter CCP3"]
pub type C0ccp3R = crate::BitReader<C0ccp3>;
impl C0ccp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp3 {
        match self.bits {
            false => C0ccp3::Input,
            true => C0ccp3::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C0ccp3::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C0ccp3::Output
    }
}
#[doc = "Field `C0CCP3` writer - Counter CCP3"]
pub type C0ccp3W<'a, REG> = crate::BitWriter<'a, REG, C0ccp3>;
impl<'a, REG> C0ccp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp3::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp3::Output)
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
    #[doc = "Bit 2 - Counter CCP2"]
    #[inline(always)]
    pub fn c0ccp2(&self) -> C0ccp2R {
        C0ccp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter CCP3"]
    #[inline(always)]
    pub fn c0ccp3(&self) -> C0ccp3R {
        C0ccp3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter CCP0"]
    #[inline(always)]
    pub fn c0ccp0(&mut self) -> C0ccp0W<'_, Tima0CcpdSpec> {
        C0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter CCP1"]
    #[inline(always)]
    pub fn c0ccp1(&mut self) -> C0ccp1W<'_, Tima0CcpdSpec> {
        C0ccp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Counter CCP2"]
    #[inline(always)]
    pub fn c0ccp2(&mut self) -> C0ccp2W<'_, Tima0CcpdSpec> {
        C0ccp2W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter CCP3"]
    #[inline(always)]
    pub fn c0ccp3(&mut self) -> C0ccp3W<'_, Tima0CcpdSpec> {
        C0ccp3W::new(self, 3)
    }
}
#[doc = "CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_ccpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_ccpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0CcpdSpec;
impl crate::RegisterSpec for Tima0CcpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_ccpd::R`](R) reader structure"]
impl crate::Readable for Tima0CcpdSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_ccpd::W`](W) writer structure"]
impl crate::Writable for Tima0CcpdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_CCPD to value 0"]
impl crate::Resettable for Tima0CcpdSpec {}
