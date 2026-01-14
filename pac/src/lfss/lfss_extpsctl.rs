#[doc = "Register `LFSS_EXTPSCTL` reader"]
pub type R = crate::R<LfssExtpsctlSpec>;
#[doc = "Register `LFSS_EXTPSCTL` writer"]
pub type W = crate::W<LfssExtpsctlSpec>;
#[doc = "Prescale timer 2 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt2ps {
    #[doc = "0: Interval every 4 second"]
    Int4s = 0,
    #[doc = "1: Interval every 8 second"]
    Int8s = 1,
    #[doc = "2: Interval every 16 second"]
    Int16s = 2,
}
impl From<Rt2ps> for u8 {
    #[inline(always)]
    fn from(variant: Rt2ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt2ps {
    type Ux = u8;
}
impl crate::IsEnum for Rt2ps {}
#[doc = "Field `RT2PS` reader - Prescale timer 2 interrupt interval"]
pub type Rt2psR = crate::FieldReader<Rt2ps>;
impl Rt2psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rt2ps> {
        match self.bits {
            0 => Some(Rt2ps::Int4s),
            1 => Some(Rt2ps::Int8s),
            2 => Some(Rt2ps::Int16s),
            _ => None,
        }
    }
    #[doc = "Interval every 4 second"]
    #[inline(always)]
    pub fn is_int4s(&self) -> bool {
        *self == Rt2ps::Int4s
    }
    #[doc = "Interval every 8 second"]
    #[inline(always)]
    pub fn is_int8s(&self) -> bool {
        *self == Rt2ps::Int8s
    }
    #[doc = "Interval every 16 second"]
    #[inline(always)]
    pub fn is_int16s(&self) -> bool {
        *self == Rt2ps::Int16s
    }
}
#[doc = "Field `RT2PS` writer - Prescale timer 2 interrupt interval"]
pub type Rt2psW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rt2ps>;
impl<'a, REG> Rt2psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interval every 4 second"]
    #[inline(always)]
    pub fn int4s(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Int4s)
    }
    #[doc = "Interval every 8 second"]
    #[inline(always)]
    pub fn int8s(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Int8s)
    }
    #[doc = "Interval every 16 second"]
    #[inline(always)]
    pub fn int16s(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Int16s)
    }
}
impl R {
    #[doc = "Bits 2:3 - Prescale timer 2 interrupt interval"]
    #[inline(always)]
    pub fn rt2ps(&self) -> Rt2psR {
        Rt2psR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Prescale timer 2 interrupt interval"]
    #[inline(always)]
    pub fn rt2ps(&mut self) -> Rt2psW<'_, LfssExtpsctlSpec> {
        Rt2psW::new(self, 2)
    }
}
#[doc = "RTC Prescale Timer 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_extpsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_extpsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssExtpsctlSpec;
impl crate::RegisterSpec for LfssExtpsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_extpsctl::R`](R) reader structure"]
impl crate::Readable for LfssExtpsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_extpsctl::W`](W) writer structure"]
impl crate::Writable for LfssExtpsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_EXTPSCTL to value 0"]
impl crate::Resettable for LfssExtpsctlSpec {}
