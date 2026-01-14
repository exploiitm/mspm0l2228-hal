#[doc = "Register `LCD_ICLR` writer"]
pub type W = crate::W<LcdIclrSpec>;
#[doc = "Clear FRMSTART RIS flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmstart {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear corresponding RIS flag"]
    Clr = 1,
}
impl From<Frmstart> for bool {
    #[inline(always)]
    fn from(variant: Frmstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMSTART` writer - Clear FRMSTART RIS flag"]
pub type FrmstartW<'a, REG> = crate::BitWriter<'a, REG, Frmstart>;
impl<'a, REG> FrmstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Frmstart::NoEffect)
    }
    #[doc = "Clear corresponding RIS flag"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Frmstart::Clr)
    }
}
#[doc = "Clear BLKOFF RIS flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkoff {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear corresponding RIS flag"]
    Clr = 1,
}
impl From<Blkoff> for bool {
    #[inline(always)]
    fn from(variant: Blkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKOFF` writer - Clear BLKOFF RIS flag"]
pub type BlkoffW<'a, REG> = crate::BitWriter<'a, REG, Blkoff>;
impl<'a, REG> BlkoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Blkoff::NoEffect)
    }
    #[doc = "Clear corresponding RIS flag"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Blkoff::Clr)
    }
}
#[doc = "Clear BLKON RIS flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkon {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear corresponding RIS flag"]
    Clr = 1,
}
impl From<Blkon> for bool {
    #[inline(always)]
    fn from(variant: Blkon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKON` writer - Clear BLKON RIS flag"]
pub type BlkonW<'a, REG> = crate::BitWriter<'a, REG, Blkon>;
impl<'a, REG> BlkonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Blkon::NoEffect)
    }
    #[doc = "Clear corresponding RIS flag"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Blkon::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear FRMSTART RIS flag"]
    #[inline(always)]
    pub fn frmstart(&mut self) -> FrmstartW<'_, LcdIclrSpec> {
        FrmstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear BLKOFF RIS flag"]
    #[inline(always)]
    pub fn blkoff(&mut self) -> BlkoffW<'_, LcdIclrSpec> {
        BlkoffW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear BLKON RIS flag"]
    #[inline(always)]
    pub fn blkon(&mut self) -> BlkonW<'_, LcdIclrSpec> {
        BlkonW::new(self, 2)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdIclrSpec;
impl crate::RegisterSpec for LcdIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lcd_iclr::W`](W) writer structure"]
impl crate::Writable for LcdIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_ICLR to value 0"]
impl crate::Resettable for LcdIclrSpec {}
