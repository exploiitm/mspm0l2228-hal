#[doc = "Register `FLASHCTL_DESC` reader"]
pub type R = crate::R<FlashctlDescSpec>;
#[doc = "Field `MINREV` reader - Minor Revision"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major Revision"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `INSTNUM` reader - Instance number"]
pub type InstnumR = crate::FieldReader;
#[doc = "Field `FEATUREVER` reader - Feature set"]
pub type FeatureverR = crate::FieldReader;
#[doc = "Field `MODULEID` reader - Module ID"]
pub type ModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor Revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major Revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Instance number"]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature set"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module ID"]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Hardware Version Description Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlDescSpec;
impl crate::RegisterSpec for FlashctlDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_desc::R`](R) reader structure"]
impl crate::Readable for FlashctlDescSpec {}
#[doc = "`reset()` method sets FLASHCTL_DESC to value 0"]
impl crate::Resettable for FlashctlDescSpec {}
