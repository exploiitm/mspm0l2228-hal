#[doc = "Register `FLASHCTL_MIS` reader"]
pub type R = crate::R<FlashctlMisSpec>;
#[doc = "Flash wrapper operation completed. This masked interrupt bit reflects the bitwise AND of the corresponding RIS and IMASK bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Masked interrupt did not occur"]
    Clr = 0,
    #[doc = "1: Masked interrupt occurred"]
    Set = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Flash wrapper operation completed. This masked interrupt bit reflects the bitwise AND of the corresponding RIS and IMASK bits."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Clr,
            true => Done::Set,
        }
    }
    #[doc = "Masked interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Done::Clr
    }
    #[doc = "Masked interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Done::Set
    }
}
impl R {
    #[doc = "Bit 0 - Flash wrapper operation completed. This masked interrupt bit reflects the bitwise AND of the corresponding RIS and IMASK bits."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlMisSpec;
impl crate::RegisterSpec for FlashctlMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_mis::R`](R) reader structure"]
impl crate::Readable for FlashctlMisSpec {}
#[doc = "`reset()` method sets FLASHCTL_MIS to value 0"]
impl crate::Resettable for FlashctlMisSpec {}
