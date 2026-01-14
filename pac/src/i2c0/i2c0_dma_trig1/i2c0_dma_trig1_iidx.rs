#[doc = "Register `I2C0_DMA_TRIG1_IIDX` reader"]
pub type R = crate::R<I2c0DmaTrig1IidxSpec>;
#[doc = "I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Controller receive FIFO Trigger Level"]
    Crxfifotrg = 1,
    #[doc = "2: Controller transmit FIFO Trigger level"]
    Ctxfifotrg = 2,
    #[doc = "3: Target receive FIFO Trigger Level"]
    Trxfifotrg = 3,
    #[doc = "4: Target transmit FIFO Trigger level"]
    Ttxfifotrg = 4,
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
#[doc = "Field `STAT` reader - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Crxfifotrg),
            2 => Some(Stat::Ctxfifotrg),
            3 => Some(Stat::Trxfifotrg),
            4 => Some(Stat::Ttxfifotrg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Controller receive FIFO Trigger Level"]
    #[inline(always)]
    pub fn is_crxfifotrg(&self) -> bool {
        *self == Stat::Crxfifotrg
    }
    #[doc = "Controller transmit FIFO Trigger level"]
    #[inline(always)]
    pub fn is_ctxfifotrg(&self) -> bool {
        *self == Stat::Ctxfifotrg
    }
    #[doc = "Target receive FIFO Trigger Level"]
    #[inline(always)]
    pub fn is_trxfifotrg(&self) -> bool {
        *self == Stat::Trxfifotrg
    }
    #[doc = "Target transmit FIFO Trigger level"]
    #[inline(always)]
    pub fn is_ttxfifotrg(&self) -> bool {
        *self == Stat::Ttxfifotrg
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_dma_trig1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0DmaTrig1IidxSpec;
impl crate::RegisterSpec for I2c0DmaTrig1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_dma_trig1_iidx::R`](R) reader structure"]
impl crate::Readable for I2c0DmaTrig1IidxSpec {}
#[doc = "`reset()` method sets I2C0_DMA_TRIG1_IIDX to value 0"]
impl crate::Resettable for I2c0DmaTrig1IidxSpec {}
