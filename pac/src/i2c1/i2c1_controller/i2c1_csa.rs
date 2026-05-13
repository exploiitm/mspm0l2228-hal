#[doc = "Register `I2C1_CSA` reader"]
pub type R = crate::R<I2c1CsaSpec>;
#[doc = "Register `I2C1_CSA` writer"]
pub type W = crate::W<I2c1CsaSpec>;
#[doc = "Receive/Send The DIR bit specifies if the next Controller operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: The Controller is in transmit mode."]
    Transmit = 0,
    #[doc = "1: The Controller is in receive mode."]
    Receive = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Receive/Send The DIR bit specifies if the next Controller operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Transmit,
            true => Dir::Receive,
        }
    }
    #[doc = "The Controller is in transmit mode."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == Dir::Transmit
    }
    #[doc = "The Controller is in receive mode."]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Dir::Receive
    }
}
#[doc = "Field `DIR` writer - Receive/Send The DIR bit specifies if the next Controller operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Controller is in transmit mode."]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Transmit)
    }
    #[doc = "The Controller is in receive mode."]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Receive)
    }
}
#[doc = "Field `TADDR` reader - I2C Target Address This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
pub type TaddrR = crate::FieldReader<u16>;
#[doc = "Field `TADDR` writer - I2C Target Address This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
pub type TaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit selects the adressing mode to be used in Controller mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmode {
    #[doc = "0: 7-bit addressing mode"]
    Mode7 = 0,
    #[doc = "1: 10-bit addressing mode"]
    Mode10 = 1,
}
impl From<Cmode> for bool {
    #[inline(always)]
    fn from(variant: Cmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMODE` reader - This bit selects the adressing mode to be used in Controller mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type CmodeR = crate::BitReader<Cmode>;
impl CmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmode {
        match self.bits {
            false => Cmode::Mode7,
            true => Cmode::Mode10,
        }
    }
    #[doc = "7-bit addressing mode"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Cmode::Mode7
    }
    #[doc = "10-bit addressing mode"]
    #[inline(always)]
    pub fn is_mode10(&self) -> bool {
        *self == Cmode::Mode10
    }
}
#[doc = "Field `CMODE` writer - This bit selects the adressing mode to be used in Controller mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmode>;
impl<'a, REG> CmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit addressing mode"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Cmode::Mode7)
    }
    #[doc = "10-bit addressing mode"]
    #[inline(always)]
    pub fn mode10(self) -> &'a mut crate::W<REG> {
        self.variant(Cmode::Mode10)
    }
}
impl R {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next Controller operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - I2C Target Address This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn taddr(&self) -> TaddrR {
        TaddrR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in Controller mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next Controller operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, I2c1CsaSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bits 1:10 - I2C Target Address This field specifies bits A9 through A0 of the Target address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn taddr(&mut self) -> TaddrW<'_, I2c1CsaSpec> {
        TaddrW::new(self, 1)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in Controller mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn cmode(&mut self) -> CmodeW<'_, I2c1CsaSpec> {
        CmodeW::new(self, 15)
    }
}
#[doc = "I2C Controller Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1CsaSpec;
impl crate::RegisterSpec for I2c1CsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_csa::R`](R) reader structure"]
impl crate::Readable for I2c1CsaSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_csa::W`](W) writer structure"]
impl crate::Writable for I2c1CsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1_CSA to value 0"]
impl crate::Resettable for I2c1CsaSpec {}
