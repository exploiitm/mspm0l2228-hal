#[doc = "Register `ADC0_SVT_FIFODATA` reader"]
pub type R = crate::R<Adc0SvtFifodataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_svt_fifodata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0SvtFifodataSpec;
impl crate::RegisterSpec for Adc0SvtFifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_svt_fifodata::R`](R) reader structure"]
impl crate::Readable for Adc0SvtFifodataSpec {}
#[doc = "`reset()` method sets ADC0_SVT_FIFODATA to value 0"]
impl crate::Resettable for Adc0SvtFifodataSpec {}
