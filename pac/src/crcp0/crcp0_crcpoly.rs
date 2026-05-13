#[doc = "Register `CRCP0_CRCPOLY` reader"]
pub type R = crate::R<Crcp0CrcpolySpec>;
#[doc = "Register `CRCP0_CRCPOLY` writer"]
pub type W = crate::W<Crcp0CrcpolySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC Polynomial configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcp0CrcpolySpec;
impl crate::RegisterSpec for Crcp0CrcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcp0_crcpoly::R`](R) reader structure"]
impl crate::Readable for Crcp0CrcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crcp0_crcpoly::W`](W) writer structure"]
impl crate::Writable for Crcp0CrcpolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCP0_CRCPOLY to value 0"]
impl crate::Resettable for Crcp0CrcpolySpec {}
