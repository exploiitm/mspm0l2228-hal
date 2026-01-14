#[doc = "Register `FLASHCTL_BANK4INFO0` reader"]
pub type R = crate::R<FlashctlBank4info0Spec>;
#[doc = "Main region size in sectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Mainsize {
    #[doc = "8: Minimum value of \\[MAINSIZE\\]"]
    Minsectors = 8,
    #[doc = "512: Maximum value of \\[MAINSIZE\\]"]
    Maxsectors = 512,
}
impl From<Mainsize> for u16 {
    #[inline(always)]
    fn from(variant: Mainsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mainsize {
    type Ux = u16;
}
impl crate::IsEnum for Mainsize {}
#[doc = "Field `MAINSIZE` reader - Main region size in sectors"]
pub type MainsizeR = crate::FieldReader<Mainsize>;
impl MainsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mainsize> {
        match self.bits {
            8 => Some(Mainsize::Minsectors),
            512 => Some(Mainsize::Maxsectors),
            _ => None,
        }
    }
    #[doc = "Minimum value of \\[MAINSIZE\\]"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Mainsize::Minsectors
    }
    #[doc = "Maximum value of \\[MAINSIZE\\]"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Mainsize::Maxsectors
    }
}
impl R {
    #[doc = "Bits 0:11 - Main region size in sectors"]
    #[inline(always)]
    pub fn mainsize(&self) -> MainsizeR {
        MainsizeR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Bank Information Register 0 for Bank 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_bank4info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlBank4info0Spec;
impl crate::RegisterSpec for FlashctlBank4info0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_bank4info0::R`](R) reader structure"]
impl crate::Readable for FlashctlBank4info0Spec {}
#[doc = "`reset()` method sets FLASHCTL_BANK4INFO0 to value 0"]
impl crate::Resettable for FlashctlBank4info0Spec {}
