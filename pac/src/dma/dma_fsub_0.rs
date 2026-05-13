#[doc = "Register `DMA_FSUB_0` reader"]
pub type R = crate::R<DmaFsub0Spec>;
#[doc = "Register `DMA_FSUB_0` writer"]
pub type W = crate::W<DmaFsub0Spec>;
#[doc = "0 = disconnected. 1-255 = connected to channelID = CHANID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chanid {
    #[doc = "0: A value of 0 specifies that the event is not connected"]
    Unconnected = 0,
}
impl From<Chanid> for u8 {
    #[inline(always)]
    fn from(variant: Chanid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chanid {
    type Ux = u8;
}
impl crate::IsEnum for Chanid {}
#[doc = "Field `CHANID` reader - 0 = disconnected. 1-255 = connected to channelID = CHANID."]
pub type ChanidR = crate::FieldReader<Chanid>;
impl ChanidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chanid> {
        match self.bits {
            0 => Some(Chanid::Unconnected),
            _ => None,
        }
    }
    #[doc = "A value of 0 specifies that the event is not connected"]
    #[inline(always)]
    pub fn is_unconnected(&self) -> bool {
        *self == Chanid::Unconnected
    }
}
#[doc = "Field `CHANID` writer - 0 = disconnected. 1-255 = connected to channelID = CHANID."]
pub type ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 4, Chanid>;
impl<'a, REG> ChanidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A value of 0 specifies that the event is not connected"]
    #[inline(always)]
    pub fn unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Chanid::Unconnected)
    }
}
impl R {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-255 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn chanid(&self) -> ChanidR {
        ChanidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-255 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn chanid(&mut self) -> ChanidW<'_, DmaFsub0Spec> {
        ChanidW::new(self, 0)
    }
}
#[doc = "Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_fsub_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_fsub_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaFsub0Spec;
impl crate::RegisterSpec for DmaFsub0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_fsub_0::R`](R) reader structure"]
impl crate::Readable for DmaFsub0Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_fsub_0::W`](W) writer structure"]
impl crate::Writable for DmaFsub0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_FSUB_0 to value 0"]
impl crate::Resettable for DmaFsub0Spec {}
