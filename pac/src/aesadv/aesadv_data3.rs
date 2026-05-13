#[doc = "Register `AESADV_DATA3` reader"]
pub type R = crate::R<AesadvData3Spec>;
#[doc = "Register `AESADV_DATA3` writer"]
pub type W = crate::W<AesadvData3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data input (LSW) / Data output (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvData3Spec;
impl crate::RegisterSpec for AesadvData3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_data3::R`](R) reader structure"]
impl crate::Readable for AesadvData3Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_data3::W`](W) writer structure"]
impl crate::Writable for AesadvData3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_DATA3 to value 0"]
impl crate::Resettable for AesadvData3Spec {}
