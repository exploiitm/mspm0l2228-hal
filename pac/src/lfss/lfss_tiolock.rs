#[doc = "Register `LFSS_TIOLOCK` reader"]
pub type R = crate::R<LfssTiolockSpec>;
#[doc = "Register `LFSS_TIOLOCK` writer"]
pub type W = crate::W<LfssTiolockSpec>;
#[doc = "If set, the register bit will protect the TIOCTL and HEARTBEAT from accidental writes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: Tamper I/O control is writable."]
    Clr = 0,
    #[doc = "1: Tamper I/O control is read only access."]
    Set = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - If set, the register bit will protect the TIOCTL and HEARTBEAT from accidental writes."]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::Clr,
            true => Protect::Set,
        }
    }
    #[doc = "Tamper I/O control is writable."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Protect::Clr
    }
    #[doc = "Tamper I/O control is read only access."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Protect::Set
    }
}
#[doc = "Field `PROTECT` writer - If set, the register bit will protect the TIOCTL and HEARTBEAT from accidental writes."]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper I/O control is writable."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Clr)
    }
    #[doc = "Tamper I/O control is read only access."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Set)
    }
}
impl R {
    #[doc = "Bit 0 - If set, the register bit will protect the TIOCTL and HEARTBEAT from accidental writes."]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, the register bit will protect the TIOCTL and HEARTBEAT from accidental writes."]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<'_, LfssTiolockSpec> {
        ProtectW::new(self, 0)
    }
}
#[doc = "Tamper I/O lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tiolock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tiolock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTiolockSpec;
impl crate::RegisterSpec for LfssTiolockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tiolock::R`](R) reader structure"]
impl crate::Readable for LfssTiolockSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tiolock::W`](W) writer structure"]
impl crate::Writable for LfssTiolockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TIOLOCK to value 0"]
impl crate::Resettable for LfssTiolockSpec {}
