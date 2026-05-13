#[doc = "Register `I2C0_TOAR` reader"]
pub type R = crate::R<I2c0ToarSpec>;
#[doc = "Register `I2C0_TOAR` writer"]
pub type W = crate::W<I2c0ToarSpec>;
#[doc = "Field `OAR` reader - I2C Target Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type OarR = crate::FieldReader<u16>;
#[doc = "Field `OAR` writer - I2C Target Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type OarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "I2C Target Own Address Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oaren {
    #[doc = "0: Disable OAR address"]
    Disable = 0,
    #[doc = "1: Enable OAR address"]
    Enable = 1,
}
impl From<Oaren> for bool {
    #[inline(always)]
    fn from(variant: Oaren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAREN` reader - I2C Target Own Address Enable"]
pub type OarenR = crate::BitReader<Oaren>;
impl OarenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oaren {
        match self.bits {
            false => Oaren::Disable,
            true => Oaren::Enable,
        }
    }
    #[doc = "Disable OAR address"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Oaren::Disable
    }
    #[doc = "Enable OAR address"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Oaren::Enable
    }
}
#[doc = "Field `OAREN` writer - I2C Target Own Address Enable"]
pub type OarenW<'a, REG> = crate::BitWriter<'a, REG, Oaren>;
impl<'a, REG> OarenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable OAR address"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Oaren::Disable)
    }
    #[doc = "Enable OAR address"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Oaren::Enable)
    }
}
#[doc = "This bit selects the adressing mode to be used in Target mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmode {
    #[doc = "0: Enable 7-bit addressing"]
    Mode7 = 0,
    #[doc = "1: Enable 10-bit addressing"]
    Mode10 = 1,
}
impl From<Tmode> for bool {
    #[inline(always)]
    fn from(variant: Tmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMODE` reader - This bit selects the adressing mode to be used in Target mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type TmodeR = crate::BitReader<Tmode>;
impl TmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmode {
        match self.bits {
            false => Tmode::Mode7,
            true => Tmode::Mode10,
        }
    }
    #[doc = "Enable 7-bit addressing"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Tmode::Mode7
    }
    #[doc = "Enable 10-bit addressing"]
    #[inline(always)]
    pub fn is_mode10(&self) -> bool {
        *self == Tmode::Mode10
    }
}
#[doc = "Field `TMODE` writer - This bit selects the adressing mode to be used in Target mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type TmodeW<'a, REG> = crate::BitWriter<'a, REG, Tmode>;
impl<'a, REG> TmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable 7-bit addressing"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Mode7)
    }
    #[doc = "Enable 10-bit addressing"]
    #[inline(always)]
    pub fn mode10(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Mode10)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C Target Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn oar(&self) -> OarR {
        OarR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - I2C Target Own Address Enable"]
    #[inline(always)]
    pub fn oaren(&self) -> OarenR {
        OarenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in Target mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn tmode(&self) -> TmodeR {
        TmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Target Own Address: This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn oar(&mut self) -> OarW<'_, I2c0ToarSpec> {
        OarW::new(self, 0)
    }
    #[doc = "Bit 14 - I2C Target Own Address Enable"]
    #[inline(always)]
    pub fn oaren(&mut self) -> OarenW<'_, I2c0ToarSpec> {
        OarenW::new(self, 14)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in Target mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn tmode(&mut self) -> TmodeW<'_, I2c0ToarSpec> {
        TmodeW::new(self, 15)
    }
}
#[doc = "I2C Target Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_toar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_toar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0ToarSpec;
impl crate::RegisterSpec for I2c0ToarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_toar::R`](R) reader structure"]
impl crate::Readable for I2c0ToarSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_toar::W`](W) writer structure"]
impl crate::Writable for I2c0ToarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_TOAR to value 0x4000"]
impl crate::Resettable for I2c0ToarSpec {
    const RESET_VALUE: u32 = 0x4000;
}
