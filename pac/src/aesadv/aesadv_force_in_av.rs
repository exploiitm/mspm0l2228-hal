#[doc = "Register `AESADV_FORCE_IN_AV` writer"]
pub type W = crate::W<AesadvForceInAvSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesadvForceInAvSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data control register for input data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_force_in_av::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadvForceInAvSpec;
impl crate::RegisterSpec for AesadvForceInAvSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadv_force_in_av::W`](W) writer structure"]
impl crate::Writable for AesadvForceInAvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADV_FORCE_IN_AV to value 0"]
impl crate::Resettable for AesadvForceInAvSpec {}
