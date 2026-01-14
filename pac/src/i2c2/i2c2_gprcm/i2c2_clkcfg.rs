#[doc = "Register `I2C2_CLKCFG` reader"]
pub type R = crate::R<I2c2ClkcfgSpec>;
#[doc = "Register `I2C2_CLKCFG` writer"]
pub type W = crate::W<I2c2ClkcfgSpec>;
#[doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockasync {
    #[doc = "0: Not block async clock request"]
    Disable = 0,
    #[doc = "1: Block async clock request"]
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
    #[doc = "Not block async clock request"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blockasync::Disable
    }
    #[doc = "Block async clock request"]
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
    #[doc = "Not block async clock request"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blockasync::Disable)
    }
    #[doc = "Block async clock request"]
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
    pub fn blockasync(&mut self) -> BlockasyncW<'_, I2c2ClkcfgSpec> {
        BlockasyncW::new(self, 8)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2ClkcfgSpec;
impl crate::RegisterSpec for I2c2ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_clkcfg::R`](R) reader structure"]
impl crate::Readable for I2c2ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c2_clkcfg::W`](W) writer structure"]
impl crate::Writable for I2c2ClkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_CLKCFG to value 0"]
impl crate::Resettable for I2c2ClkcfgSpec {}
