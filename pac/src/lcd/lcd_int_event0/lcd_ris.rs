#[doc = "Register `LCD_RIS` reader"]
pub type R = crate::R<LcdRisSpec>;
#[doc = "Set in start of a new frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmstart {
    #[doc = "0: Interrupt flag cleared"]
    Clr = 0,
    #[doc = "1: Interrupt flag set"]
    Set = 1,
}
impl From<Frmstart> for bool {
    #[inline(always)]
    fn from(variant: Frmstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMSTART` reader - Set in start of a new frame."]
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
    #[doc = "Interrupt flag cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Frmstart::Clr
    }
    #[doc = "Interrupt flag set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Frmstart::Set
    }
}
#[doc = "Blinking segments turned off interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkoff {
    #[doc = "0: Interrupt flag cleared"]
    Clr = 0,
    #[doc = "1: Interrupt flag set"]
    Set = 1,
}
impl From<Blkoff> for bool {
    #[inline(always)]
    fn from(variant: Blkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKOFF` reader - Blinking segments turned off interrupt flag."]
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
    #[doc = "Interrupt flag cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Blkoff::Clr
    }
    #[doc = "Interrupt flag set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Blkoff::Set
    }
}
#[doc = "Blinking segments turned on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blkon {
    #[doc = "0: Interrupt flag cleared"]
    Clr = 0,
    #[doc = "1: Interrupt flag set"]
    Set = 1,
}
impl From<Blkon> for bool {
    #[inline(always)]
    fn from(variant: Blkon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKON` reader - Blinking segments turned on."]
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
    #[doc = "Interrupt flag cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Blkon::Clr
    }
    #[doc = "Interrupt flag set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Blkon::Set
    }
}
impl R {
    #[doc = "Bit 0 - Set in start of a new frame."]
    #[inline(always)]
    pub fn frmstart(&self) -> FrmstartR {
        FrmstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blinking segments turned off interrupt flag."]
    #[inline(always)]
    pub fn blkoff(&self) -> BlkoffR {
        BlkoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Blinking segments turned on."]
    #[inline(always)]
    pub fn blkon(&self) -> BlkonR {
        BlkonR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdRisSpec;
impl crate::RegisterSpec for LcdRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ris::R`](R) reader structure"]
impl crate::Readable for LcdRisSpec {}
#[doc = "`reset()` method sets LCD_RIS to value 0"]
impl crate::Resettable for LcdRisSpec {}
