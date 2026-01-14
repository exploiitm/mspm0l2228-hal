#[doc = "Register `VREF_CTL1` reader"]
pub type R = crate::R<VrefCtl1Spec>;
#[doc = "Register `VREF_CTL1` writer"]
pub type W = crate::W<VrefCtl1Spec>;
#[doc = "These bits defines status of VREF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: VREF output is not ready"]
    Notrdy = 0,
    #[doc = "1: VREF output is ready"]
    Rdy = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - These bits defines status of VREF"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Notrdy,
            true => Ready::Rdy,
        }
    }
    #[doc = "VREF output is not ready"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == Ready::Notrdy
    }
    #[doc = "VREF output is ready"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == Ready::Rdy
    }
}
impl R {
    #[doc = "Bit 0 - These bits defines status of VREF"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefCtl1Spec;
impl crate::RegisterSpec for VrefCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_ctl1::R`](R) reader structure"]
impl crate::Readable for VrefCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_ctl1::W`](W) writer structure"]
impl crate::Writable for VrefCtl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREF_CTL1 to value 0"]
impl crate::Resettable for VrefCtl1Spec {}
