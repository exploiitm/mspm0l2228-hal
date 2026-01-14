#[doc = "Register `TRNG_DESC` reader"]
pub type R = crate::R<TrngDescSpec>;
#[doc = "Field `MINREV` reader - Minor rev of the IP"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major rev of the IP"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `INSTNUM` reader - Instance Number within the device. This will be a parameter to the RTL for modules that can have multiple instances"]
pub type InstnumR = crate::FieldReader;
#[doc = "Field `FEATUREVER` reader - Feature Set for the module *instance*"]
pub type FeatureverR = crate::FieldReader;
#[doc = "Field `MODULEID` reader - Module Identifier - An internal TI page has been created to request unique module IDs"]
pub type ModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor rev of the IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major rev of the IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Instance Number within the device. This will be a parameter to the RTL for modules that can have multiple instances"]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature Set for the module *instance*"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module Identifier - An internal TI page has been created to request unique module IDs"]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module descriptions\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngDescSpec;
impl crate::RegisterSpec for TrngDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_desc::R`](R) reader structure"]
impl crate::Readable for TrngDescSpec {}
#[doc = "`reset()` method sets TRNG_DESC to value 0x0511_0000"]
impl crate::Resettable for TrngDescSpec {
    const RESET_VALUE: u32 = 0x0511_0000;
}
