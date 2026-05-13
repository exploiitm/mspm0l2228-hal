#[doc = "Register `AESADV_DATA1` reader"]
pub type R = crate::R<AesadvData1Spec>;
#[doc = "Register `AESADV_DATA1` writer"]
pub type W = crate::W<AesadvData1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvData1Spec;
impl crate::RegisterSpec for AesadvData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_data1::R`](R) reader structure"]
impl crate::Readable for AesadvData1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_data1::W`](W) writer structure"]
impl crate::Writable for AesadvData1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_DATA1 to value 0"]
impl crate::Resettable for AesadvData1Spec {}
