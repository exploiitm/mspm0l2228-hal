#[doc = "Register `DMA_DMAPRIO` reader"]
pub type R = crate::R<DmaDmaprioSpec>;
#[doc = "Register `DMA_DMAPRIO` writer"]
pub type W = crate::W<DmaDmaprioSpec>;
#[doc = "Round robin. This bit enables the round-robin DMA channel priorities.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roundrobin {
    #[doc = "0: Roundrobin priority disabled, DMA channel priority is fixed: DMA0-DMA1-DMA2-...-DMA16"]
    Disable = 0,
    #[doc = "1: Roundrobin priority enabled, DMA channel priority changes with each transfer"]
    Enable = 1,
}
impl From<Roundrobin> for bool {
    #[inline(always)]
    fn from(variant: Roundrobin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROUNDROBIN` reader - Round robin. This bit enables the round-robin DMA channel priorities."]
pub type RoundrobinR = crate::BitReader<Roundrobin>;
impl RoundrobinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Roundrobin {
        match self.bits {
            false => Roundrobin::Disable,
            true => Roundrobin::Enable,
        }
    }
    #[doc = "Roundrobin priority disabled, DMA channel priority is fixed: DMA0-DMA1-DMA2-...-DMA16"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Roundrobin::Disable
    }
    #[doc = "Roundrobin priority enabled, DMA channel priority changes with each transfer"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Roundrobin::Enable
    }
}
#[doc = "Field `ROUNDROBIN` writer - Round robin. This bit enables the round-robin DMA channel priorities."]
pub type RoundrobinW<'a, REG> = crate::BitWriter<'a, REG, Roundrobin>;
impl<'a, REG> RoundrobinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Roundrobin priority disabled, DMA channel priority is fixed: DMA0-DMA1-DMA2-...-DMA16"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Roundrobin::Disable)
    }
    #[doc = "Roundrobin priority enabled, DMA channel priority changes with each transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Roundrobin::Enable)
    }
}
#[doc = "Define the burst size of a block transfer, before the priority is re-evaluated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstsz {
    #[doc = "0: There is no burst size, the whole block transfer is completed on one transfer without interruption"]
    Infiniti = 0,
    #[doc = "1: The burst size is 8, after 8 transfers the block transfer is interrupted and the priority is reevaluated"]
    Burst8 = 1,
    #[doc = "2: The burst size is 16, after 16 transfers the block transfer is interrupted and the priority is reevaluated"]
    Busrt16 = 2,
    #[doc = "3: The burst size is 32, after 32 transfers the block transfer is interrupted and the priority is reevaluated"]
    Burst32 = 3,
}
impl From<Burstsz> for u8 {
    #[inline(always)]
    fn from(variant: Burstsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstsz {
    type Ux = u8;
}
impl crate::IsEnum for Burstsz {}
#[doc = "Field `BURSTSZ` reader - Define the burst size of a block transfer, before the priority is re-evaluated"]
pub type BurstszR = crate::FieldReader<Burstsz>;
impl BurstszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstsz {
        match self.bits {
            0 => Burstsz::Infiniti,
            1 => Burstsz::Burst8,
            2 => Burstsz::Busrt16,
            3 => Burstsz::Burst32,
            _ => unreachable!(),
        }
    }
    #[doc = "There is no burst size, the whole block transfer is completed on one transfer without interruption"]
    #[inline(always)]
    pub fn is_infiniti(&self) -> bool {
        *self == Burstsz::Infiniti
    }
    #[doc = "The burst size is 8, after 8 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn is_burst_8(&self) -> bool {
        *self == Burstsz::Burst8
    }
    #[doc = "The burst size is 16, after 16 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn is_busrt_16(&self) -> bool {
        *self == Burstsz::Busrt16
    }
    #[doc = "The burst size is 32, after 32 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn is_burst_32(&self) -> bool {
        *self == Burstsz::Burst32
    }
}
#[doc = "Field `BURSTSZ` writer - Define the burst size of a block transfer, before the priority is re-evaluated"]
pub type BurstszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Burstsz, crate::Safe>;
impl<'a, REG> BurstszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "There is no burst size, the whole block transfer is completed on one transfer without interruption"]
    #[inline(always)]
    pub fn infiniti(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsz::Infiniti)
    }
    #[doc = "The burst size is 8, after 8 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn burst_8(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsz::Burst8)
    }
    #[doc = "The burst size is 16, after 16 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn busrt_16(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsz::Busrt16)
    }
    #[doc = "The burst size is 32, after 32 transfers the block transfer is interrupted and the priority is reevaluated"]
    #[inline(always)]
    pub fn burst_32(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsz::Burst32)
    }
}
impl R {
    #[doc = "Bit 0 - Round robin. This bit enables the round-robin DMA channel priorities."]
    #[inline(always)]
    pub fn roundrobin(&self) -> RoundrobinR {
        RoundrobinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - Define the burst size of a block transfer, before the priority is re-evaluated"]
    #[inline(always)]
    pub fn burstsz(&self) -> BurstszR {
        BurstszR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Round robin. This bit enables the round-robin DMA channel priorities."]
    #[inline(always)]
    pub fn roundrobin(&mut self) -> RoundrobinW<'_, DmaDmaprioSpec> {
        RoundrobinW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Define the burst size of a block transfer, before the priority is re-evaluated"]
    #[inline(always)]
    pub fn burstsz(&mut self) -> BurstszW<'_, DmaDmaprioSpec> {
        BurstszW::new(self, 16)
    }
}
#[doc = "DMA Channel Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dmaprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dmaprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDmaprioSpec;
impl crate::RegisterSpec for DmaDmaprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_dmaprio::R`](R) reader structure"]
impl crate::Readable for DmaDmaprioSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_dmaprio::W`](W) writer structure"]
impl crate::Writable for DmaDmaprioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DMAPRIO to value 0"]
impl crate::Resettable for DmaDmaprioSpec {}
