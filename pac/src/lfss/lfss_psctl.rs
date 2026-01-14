#[doc = "Register `LFSS_PSCTL` reader"]
pub type R = crate::R<LfssPsctlSpec>;
#[doc = "Register `LFSS_PSCTL` writer"]
pub type W = crate::W<LfssPsctlSpec>;
#[doc = "Prescale timer 0 interrupt interval\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt0ip {
    #[doc = "2: Interval every 244 microsecond"]
    Int244us = 2,
    #[doc = "3: Interval every 488 microsecond"]
    Int488us = 3,
    #[doc = "4: Interval every 0.98 milisecond"]
    Int0p98ms = 4,
    #[doc = "5: Interval every 1.95 milisecond"]
    Int1p95ms = 5,
    #[doc = "6: Interval every 3.91 milisecond"]
    Int3p91ms = 6,
    #[doc = "7: Interval every 7.81 milisecond"]
    Int7p81ms = 7,
}
impl From<Rt0ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt0ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt0ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt0ip {}
#[doc = "Field `RT0IP` reader - Prescale timer 0 interrupt interval"]
pub type Rt0ipR = crate::FieldReader<Rt0ip>;
impl Rt0ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rt0ip> {
        match self.bits {
            2 => Some(Rt0ip::Int244us),
            3 => Some(Rt0ip::Int488us),
            4 => Some(Rt0ip::Int0p98ms),
            5 => Some(Rt0ip::Int1p95ms),
            6 => Some(Rt0ip::Int3p91ms),
            7 => Some(Rt0ip::Int7p81ms),
            _ => None,
        }
    }
    #[doc = "Interval every 244 microsecond"]
    #[inline(always)]
    pub fn is_int244us(&self) -> bool {
        *self == Rt0ip::Int244us
    }
    #[doc = "Interval every 488 microsecond"]
    #[inline(always)]
    pub fn is_int488us(&self) -> bool {
        *self == Rt0ip::Int488us
    }
    #[doc = "Interval every 0.98 milisecond"]
    #[inline(always)]
    pub fn is_int0p98ms(&self) -> bool {
        *self == Rt0ip::Int0p98ms
    }
    #[doc = "Interval every 1.95 milisecond"]
    #[inline(always)]
    pub fn is_int1p95ms(&self) -> bool {
        *self == Rt0ip::Int1p95ms
    }
    #[doc = "Interval every 3.91 milisecond"]
    #[inline(always)]
    pub fn is_int3p91ms(&self) -> bool {
        *self == Rt0ip::Int3p91ms
    }
    #[doc = "Interval every 7.81 milisecond"]
    #[inline(always)]
    pub fn is_int7p81ms(&self) -> bool {
        *self == Rt0ip::Int7p81ms
    }
}
#[doc = "Field `RT0IP` writer - Prescale timer 0 interrupt interval"]
pub type Rt0ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt0ip>;
impl<'a, REG> Rt0ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interval every 244 microsecond"]
    #[inline(always)]
    pub fn int244us(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int244us)
    }
    #[doc = "Interval every 488 microsecond"]
    #[inline(always)]
    pub fn int488us(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int488us)
    }
    #[doc = "Interval every 0.98 milisecond"]
    #[inline(always)]
    pub fn int0p98ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int0p98ms)
    }
    #[doc = "Interval every 1.95 milisecond"]
    #[inline(always)]
    pub fn int1p95ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int1p95ms)
    }
    #[doc = "Interval every 3.91 milisecond"]
    #[inline(always)]
    pub fn int3p91ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int3p91ms)
    }
    #[doc = "Interval every 7.81 milisecond"]
    #[inline(always)]
    pub fn int7p81ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Int7p81ms)
    }
}
#[doc = "Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt1ip {
    #[doc = "0: Interval every 15.6 milisecond"]
    Int15p6ms = 0,
    #[doc = "1: Interval every 31.3 milisecond"]
    Int31p3ms = 1,
    #[doc = "2: Interval every 62.5 milisecond"]
    Int62p5ms = 2,
    #[doc = "3: Interval every 125 milisecond"]
    Int125ms = 3,
    #[doc = "4: Interval every 250 milisecond"]
    Int250ms = 4,
    #[doc = "5: Interval every 500 milisecond"]
    Int500ms = 5,
    #[doc = "6: Interval every 1 second"]
    Int1s = 6,
    #[doc = "7: Interval every 2 second"]
    Int2s = 7,
}
impl From<Rt1ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt1ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt1ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt1ip {}
#[doc = "Field `RT1IP` reader - Prescale timer 1 interrupt interval"]
pub type Rt1ipR = crate::FieldReader<Rt1ip>;
impl Rt1ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1ip {
        match self.bits {
            0 => Rt1ip::Int15p6ms,
            1 => Rt1ip::Int31p3ms,
            2 => Rt1ip::Int62p5ms,
            3 => Rt1ip::Int125ms,
            4 => Rt1ip::Int250ms,
            5 => Rt1ip::Int500ms,
            6 => Rt1ip::Int1s,
            7 => Rt1ip::Int2s,
            _ => unreachable!(),
        }
    }
    #[doc = "Interval every 15.6 milisecond"]
    #[inline(always)]
    pub fn is_int15p6ms(&self) -> bool {
        *self == Rt1ip::Int15p6ms
    }
    #[doc = "Interval every 31.3 milisecond"]
    #[inline(always)]
    pub fn is_int31p3ms(&self) -> bool {
        *self == Rt1ip::Int31p3ms
    }
    #[doc = "Interval every 62.5 milisecond"]
    #[inline(always)]
    pub fn is_int62p5ms(&self) -> bool {
        *self == Rt1ip::Int62p5ms
    }
    #[doc = "Interval every 125 milisecond"]
    #[inline(always)]
    pub fn is_int125ms(&self) -> bool {
        *self == Rt1ip::Int125ms
    }
    #[doc = "Interval every 250 milisecond"]
    #[inline(always)]
    pub fn is_int250ms(&self) -> bool {
        *self == Rt1ip::Int250ms
    }
    #[doc = "Interval every 500 milisecond"]
    #[inline(always)]
    pub fn is_int500ms(&self) -> bool {
        *self == Rt1ip::Int500ms
    }
    #[doc = "Interval every 1 second"]
    #[inline(always)]
    pub fn is_int1s(&self) -> bool {
        *self == Rt1ip::Int1s
    }
    #[doc = "Interval every 2 second"]
    #[inline(always)]
    pub fn is_int2s(&self) -> bool {
        *self == Rt1ip::Int2s
    }
}
#[doc = "Field `RT1IP` writer - Prescale timer 1 interrupt interval"]
pub type Rt1ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt1ip, crate::Safe>;
impl<'a, REG> Rt1ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interval every 15.6 milisecond"]
    #[inline(always)]
    pub fn int15p6ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int15p6ms)
    }
    #[doc = "Interval every 31.3 milisecond"]
    #[inline(always)]
    pub fn int31p3ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int31p3ms)
    }
    #[doc = "Interval every 62.5 milisecond"]
    #[inline(always)]
    pub fn int62p5ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int62p5ms)
    }
    #[doc = "Interval every 125 milisecond"]
    #[inline(always)]
    pub fn int125ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int125ms)
    }
    #[doc = "Interval every 250 milisecond"]
    #[inline(always)]
    pub fn int250ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int250ms)
    }
    #[doc = "Interval every 500 milisecond"]
    #[inline(always)]
    pub fn int500ms(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int500ms)
    }
    #[doc = "Interval every 1 second"]
    #[inline(always)]
    pub fn int1s(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int1s)
    }
    #[doc = "Interval every 2 second"]
    #[inline(always)]
    pub fn int2s(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Int2s)
    }
}
impl R {
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&self) -> Rt0ipR {
        Rt0ipR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&self) -> Rt1ipR {
        Rt1ipR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> Rt0ipW<'_, LfssPsctlSpec> {
        Rt0ipW::new(self, 2)
    }
    #[doc = "Bits 18:20 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> Rt1ipW<'_, LfssPsctlSpec> {
        Rt1ipW::new(self, 18)
    }
}
#[doc = "RTC Prescale Timer 0/1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_psctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_psctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssPsctlSpec;
impl crate::RegisterSpec for LfssPsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_psctl::R`](R) reader structure"]
impl crate::Readable for LfssPsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_psctl::W`](W) writer structure"]
impl crate::Writable for LfssPsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_PSCTL to value 0x08"]
impl crate::Resettable for LfssPsctlSpec {
    const RESET_VALUE: u32 = 0x08;
}
