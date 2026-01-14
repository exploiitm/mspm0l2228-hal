#[doc = "Register `DMA_DMATCTL` reader"]
pub type R = crate::R<DmaDmatctlSpec>;
#[doc = "Register `DMA_DMATCTL` writer"]
pub type W = crate::W<DmaDmatctlSpec>;
#[doc = "DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmatsel {
    #[doc = "0: Software trigger request"]
    Dmareq = 0,
}
impl From<Dmatsel> for u8 {
    #[inline(always)]
    fn from(variant: Dmatsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmatsel {
    type Ux = u8;
}
impl crate::IsEnum for Dmatsel {}
#[doc = "Field `DMATSEL` reader - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
pub type DmatselR = crate::FieldReader<Dmatsel>;
impl DmatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmatsel> {
        match self.bits {
            0 => Some(Dmatsel::Dmareq),
            _ => None,
        }
    }
    #[doc = "Software trigger request"]
    #[inline(always)]
    pub fn is_dmareq(&self) -> bool {
        *self == Dmatsel::Dmareq
    }
}
#[doc = "Field `DMATSEL` writer - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
pub type DmatselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Dmatsel>;
impl<'a, REG> DmatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software trigger request"]
    #[inline(always)]
    pub fn dmareq(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatsel::Dmareq)
    }
}
#[doc = "DMA Trigger by Internal Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatint {
    #[doc = "0: DMATSEL will define external trigger select as transfer trigger."]
    External = 0,
    #[doc = "1: DMATSEL will define internal channel as transfer trigger select. 0-> Channel0-done, 1-> Channel1-done, ..."]
    Internal = 1,
}
impl From<Dmatint> for bool {
    #[inline(always)]
    fn from(variant: Dmatint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATINT` reader - DMA Trigger by Internal Channel"]
pub type DmatintR = crate::BitReader<Dmatint>;
impl DmatintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatint {
        match self.bits {
            false => Dmatint::External,
            true => Dmatint::Internal,
        }
    }
    #[doc = "DMATSEL will define external trigger select as transfer trigger."]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Dmatint::External
    }
    #[doc = "DMATSEL will define internal channel as transfer trigger select. 0-> Channel0-done, 1-> Channel1-done, ..."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Dmatint::Internal
    }
}
#[doc = "Field `DMATINT` writer - DMA Trigger by Internal Channel"]
pub type DmatintW<'a, REG> = crate::BitWriter<'a, REG, Dmatint>;
impl<'a, REG> DmatintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMATSEL will define external trigger select as transfer trigger."]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatint::External)
    }
    #[doc = "DMATSEL will define internal channel as transfer trigger select. 0-> Channel0-done, 1-> Channel1-done, ..."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatint::Internal)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
    #[inline(always)]
    pub fn dmatsel(&self) -> DmatselR {
        DmatselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - DMA Trigger by Internal Channel"]
    #[inline(always)]
    pub fn dmatint(&self) -> DmatintR {
        DmatintR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
    #[inline(always)]
    pub fn dmatsel(&mut self) -> DmatselW<'_, DmaDmatctlSpec> {
        DmatselW::new(self, 0)
    }
    #[doc = "Bit 7 - DMA Trigger by Internal Channel"]
    #[inline(always)]
    pub fn dmatint(&mut self) -> DmatintW<'_, DmaDmatctlSpec> {
        DmatintW::new(self, 7)
    }
}
#[doc = "DMA Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmatctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmatctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDmatctlSpec;
impl crate::RegisterSpec for DmaDmatctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_dmatctl::R`](R) reader structure"]
impl crate::Readable for DmaDmatctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_dmatctl::W`](W) writer structure"]
impl crate::Writable for DmaDmatctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DMATCTL to value 0"]
impl crate::Resettable for DmaDmatctlSpec {}
