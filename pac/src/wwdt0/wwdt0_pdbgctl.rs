#[doc = "Register `WWDT0_PDBGCTL` reader"]
pub type R = crate::R<Wwdt0PdbgctlSpec>;
#[doc = "Register `WWDT0_PDBGCTL` writer"]
pub type W = crate::W<Wwdt0PdbgctlSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Free {
    #[doc = "0: The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    Stop = 0,
    #[doc = "1: The peripheral ignores the state of the Core Halted input"]
    Run = 1,
}
impl From<Free> for bool {
    #[inline(always)]
    fn from(variant: Free) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREE` reader - Free run control"]
pub type FreeR = crate::BitReader<Free>;
impl FreeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Free {
        match self.bits {
            false => Free::Stop,
            true => Free::Run,
        }
    }
    #[doc = "The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Free::Stop
    }
    #[doc = "The peripheral ignores the state of the Core Halted input"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Free::Run
    }
}
#[doc = "Field `FREE` writer - Free run control"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG, Free>;
impl<'a, REG> FreeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral freezes functionality while the Core Halted input is asserted and resumes when it is deasserted."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Stop)
    }
    #[doc = "The peripheral ignores the state of the Core Halted input"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Free::Run)
    }
}
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn free(&mut self) -> FreeW<'_, Wwdt0PdbgctlSpec> {
        FreeW::new(self, 0)
    }
}
#[doc = "Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_pdbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_pdbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0PdbgctlSpec;
impl crate::RegisterSpec for Wwdt0PdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_pdbgctl::R`](R) reader structure"]
impl crate::Readable for Wwdt0PdbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdt0_pdbgctl::W`](W) writer structure"]
impl crate::Writable for Wwdt0PdbgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WWDT0_PDBGCTL to value 0"]
impl crate::Resettable for Wwdt0PdbgctlSpec {}
