#[doc = "Register `FLASHCTL_CFGCMD` reader"]
pub type R = crate::R<FlashctlCfgcmdSpec>;
#[doc = "Register `FLASHCTL_CFGCMD` writer"]
pub type W = crate::W<FlashctlCfgcmdSpec>;
#[doc = "Field `WAITSTATE` reader - Wait State setting for program verify, erase verify and read verify"]
pub type WaitstateR = crate::FieldReader;
#[doc = "Field `WAITSTATE` writer - Wait State setting for program verify, erase verify and read verify"]
pub type WaitstateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Enable pulse stretching when generating a read clock to the flash bank from the flash wrapper. This effectively divides the read clock driven to the bank in order to avoid minimum pulse width requirements at the bank.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdclkstren {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Rdclkstren> for bool {
    #[inline(always)]
    fn from(variant: Rdclkstren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCLKSTREN` reader - Enable pulse stretching when generating a read clock to the flash bank from the flash wrapper. This effectively divides the read clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
pub type RdclkstrenR = crate::BitReader<Rdclkstren>;
impl RdclkstrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdclkstren {
        match self.bits {
            false => Rdclkstren::Disable,
            true => Rdclkstren::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rdclkstren::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rdclkstren::Enable
    }
}
#[doc = "Field `RDCLKSTREN` writer - Enable pulse stretching when generating a read clock to the flash bank from the flash wrapper. This effectively divides the read clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
pub type RdclkstrenW<'a, REG> = crate::BitWriter<'a, REG, Rdclkstren>;
impl<'a, REG> RdclkstrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rdclkstren::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rdclkstren::Enable)
    }
}
#[doc = "Enable pulse stretching when generating a control clock to the flash bank from the flash wrapper. This effectively divides the control clock driven to the bank in order to avoid minimum pulse width requirements at the bank.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlclkstren {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Ctrlclkstren> for bool {
    #[inline(always)]
    fn from(variant: Ctrlclkstren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLCLKSTREN` reader - Enable pulse stretching when generating a control clock to the flash bank from the flash wrapper. This effectively divides the control clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
pub type CtrlclkstrenR = crate::BitReader<Ctrlclkstren>;
impl CtrlclkstrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlclkstren {
        match self.bits {
            false => Ctrlclkstren::Disable,
            true => Ctrlclkstren::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctrlclkstren::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctrlclkstren::Enable
    }
}
#[doc = "Field `CTRLCLKSTREN` writer - Enable pulse stretching when generating a control clock to the flash bank from the flash wrapper. This effectively divides the control clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
pub type CtrlclkstrenW<'a, REG> = crate::BitWriter<'a, REG, Ctrlclkstren>;
impl<'a, REG> CtrlclkstrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlclkstren::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlclkstren::Enable)
    }
}
#[doc = "Enable pulse stretching for the clocking of the hold latches for inputs to the flash bank. This effectively divides the flash controller internal clock in order to create a 50/50 duty cycle clock for hold latching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holdclkstren {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Holdclkstren> for bool {
    #[inline(always)]
    fn from(variant: Holdclkstren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOLDCLKSTREN` reader - Enable pulse stretching for the clocking of the hold latches for inputs to the flash bank. This effectively divides the flash controller internal clock in order to create a 50/50 duty cycle clock for hold latching."]
pub type HoldclkstrenR = crate::BitReader<Holdclkstren>;
impl HoldclkstrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Holdclkstren {
        match self.bits {
            false => Holdclkstren::Disable,
            true => Holdclkstren::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Holdclkstren::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Holdclkstren::Enable
    }
}
#[doc = "Field `HOLDCLKSTREN` writer - Enable pulse stretching for the clocking of the hold latches for inputs to the flash bank. This effectively divides the flash controller internal clock in order to create a 50/50 duty cycle clock for hold latching."]
pub type HoldclkstrenW<'a, REG> = crate::BitWriter<'a, REG, Holdclkstren>;
impl<'a, REG> HoldclkstrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Holdclkstren::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Holdclkstren::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    pub fn waitstate(&self) -> WaitstateR {
        WaitstateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable pulse stretching when generating a read clock to the flash bank from the flash wrapper. This effectively divides the read clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
    #[inline(always)]
    pub fn rdclkstren(&self) -> RdclkstrenR {
        RdclkstrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable pulse stretching when generating a control clock to the flash bank from the flash wrapper. This effectively divides the control clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
    #[inline(always)]
    pub fn ctrlclkstren(&self) -> CtrlclkstrenR {
        CtrlclkstrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable pulse stretching for the clocking of the hold latches for inputs to the flash bank. This effectively divides the flash controller internal clock in order to create a 50/50 duty cycle clock for hold latching."]
    #[inline(always)]
    pub fn holdclkstren(&self) -> HoldclkstrenR {
        HoldclkstrenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    pub fn waitstate(&mut self) -> WaitstateW<'_, FlashctlCfgcmdSpec> {
        WaitstateW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable pulse stretching when generating a read clock to the flash bank from the flash wrapper. This effectively divides the read clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
    #[inline(always)]
    pub fn rdclkstren(&mut self) -> RdclkstrenW<'_, FlashctlCfgcmdSpec> {
        RdclkstrenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable pulse stretching when generating a control clock to the flash bank from the flash wrapper. This effectively divides the control clock driven to the bank in order to avoid minimum pulse width requirements at the bank."]
    #[inline(always)]
    pub fn ctrlclkstren(&mut self) -> CtrlclkstrenW<'_, FlashctlCfgcmdSpec> {
        CtrlclkstrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable pulse stretching for the clocking of the hold latches for inputs to the flash bank. This effectively divides the flash controller internal clock in order to create a 50/50 duty cycle clock for hold latching."]
    #[inline(always)]
    pub fn holdclkstren(&mut self) -> HoldclkstrenW<'_, FlashctlCfgcmdSpec> {
        HoldclkstrenW::new(self, 6)
    }
}
#[doc = "Command Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cfgcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cfgcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCfgcmdSpec;
impl crate::RegisterSpec for FlashctlCfgcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cfgcmd::R`](R) reader structure"]
impl crate::Readable for FlashctlCfgcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cfgcmd::W`](W) writer structure"]
impl crate::Writable for FlashctlCfgcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CFGCMD to value 0"]
impl crate::Resettable for FlashctlCfgcmdSpec {}
