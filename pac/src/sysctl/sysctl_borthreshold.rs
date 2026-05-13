#[doc = "Register `SYSCTL_BORTHRESHOLD` reader"]
pub type R = crate::R<SysctlBorthresholdSpec>;
#[doc = "Register `SYSCTL_BORTHRESHOLD` writer"]
pub type W = crate::W<SysctlBorthresholdSpec>;
#[doc = "LEVEL specifies the desired BOR threshold and BOR mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    #[doc = "0: Default minimum threshold; a BOR0- violation triggers a BOR"]
    Bormin = 0,
    #[doc = "1: A BOR1- violation generates a BORLVL interrupt"]
    Borlevel1 = 1,
    #[doc = "2: A BOR2- violation generates a BORLVL interrupt"]
    Borlevel2 = 2,
    #[doc = "3: A BOR3- violation generates a BORLVL interrupt"]
    Borlevel3 = 3,
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Level {
    type Ux = u8;
}
impl crate::IsEnum for Level {}
#[doc = "Field `LEVEL` reader - LEVEL specifies the desired BOR threshold and BOR mode."]
pub type LevelR = crate::FieldReader<Level>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Level {
        match self.bits {
            0 => Level::Bormin,
            1 => Level::Borlevel1,
            2 => Level::Borlevel2,
            3 => Level::Borlevel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Default minimum threshold; a BOR0- violation triggers a BOR"]
    #[inline(always)]
    pub fn is_bormin(&self) -> bool {
        *self == Level::Bormin
    }
    #[doc = "A BOR1- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel1(&self) -> bool {
        *self == Level::Borlevel1
    }
    #[doc = "A BOR2- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel2(&self) -> bool {
        *self == Level::Borlevel2
    }
    #[doc = "A BOR3- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn is_borlevel3(&self) -> bool {
        *self == Level::Borlevel3
    }
}
#[doc = "Field `LEVEL` writer - LEVEL specifies the desired BOR threshold and BOR mode."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Level, crate::Safe>;
impl<'a, REG> LevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default minimum threshold; a BOR0- violation triggers a BOR"]
    #[inline(always)]
    pub fn bormin(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Bormin)
    }
    #[doc = "A BOR1- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn borlevel1(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Borlevel1)
    }
    #[doc = "A BOR2- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn borlevel2(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Borlevel2)
    }
    #[doc = "A BOR3- violation generates a BORLVL interrupt"]
    #[inline(always)]
    pub fn borlevel3(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Borlevel3)
    }
}
impl R {
    #[doc = "Bits 0:1 - LEVEL specifies the desired BOR threshold and BOR mode."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEVEL specifies the desired BOR threshold and BOR mode."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, SysctlBorthresholdSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "BOR threshold selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_borthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_borthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlBorthresholdSpec;
impl crate::RegisterSpec for SysctlBorthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_borthreshold::R`](R) reader structure"]
impl crate::Readable for SysctlBorthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_borthreshold::W`](W) writer structure"]
impl crate::Writable for SysctlBorthresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_BORTHRESHOLD to value 0"]
impl crate::Resettable for SysctlBorthresholdSpec {}
