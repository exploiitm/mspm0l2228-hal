#[doc = "Register `DEBUGSS_RXD` reader"]
pub type R = crate::R<DebugssRxdSpec>;
#[doc = "Register `DEBUGSS_RXD` writer"]
pub type W = crate::W<DebugssRxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_rxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_rxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssRxdSpec;
impl crate::RegisterSpec for DebugssRxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_rxd::R`](R) reader structure"]
impl crate::Readable for DebugssRxdSpec {}
#[doc = "`write(|w| ..)` method takes [`debugss_rxd::W`](W) writer structure"]
impl crate::Writable for DebugssRxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGSS_RXD to value 0"]
impl crate::Resettable for DebugssRxdSpec {}
