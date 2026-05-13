#[doc = "Register `SRAMFLASH` reader"]
pub type R = crate::R<SramflashSpec>;
#[doc = "Field `MAINFLASH_SZ` reader - The encoding of the field is that value of the field is an interger to be interpreted as number of KB. For example, if the value of the field id 4, then it is 4KB, the value is 32KB, and so on."]
pub type MainflashSzR = crate::FieldReader<u16>;
#[doc = "Field `MAINNUMBANKS` reader - "]
pub type MainnumbanksR = crate::FieldReader;
#[doc = "Field `SRAM_SZ` reader - The encoding of the field is that the value of the field is an integer to be interpreted as number of KB. For example id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
pub type SramSzR = crate::FieldReader<u16>;
#[doc = "Field `DATAFLASH_SZ` reader - The encoding of the field is that the value of the field is an integer to be interpreted as number of KB. For example id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
pub type DataflashSzR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - The encoding of the field is that value of the field is an interger to be interpreted as number of KB. For example, if the value of the field id 4, then it is 4KB, the value is 32KB, and so on."]
    #[inline(always)]
    pub fn mainflash_sz(&self) -> MainflashSzR {
        MainflashSzR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mainnumbanks(&self) -> MainnumbanksR {
        MainnumbanksR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:25 - The encoding of the field is that the value of the field is an integer to be interpreted as number of KB. For example id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
    #[inline(always)]
    pub fn sram_sz(&self) -> SramSzR {
        SramSzR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - The encoding of the field is that the value of the field is an integer to be interpreted as number of KB. For example id 4, then it is 4KB, if the value is 32, then 32KB, and so on."]
    #[inline(always)]
    pub fn dataflash_sz(&self) -> DataflashSzR {
        DataflashSzR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[doc = "SRAM flash\n\nYou can [`read`](crate::Reg::read) this register and get [`sramflash::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramflashSpec;
impl crate::RegisterSpec for SramflashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramflash::R`](R) reader structure"]
impl crate::Readable for SramflashSpec {}
#[doc = "`reset()` method sets SRAMFLASH to value 0"]
impl crate::Resettable for SramflashSpec {}
