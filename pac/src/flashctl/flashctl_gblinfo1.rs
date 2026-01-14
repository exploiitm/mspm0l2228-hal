#[doc = "Register `FLASHCTL_GBLINFO1` reader"]
pub type R = crate::R<FlashctlGblinfo1Spec>;
#[doc = "Data width in bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datawidth {
    #[doc = "64: Data width is 64 bits"]
    W64bit = 64,
    #[doc = "128: Data width is 128 bits"]
    W128bit = 128,
}
impl From<Datawidth> for u8 {
    #[inline(always)]
    fn from(variant: Datawidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datawidth {
    type Ux = u8;
}
impl crate::IsEnum for Datawidth {}
#[doc = "Field `DATAWIDTH` reader - Data width in bits"]
pub type DatawidthR = crate::FieldReader<Datawidth>;
impl DatawidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datawidth> {
        match self.bits {
            64 => Some(Datawidth::W64bit),
            128 => Some(Datawidth::W128bit),
            _ => None,
        }
    }
    #[doc = "Data width is 64 bits"]
    #[inline(always)]
    pub fn is_w64bit(&self) -> bool {
        *self == Datawidth::W64bit
    }
    #[doc = "Data width is 128 bits"]
    #[inline(always)]
    pub fn is_w128bit(&self) -> bool {
        *self == Datawidth::W128bit
    }
}
#[doc = "ECC data width in bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eccwidth {
    #[doc = "0: ECC data width is 0. ECC not used."]
    W0bit = 0,
    #[doc = "8: ECC data width is 8 bits"]
    W8bit = 8,
    #[doc = "16: ECC data width is 16 bits"]
    W16bit = 16,
}
impl From<Eccwidth> for u8 {
    #[inline(always)]
    fn from(variant: Eccwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eccwidth {
    type Ux = u8;
}
impl crate::IsEnum for Eccwidth {}
#[doc = "Field `ECCWIDTH` reader - ECC data width in bits"]
pub type EccwidthR = crate::FieldReader<Eccwidth>;
impl EccwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eccwidth> {
        match self.bits {
            0 => Some(Eccwidth::W0bit),
            8 => Some(Eccwidth::W8bit),
            16 => Some(Eccwidth::W16bit),
            _ => None,
        }
    }
    #[doc = "ECC data width is 0. ECC not used."]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == Eccwidth::W0bit
    }
    #[doc = "ECC data width is 8 bits"]
    #[inline(always)]
    pub fn is_w8bit(&self) -> bool {
        *self == Eccwidth::W8bit
    }
    #[doc = "ECC data width is 16 bits"]
    #[inline(always)]
    pub fn is_w16bit(&self) -> bool {
        *self == Eccwidth::W16bit
    }
}
#[doc = "Redundant data width in bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Redwidth {
    #[doc = "0: Redundant data width is 0. Redundancy/Repair not present."]
    W0bit = 0,
    #[doc = "2: Redundant data width is 2 bits"]
    W2bit = 2,
    #[doc = "4: Redundant data width is 4 bits"]
    W4bit = 4,
}
impl From<Redwidth> for u8 {
    #[inline(always)]
    fn from(variant: Redwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Redwidth {
    type Ux = u8;
}
impl crate::IsEnum for Redwidth {}
#[doc = "Field `REDWIDTH` reader - Redundant data width in bits"]
pub type RedwidthR = crate::FieldReader<Redwidth>;
impl RedwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Redwidth> {
        match self.bits {
            0 => Some(Redwidth::W0bit),
            2 => Some(Redwidth::W2bit),
            4 => Some(Redwidth::W4bit),
            _ => None,
        }
    }
    #[doc = "Redundant data width is 0. Redundancy/Repair not present."]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == Redwidth::W0bit
    }
    #[doc = "Redundant data width is 2 bits"]
    #[inline(always)]
    pub fn is_w2bit(&self) -> bool {
        *self == Redwidth::W2bit
    }
    #[doc = "Redundant data width is 4 bits"]
    #[inline(always)]
    pub fn is_w4bit(&self) -> bool {
        *self == Redwidth::W4bit
    }
}
impl R {
    #[doc = "Bits 0:7 - Data width in bits"]
    #[inline(always)]
    pub fn datawidth(&self) -> DatawidthR {
        DatawidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - ECC data width in bits"]
    #[inline(always)]
    pub fn eccwidth(&self) -> EccwidthR {
        EccwidthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Redundant data width in bits"]
    #[inline(always)]
    pub fn redwidth(&self) -> RedwidthR {
        RedwidthR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Global Information Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_gblinfo1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlGblinfo1Spec;
impl crate::RegisterSpec for FlashctlGblinfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_gblinfo1::R`](R) reader structure"]
impl crate::Readable for FlashctlGblinfo1Spec {}
#[doc = "`reset()` method sets FLASHCTL_GBLINFO1 to value 0"]
impl crate::Resettable for FlashctlGblinfo1Spec {}
