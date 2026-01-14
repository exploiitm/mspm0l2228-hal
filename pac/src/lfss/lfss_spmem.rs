#[doc = "Register `LFSS_SPMEM[%s]` reader"]
pub type R = crate::R<LfssSpmemSpec>;
#[doc = "Register `LFSS_SPMEM[%s]` writer"]
pub type W = crate::W<LfssSpmemSpec>;
#[doc = "Field `DATA0` reader - memory data byte 0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - memory data byte 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - memory data byte 1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - memory data byte 1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - memory data byte 2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - memory data byte 2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - memory data byte 3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - memory data byte 3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - memory data byte 0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - memory data byte 1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - memory data byte 2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - memory data byte 3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - memory data byte 0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, LfssSpmemSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - memory data byte 1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, LfssSpmemSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - memory data byte 2"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, LfssSpmemSpec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - memory data byte 3"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, LfssSpmemSpec> {
        Data3W::new(self, 24)
    }
}
#[doc = "Scratch Pad Memory Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_spmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_spmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssSpmemSpec;
impl crate::RegisterSpec for LfssSpmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_spmem::R`](R) reader structure"]
impl crate::Readable for LfssSpmemSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_spmem::W`](W) writer structure"]
impl crate::Writable for LfssSpmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_SPMEM[%s] to value 0"]
impl crate::Resettable for LfssSpmemSpec {}
