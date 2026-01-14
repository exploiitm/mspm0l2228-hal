#[doc = "Register `LCD_ISET` writer"]
pub type W = crate::W<LcdIsetSpec>;
#[doc = "Set FRMSTART RIS flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmstart {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set corresponding RIS flag"]
    Set = 1,
}
impl From<Frmstart> for bool {
    #[inline(always)]
    fn from(variant: Frmstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMSTART` writer - Set FRMSTART RIS flag."]
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
    #[doc = "Set corresponding RIS flag"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Frmstart::Set)
    }
}
#[doc = "Set BLKOFF RIS flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkoff {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set corresponding RIS flag"]
    Set = 1,
}
impl From<Blkoff> for bool {
    #[inline(always)]
    fn from(variant: Blkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKOFF` writer - Set BLKOFF RIS flag"]
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
    #[doc = "Set corresponding RIS flag"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Blkoff::Set)
    }
}
#[doc = "Set BLKON RIS flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkon {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Set corresponding RIS flag"]
    Set = 1,
}
impl From<Blkon> for bool {
    #[inline(always)]
    fn from(variant: Blkon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKON` writer - Set BLKON RIS flag"]
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
    #[doc = "Set corresponding RIS flag"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Blkon::Set)
    }
}
impl W {
    #[doc = "Bit 0 - Set FRMSTART RIS flag."]
    #[inline(always)]
    pub fn frmstart(&mut self) -> FrmstartW<'_, LcdIsetSpec> {
        FrmstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Set BLKOFF RIS flag"]
    #[inline(always)]
    pub fn blkoff(&mut self) -> BlkoffW<'_, LcdIsetSpec> {
        BlkoffW::new(self, 1)
    }
    #[doc = "Bit 2 - Set BLKON RIS flag"]
    #[inline(always)]
    pub fn blkon(&mut self) -> BlkonW<'_, LcdIsetSpec> {
        BlkonW::new(self, 2)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdIsetSpec;
impl crate::RegisterSpec for LcdIsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lcd_iset::W`](W) writer structure"]
impl crate::Writable for LcdIsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_ISET to value 0"]
impl crate::Resettable for LcdIsetSpec {}
