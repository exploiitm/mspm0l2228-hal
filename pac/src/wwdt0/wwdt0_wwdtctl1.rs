#[doc = "Register `WWDT0_WWDTCTL1` reader"]
pub type R = crate::R<Wwdt0Wwdtctl1Spec>;
#[doc = "Register `WWDT0_WWDTCTL1` writer"]
pub type W = crate::W<Wwdt0Wwdtctl1Spec>;
#[doc = "Close Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winsel {
    #[doc = "0: In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    Win0 = 0,
    #[doc = "1: In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    Win1 = 1,
}
impl From<Winsel> for bool {
    #[inline(always)]
    fn from(variant: Winsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINSEL` reader - Close Window Select"]
pub type WinselR = crate::BitReader<Winsel>;
impl WinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winsel {
        match self.bits {
            false => Winsel::Win0,
            true => Winsel::Win1,
        }
    }
    #[doc = "In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn is_win0(&self) -> bool {
        *self == Winsel::Win0
    }
    #[doc = "In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn is_win1(&self) -> bool {
        *self == Winsel::Win1
    }
}
#[doc = "Field `WINSEL` writer - Close Window Select"]
pub type WinselW<'a, REG> = crate::BitWriter<'a, REG, Winsel>;
impl<'a, REG> WinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In window mode field WINDOW0 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn win0(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Win0)
    }
    #[doc = "In window mode field WINDOW1 of WDDTCTL0 defines the closed window size."]
    #[inline(always)]
    pub fn win1(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Win1)
    }
}
impl R {
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WinselR {
        WinselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Close Window Select"]
    #[inline(always)]
    pub fn winsel(&mut self) -> WinselW<'_, Wwdt0Wwdtctl1Spec> {
        WinselW::new(self, 0)
    }
}
#[doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdt0_wwdtctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdt0_wwdtctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdt0Wwdtctl1Spec;
impl crate::RegisterSpec for Wwdt0Wwdtctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdt0_wwdtctl1::R`](R) reader structure"]
impl crate::Readable for Wwdt0Wwdtctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`wwdt0_wwdtctl1::W`](W) writer structure"]
impl crate::Writable for Wwdt0Wwdtctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WWDT0_WWDTCTL1 to value 0"]
impl crate::Resettable for Wwdt0Wwdtctl1Spec {}
