#[doc = "Register `FLASHCTL_STATADDR` reader"]
pub type R = crate::R<FlashctlStataddrSpec>;
#[doc = "Field `BANKADDR` reader - Current Bank Address A bank offset address is stored in this register."]
pub type BankaddrR = crate::FieldReader<u16>;
#[doc = "Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Regionid {
    #[doc = "1: Main Region"]
    Main = 1,
    #[doc = "2: Non-Main Region"]
    Nonmain = 2,
    #[doc = "4: Trim Region"]
    Trim = 4,
    #[doc = "8: Engr Region"]
    Engr = 8,
}
impl From<Regionid> for u8 {
    #[inline(always)]
    fn from(variant: Regionid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Regionid {
    type Ux = u8;
}
impl crate::IsEnum for Regionid {}
#[doc = "Field `REGIONID` reader - Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
pub type RegionidR = crate::FieldReader<Regionid>;
impl RegionidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Regionid> {
        match self.bits {
            1 => Some(Regionid::Main),
            2 => Some(Regionid::Nonmain),
            4 => Some(Regionid::Trim),
            8 => Some(Regionid::Engr),
            _ => None,
        }
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == Regionid::Main
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == Regionid::Nonmain
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == Regionid::Trim
    }
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == Regionid::Engr
    }
}
#[doc = "Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bankid {
    #[doc = "1: Bank 0"]
    Bank0 = 1,
    #[doc = "2: Bank 1"]
    Bank1 = 2,
    #[doc = "4: Bank 2"]
    Bank2 = 4,
    #[doc = "8: Bank 3"]
    Bank3 = 8,
    #[doc = "16: Bank 4"]
    Bank4 = 16,
}
impl From<Bankid> for u8 {
    #[inline(always)]
    fn from(variant: Bankid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankid {
    type Ux = u8;
}
impl crate::IsEnum for Bankid {}
#[doc = "Field `BANKID` reader - Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
pub type BankidR = crate::FieldReader<Bankid>;
impl BankidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankid> {
        match self.bits {
            1 => Some(Bankid::Bank0),
            2 => Some(Bankid::Bank1),
            4 => Some(Bankid::Bank2),
            8 => Some(Bankid::Bank3),
            16 => Some(Bankid::Bank4),
            _ => None,
        }
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Bankid::Bank0
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Bankid::Bank1
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Bankid::Bank2
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == Bankid::Bank3
    }
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Bankid::Bank4
    }
}
impl R {
    #[doc = "Bits 0:15 - Current Bank Address A bank offset address is stored in this register."]
    #[inline(always)]
    pub fn bankaddr(&self) -> BankaddrR {
        BankaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."]
    #[inline(always)]
    pub fn regionid(&self) -> RegionidR {
        RegionidR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."]
    #[inline(always)]
    pub fn bankid(&self) -> BankidR {
        BankidR::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
#[doc = "Address Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_stataddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlStataddrSpec;
impl crate::RegisterSpec for FlashctlStataddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_stataddr::R`](R) reader structure"]
impl crate::Readable for FlashctlStataddrSpec {}
#[doc = "`reset()` method sets FLASHCTL_STATADDR to value 0"]
impl crate::Resettable for FlashctlStataddrSpec {}
