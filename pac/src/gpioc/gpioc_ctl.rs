#[doc = "Register `GPIOC_CTL` reader"]
pub type R = crate::R<GpiocCtlSpec>;
#[doc = "Register `GPIOC_CTL` writer"]
pub type W = crate::W<GpiocCtlSpec>;
#[doc = "FASTWAKEONLY for the global control of fastwake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastwakeonly {
    #[doc = "0: The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    NotGlobalEn = 0,
    #[doc = "1: The global control of fastwake is enabled"]
    GlobalEn = 1,
}
impl From<Fastwakeonly> for bool {
    #[inline(always)]
    fn from(variant: Fastwakeonly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKEONLY` reader - FASTWAKEONLY for the global control of fastwake"]
pub type FastwakeonlyR = crate::BitReader<Fastwakeonly>;
impl FastwakeonlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastwakeonly {
        match self.bits {
            false => Fastwakeonly::NotGlobalEn,
            true => Fastwakeonly::GlobalEn,
        }
    }
    #[doc = "The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    #[inline(always)]
    pub fn is_not_global_en(&self) -> bool {
        *self == Fastwakeonly::NotGlobalEn
    }
    #[doc = "The global control of fastwake is enabled"]
    #[inline(always)]
    pub fn is_global_en(&self) -> bool {
        *self == Fastwakeonly::GlobalEn
    }
}
#[doc = "Field `FASTWAKEONLY` writer - FASTWAKEONLY for the global control of fastwake"]
pub type FastwakeonlyW<'a, REG> = crate::BitWriter<'a, REG, Fastwakeonly>;
impl<'a, REG> FastwakeonlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The global control of fastwake is not enabled, per bit fast wake feature depends on FASTWAKE.DIN"]
    #[inline(always)]
    pub fn not_global_en(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwakeonly::NotGlobalEn)
    }
    #[doc = "The global control of fastwake is enabled"]
    #[inline(always)]
    pub fn global_en(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwakeonly::GlobalEn)
    }
}
impl R {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn fastwakeonly(&self) -> FastwakeonlyR {
        FastwakeonlyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn fastwakeonly(&mut self) -> FastwakeonlyW<'_, GpiocCtlSpec> {
        FastwakeonlyW::new(self, 0)
    }
}
#[doc = "FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocCtlSpec;
impl crate::RegisterSpec for GpiocCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_ctl::R`](R) reader structure"]
impl crate::Readable for GpiocCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc_ctl::W`](W) writer structure"]
impl crate::Writable for GpiocCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC_CTL to value 0"]
impl crate::Resettable for GpiocCtlSpec {}
