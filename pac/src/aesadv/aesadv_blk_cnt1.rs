#[doc = "Register `AESADV_BLK_CNT1` reader"]
pub type R = crate::R<AesadvBlkCnt1Spec>;
#[doc = "Register `AESADV_BLK_CNT1` writer"]
pub type W = crate::W<AesadvBlkCnt1Spec>;
#[doc = "Field `DATA` reader - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, AesadvBlkCnt1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Internal block counter (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_blk_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_blk_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvBlkCnt1Spec;
impl crate::RegisterSpec for AesadvBlkCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_blk_cnt1::R`](R) reader structure"]
impl crate::Readable for AesadvBlkCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_blk_cnt1::W`](W) writer structure"]
impl crate::Writable for AesadvBlkCnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_BLK_CNT1 to value 0"]
impl crate::Resettable for AesadvBlkCnt1Spec {}
