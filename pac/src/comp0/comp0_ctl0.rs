#[doc = "Register `COMP0_CTL0` reader"]
pub type R = crate::R<Comp0Ctl0Spec>;
#[doc = "Register `COMP0_CTL0` writer"]
pub type W = crate::W<Comp0Ctl0Spec>;
#[doc = "Channel input selected for the positive terminal of the comparator if IPEN is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipsel {
    #[doc = "0: Channel 0 selected"]
    Ch0 = 0,
    #[doc = "1: Channel 1 selected"]
    Ch1 = 1,
    #[doc = "2: Channel 2 selected"]
    Ch2 = 2,
    #[doc = "3: Channel 3 selected"]
    Ch3 = 3,
    #[doc = "4: Channel 4 selected"]
    Ch4 = 4,
    #[doc = "5: Channel 5 selected"]
    Ch5 = 5,
    #[doc = "6: Channel 6 selected"]
    Ch6 = 6,
    #[doc = "7: Channel 7 selected"]
    Ch7 = 7,
}
impl From<Ipsel> for u8 {
    #[inline(always)]
    fn from(variant: Ipsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipsel {
    type Ux = u8;
}
impl crate::IsEnum for Ipsel {}
#[doc = "Field `IPSEL` reader - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
pub type IpselR = crate::FieldReader<Ipsel>;
impl IpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipsel {
        match self.bits {
            0 => Ipsel::Ch0,
            1 => Ipsel::Ch1,
            2 => Ipsel::Ch2,
            3 => Ipsel::Ch3,
            4 => Ipsel::Ch4,
            5 => Ipsel::Ch5,
            6 => Ipsel::Ch6,
            7 => Ipsel::Ch7,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn is_ch_0(&self) -> bool {
        *self == Ipsel::Ch0
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn is_ch_1(&self) -> bool {
        *self == Ipsel::Ch1
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn is_ch_2(&self) -> bool {
        *self == Ipsel::Ch2
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn is_ch_3(&self) -> bool {
        *self == Ipsel::Ch3
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn is_ch_4(&self) -> bool {
        *self == Ipsel::Ch4
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn is_ch_5(&self) -> bool {
        *self == Ipsel::Ch5
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn is_ch_6(&self) -> bool {
        *self == Ipsel::Ch6
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn is_ch_7(&self) -> bool {
        *self == Ipsel::Ch7
    }
}
#[doc = "Field `IPSEL` writer - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
pub type IpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ipsel, crate::Safe>;
impl<'a, REG> IpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel::Ch7)
    }
}
#[doc = "Channel input enable for the positive terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipen {
    #[doc = "0: Selected analog input channel for positive terminal is disabled"]
    Disable = 0,
    #[doc = "1: Selected analog input channel for positive terminal is enabled"]
    Enable = 1,
}
impl From<Ipen> for bool {
    #[inline(always)]
    fn from(variant: Ipen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEN` reader - Channel input enable for the positive terminal of the comparator."]
pub type IpenR = crate::BitReader<Ipen>;
impl IpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipen {
        match self.bits {
            false => Ipen::Disable,
            true => Ipen::Enable,
        }
    }
    #[doc = "Selected analog input channel for positive terminal is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ipen::Disable
    }
    #[doc = "Selected analog input channel for positive terminal is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ipen::Enable
    }
}
#[doc = "Field `IPEN` writer - Channel input enable for the positive terminal of the comparator."]
pub type IpenW<'a, REG> = crate::BitWriter<'a, REG, Ipen>;
impl<'a, REG> IpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for positive terminal is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen::Disable)
    }
    #[doc = "Selected analog input channel for positive terminal is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen::Enable)
    }
}
#[doc = "Channel input selected for the negative terminal of the comparator if IMEN is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imsel {
    #[doc = "0: Channel 0 selected"]
    Ch0 = 0,
    #[doc = "1: Channel 1 selected"]
    Ch1 = 1,
    #[doc = "2: Channel 2 selected"]
    Ch2 = 2,
    #[doc = "3: Channel 3 selected"]
    Ch3 = 3,
    #[doc = "4: Channel 4 selected"]
    Ch4 = 4,
    #[doc = "5: Channel 5 selected"]
    Ch5 = 5,
    #[doc = "6: Channel 6 selected"]
    Ch6 = 6,
    #[doc = "7: Channel 7 selected"]
    Ch7 = 7,
}
impl From<Imsel> for u8 {
    #[inline(always)]
    fn from(variant: Imsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imsel {
    type Ux = u8;
}
impl crate::IsEnum for Imsel {}
#[doc = "Field `IMSEL` reader - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
pub type ImselR = crate::FieldReader<Imsel>;
impl ImselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imsel {
        match self.bits {
            0 => Imsel::Ch0,
            1 => Imsel::Ch1,
            2 => Imsel::Ch2,
            3 => Imsel::Ch3,
            4 => Imsel::Ch4,
            5 => Imsel::Ch5,
            6 => Imsel::Ch6,
            7 => Imsel::Ch7,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn is_ch_0(&self) -> bool {
        *self == Imsel::Ch0
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn is_ch_1(&self) -> bool {
        *self == Imsel::Ch1
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn is_ch_2(&self) -> bool {
        *self == Imsel::Ch2
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn is_ch_3(&self) -> bool {
        *self == Imsel::Ch3
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn is_ch_4(&self) -> bool {
        *self == Imsel::Ch4
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn is_ch_5(&self) -> bool {
        *self == Imsel::Ch5
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn is_ch_6(&self) -> bool {
        *self == Imsel::Ch6
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn is_ch_7(&self) -> bool {
        *self == Imsel::Ch7
    }
}
#[doc = "Field `IMSEL` writer - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
pub type ImselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Imsel, crate::Safe>;
impl<'a, REG> ImselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Imsel::Ch7)
    }
}
#[doc = "Channel input enable for the negative terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imen {
    #[doc = "0: Selected analog input channel for negative terminal is disabled"]
    Disable = 0,
    #[doc = "1: Selected analog input channel for negative terminal is enabled"]
    Enable = 1,
}
impl From<Imen> for bool {
    #[inline(always)]
    fn from(variant: Imen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMEN` reader - Channel input enable for the negative terminal of the comparator."]
pub type ImenR = crate::BitReader<Imen>;
impl ImenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imen {
        match self.bits {
            false => Imen::Disable,
            true => Imen::Enable,
        }
    }
    #[doc = "Selected analog input channel for negative terminal is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Imen::Disable
    }
    #[doc = "Selected analog input channel for negative terminal is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Imen::Enable
    }
}
#[doc = "Field `IMEN` writer - Channel input enable for the negative terminal of the comparator."]
pub type ImenW<'a, REG> = crate::BitWriter<'a, REG, Imen>;
impl<'a, REG> ImenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for negative terminal is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Imen::Disable)
    }
    #[doc = "Selected analog input channel for negative terminal is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Imen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
    #[inline(always)]
    pub fn ipsel(&self) -> IpselR {
        IpselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."]
    #[inline(always)]
    pub fn ipen(&self) -> IpenR {
        IpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
    #[inline(always)]
    pub fn imsel(&self) -> ImselR {
        ImselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."]
    #[inline(always)]
    pub fn imen(&self) -> ImenR {
        ImenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
    #[inline(always)]
    pub fn ipsel(&mut self) -> IpselW<'_, Comp0Ctl0Spec> {
        IpselW::new(self, 0)
    }
    #[doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."]
    #[inline(always)]
    pub fn ipen(&mut self) -> IpenW<'_, Comp0Ctl0Spec> {
        IpenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
    #[inline(always)]
    pub fn imsel(&mut self) -> ImselW<'_, Comp0Ctl0Spec> {
        ImselW::new(self, 16)
    }
    #[doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."]
    #[inline(always)]
    pub fn imen(&mut self) -> ImenW<'_, Comp0Ctl0Spec> {
        ImenW::new(self, 31)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Ctl0Spec;
impl crate::RegisterSpec for Comp0Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_ctl0::R`](R) reader structure"]
impl crate::Readable for Comp0Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0_ctl0::W`](W) writer structure"]
impl crate::Writable for Comp0Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CTL0 to value 0"]
impl crate::Resettable for Comp0Ctl0Spec {}
