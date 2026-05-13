#[doc = "Register `AESADV_BLK_CNT0` reader"]
pub type R = crate::R<AesadvBlkCnt0Spec>;
#[doc = "Register `AESADV_BLK_CNT0` writer"]
pub type W = crate::W<AesadvBlkCnt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal block counter (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_blk_cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_blk_cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvBlkCnt0Spec;
impl crate::RegisterSpec for AesadvBlkCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_blk_cnt0::R`](R) reader structure"]
impl crate::Readable for AesadvBlkCnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_blk_cnt0::W`](W) writer structure"]
impl crate::Writable for AesadvBlkCnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_BLK_CNT0 to value 0"]
impl crate::Resettable for AesadvBlkCnt0Spec {}
