#[doc = "Register `TIMG8_ODIS` reader"]
pub type R = crate::R<Timg8OdisSpec>;
#[doc = "Register `TIMG8_ODIS` writer"]
pub type W = crate::W<Timg8OdisSpec>;
#[doc = "Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp0 {
    #[doc = "0: Output function as selected by the OCTL register CCPO field are provided to occpout\\[0\\]."]
    CcpOutputOctl = 0,
    #[doc = "1: CCP output occpout\\[0\\] is forced low."]
    CcpOutputLow = 1,
}
impl From<C0ccp0> for bool {
    #[inline(always)]
    fn from(variant: C0ccp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP0` reader - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type C0ccp0R = crate::BitReader<C0ccp0>;
impl C0ccp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp0 {
        match self.bits {
            false => C0ccp0::CcpOutputOctl,
            true => C0ccp0::CcpOutputLow,
        }
    }
    #[doc = "Output function as selected by the OCTL register CCPO field are provided to occpout\\[0\\]."]
    #[inline(always)]
    pub fn is_ccp_output_octl(&self) -> bool {
        *self == C0ccp0::CcpOutputOctl
    }
    #[doc = "CCP output occpout\\[0\\] is forced low."]
    #[inline(always)]
    pub fn is_ccp_output_low(&self) -> bool {
        *self == C0ccp0::CcpOutputLow
    }
}
#[doc = "Field `C0CCP0` writer - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type C0ccp0W<'a, REG> = crate::BitWriter<'a, REG, C0ccp0>;
impl<'a, REG> C0ccp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output function as selected by the OCTL register CCPO field are provided to occpout\\[0\\]."]
    #[inline(always)]
    pub fn ccp_output_octl(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::CcpOutputOctl)
    }
    #[doc = "CCP output occpout\\[0\\] is forced low."]
    #[inline(always)]
    pub fn ccp_output_low(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp0::CcpOutputLow)
    }
}
#[doc = "Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ccp1 {
    #[doc = "0: Output function as selected by the OCTL register CCPO field are provided to occpout\\[1\\]."]
    CcpOutputOctl = 0,
    #[doc = "1: CCP output occpout\\[1\\] is forced low."]
    CcpOutputLow = 1,
}
impl From<C0ccp1> for bool {
    #[inline(always)]
    fn from(variant: C0ccp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CCP1` reader - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type C0ccp1R = crate::BitReader<C0ccp1>;
impl C0ccp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ccp1 {
        match self.bits {
            false => C0ccp1::CcpOutputOctl,
            true => C0ccp1::CcpOutputLow,
        }
    }
    #[doc = "Output function as selected by the OCTL register CCPO field are provided to occpout\\[1\\]."]
    #[inline(always)]
    pub fn is_ccp_output_octl(&self) -> bool {
        *self == C0ccp1::CcpOutputOctl
    }
    #[doc = "CCP output occpout\\[1\\] is forced low."]
    #[inline(always)]
    pub fn is_ccp_output_low(&self) -> bool {
        *self == C0ccp1::CcpOutputLow
    }
}
#[doc = "Field `C0CCP1` writer - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type C0ccp1W<'a, REG> = crate::BitWriter<'a, REG, C0ccp1>;
impl<'a, REG> C0ccp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output function as selected by the OCTL register CCPO field are provided to occpout\\[1\\]."]
    #[inline(always)]
    pub fn ccp_output_octl(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::CcpOutputOctl)
    }
    #[doc = "CCP output occpout\\[1\\] is forced low."]
    #[inline(always)]
    pub fn ccp_output_low(self) -> &'a mut crate::W<REG> {
        self.variant(C0ccp1::CcpOutputLow)
    }
}
impl R {
    #[doc = "Bit 0 - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn c0ccp0(&self) -> C0ccp0R {
        C0ccp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn c0ccp1(&self) -> C0ccp1R {
        C0ccp1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn c0ccp0(&mut self) -> C0ccp0W<'_, Timg8OdisSpec> {
        C0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn c0ccp1(&mut self) -> C0ccp1W<'_, Timg8OdisSpec> {
        C0ccp1W::new(self, 1)
    }
}
#[doc = "Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_odis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg8_odis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg8OdisSpec;
impl crate::RegisterSpec for Timg8OdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg8_odis::R`](R) reader structure"]
impl crate::Readable for Timg8OdisSpec {}
#[doc = "`write(|w| ..)` method takes [`timg8_odis::W`](W) writer structure"]
impl crate::Writable for Timg8OdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG8_ODIS to value 0"]
impl crate::Resettable for Timg8OdisSpec {}
