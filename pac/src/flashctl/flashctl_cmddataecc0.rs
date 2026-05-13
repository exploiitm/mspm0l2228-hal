#[doc = "Register `FLASHCTL_CMDDATAECC0` reader"]
pub type R = crate::R<FlashctlCmddataecc0Spec>;
#[doc = "Register `FLASHCTL_CMDDATAECC0` writer"]
pub type W = crate::W<FlashctlCmddataecc0Spec>;
#[doc = "Field `VAL0` reader - ECC data for bits 63:0 of the data is placed here."]
pub type Val0R = crate::FieldReader;
#[doc = "Field `VAL0` writer - ECC data for bits 63:0 of the data is placed here."]
pub type Val0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VAL1` reader - ECC data for bits 127:64 of the data is placed here."]
pub type Val1R = crate::FieldReader;
#[doc = "Field `VAL1` writer - ECC data for bits 127:64 of the data is placed here."]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ECC data for bits 63:0 of the data is placed here."]
    #[inline(always)]
    pub fn val0(&self) -> Val0R {
        Val0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECC data for bits 127:64 of the data is placed here."]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ECC data for bits 63:0 of the data is placed here."]
    #[inline(always)]
    pub fn val0(&mut self) -> Val0W<'_, FlashctlCmddataecc0Spec> {
        Val0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECC data for bits 127:64 of the data is placed here."]
    #[inline(always)]
    pub fn val1(&mut self) -> Val1W<'_, FlashctlCmddataecc0Spec> {
        Val1W::new(self, 8)
    }
}
#[doc = "Command Data Register ECC 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmddataecc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmddataecc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmddataecc0Spec;
impl crate::RegisterSpec for FlashctlCmddataecc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmddataecc0::R`](R) reader structure"]
impl crate::Readable for FlashctlCmddataecc0Spec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmddataecc0::W`](W) writer structure"]
impl crate::Writable for FlashctlCmddataecc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDDATAECC0 to value 0"]
impl crate::Resettable for FlashctlCmddataecc0Spec {}
