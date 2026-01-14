#[doc = "Register `COMP0_CTL3` reader"]
pub type R = crate::R<Comp0Ctl3Spec>;
#[doc = "Register `COMP0_CTL3` writer"]
pub type W = crate::W<Comp0Ctl3Spec>;
#[doc = "Field `DACCODE0` reader - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Daccode0R = crate::FieldReader;
#[doc = "Field `DACCODE0` writer - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Daccode0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACCODE1` reader - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Daccode1R = crate::FieldReader;
#[doc = "Field `DACCODE1` writer - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
pub type Daccode1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn daccode0(&self) -> Daccode0R {
        Daccode0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn daccode1(&self) -> Daccode1R {
        Daccode1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is the first 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn daccode0(&mut self) -> Daccode0W<'_, Comp0Ctl3Spec> {
        Daccode0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - This is the second 8-bit DAC code. When the DAC code is 0x0 the DAC output will be 0 V. When the DAC code is 0xFF the DAC output will be selected reference voltage x 255/256."]
    #[inline(always)]
    pub fn daccode1(&mut self) -> Daccode1W<'_, Comp0Ctl3Spec> {
        Daccode1W::new(self, 16)
    }
}
#[doc = "Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Ctl3Spec;
impl crate::RegisterSpec for Comp0Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_ctl3::R`](R) reader structure"]
impl crate::Readable for Comp0Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0_ctl3::W`](W) writer structure"]
impl crate::Writable for Comp0Ctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CTL3 to value 0"]
impl crate::Resettable for Comp0Ctl3Spec {}
