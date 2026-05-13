#[doc = "Register `AESADV_CCM_ALN_WRD` reader"]
pub type R = crate::R<AesadvCcmAlnWrdSpec>;
#[doc = "Register `AESADV_CCM_ALN_WRD` writer"]
pub type W = crate::W<AesadvCcmAlnWrdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES-CCM AAD alignment data word\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_ccm_aln_wrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ccm_aln_wrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvCcmAlnWrdSpec;
impl crate::RegisterSpec for AesadvCcmAlnWrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadv_ccm_aln_wrd::R`](R) reader structure"]
impl crate::Readable for AesadvCcmAlnWrdSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadv_ccm_aln_wrd::W`](W) writer structure"]
impl crate::Writable for AesadvCcmAlnWrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_CCM_ALN_WRD to value 0"]
impl crate::Resettable for AesadvCcmAlnWrdSpec {}
