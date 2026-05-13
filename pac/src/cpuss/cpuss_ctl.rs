#[doc = "Register `CPUSS_CTL` reader"]
pub type R = crate::R<CpussCtlSpec>;
#[doc = "Register `CPUSS_CTL` writer"]
pub type W = crate::W<CpussCtlSpec>;
#[doc = "Used to enable/disable instruction prefetch to Flash.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefetch {
    #[doc = "0: Disable instruction prefetch."]
    Disable = 0,
    #[doc = "1: Enable instruction prefetch."]
    Enable = 1,
}
impl From<Prefetch> for bool {
    #[inline(always)]
    fn from(variant: Prefetch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFETCH` reader - Used to enable/disable instruction prefetch to Flash."]
pub type PrefetchR = crate::BitReader<Prefetch>;
impl PrefetchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prefetch {
        match self.bits {
            false => Prefetch::Disable,
            true => Prefetch::Enable,
        }
    }
    #[doc = "Disable instruction prefetch."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prefetch::Disable
    }
    #[doc = "Enable instruction prefetch."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prefetch::Enable
    }
}
#[doc = "Field `PREFETCH` writer - Used to enable/disable instruction prefetch to Flash."]
pub type PrefetchW<'a, REG> = crate::BitWriter<'a, REG, Prefetch>;
impl<'a, REG> PrefetchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable instruction prefetch."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefetch::Disable)
    }
    #[doc = "Enable instruction prefetch."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prefetch::Enable)
    }
}
#[doc = "Used to enable/disable Instruction caching on flash access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icache {
    #[doc = "0: Disable instruction caching."]
    Disable = 0,
    #[doc = "1: Enable instruction caching."]
    Enable = 1,
}
impl From<Icache> for bool {
    #[inline(always)]
    fn from(variant: Icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHE` reader - Used to enable/disable Instruction caching on flash access."]
pub type IcacheR = crate::BitReader<Icache>;
impl IcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icache {
        match self.bits {
            false => Icache::Disable,
            true => Icache::Enable,
        }
    }
    #[doc = "Disable instruction caching."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Icache::Disable
    }
    #[doc = "Enable instruction caching."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Icache::Enable
    }
}
#[doc = "Field `ICACHE` writer - Used to enable/disable Instruction caching on flash access."]
pub type IcacheW<'a, REG> = crate::BitWriter<'a, REG, Icache>;
impl<'a, REG> IcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable instruction caching."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Icache::Disable)
    }
    #[doc = "Enable instruction caching."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Icache::Enable)
    }
}
#[doc = "Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Liten {
    #[doc = "0: Literal caching disabled"]
    Disable = 0,
    #[doc = "1: Literal caching enabled"]
    Enable = 1,
}
impl From<Liten> for bool {
    #[inline(always)]
    fn from(variant: Liten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LITEN` reader - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
pub type LitenR = crate::BitReader<Liten>;
impl LitenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Liten {
        match self.bits {
            false => Liten::Disable,
            true => Liten::Enable,
        }
    }
    #[doc = "Literal caching disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Liten::Disable
    }
    #[doc = "Literal caching enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Liten::Enable
    }
}
#[doc = "Field `LITEN` writer - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
pub type LitenW<'a, REG> = crate::BitWriter<'a, REG, Liten>;
impl<'a, REG> LitenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Literal caching disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Liten::Disable)
    }
    #[doc = "Literal caching enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Liten::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."]
    #[inline(always)]
    pub fn prefetch(&self) -> PrefetchR {
        PrefetchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."]
    #[inline(always)]
    pub fn icache(&self) -> IcacheR {
        IcacheR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
    #[inline(always)]
    pub fn liten(&self) -> LitenR {
        LitenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to enable/disable instruction prefetch to Flash."]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PrefetchW<'_, CpussCtlSpec> {
        PrefetchW::new(self, 0)
    }
    #[doc = "Bit 1 - Used to enable/disable Instruction caching on flash access."]
    #[inline(always)]
    pub fn icache(&mut self) -> IcacheW<'_, CpussCtlSpec> {
        IcacheW::new(self, 1)
    }
    #[doc = "Bit 2 - Literal caching and prefetch enable. This bit is a subset of ICACHE/PREFETCH bit i.e. literal caching or literal prefetching will only happen if ICACHE or PREFETCH bits have been set respectively When enabled, the cache and prefetcher structures inside CPUSS will cache and prefetch literals When disabled, the cache and prefetcher structures inside CPUSS will not cache and prefetch literals"]
    #[inline(always)]
    pub fn liten(&mut self) -> LitenW<'_, CpussCtlSpec> {
        LitenW::new(self, 2)
    }
}
#[doc = "Prefetch/Cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuss_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuss_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpussCtlSpec;
impl crate::RegisterSpec for CpussCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuss_ctl::R`](R) reader structure"]
impl crate::Readable for CpussCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuss_ctl::W`](W) writer structure"]
impl crate::Writable for CpussCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUSS_CTL to value 0x07"]
impl crate::Resettable for CpussCtlSpec {
    const RESET_VALUE: u32 = 0x07;
}
