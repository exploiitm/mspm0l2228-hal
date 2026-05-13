#[doc = "Register `FLASHCTL_STATPCNT` reader"]
pub type R = crate::R<FlashctlStatpcntSpec>;
#[doc = "Field `PULSECNT` reader - Current Pulse Counter Value"]
pub type PulsecntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Current Pulse Counter Value"]
    #[inline(always)]
    pub fn pulsecnt(&self) -> PulsecntR {
        PulsecntR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Pulse Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_statpcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlStatpcntSpec;
impl crate::RegisterSpec for FlashctlStatpcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_statpcnt::R`](R) reader structure"]
impl crate::Readable for FlashctlStatpcntSpec {}
#[doc = "`reset()` method sets FLASHCTL_STATPCNT to value 0"]
impl crate::Resettable for FlashctlStatpcntSpec {}
