#[doc = "Register `SYSCTL_SYSTEMCFG` reader"]
pub type R = crate::R<SysctlSystemcfgSpec>;
#[doc = "Register `SYSCTL_SYSTEMCFG` writer"]
pub type W = crate::W<SysctlSystemcfgSpec>;
#[doc = "WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdtlp0rstdis {
    #[doc = "0: WWDTLP0 Error Event will trigger a BOOTRST"]
    False = 0,
    #[doc = "1: WWDTLP0 Error Event will trigger an NMI"]
    True = 1,
}
impl From<Wwdtlp0rstdis> for bool {
    #[inline(always)]
    fn from(variant: Wwdtlp0rstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTLP0RSTDIS` reader - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type Wwdtlp0rstdisR = crate::BitReader<Wwdtlp0rstdis>;
impl Wwdtlp0rstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtlp0rstdis {
        match self.bits {
            false => Wwdtlp0rstdis::False,
            true => Wwdtlp0rstdis::True,
        }
    }
    #[doc = "WWDTLP0 Error Event will trigger a BOOTRST"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Wwdtlp0rstdis::False
    }
    #[doc = "WWDTLP0 Error Event will trigger an NMI"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Wwdtlp0rstdis::True
    }
}
#[doc = "Field `WWDTLP0RSTDIS` writer - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type Wwdtlp0rstdisW<'a, REG> = crate::BitWriter<'a, REG, Wwdtlp0rstdis>;
impl<'a, REG> Wwdtlp0rstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WWDTLP0 Error Event will trigger a BOOTRST"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtlp0rstdis::False)
    }
    #[doc = "WWDTLP0 Error Event will trigger an NMI"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtlp0rstdis::True)
    }
}
#[doc = "FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flasheccrstdis {
    #[doc = "0: Flash ECC DED will trigger a SYSRST"]
    False = 0,
    #[doc = "1: Flash ECC DED will trigger a NMI"]
    True = 1,
}
impl From<Flasheccrstdis> for bool {
    #[inline(always)]
    fn from(variant: Flasheccrstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHECCRSTDIS` reader - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type FlasheccrstdisR = crate::BitReader<Flasheccrstdis>;
impl FlasheccrstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flasheccrstdis {
        match self.bits {
            false => Flasheccrstdis::False,
            true => Flasheccrstdis::True,
        }
    }
    #[doc = "Flash ECC DED will trigger a SYSRST"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Flasheccrstdis::False
    }
    #[doc = "Flash ECC DED will trigger a NMI"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Flasheccrstdis::True
    }
}
#[doc = "Field `FLASHECCRSTDIS` writer - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type FlasheccrstdisW<'a, REG> = crate::BitWriter<'a, REG, Flasheccrstdis>;
impl<'a, REG> FlasheccrstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash ECC DED will trigger a SYSRST"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Flasheccrstdis::False)
    }
    #[doc = "Flash ECC DED will trigger a NMI"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Flasheccrstdis::True)
    }
}
#[doc = "SUPERCAP specifies whether the battery backup system can be powered by a SUPERCAP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Supercapen {
    #[doc = "0: SUPERCAP Function is not enabled"]
    False = 0,
    #[doc = "1: SUPERCAP Function is not enabled"]
    True = 1,
}
impl From<Supercapen> for bool {
    #[inline(always)]
    fn from(variant: Supercapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUPERCAPEN` reader - SUPERCAP specifies whether the battery backup system can be powered by a SUPERCAP"]
pub type SupercapenR = crate::BitReader<Supercapen>;
impl SupercapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Supercapen {
        match self.bits {
            false => Supercapen::False,
            true => Supercapen::True,
        }
    }
    #[doc = "SUPERCAP Function is not enabled"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Supercapen::False
    }
    #[doc = "SUPERCAP Function is not enabled"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Supercapen::True
    }
}
#[doc = "Field `SUPERCAPEN` writer - SUPERCAP specifies whether the battery backup system can be powered by a SUPERCAP"]
pub type SupercapenW<'a, REG> = crate::BitWriter<'a, REG, Supercapen>;
impl<'a, REG> SupercapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SUPERCAP Function is not enabled"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Supercapen::False)
    }
    #[doc = "SUPERCAP Function is not enabled"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Supercapen::True)
    }
}
impl R {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn wwdtlp0rstdis(&self) -> Wwdtlp0rstdisR {
        Wwdtlp0rstdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn flasheccrstdis(&self) -> FlasheccrstdisR {
        FlasheccrstdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SUPERCAP specifies whether the battery backup system can be powered by a SUPERCAP"]
    #[inline(always)]
    pub fn supercapen(&self) -> SupercapenR {
        SupercapenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn wwdtlp0rstdis(&mut self) -> Wwdtlp0rstdisW<'_, SysctlSystemcfgSpec> {
        Wwdtlp0rstdisW::new(self, 0)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn flasheccrstdis(&mut self) -> FlasheccrstdisW<'_, SysctlSystemcfgSpec> {
        FlasheccrstdisW::new(self, 2)
    }
    #[doc = "Bit 8 - SUPERCAP specifies whether the battery backup system can be powered by a SUPERCAP"]
    #[inline(always)]
    pub fn supercapen(&mut self) -> SupercapenW<'_, SysctlSystemcfgSpec> {
        SupercapenW::new(self, 8)
    }
}
#[doc = "System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_systemcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_systemcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSystemcfgSpec;
impl crate::RegisterSpec for SysctlSystemcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctl_systemcfg::R`](R) reader structure"]
impl crate::Readable for SysctlSystemcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl_systemcfg::W`](W) writer structure"]
impl crate::Writable for SysctlSystemcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL_SYSTEMCFG to value 0x04"]
impl crate::Resettable for SysctlSystemcfgSpec {
    const RESET_VALUE: u32 = 0x04;
}
