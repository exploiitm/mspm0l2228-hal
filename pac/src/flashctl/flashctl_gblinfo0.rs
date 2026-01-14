#[doc = "Register `FLASHCTL_GBLINFO0` reader"]
pub type R = crate::R<FlashctlGblinfo0Spec>;
#[doc = "Sector size in bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sectorsize {
    #[doc = "1024: Sector size is ONEKB"]
    Onekb = 1024,
    #[doc = "2048: Sector size is TWOKB"]
    Twokb = 2048,
}
impl From<Sectorsize> for u16 {
    #[inline(always)]
    fn from(variant: Sectorsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sectorsize {
    type Ux = u16;
}
impl crate::IsEnum for Sectorsize {}
#[doc = "Field `SECTORSIZE` reader - Sector size in bytes"]
pub type SectorsizeR = crate::FieldReader<Sectorsize>;
impl SectorsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sectorsize> {
        match self.bits {
            1024 => Some(Sectorsize::Onekb),
            2048 => Some(Sectorsize::Twokb),
            _ => None,
        }
    }
    #[doc = "Sector size is ONEKB"]
    #[inline(always)]
    pub fn is_onekb(&self) -> bool {
        *self == Sectorsize::Onekb
    }
    #[doc = "Sector size is TWOKB"]
    #[inline(always)]
    pub fn is_twokb(&self) -> bool {
        *self == Sectorsize::Twokb
    }
}
#[doc = "Field `NUMBANKS` reader - Number of banks instantiated Minimum: 1 Maximum: 5"]
pub type NumbanksR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Sector size in bytes"]
    #[inline(always)]
    pub fn sectorsize(&self) -> SectorsizeR {
        SectorsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Number of banks instantiated Minimum: 1 Maximum: 5"]
    #[inline(always)]
    pub fn numbanks(&self) -> NumbanksR {
        NumbanksR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Global Information Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlGblinfo0Spec;
impl crate::RegisterSpec for FlashctlGblinfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_gblinfo0::R`](R) reader structure"]
impl crate::Readable for FlashctlGblinfo0Spec {}
#[doc = "`reset()` method sets FLASHCTL_GBLINFO0 to value 0"]
impl crate::Resettable for FlashctlGblinfo0Spec {}
