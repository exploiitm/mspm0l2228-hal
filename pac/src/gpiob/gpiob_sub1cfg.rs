#[doc = "Register `GPIOB_SUB1CFG` reader"]
pub type R = crate::R<GpiobSub1cfgSpec>;
#[doc = "Register `GPIOB_SUB1CFG` writer"]
pub type W = crate::W<GpiobSub1cfgSpec>;
#[doc = "This bit is used to enable subscriber 1 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Subscriber 1 event is disabled"]
    Clr = 0,
    #[doc = "1: Subscriber 1 event is enabled"]
    Set = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - This bit is used to enable subscriber 1 event."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Clr,
            true => Enable::Set,
        }
    }
    #[doc = "Subscriber 1 event is disabled"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Enable::Clr
    }
    #[doc = "Subscriber 1 event is enabled"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enable::Set
    }
}
#[doc = "Field `ENABLE` writer - This bit is used to enable subscriber 1 event."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Subscriber 1 event is disabled"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Clr)
    }
    #[doc = "Subscriber 1 event is enabled"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Set)
    }
}
#[doc = "These bits configure the output policy for subscriber 1 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outpolicy {
    #[doc = "0: Selected DIO pins are set"]
    Set = 0,
    #[doc = "1: Selected DIO pins are cleared"]
    Clr = 1,
    #[doc = "2: Selected DIO pins are toggled"]
    Toggle = 2,
}
impl From<Outpolicy> for u8 {
    #[inline(always)]
    fn from(variant: Outpolicy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outpolicy {
    type Ux = u8;
}
impl crate::IsEnum for Outpolicy {}
#[doc = "Field `OUTPOLICY` reader - These bits configure the output policy for subscriber 1 event."]
pub type OutpolicyR = crate::FieldReader<Outpolicy>;
impl OutpolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outpolicy> {
        match self.bits {
            0 => Some(Outpolicy::Set),
            1 => Some(Outpolicy::Clr),
            2 => Some(Outpolicy::Toggle),
            _ => None,
        }
    }
    #[doc = "Selected DIO pins are set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Outpolicy::Set
    }
    #[doc = "Selected DIO pins are cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Outpolicy::Clr
    }
    #[doc = "Selected DIO pins are toggled"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Outpolicy::Toggle
    }
}
#[doc = "Field `OUTPOLICY` writer - These bits configure the output policy for subscriber 1 event."]
pub type OutpolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Outpolicy>;
impl<'a, REG> OutpolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selected DIO pins are set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::Set)
    }
    #[doc = "Selected DIO pins are cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::Clr)
    }
    #[doc = "Selected DIO pins are toggled"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Outpolicy::Toggle)
    }
}
#[doc = "indicates the specific bit in the upper 16 bits that is targeted by the subscriber action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Index {
    #[doc = "0: specific bit targeted by the subscriber action is bit16"]
    Min = 0,
    #[doc = "15: specific bit targeted by the subscriber action is bit31"]
    Max = 15,
}
impl From<Index> for u8 {
    #[inline(always)]
    fn from(variant: Index) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Index {
    type Ux = u8;
}
impl crate::IsEnum for Index {}
#[doc = "Field `INDEX` reader - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
pub type IndexR = crate::FieldReader<Index>;
impl IndexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Index> {
        match self.bits {
            0 => Some(Index::Min),
            15 => Some(Index::Max),
            _ => None,
        }
    }
    #[doc = "specific bit targeted by the subscriber action is bit16"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Index::Min
    }
    #[doc = "specific bit targeted by the subscriber action is bit31"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Index::Max
    }
}
#[doc = "Field `INDEX` writer - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 4, Index>;
impl<'a, REG> IndexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "specific bit targeted by the subscriber action is bit16"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Index::Min)
    }
    #[doc = "specific bit targeted by the subscriber action is bit31"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Index::Max)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable subscriber 1 event."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 1 event."]
    #[inline(always)]
    pub fn outpolicy(&self) -> OutpolicyR {
        OutpolicyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable subscriber 1 event."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, GpiobSub1cfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 1 event."]
    #[inline(always)]
    pub fn outpolicy(&mut self) -> OutpolicyW<'_, GpiobSub1cfgSpec> {
        OutpolicyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, GpiobSub1cfgSpec> {
        IndexW::new(self, 16)
    }
}
#[doc = "Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob_sub1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_sub1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobSub1cfgSpec;
impl crate::RegisterSpec for GpiobSub1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_sub1cfg::R`](R) reader structure"]
impl crate::Readable for GpiobSub1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiob_sub1cfg::W`](W) writer structure"]
impl crate::Writable for GpiobSub1cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB_SUB1CFG to value 0"]
impl crate::Resettable for GpiobSub1cfgSpec {}
