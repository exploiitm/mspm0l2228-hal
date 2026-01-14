#[doc = "Register `LCD_LCDBLKCTL` reader"]
pub type R = crate::R<LcdLcdblkctlSpec>;
#[doc = "Register `LCD_LCDBLKCTL` writer"]
pub type W = crate::W<LcdLcdblkctlSpec>;
#[doc = "Blinking mode 00b = Blinking disabled. 01b = Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled. 10b = Blinking of all segments 11b = Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkmodx {
    #[doc = "0: Blinking disabled."]
    BlinkDisable = 0,
    #[doc = "1: Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled."]
    BlinkSeleced = 1,
    #[doc = "2: Blinking of all segments"]
    BlinkAll = 2,
    #[doc = "3: Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
    BkinkToggle = 3,
}
impl From<Lcdblkmodx> for u8 {
    #[inline(always)]
    fn from(variant: Lcdblkmodx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdblkmodx {
    type Ux = u8;
}
impl crate::IsEnum for Lcdblkmodx {}
#[doc = "Field `LCDBLKMODx` reader - Blinking mode 00b = Blinking disabled. 01b = Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled. 10b = Blinking of all segments 11b = Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
pub type LcdblkmodxR = crate::FieldReader<Lcdblkmodx>;
impl LcdblkmodxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkmodx {
        match self.bits {
            0 => Lcdblkmodx::BlinkDisable,
            1 => Lcdblkmodx::BlinkSeleced,
            2 => Lcdblkmodx::BlinkAll,
            3 => Lcdblkmodx::BkinkToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "Blinking disabled."]
    #[inline(always)]
    pub fn is_blink_disable(&self) -> bool {
        *self == Lcdblkmodx::BlinkDisable
    }
    #[doc = "Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn is_blink_seleced(&self) -> bool {
        *self == Lcdblkmodx::BlinkSeleced
    }
    #[doc = "Blinking of all segments"]
    #[inline(always)]
    pub fn is_blink_all(&self) -> bool {
        *self == Lcdblkmodx::BlinkAll
    }
    #[doc = "Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn is_bkink_toggle(&self) -> bool {
        *self == Lcdblkmodx::BkinkToggle
    }
}
#[doc = "Field `LCDBLKMODx` writer - Blinking mode 00b = Blinking disabled. 01b = Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled. 10b = Blinking of all segments 11b = Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
pub type LcdblkmodxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcdblkmodx, crate::Safe>;
impl<'a, REG> LcdblkmodxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blinking disabled."]
    #[inline(always)]
    pub fn blink_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmodx::BlinkDisable)
    }
    #[doc = "Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn blink_seleced(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmodx::BlinkSeleced)
    }
    #[doc = "Blinking of all segments"]
    #[inline(always)]
    pub fn blink_all(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmodx::BlinkAll)
    }
    #[doc = "Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn bkink_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmodx::BkinkToggle)
    }
}
#[doc = "Clock prescaler for blinking frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkprex {
    #[doc = "0: Divide by 2"]
    DivBy2 = 0,
    #[doc = "1: Divide by 4"]
    DivBy4 = 1,
    #[doc = "2: Divide by 8"]
    DivBy8 = 2,
    #[doc = "3: Divide by 16"]
    DivBy16 = 3,
    #[doc = "4: Divide by 32"]
    DivBy32 = 4,
    #[doc = "5: Divide by 64"]
    DivBy64 = 5,
    #[doc = "6: Divide by 128"]
    DivBy128 = 6,
    #[doc = "7: Divide by 256"]
    DivBy256 = 7,
}
impl From<Lcdblkprex> for u8 {
    #[inline(always)]
    fn from(variant: Lcdblkprex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdblkprex {
    type Ux = u8;
}
impl crate::IsEnum for Lcdblkprex {}
#[doc = "Field `LCDBLKPREx` reader - Clock prescaler for blinking frequency."]
pub type LcdblkprexR = crate::FieldReader<Lcdblkprex>;
impl LcdblkprexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkprex {
        match self.bits {
            0 => Lcdblkprex::DivBy2,
            1 => Lcdblkprex::DivBy4,
            2 => Lcdblkprex::DivBy8,
            3 => Lcdblkprex::DivBy16,
            4 => Lcdblkprex::DivBy32,
            5 => Lcdblkprex::DivBy64,
            6 => Lcdblkprex::DivBy128,
            7 => Lcdblkprex::DivBy256,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Lcdblkprex::DivBy2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Lcdblkprex::DivBy4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Lcdblkprex::DivBy8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == Lcdblkprex::DivBy16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == Lcdblkprex::DivBy32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == Lcdblkprex::DivBy64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == Lcdblkprex::DivBy128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div_by_256(&self) -> bool {
        *self == Lcdblkprex::DivBy256
    }
}
#[doc = "Field `LCDBLKPREx` writer - Clock prescaler for blinking frequency."]
pub type LcdblkprexW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdblkprex, crate::Safe>;
impl<'a, REG> LcdblkprexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkprex::DivBy256)
    }
}
impl R {
    #[doc = "Bits 0:1 - Blinking mode 00b = Blinking disabled. 01b = Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled. 10b = Blinking of all segments 11b = Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn lcdblkmodx(&self) -> LcdblkmodxR {
        LcdblkmodxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Clock prescaler for blinking frequency."]
    #[inline(always)]
    pub fn lcdblkprex(&self) -> LcdblkprexR {
        LcdblkprexR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Blinking mode 00b = Blinking disabled. 01b = Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode >5 blinking is disabled. 10b = Blinking of all segments 11b = Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode >5 blinking is disabled."]
    #[inline(always)]
    pub fn lcdblkmodx(&mut self) -> LcdblkmodxW<'_, LcdLcdblkctlSpec> {
        LcdblkmodxW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Clock prescaler for blinking frequency."]
    #[inline(always)]
    pub fn lcdblkprex(&mut self) -> LcdblkprexW<'_, LcdLcdblkctlSpec> {
        LcdblkprexW::new(self, 2)
    }
}
#[doc = "LCD blicking control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_lcdblkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_lcdblkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLcdblkctlSpec;
impl crate::RegisterSpec for LcdLcdblkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lcdblkctl::R`](R) reader structure"]
impl crate::Readable for LcdLcdblkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_lcdblkctl::W`](W) writer structure"]
impl crate::Writable for LcdLcdblkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LCDBLKCTL to value 0"]
impl crate::Resettable for LcdLcdblkctlSpec {}
