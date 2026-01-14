#[doc = "Register `TIMA0_RC` reader"]
pub type R = crate::R<Tima0RcSpec>;
#[doc = "Field `RC` reader - Repeat Counter Value"]
pub type RcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter Value"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_rc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0RcSpec;
impl crate::RegisterSpec for Tima0RcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_rc::R`](R) reader structure"]
impl crate::Readable for Tima0RcSpec {}
#[doc = "`reset()` method sets TIMA0_RC to value 0"]
impl crate::Resettable for Tima0RcSpec {}
