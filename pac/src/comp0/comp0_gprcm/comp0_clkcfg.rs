#[doc = "Register `COMP0_CLKCFG` reader"]
pub type R = crate::R<Comp0ClkcfgSpec>;
#[doc = "Register `COMP0_CLKCFG` writer"]
pub type W = crate::W<Comp0ClkcfgSpec>;
#[doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasync {
    #[doc = "0: disable COMP to request SYSOSC"]
    Disable = 0,
    #[doc = "1: enable COMP to request SYSOSC"]
    Enable = 1,
}
impl From<Blockasync> for bool {
    #[inline(always)]
    fn from(variant: Blockasync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKASYNC` reader - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncR = crate::BitReader<Blockasync>;
impl BlockasyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockasync {
        match self.bits {
            false => Blockasync::Disable,
            true => Blockasync::Enable,
        }
    }
    #[doc = "disable COMP to request SYSOSC"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasync::Disable
    }
    #[doc = "enable COMP to request SYSOSC"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blockasync::Enable
    }
}
#[doc = "Field `BLOCKASYNC` writer - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type BlockasyncW<'a, REG> = crate::BitWriter<'a, REG, Blockasync>;
impl<'a, REG> BlockasyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable COMP to request SYSOSC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Disable)
    }
    #[doc = "enable COMP to request SYSOSC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Enable)
    }
}
impl R {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&self) -> BlockasyncR {
        BlockasyncR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn blockasync(&mut self) -> BlockasyncW<'_, Comp0ClkcfgSpec> {
        BlockasyncW::new(self, 8)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0ClkcfgSpec;
impl crate::RegisterSpec for Comp0ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_clkcfg::R`](R) reader structure"]
impl crate::Readable for Comp0ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`comp0_clkcfg::W`](W) writer structure"]
impl crate::Writable for Comp0ClkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CLKCFG to value 0"]
impl crate::Resettable for Comp0ClkcfgSpec {}
