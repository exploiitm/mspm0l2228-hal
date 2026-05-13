#[doc = "Register `I2C1_CCR` reader"]
pub type R = crate::R<I2c1CcrSpec>;
#[doc = "Register `I2C1_CCR` writer"]
pub type W = crate::W<I2c1CcrSpec>;
#[doc = "Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Disables the I2C Controller operation."]
    Disable = 0,
    #[doc = "1: Enables the I2C Controller operation."]
    Enable = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::Disable,
            true => Active::Enable,
        }
    }
    #[doc = "Disables the I2C Controller operation."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Active::Disable
    }
    #[doc = "Enables the I2C Controller operation."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Active::Enable
    }
}
#[doc = "Field `ACTIVE` writer - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the I2C Controller operation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Disable)
    }
    #[doc = "Enables the I2C Controller operation."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Enable)
    }
}
#[doc = "MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mctl {
    #[doc = "0: Disable MultiController mode."]
    Disable = 0,
    #[doc = "1: Enable MultiController mode."]
    Enable = 1,
}
impl From<Mctl> for bool {
    #[inline(always)]
    fn from(variant: Mctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTL` reader - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type MctlR = crate::BitReader<Mctl>;
impl MctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mctl {
        match self.bits {
            false => Mctl::Disable,
            true => Mctl::Enable,
        }
    }
    #[doc = "Disable MultiController mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mctl::Disable
    }
    #[doc = "Enable MultiController mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mctl::Enable
    }
}
#[doc = "Field `MCTL` writer - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type MctlW<'a, REG> = crate::BitWriter<'a, REG, Mctl>;
impl<'a, REG> MctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable MultiController mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mctl::Disable)
    }
    #[doc = "Enable MultiController mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mctl::Enable)
    }
}
#[doc = "Clock Stretching. This bit controls the support for clock stretching of the I2C bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkstretch {
    #[doc = "0: Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    Disable = 0,
    #[doc = "1: Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    Enable = 1,
}
impl From<Clkstretch> for bool {
    #[inline(always)]
    fn from(variant: Clkstretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTRETCH` reader - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
pub type ClkstretchR = crate::BitReader<Clkstretch>;
impl ClkstretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkstretch {
        match self.bits {
            false => Clkstretch::Disable,
            true => Clkstretch::Enable,
        }
    }
    #[doc = "Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Clkstretch::Disable
    }
    #[doc = "Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Clkstretch::Enable
    }
}
#[doc = "Field `CLKSTRETCH` writer - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
pub type ClkstretchW<'a, REG> = crate::BitWriter<'a, REG, Clkstretch>;
impl<'a, REG> ClkstretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the clock stretching detection. This can be disabled if no Target on the bus does support clock stretching, so that the maximum speed on the bus can be reached."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Disable)
    }
    #[doc = "Enables the clock stretching detection. Enabling the clock stretching ensures compliance to the I2C standard but could limit the speed due the clock stretching."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Clkstretch::Enable)
    }
}
#[doc = "I2C Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpbk {
    #[doc = "0: Normal operation."]
    Disable = 0,
    #[doc = "1: The controller in a test mode loopback configuration."]
    Enable = 1,
}
impl From<Lpbk> for bool {
    #[inline(always)]
    fn from(variant: Lpbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBK` reader - I2C Loopback"]
pub type LpbkR = crate::BitReader<Lpbk>;
impl LpbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpbk {
        match self.bits {
            false => Lpbk::Disable,
            true => Lpbk::Enable,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lpbk::Disable
    }
    #[doc = "The controller in a test mode loopback configuration."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpbk::Enable
    }
}
#[doc = "Field `LPBK` writer - I2C Loopback"]
pub type LpbkW<'a, REG> = crate::BitWriter<'a, REG, Lpbk>;
impl<'a, REG> LpbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbk::Disable)
    }
    #[doc = "The controller in a test mode loopback configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbk::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mctl(&self) -> MctlR {
        MctlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
    #[inline(always)]
    pub fn clkstretch(&self) -> ClkstretchR {
        ClkstretchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Loopback"]
    #[inline(always)]
    pub fn lpbk(&self) -> LpbkR {
        LpbkR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, I2c1CcrSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - MultiController mode. In MultiController mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mctl(&mut self) -> MctlW<'_, I2c1CcrSpec> {
        MctlW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
    #[inline(always)]
    pub fn clkstretch(&mut self) -> ClkstretchW<'_, I2c1CcrSpec> {
        ClkstretchW::new(self, 2)
    }
    #[doc = "Bit 8 - I2C Loopback"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LpbkW<'_, I2c1CcrSpec> {
        LpbkW::new(self, 8)
    }
}
#[doc = "I2C Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1CcrSpec;
impl crate::RegisterSpec for I2c1CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_ccr::R`](R) reader structure"]
impl crate::Readable for I2c1CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_ccr::W`](W) writer structure"]
impl crate::Writable for I2c1CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1_CCR to value 0"]
impl crate::Resettable for I2c1CcrSpec {}
