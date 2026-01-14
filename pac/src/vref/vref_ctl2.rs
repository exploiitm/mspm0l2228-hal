#[doc = "Register `VREF_CTL2` reader"]
pub type R = crate::R<VrefCtl2Spec>;
#[doc = "Register `VREF_CTL2` writer"]
pub type W = crate::W<VrefCtl2Spec>;
#[doc = "Field `SHCYCLE` reader - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type ShcycleR = crate::FieldReader<u16>;
#[doc = "Field `SHCYCLE` writer - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type ShcycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HCYCLE` reader - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type HcycleR = crate::FieldReader<u16>;
#[doc = "Field `HCYCLE` writer - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type HcycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn shcycle(&self) -> ShcycleR {
        ShcycleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn hcycle(&self) -> HcycleR {
        HcycleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn shcycle(&mut self) -> ShcycleW<'_, VrefCtl2Spec> {
        ShcycleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn hcycle(&mut self) -> HcycleW<'_, VrefCtl2Spec> {
        HcycleW::new(self, 16)
    }
}
#[doc = "Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefCtl2Spec;
impl crate::RegisterSpec for VrefCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_ctl2::R`](R) reader structure"]
impl crate::Readable for VrefCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_ctl2::W`](W) writer structure"]
impl crate::Writable for VrefCtl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREF_CTL2 to value 0"]
impl crate::Resettable for VrefCtl2Spec {}
