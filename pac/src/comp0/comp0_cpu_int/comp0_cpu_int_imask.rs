#[doc = "Register `COMP0_CPU_INT_IMASK` reader"]
pub type R = crate::R<Comp0CpuIntImaskSpec>;
#[doc = "Register `COMP0_CPU_INT_IMASK` writer"]
pub type W = crate::W<Comp0CpuIntImaskSpec>;
#[doc = "Masks COMPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Compifg> for bool {
    #[inline(always)]
    fn from(variant: Compifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPIFG` reader - Masks COMPIFG"]
pub type CompifgR = crate::BitReader<Compifg>;
impl CompifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compifg {
        match self.bits {
            false => Compifg::Clr,
            true => Compifg::Set,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compifg::Set
    }
}
#[doc = "Field `COMPIFG` writer - Masks COMPIFG"]
pub type CompifgW<'a, REG> = crate::BitWriter<'a, REG, Compifg>;
impl<'a, REG> CompifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Compifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compifg::Set)
    }
}
#[doc = "Masks COMPINVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compinvifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Compinvifg> for bool {
    #[inline(always)]
    fn from(variant: Compinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPINVIFG` reader - Masks COMPINVIFG"]
pub type CompinvifgR = crate::BitReader<Compinvifg>;
impl CompinvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compinvifg {
        match self.bits {
            false => Compinvifg::Clr,
            true => Compinvifg::Set,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Compinvifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Compinvifg::Set
    }
}
#[doc = "Field `COMPINVIFG` writer - Masks COMPINVIFG"]
pub type CompinvifgW<'a, REG> = crate::BitWriter<'a, REG, Compinvifg>;
impl<'a, REG> CompinvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Compinvifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compinvifg::Set)
    }
}
#[doc = "Masks OUTRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outrdyifg {
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
}
impl From<Outrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Outrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTRDYIFG` reader - Masks OUTRDYIFG"]
pub type OutrdyifgR = crate::BitReader<Outrdyifg>;
impl OutrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outrdyifg {
        match self.bits {
            false => Outrdyifg::Clr,
            true => Outrdyifg::Set,
        }
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Outrdyifg::Clr
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Outrdyifg::Set
    }
}
#[doc = "Field `OUTRDYIFG` writer - Masks OUTRDYIFG"]
pub type OutrdyifgW<'a, REG> = crate::BitWriter<'a, REG, Outrdyifg>;
impl<'a, REG> OutrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Outrdyifg::Clr)
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Outrdyifg::Set)
    }
}
impl R {
    #[doc = "Bit 1 - Masks COMPIFG"]
    #[inline(always)]
    pub fn compifg(&self) -> CompifgR {
        CompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks COMPINVIFG"]
    #[inline(always)]
    pub fn compinvifg(&self) -> CompinvifgR {
        CompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks OUTRDYIFG"]
    #[inline(always)]
    pub fn outrdyifg(&self) -> OutrdyifgR {
        OutrdyifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Masks COMPIFG"]
    #[inline(always)]
    pub fn compifg(&mut self) -> CompifgW<'_, Comp0CpuIntImaskSpec> {
        CompifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks COMPINVIFG"]
    #[inline(always)]
    pub fn compinvifg(&mut self) -> CompinvifgW<'_, Comp0CpuIntImaskSpec> {
        CompinvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks OUTRDYIFG"]
    #[inline(always)]
    pub fn outrdyifg(&mut self) -> OutrdyifgW<'_, Comp0CpuIntImaskSpec> {
        OutrdyifgW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0_cpu_int_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_cpu_int_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0CpuIntImaskSpec;
impl crate::RegisterSpec for Comp0CpuIntImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0_cpu_int_imask::R`](R) reader structure"]
impl crate::Readable for Comp0CpuIntImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`comp0_cpu_int_imask::W`](W) writer structure"]
impl crate::Writable for Comp0CpuIntImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0_CPU_INT_IMASK to value 0"]
impl crate::Resettable for Comp0CpuIntImaskSpec {}
