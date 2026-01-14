#[doc = "Register `UART2_GFCTL` reader"]
pub type R = crate::R<Uart2GfctlSpec>;
#[doc = "Register `UART2_GFCTL` writer"]
pub type W = crate::W<Uart2GfctlSpec>;
#[doc = "Analog Glitch Suppression Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agfen {
    #[doc = "0: Analog Glitch Filter disable"]
    Disable = 0,
    #[doc = "1: Analog Glitch Filter enable"]
    Enable = 1,
}
impl From<Agfen> for bool {
    #[inline(always)]
    fn from(variant: Agfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGFEN` reader - Analog Glitch Suppression Enable"]
pub type AgfenR = crate::BitReader<Agfen>;
impl AgfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agfen {
        match self.bits {
            false => Agfen::Disable,
            true => Agfen::Enable,
        }
    }
    #[doc = "Analog Glitch Filter disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Agfen::Disable
    }
    #[doc = "Analog Glitch Filter enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Agfen::Enable
    }
}
#[doc = "Field `AGFEN` writer - Analog Glitch Suppression Enable"]
pub type AgfenW<'a, REG> = crate::BitWriter<'a, REG, Agfen>;
impl<'a, REG> AgfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Glitch Filter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Agfen::Disable)
    }
    #[doc = "Analog Glitch Filter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Agfen::Enable)
    }
}
#[doc = "Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Agfsel {
    #[doc = "0: Pulses shorter then 5ns length are filtered."]
    Aglit5 = 0,
    #[doc = "1: Pulses shorter then 10ns length are filtered."]
    Aglit10 = 1,
    #[doc = "2: Pulses shorter then 25ns length are filtered."]
    Aglit25 = 2,
    #[doc = "3: Pulses shorter then 50ns length are filtered."]
    Aglit50 = 3,
}
impl From<Agfsel> for u8 {
    #[inline(always)]
    fn from(variant: Agfsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Agfsel {
    type Ux = u8;
}
impl crate::IsEnum for Agfsel {}
#[doc = "Field `AGFSEL` reader - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
pub type AgfselR = crate::FieldReader<Agfsel>;
impl AgfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agfsel {
        match self.bits {
            0 => Agfsel::Aglit5,
            1 => Agfsel::Aglit10,
            2 => Agfsel::Aglit25,
            3 => Agfsel::Aglit50,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulses shorter then 5ns length are filtered."]
    #[inline(always)]
    pub fn is_aglit_5(&self) -> bool {
        *self == Agfsel::Aglit5
    }
    #[doc = "Pulses shorter then 10ns length are filtered."]
    #[inline(always)]
    pub fn is_aglit_10(&self) -> bool {
        *self == Agfsel::Aglit10
    }
    #[doc = "Pulses shorter then 25ns length are filtered."]
    #[inline(always)]
    pub fn is_aglit_25(&self) -> bool {
        *self == Agfsel::Aglit25
    }
    #[doc = "Pulses shorter then 50ns length are filtered."]
    #[inline(always)]
    pub fn is_aglit_50(&self) -> bool {
        *self == Agfsel::Aglit50
    }
}
#[doc = "Field `AGFSEL` writer - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
pub type AgfselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Agfsel, crate::Safe>;
impl<'a, REG> AgfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulses shorter then 5ns length are filtered."]
    #[inline(always)]
    pub fn aglit_5(self) -> &'a mut crate::W<REG> {
        self.variant(Agfsel::Aglit5)
    }
    #[doc = "Pulses shorter then 10ns length are filtered."]
    #[inline(always)]
    pub fn aglit_10(self) -> &'a mut crate::W<REG> {
        self.variant(Agfsel::Aglit10)
    }
    #[doc = "Pulses shorter then 25ns length are filtered."]
    #[inline(always)]
    pub fn aglit_25(self) -> &'a mut crate::W<REG> {
        self.variant(Agfsel::Aglit25)
    }
    #[doc = "Pulses shorter then 50ns length are filtered."]
    #[inline(always)]
    pub fn aglit_50(self) -> &'a mut crate::W<REG> {
        self.variant(Agfsel::Aglit50)
    }
}
impl R {
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn agfen(&self) -> AgfenR {
        AgfenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
    #[inline(always)]
    pub fn agfsel(&self) -> AgfselR {
        AgfselR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn agfen(&mut self) -> AgfenW<'_, Uart2GfctlSpec> {
        AgfenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
    #[inline(always)]
    pub fn agfsel(&mut self) -> AgfselW<'_, Uart2GfctlSpec> {
        AgfselW::new(self, 9)
    }
}
#[doc = "Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_gfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_gfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2GfctlSpec;
impl crate::RegisterSpec for Uart2GfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_gfctl::R`](R) reader structure"]
impl crate::Readable for Uart2GfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart2_gfctl::W`](W) writer structure"]
impl crate::Writable for Uart2GfctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART2_GFCTL to value 0"]
impl crate::Resettable for Uart2GfctlSpec {}
