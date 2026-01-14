#[doc = "Register `LCD_IIDX` reader"]
pub type R = crate::R<LcdIidxSpec>;
#[doc = "LCD Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Start of new frame interrupt"]
    FrameStart = 1,
    #[doc = "2: Blinking segments off interrupt"]
    BlkOff = 2,
    #[doc = "3: Blinking segments on interrupt"]
    BlkOn = 3,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - LCD Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::FrameStart),
            2 => Some(Stat::BlkOff),
            3 => Some(Stat::BlkOn),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Start of new frame interrupt"]
    #[inline(always)]
    pub fn is_frame_start(&self) -> bool {
        *self == Stat::FrameStart
    }
    #[doc = "Blinking segments off interrupt"]
    #[inline(always)]
    pub fn is_blk_off(&self) -> bool {
        *self == Stat::BlkOff
    }
    #[doc = "Blinking segments on interrupt"]
    #[inline(always)]
    pub fn is_blk_on(&self) -> bool {
        *self == Stat::BlkOn
    }
}
impl R {
    #[doc = "Bits 0:7 - LCD Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdIidxSpec;
impl crate::RegisterSpec for LcdIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_iidx::R`](R) reader structure"]
impl crate::Readable for LcdIidxSpec {}
#[doc = "`reset()` method sets LCD_IIDX to value 0"]
impl crate::Resettable for LcdIidxSpec {}
