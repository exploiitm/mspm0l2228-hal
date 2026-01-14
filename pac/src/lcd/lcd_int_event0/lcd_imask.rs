#[doc = "Register `LCD_IMASK` reader"]
pub type R = crate::R<LcdImaskSpec>;
#[doc = "Register `LCD_IMASK` writer"]
pub type W = crate::W<LcdImaskSpec>;
#[doc = "Start of new LCD frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmstart {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Frmstart> for bool {
    #[inline(always)]
    fn from(variant: Frmstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMSTART` reader - Start of new LCD frame."]
pub type FrmstartR = crate::BitReader<Frmstart>;
impl FrmstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmstart {
        match self.bits {
            false => Frmstart::Clr,
            true => Frmstart::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Frmstart::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Frmstart::Set
    }
}
#[doc = "Field `FRMSTART` writer - Start of new LCD frame."]
pub type FrmstartW<'a, REG> = crate::BitWriter<'a, REG, Frmstart>;
impl<'a, REG> FrmstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Frmstart::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Frmstart::Set)
    }
}
#[doc = "Blinking segments off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkoff {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Blkoff> for bool {
    #[inline(always)]
    fn from(variant: Blkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKOFF` reader - Blinking segments off."]
pub type BlkoffR = crate::BitReader<Blkoff>;
impl BlkoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blkoff {
        match self.bits {
            false => Blkoff::Clr,
            true => Blkoff::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Blkoff::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Blkoff::Set
    }
}
#[doc = "Field `BLKOFF` writer - Blinking segments off."]
pub type BlkoffW<'a, REG> = crate::BitWriter<'a, REG, Blkoff>;
impl<'a, REG> BlkoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Blkoff::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Blkoff::Set)
    }
}
#[doc = "Blinkking segments on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkon {
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
}
impl From<Blkon> for bool {
    #[inline(always)]
    fn from(variant: Blkon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKON` reader - Blinkking segments on."]
pub type BlkonR = crate::BitReader<Blkon>;
impl BlkonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blkon {
        match self.bits {
            false => Blkon::Clr,
            true => Blkon::Set,
        }
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Blkon::Clr
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Blkon::Set
    }
}
#[doc = "Field `BLKON` writer - Blinkking segments on."]
pub type BlkonW<'a, REG> = crate::BitWriter<'a, REG, Blkon>;
impl<'a, REG> BlkonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Blkon::Clr)
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Blkon::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Start of new LCD frame."]
    #[inline(always)]
    pub fn frmstart(&self) -> FrmstartR {
        FrmstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blinking segments off."]
    #[inline(always)]
    pub fn blkoff(&self) -> BlkoffR {
        BlkoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Blinkking segments on."]
    #[inline(always)]
    pub fn blkon(&self) -> BlkonR {
        BlkonR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of new LCD frame."]
    #[inline(always)]
    pub fn frmstart(&mut self) -> FrmstartW<'_, LcdImaskSpec> {
        FrmstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Blinking segments off."]
    #[inline(always)]
    pub fn blkoff(&mut self) -> BlkoffW<'_, LcdImaskSpec> {
        BlkoffW::new(self, 1)
    }
    #[doc = "Bit 2 - Blinkking segments on."]
    #[inline(always)]
    pub fn blkon(&mut self) -> BlkonW<'_, LcdImaskSpec> {
        BlkonW::new(self, 2)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdImaskSpec;
impl crate::RegisterSpec for LcdImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_imask::R`](R) reader structure"]
impl crate::Readable for LcdImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_imask::W`](W) writer structure"]
impl crate::Writable for LcdImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_IMASK to value 0"]
impl crate::Resettable for LcdImaskSpec {}
