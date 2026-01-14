#[doc = "Register `ADC0_SVT_MEMRES[%s]` reader"]
pub type R = crate::R<Adc0SvtMemresSpec>;
#[doc = "Field `DATA` reader - MEMRES result register. If DF = 0, unsigned binary: The conversion results are right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion results are left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MEMRES result register. If DF = 0, unsigned binary: The conversion results are right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion results are left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Memory Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_svt_memres::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0SvtMemresSpec;
impl crate::RegisterSpec for Adc0SvtMemresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_svt_memres::R`](R) reader structure"]
impl crate::Readable for Adc0SvtMemresSpec {}
#[doc = "`reset()` method sets ADC0_SVT_MEMRES[%s] to value 0"]
impl crate::Resettable for Adc0SvtMemresSpec {}
