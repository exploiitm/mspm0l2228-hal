#[doc = "Register `AESADV_DATA2` reader"]
pub type R = crate::R<AesadvData2Spec>;
#[doc = "Register `AESADV_DATA2` writer"]
pub type W = crate::W<AesadvData2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvData2Spec;
impl crate::RegisterSpec for AesadvData2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_data2::R`](R) reader structure"]
impl crate::Readable for AesadvData2Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_data2::W`](W) writer structure"]
impl crate::Writable for AesadvData2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_DATA2 to value 0"]
impl crate::Resettable for AesadvData2Spec {}
