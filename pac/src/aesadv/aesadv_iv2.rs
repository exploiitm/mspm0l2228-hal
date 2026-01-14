#[doc = "Register `AESADV_IV2` reader"]
pub type R = crate::R<AesadvIv2Spec>;
#[doc = "Register `AESADV_IV2` writer"]
pub type W = crate::W<AesadvIv2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IV\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_iv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_iv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvIv2Spec;
impl crate::RegisterSpec for AesadvIv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_iv2::R`](R) reader structure"]
impl crate::Readable for AesadvIv2Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_iv2::W`](W) writer structure"]
impl crate::Writable for AesadvIv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_IV2 to value 0"]
impl crate::Resettable for AesadvIv2Spec {}
