#[doc = "Register `FLASHCTL_STATMODE` reader"]
pub type R = crate::R<FlashctlStatmodeSpec>;
#[doc = "Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Banknotinrd {
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
impl From<Banknotinrd> for u8 {
    #[inline(always)]
    fn from(variant: Banknotinrd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Banknotinrd {
    type Ux = u8;
}
impl crate::IsEnum for Banknotinrd {}
#[doc = "Field `BANKNOTINRD` reader - Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
pub type BanknotinrdR = crate::FieldReader<Banknotinrd>;
impl BanknotinrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Banknotinrd> {
        match self.bits {
            1 => Some(Banknotinrd::Bank0),
            2 => Some(Banknotinrd::Bank1),
            4 => Some(Banknotinrd::Bank2),
            8 => Some(Banknotinrd::Bank3),
            16 => Some(Banknotinrd::Bank4),
            _ => None,
        }
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Banknotinrd::Bank0
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Banknotinrd::Bank1
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Banknotinrd::Bank2
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == Banknotinrd::Bank3
    }
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Banknotinrd::Bank4
    }
}
#[doc = "Indicates mode of bank(s) that are not in READ mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bankmode {
    #[doc = "0: Read Mode"]
    Read = 0,
    #[doc = "2: Read Margin 0 Mode"]
    Rdmarg0 = 2,
    #[doc = "4: Read Margin 1 Mode"]
    Rdmarg1 = 4,
    #[doc = "6: Read Margin 0B Mode"]
    Rdmarg0b = 6,
    #[doc = "7: Read Margin 1B Mode"]
    Rdmarg1b = 7,
    #[doc = "9: Program Verify Mode"]
    Pgmver = 9,
    #[doc = "10: Program Single Word"]
    Pgmsw = 10,
    #[doc = "11: Erase Verify Mode"]
    Erasever = 11,
    #[doc = "12: Erase Sector"]
    Erasesect = 12,
    #[doc = "14: Program Multiple Word"]
    Pgmmw = 14,
    #[doc = "15: Erase Bank"]
    Erasebnk = 15,
}
impl From<Bankmode> for u8 {
    #[inline(always)]
    fn from(variant: Bankmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankmode {
    type Ux = u8;
}
impl crate::IsEnum for Bankmode {}
#[doc = "Field `BANKMODE` reader - Indicates mode of bank(s) that are not in READ mode"]
pub type BankmodeR = crate::FieldReader<Bankmode>;
impl BankmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankmode> {
        match self.bits {
            0 => Some(Bankmode::Read),
            2 => Some(Bankmode::Rdmarg0),
            4 => Some(Bankmode::Rdmarg1),
            6 => Some(Bankmode::Rdmarg0b),
            7 => Some(Bankmode::Rdmarg1b),
            9 => Some(Bankmode::Pgmver),
            10 => Some(Bankmode::Pgmsw),
            11 => Some(Bankmode::Erasever),
            12 => Some(Bankmode::Erasesect),
            14 => Some(Bankmode::Pgmmw),
            15 => Some(Bankmode::Erasebnk),
            _ => None,
        }
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Bankmode::Read
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn is_rdmarg0(&self) -> bool {
        *self == Bankmode::Rdmarg0
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn is_rdmarg1(&self) -> bool {
        *self == Bankmode::Rdmarg1
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn is_rdmarg0b(&self) -> bool {
        *self == Bankmode::Rdmarg0b
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn is_rdmarg1b(&self) -> bool {
        *self == Bankmode::Rdmarg1b
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn is_pgmver(&self) -> bool {
        *self == Bankmode::Pgmver
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn is_pgmsw(&self) -> bool {
        *self == Bankmode::Pgmsw
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn is_erasever(&self) -> bool {
        *self == Bankmode::Erasever
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn is_erasesect(&self) -> bool {
        *self == Bankmode::Erasesect
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn is_pgmmw(&self) -> bool {
        *self == Bankmode::Pgmmw
    }
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn is_erasebnk(&self) -> bool {
        *self == Bankmode::Erasebnk
    }
}
#[doc = "Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bank2trdy {
    #[doc = "0: Not ready"]
    False = 0,
    #[doc = "1: Ready"]
    True = 1,
}
impl From<Bank2trdy> for bool {
    #[inline(always)]
    fn from(variant: Bank2trdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BANK2TRDY` reader - Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
pub type Bank2trdyR = crate::BitReader<Bank2trdy>;
impl Bank2trdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bank2trdy {
        match self.bits {
            false => Bank2trdy::False,
            true => Bank2trdy::True,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Bank2trdy::False
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Bank2trdy::True
    }
}
#[doc = "Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bank1trdy {
    #[doc = "0: Not ready"]
    False = 0,
    #[doc = "1: Ready"]
    True = 1,
}
impl From<Bank1trdy> for bool {
    #[inline(always)]
    fn from(variant: Bank1trdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BANK1TRDY` reader - Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
pub type Bank1trdyR = crate::BitReader<Bank1trdy>;
impl Bank1trdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bank1trdy {
        match self.bits {
            false => Bank1trdy::False,
            true => Bank1trdy::True,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Bank1trdy::False
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Bank1trdy::True
    }
}
impl R {
    #[doc = "Bits 0:4 - Bank not in read mode. Indicates which banks are not in READ mode. There is 1 bit per bank."]
    #[inline(always)]
    pub fn banknotinrd(&self) -> BanknotinrdR {
        BanknotinrdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates mode of bank(s) that are not in READ mode"]
    #[inline(always)]
    pub fn bankmode(&self) -> BankmodeR {
        BankmodeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Bank 2T Ready. Bank(s) are ready for 2T access. This is accomplished when the pump has fully driven power rails to the bank(s)."]
    #[inline(always)]
    pub fn bank2trdy(&self) -> Bank2trdyR {
        Bank2trdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1T Ready. Bank(s) are ready for 1T access. This is accomplished when the bank and pump have been trimmed."]
    #[inline(always)]
    pub fn bank1trdy(&self) -> Bank1trdyR {
        Bank1trdyR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Mode Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_statmode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlStatmodeSpec;
impl crate::RegisterSpec for FlashctlStatmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_statmode::R`](R) reader structure"]
impl crate::Readable for FlashctlStatmodeSpec {}
#[doc = "`reset()` method sets FLASHCTL_STATMODE to value 0"]
impl crate::Resettable for FlashctlStatmodeSpec {}
