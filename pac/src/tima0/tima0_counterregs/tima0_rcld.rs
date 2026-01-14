#[doc = "Register `TIMA0_RCLD` reader"]
pub type R = crate::R<Tima0RcldSpec>;
#[doc = "Register `TIMA0_RCLD` writer"]
pub type W = crate::W<Tima0RcldSpec>;
#[doc = "Field `RCLD` reader - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
pub type RcldR = crate::FieldReader;
#[doc = "Field `RCLD` writer - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
pub type RcldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
    #[inline(always)]
    pub fn rcld(&self) -> RcldR {
        RcldR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter Load Value This field provides the value loaded into the repeat counter at a load event following the repeat counter value equaling 0."]
    #[inline(always)]
    pub fn rcld(&mut self) -> RcldW<'_, Tima0RcldSpec> {
        RcldW::new(self, 0)
    }
}
#[doc = "Repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_rcld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_rcld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0RcldSpec;
impl crate::RegisterSpec for Tima0RcldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_rcld::R`](R) reader structure"]
impl crate::Readable for Tima0RcldSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_rcld::W`](W) writer structure"]
impl crate::Writable for Tima0RcldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_RCLD to value 0"]
impl crate::Resettable for Tima0RcldSpec {}
