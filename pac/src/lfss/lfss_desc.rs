#[doc = "Register `LFSS_DESC` reader"]
pub type R = crate::R<LfssDescSpec>;
#[doc = "Field `MINREV` reader - Minor revision. This number holds the module revision and is incremented by the module developers. n = Minor module revision (see device-specific data sheet)"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major revision. This number holds the module revision and is incremented by the module developers. n = Major version (see device-specific data sheet)"]
pub type MajrevR = crate::FieldReader;
#[doc = "Instantiated version. Describes which instance of the module accessed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instnum {
    #[doc = "0: This is the default, if there is only one instance - like for SSIM"]
    Inst0 = 0,
}
impl From<Instnum> for u8 {
    #[inline(always)]
    fn from(variant: Instnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Instnum {
    type Ux = u8;
}
impl crate::IsEnum for Instnum {}
#[doc = "Field `INSTNUM` reader - Instantiated version. Describes which instance of the module accessed."]
pub type InstnumR = crate::FieldReader<Instnum>;
impl InstnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Instnum> {
        match self.bits {
            0 => Some(Instnum::Inst0),
            _ => None,
        }
    }
    #[doc = "This is the default, if there is only one instance - like for SSIM"]
    #[inline(always)]
    pub fn is_inst0(&self) -> bool {
        *self == Instnum::Inst0
    }
}
#[doc = "Field `FEATUREVER` reader - Feature set of this module. Differentiates the complexity of the actually instantiated module if there are differences."]
pub type FeatureverR = crate::FieldReader;
#[doc = "Field `MODULEID` reader - Module identifier. This ID is unique for each module. 0x2911 = Module ID of the LFSS Module"]
pub type ModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor revision. This number holds the module revision and is incremented by the module developers. n = Minor module revision (see device-specific data sheet)"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision. This number holds the module revision and is incremented by the module developers. n = Major version (see device-specific data sheet)"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Instantiated version. Describes which instance of the module accessed."]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature set of this module. Differentiates the complexity of the actually instantiated module if there are differences."]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier. This ID is unique for each module. 0x2911 = Module ID of the LFSS Module"]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "LFSS Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssDescSpec;
impl crate::RegisterSpec for LfssDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_desc::R`](R) reader structure"]
impl crate::Readable for LfssDescSpec {}
#[doc = "`reset()` method sets LFSS_DESC to value 0"]
impl crate::Resettable for LfssDescSpec {}
