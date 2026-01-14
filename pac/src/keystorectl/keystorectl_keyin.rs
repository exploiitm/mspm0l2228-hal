#[doc = "Register `KEYSTORECTL_KEYIN` writer"]
pub type W = crate::W<KeystorectlKeyinSpec>;
impl core::fmt::Debug for crate::generic::Reg<KeystorectlKeyinSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Input key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_keyin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeystorectlKeyinSpec;
impl crate::RegisterSpec for KeystorectlKeyinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keystorectl_keyin::W`](W) writer structure"]
impl crate::Writable for KeystorectlKeyinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSTORECTL_KEYIN to value 0"]
impl crate::Resettable for KeystorectlKeyinSpec {}
