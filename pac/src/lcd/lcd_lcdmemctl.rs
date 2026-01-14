#[doc = "Register `LCD_LCDMEMCTL` reader"]
pub type R = crate::R<LcdLcdmemctlSpec>;
#[doc = "Register `LCD_LCDMEMCTL` writer"]
pub type W = crate::W<LcdLcdmemctlSpec>;
#[doc = "Select LCD memory registers for display When LCDBLKMODx = 00, LCDDISP can be set by software. The bit is cleared in LCDBLKMODx = 01 and LCDBLKMODx = 10 or if a mux mode =5 is selected and cannot be changed by software. When LCDBLKMODx = 11, this bit reflects the currently displayed memory but cannot be changed by software. When returning to LCDBLKMODx = 00 the bit is cleared. 0b = Display content of LCD memory registers LCDMx 1b = Display content of LCD blinking memory registers LCDBMx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcddisp {
    #[doc = "0: Display content of LCD memory registers LCDMx"]
    SelLcdMemRegs = 0,
    #[doc = "1: Display content of LCD blinking memory registers LCDBMx"]
    SelBlnkMemRegs = 1,
}
impl From<Lcddisp> for bool {
    #[inline(always)]
    fn from(variant: Lcddisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDDISP` reader - Select LCD memory registers for display When LCDBLKMODx = 00, LCDDISP can be set by software. The bit is cleared in LCDBLKMODx = 01 and LCDBLKMODx = 10 or if a mux mode =5 is selected and cannot be changed by software. When LCDBLKMODx = 11, this bit reflects the currently displayed memory but cannot be changed by software. When returning to LCDBLKMODx = 00 the bit is cleared. 0b = Display content of LCD memory registers LCDMx 1b = Display content of LCD blinking memory registers LCDBMx"]
pub type LcddispR = crate::BitReader<Lcddisp>;
impl LcddispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddisp {
        match self.bits {
            false => Lcddisp::SelLcdMemRegs,
            true => Lcddisp::SelBlnkMemRegs,
        }
    }
    #[doc = "Display content of LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn is_sel_lcd_mem_regs(&self) -> bool {
        *self == Lcddisp::SelLcdMemRegs
    }
    #[doc = "Display content of LCD blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn is_sel_blnk_mem_regs(&self) -> bool {
        *self == Lcddisp::SelBlnkMemRegs
    }
}
#[doc = "Field `LCDDISP` writer - Select LCD memory registers for display When LCDBLKMODx = 00, LCDDISP can be set by software. The bit is cleared in LCDBLKMODx = 01 and LCDBLKMODx = 10 or if a mux mode =5 is selected and cannot be changed by software. When LCDBLKMODx = 11, this bit reflects the currently displayed memory but cannot be changed by software. When returning to LCDBLKMODx = 00 the bit is cleared. 0b = Display content of LCD memory registers LCDMx 1b = Display content of LCD blinking memory registers LCDBMx"]
pub type LcddispW<'a, REG> = crate::BitWriter<'a, REG, Lcddisp>;
impl<'a, REG> LcddispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Display content of LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn sel_lcd_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddisp::SelLcdMemRegs)
    }
    #[doc = "Display content of LCD blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn sel_blnk_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddisp::SelBlnkMemRegs)
    }
}
#[doc = "Clear LCD memory Clears all LCD memory registers LCDMx. The bit is automatically reset when the LCD memory is cleared. 0b = Contents of LCD memory registers LCDMx remain unchanged 1b = Clear content of all LCD memory registers LCDMx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdclrm {
    #[doc = "0: Contents of LCD memory registers LCDMx remain unchanged"]
    NoClrLcdMemRegs = 0,
    #[doc = "1: Clear content of all LCD memory registers LCDMx"]
    ClrLcdMemRegs = 1,
}
impl From<Lcdclrm> for bool {
    #[inline(always)]
    fn from(variant: Lcdclrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCLRM` reader - Clear LCD memory Clears all LCD memory registers LCDMx. The bit is automatically reset when the LCD memory is cleared. 0b = Contents of LCD memory registers LCDMx remain unchanged 1b = Clear content of all LCD memory registers LCDMx"]
pub type LcdclrmR = crate::BitReader<Lcdclrm>;
impl LcdclrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdclrm {
        match self.bits {
            false => Lcdclrm::NoClrLcdMemRegs,
            true => Lcdclrm::ClrLcdMemRegs,
        }
    }
    #[doc = "Contents of LCD memory registers LCDMx remain unchanged"]
    #[inline(always)]
    pub fn is_no_clr_lcd_mem_regs(&self) -> bool {
        *self == Lcdclrm::NoClrLcdMemRegs
    }
    #[doc = "Clear content of all LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn is_clr_lcd_mem_regs(&self) -> bool {
        *self == Lcdclrm::ClrLcdMemRegs
    }
}
#[doc = "Field `LCDCLRM` writer - Clear LCD memory Clears all LCD memory registers LCDMx. The bit is automatically reset when the LCD memory is cleared. 0b = Contents of LCD memory registers LCDMx remain unchanged 1b = Clear content of all LCD memory registers LCDMx"]
pub type LcdclrmW<'a, REG> = crate::BitWriter<'a, REG, Lcdclrm>;
impl<'a, REG> LcdclrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Contents of LCD memory registers LCDMx remain unchanged"]
    #[inline(always)]
    pub fn no_clr_lcd_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrm::NoClrLcdMemRegs)
    }
    #[doc = "Clear content of all LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn clr_lcd_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrm::ClrLcdMemRegs)
    }
}
#[doc = "Clear LCD blinking memory Clears all blinking memory registers LCDBMx. The bit is automatically reset when the blinking memory is cleared. Setting this bit in 5-mux mode and above has no effect. It is immediately reset again. 0b = Contents of blinking memory registers LCDBMx remain unchanged 1b = Clear content of all blinking memory registers LCDBMx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdclrbm {
    #[doc = "0: Contents of blinking memory registers LCDBMx remain unchanged"]
    NoClrBlnkMemRegs = 0,
    #[doc = "1: Clear content of all blinking memory registers LCDBMx"]
    ClrBlnkMemRegs = 1,
}
impl From<Lcdclrbm> for bool {
    #[inline(always)]
    fn from(variant: Lcdclrbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCLRBM` reader - Clear LCD blinking memory Clears all blinking memory registers LCDBMx. The bit is automatically reset when the blinking memory is cleared. Setting this bit in 5-mux mode and above has no effect. It is immediately reset again. 0b = Contents of blinking memory registers LCDBMx remain unchanged 1b = Clear content of all blinking memory registers LCDBMx"]
pub type LcdclrbmR = crate::BitReader<Lcdclrbm>;
impl LcdclrbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdclrbm {
        match self.bits {
            false => Lcdclrbm::NoClrBlnkMemRegs,
            true => Lcdclrbm::ClrBlnkMemRegs,
        }
    }
    #[doc = "Contents of blinking memory registers LCDBMx remain unchanged"]
    #[inline(always)]
    pub fn is_no_clr_blnk_mem_regs(&self) -> bool {
        *self == Lcdclrbm::NoClrBlnkMemRegs
    }
    #[doc = "Clear content of all blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn is_clr_blnk_mem_regs(&self) -> bool {
        *self == Lcdclrbm::ClrBlnkMemRegs
    }
}
#[doc = "Field `LCDCLRBM` writer - Clear LCD blinking memory Clears all blinking memory registers LCDBMx. The bit is automatically reset when the blinking memory is cleared. Setting this bit in 5-mux mode and above has no effect. It is immediately reset again. 0b = Contents of blinking memory registers LCDBMx remain unchanged 1b = Clear content of all blinking memory registers LCDBMx"]
pub type LcdclrbmW<'a, REG> = crate::BitWriter<'a, REG, Lcdclrbm>;
impl<'a, REG> LcdclrbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Contents of blinking memory registers LCDBMx remain unchanged"]
    #[inline(always)]
    pub fn no_clr_blnk_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrbm::NoClrBlnkMemRegs)
    }
    #[doc = "Clear content of all blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn clr_blnk_mem_regs(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrbm::ClrBlnkMemRegs)
    }
}
impl R {
    #[doc = "Bit 0 - Select LCD memory registers for display When LCDBLKMODx = 00, LCDDISP can be set by software. The bit is cleared in LCDBLKMODx = 01 and LCDBLKMODx = 10 or if a mux mode =5 is selected and cannot be changed by software. When LCDBLKMODx = 11, this bit reflects the currently displayed memory but cannot be changed by software. When returning to LCDBLKMODx = 00 the bit is cleared. 0b = Display content of LCD memory registers LCDMx 1b = Display content of LCD blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LcddispR {
        LcddispR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear LCD memory Clears all LCD memory registers LCDMx. The bit is automatically reset when the LCD memory is cleared. 0b = Contents of LCD memory registers LCDMx remain unchanged 1b = Clear content of all LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LcdclrmR {
        LcdclrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear LCD blinking memory Clears all blinking memory registers LCDBMx. The bit is automatically reset when the blinking memory is cleared. Setting this bit in 5-mux mode and above has no effect. It is immediately reset again. 0b = Contents of blinking memory registers LCDBMx remain unchanged 1b = Clear content of all blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LcdclrbmR {
        LcdclrbmR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select LCD memory registers for display When LCDBLKMODx = 00, LCDDISP can be set by software. The bit is cleared in LCDBLKMODx = 01 and LCDBLKMODx = 10 or if a mux mode =5 is selected and cannot be changed by software. When LCDBLKMODx = 11, this bit reflects the currently displayed memory but cannot be changed by software. When returning to LCDBLKMODx = 00 the bit is cleared. 0b = Display content of LCD memory registers LCDMx 1b = Display content of LCD blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LcddispW<'_, LcdLcdmemctlSpec> {
        LcddispW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear LCD memory Clears all LCD memory registers LCDMx. The bit is automatically reset when the LCD memory is cleared. 0b = Contents of LCD memory registers LCDMx remain unchanged 1b = Clear content of all LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LcdclrmW<'_, LcdLcdmemctlSpec> {
        LcdclrmW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LCD blinking memory Clears all blinking memory registers LCDBMx. The bit is automatically reset when the blinking memory is cleared. Setting this bit in 5-mux mode and above has no effect. It is immediately reset again. 0b = Contents of blinking memory registers LCDBMx remain unchanged 1b = Clear content of all blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LcdclrbmW<'_, LcdLcdmemctlSpec> {
        LcdclrbmW::new(self, 2)
    }
}
#[doc = "LCD memory control LCD memory control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdmemctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdmemctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdmemctlSpec;
impl crate::RegisterSpec for LcdLcdmemctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdmemctl::R`](R) reader structure"]
impl crate::Readable for LcdLcdmemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdmemctl::W`](W) writer structure"]
impl crate::Writable for LcdLcdmemctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDMEMCTL to value 0"]
impl crate::Resettable for LcdLcdmemctlSpec {}
