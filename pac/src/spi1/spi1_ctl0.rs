#[doc = "Register `SPI1_CTL0` reader"]
pub type R = crate::R<Spi1Ctl0Spec>;
#[doc = "Register `SPI1_CTL0` writer"]
pub type W = crate::W<Spi1Ctl0Spec>;
#[doc = "Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "3: Data Size Select bits: 4"]
    Dss4 = 3,
    #[doc = "4: Data Size Select bits: 5"]
    Dss5 = 4,
    #[doc = "5: Data Size Select bits: 6"]
    Dss6 = 5,
    #[doc = "6: Data Size Select bits: 7"]
    Dss7 = 6,
    #[doc = "7: Data Size Select bits: 8"]
    Dss8 = 7,
    #[doc = "8: Data Size Select bits: 9"]
    Dss9 = 8,
    #[doc = "9: Data Size Select bits: 10"]
    Dss10 = 9,
    #[doc = "10: Data Size Select bits: 11"]
    Dss11 = 10,
    #[doc = "11: Data Size Select bits: 12"]
    Dss12 = 11,
    #[doc = "12: Data Size Select bits: 13"]
    Dss13 = 12,
    #[doc = "13: Data Size Select bits: 14"]
    Dss14 = 13,
    #[doc = "14: Data Size Select bits: 15"]
    Dss15 = 14,
    #[doc = "15: Data Size Select bits: 16"]
    Dss16 = 15,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::Dss4),
            4 => Some(Dss::Dss5),
            5 => Some(Dss::Dss6),
            6 => Some(Dss::Dss7),
            7 => Some(Dss::Dss8),
            8 => Some(Dss::Dss9),
            9 => Some(Dss::Dss10),
            10 => Some(Dss::Dss11),
            11 => Some(Dss::Dss12),
            12 => Some(Dss::Dss13),
            13 => Some(Dss::Dss14),
            14 => Some(Dss::Dss15),
            15 => Some(Dss::Dss16),
            _ => None,
        }
    }
    #[doc = "Data Size Select bits: 4"]
    #[inline(always)]
    pub fn is_dss_4(&self) -> bool {
        *self == Dss::Dss4
    }
    #[doc = "Data Size Select bits: 5"]
    #[inline(always)]
    pub fn is_dss_5(&self) -> bool {
        *self == Dss::Dss5
    }
    #[doc = "Data Size Select bits: 6"]
    #[inline(always)]
    pub fn is_dss_6(&self) -> bool {
        *self == Dss::Dss6
    }
    #[doc = "Data Size Select bits: 7"]
    #[inline(always)]
    pub fn is_dss_7(&self) -> bool {
        *self == Dss::Dss7
    }
    #[doc = "Data Size Select bits: 8"]
    #[inline(always)]
    pub fn is_dss_8(&self) -> bool {
        *self == Dss::Dss8
    }
    #[doc = "Data Size Select bits: 9"]
    #[inline(always)]
    pub fn is_dss_9(&self) -> bool {
        *self == Dss::Dss9
    }
    #[doc = "Data Size Select bits: 10"]
    #[inline(always)]
    pub fn is_dss_10(&self) -> bool {
        *self == Dss::Dss10
    }
    #[doc = "Data Size Select bits: 11"]
    #[inline(always)]
    pub fn is_dss_11(&self) -> bool {
        *self == Dss::Dss11
    }
    #[doc = "Data Size Select bits: 12"]
    #[inline(always)]
    pub fn is_dss_12(&self) -> bool {
        *self == Dss::Dss12
    }
    #[doc = "Data Size Select bits: 13"]
    #[inline(always)]
    pub fn is_dss_13(&self) -> bool {
        *self == Dss::Dss13
    }
    #[doc = "Data Size Select bits: 14"]
    #[inline(always)]
    pub fn is_dss_14(&self) -> bool {
        *self == Dss::Dss14
    }
    #[doc = "Data Size Select bits: 15"]
    #[inline(always)]
    pub fn is_dss_15(&self) -> bool {
        *self == Dss::Dss15
    }
    #[doc = "Data Size Select bits: 16"]
    #[inline(always)]
    pub fn is_dss_16(&self) -> bool {
        *self == Dss::Dss16
    }
}
#[doc = "Field `DSS` writer - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data Size Select bits: 4"]
    #[inline(always)]
    pub fn dss_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss4)
    }
    #[doc = "Data Size Select bits: 5"]
    #[inline(always)]
    pub fn dss_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss5)
    }
    #[doc = "Data Size Select bits: 6"]
    #[inline(always)]
    pub fn dss_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss6)
    }
    #[doc = "Data Size Select bits: 7"]
    #[inline(always)]
    pub fn dss_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss7)
    }
    #[doc = "Data Size Select bits: 8"]
    #[inline(always)]
    pub fn dss_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss8)
    }
    #[doc = "Data Size Select bits: 9"]
    #[inline(always)]
    pub fn dss_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss9)
    }
    #[doc = "Data Size Select bits: 10"]
    #[inline(always)]
    pub fn dss_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss10)
    }
    #[doc = "Data Size Select bits: 11"]
    #[inline(always)]
    pub fn dss_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss11)
    }
    #[doc = "Data Size Select bits: 12"]
    #[inline(always)]
    pub fn dss_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss12)
    }
    #[doc = "Data Size Select bits: 13"]
    #[inline(always)]
    pub fn dss_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss13)
    }
    #[doc = "Data Size Select bits: 14"]
    #[inline(always)]
    pub fn dss_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss14)
    }
    #[doc = "Data Size Select bits: 15"]
    #[inline(always)]
    pub fn dss_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss15)
    }
    #[doc = "Data Size Select bits: 16"]
    #[inline(always)]
    pub fn dss_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss16)
    }
}
#[doc = "Frame format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: Motorola SPI frame format (3 wire mode)"]
    Motorola3wire = 0,
    #[doc = "1: Motorola SPI frame format (4 wire mode)"]
    Motorola4wire = 1,
    #[doc = "2: TI synchronous serial frame format"]
    TiSync = 2,
    #[doc = "3: National Microwire frame format"]
    Mircowire = 3,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - Frame format Select"]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frf {
        match self.bits {
            0 => Frf::Motorola3wire,
            1 => Frf::Motorola4wire,
            2 => Frf::TiSync,
            3 => Frf::Mircowire,
            _ => unreachable!(),
        }
    }
    #[doc = "Motorola SPI frame format (3 wire mode)"]
    #[inline(always)]
    pub fn is_motorola_3wire(&self) -> bool {
        *self == Frf::Motorola3wire
    }
    #[doc = "Motorola SPI frame format (4 wire mode)"]
    #[inline(always)]
    pub fn is_motorola_4wire(&self) -> bool {
        *self == Frf::Motorola4wire
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn is_ti_sync(&self) -> bool {
        *self == Frf::TiSync
    }
    #[doc = "National Microwire frame format"]
    #[inline(always)]
    pub fn is_mircowire(&self) -> bool {
        *self == Frf::Mircowire
    }
}
#[doc = "Field `FRF` writer - Frame format Select"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf, crate::Safe>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Motorola SPI frame format (3 wire mode)"]
    #[inline(always)]
    pub fn motorola_3wire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola3wire)
    }
    #[doc = "Motorola SPI frame format (4 wire mode)"]
    #[inline(always)]
    pub fn motorola_4wire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola4wire)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn ti_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::TiSync)
    }
    #[doc = "National Microwire frame format"]
    #[inline(always)]
    pub fn mircowire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Mircowire)
    }
}
#[doc = "Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Packen {
    #[doc = "0: Packing feature disabled"]
    Disabled = 0,
    #[doc = "1: Packing feature enabled"]
    Enabled = 1,
}
impl From<Packen> for bool {
    #[inline(always)]
    fn from(variant: Packen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PACKEN` reader - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
pub type PackenR = crate::BitReader<Packen>;
impl PackenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Packen {
        match self.bits {
            false => Packen::Disabled,
            true => Packen::Enabled,
        }
    }
    #[doc = "Packing feature disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Packen::Disabled
    }
    #[doc = "Packing feature enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Packen::Enabled
    }
}
#[doc = "Field `PACKEN` writer - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
pub type PackenW<'a, REG> = crate::BitWriter<'a, REG, Packen>;
impl<'a, REG> PackenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packing feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Packen::Disabled)
    }
    #[doc = "Packing feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Packen::Enabled)
    }
}
#[doc = "CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spo {
    #[doc = "0: SPI produces a steady state LOW value on the CLKOUT"]
    Low = 0,
    #[doc = "1: SPI produces a steady state HIGH value on the CLKOUT"]
    High = 1,
}
impl From<Spo> for bool {
    #[inline(always)]
    fn from(variant: Spo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPO` reader - CLKOUT polarity (Motorola SPI frame format only)"]
pub type SpoR = crate::BitReader<Spo>;
impl SpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spo {
        match self.bits {
            false => Spo::Low,
            true => Spo::High,
        }
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spo::Low
    }
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spo::High
    }
}
#[doc = "Field `SPO` writer - CLKOUT polarity (Motorola SPI frame format only)"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG, Spo>;
impl<'a, REG> SpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI produces a steady state LOW value on the CLKOUT"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::Low)
    }
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::High)
    }
}
#[doc = "CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sph {
    #[doc = "0: Data is captured on the first clock edge transition."]
    First = 0,
    #[doc = "1: Data is captured on the second clock edge transition."]
    Second = 1,
}
impl From<Sph> for bool {
    #[inline(always)]
    fn from(variant: Sph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPH` reader - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphR = crate::BitReader<Sph>;
impl SphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sph {
        match self.bits {
            false => Sph::First,
            true => Sph::Second,
        }
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == Sph::First
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Sph::Second
    }
}
#[doc = "Field `SPH` writer - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG, Sph>;
impl<'a, REG> SphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::First)
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::Second)
    }
}
#[doc = "Select the CS line to control on data transfer This bit is applicable for both controller/target mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cssel {
    #[doc = "0: CS line select: 0"]
    Cssel0 = 0,
    #[doc = "1: CS line select: 1"]
    Cssel1 = 1,
    #[doc = "2: CS line select: 2"]
    Cssel2 = 2,
    #[doc = "3: CS line select: 3"]
    Cssel3 = 3,
}
impl From<Cssel> for u8 {
    #[inline(always)]
    fn from(variant: Cssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cssel {
    type Ux = u8;
}
impl crate::IsEnum for Cssel {}
#[doc = "Field `CSSEL` reader - Select the CS line to control on data transfer This bit is applicable for both controller/target mode"]
pub type CsselR = crate::FieldReader<Cssel>;
impl CsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cssel {
        match self.bits {
            0 => Cssel::Cssel0,
            1 => Cssel::Cssel1,
            2 => Cssel::Cssel2,
            3 => Cssel::Cssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "CS line select: 0"]
    #[inline(always)]
    pub fn is_cssel_0(&self) -> bool {
        *self == Cssel::Cssel0
    }
    #[doc = "CS line select: 1"]
    #[inline(always)]
    pub fn is_cssel_1(&self) -> bool {
        *self == Cssel::Cssel1
    }
    #[doc = "CS line select: 2"]
    #[inline(always)]
    pub fn is_cssel_2(&self) -> bool {
        *self == Cssel::Cssel2
    }
    #[doc = "CS line select: 3"]
    #[inline(always)]
    pub fn is_cssel_3(&self) -> bool {
        *self == Cssel::Cssel3
    }
}
#[doc = "Field `CSSEL` writer - Select the CS line to control on data transfer This bit is applicable for both controller/target mode"]
pub type CsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cssel, crate::Safe>;
impl<'a, REG> CsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS line select: 0"]
    #[inline(always)]
    pub fn cssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cssel::Cssel0)
    }
    #[doc = "CS line select: 1"]
    #[inline(always)]
    pub fn cssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cssel::Cssel1)
    }
    #[doc = "CS line select: 2"]
    #[inline(always)]
    pub fn cssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cssel::Cssel2)
    }
    #[doc = "CS line select: 3"]
    #[inline(always)]
    pub fn cssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cssel::Cssel3)
    }
}
#[doc = "Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.CP=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csclr {
    #[doc = "0: Disable automatic clear of shift register when CS goes to disable."]
    Disable = 0,
    #[doc = "1: Enable automatic clear of shift register when CS goes to disable."]
    Enable = 1,
}
impl From<Csclr> for bool {
    #[inline(always)]
    fn from(variant: Csclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCLR` reader - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.CP=0."]
pub type CsclrR = crate::BitReader<Csclr>;
impl CsclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csclr {
        match self.bits {
            false => Csclr::Disable,
            true => Csclr::Enable,
        }
    }
    #[doc = "Disable automatic clear of shift register when CS goes to disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Csclr::Disable
    }
    #[doc = "Enable automatic clear of shift register when CS goes to disable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Csclr::Enable
    }
}
#[doc = "Field `CSCLR` writer - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.CP=0."]
pub type CsclrW<'a, REG> = crate::BitWriter<'a, REG, Csclr>;
impl<'a, REG> CsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable automatic clear of shift register when CS goes to disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Disable)
    }
    #[doc = "Enable automatic clear of shift register when CS goes to disable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Enable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Frame format Select"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
    #[inline(always)]
    pub fn packen(&self) -> PackenR {
        PackenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Select the CS line to control on data transfer This bit is applicable for both controller/target mode"]
    #[inline(always)]
    pub fn cssel(&self) -> CsselR {
        CsselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.CP=0."]
    #[inline(always)]
    pub fn csclr(&self) -> CsclrR {
        CsclrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<'_, Spi1Ctl0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Frame format Select"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, Spi1Ctl0Spec> {
        FrfW::new(self, 5)
    }
    #[doc = "Bit 7 - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
    #[inline(always)]
    pub fn packen(&mut self) -> PackenW<'_, Spi1Ctl0Spec> {
        PackenW::new(self, 7)
    }
    #[doc = "Bit 8 - CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<'_, Spi1Ctl0Spec> {
        SpoW::new(self, 8)
    }
    #[doc = "Bit 9 - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<'_, Spi1Ctl0Spec> {
        SphW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Select the CS line to control on data transfer This bit is applicable for both controller/target mode"]
    #[inline(always)]
    pub fn cssel(&mut self) -> CsselW<'_, Spi1Ctl0Spec> {
        CsselW::new(self, 12)
    }
    #[doc = "Bit 14 - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.CP=0."]
    #[inline(always)]
    pub fn csclr(&mut self) -> CsclrW<'_, Spi1Ctl0Spec> {
        CsclrW::new(self, 14)
    }
}
#[doc = "SPI control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1Ctl0Spec;
impl crate::RegisterSpec for Spi1Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_ctl0::R`](R) reader structure"]
impl crate::Readable for Spi1Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1_ctl0::W`](W) writer structure"]
impl crate::Writable for Spi1Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_CTL0 to value 0"]
impl crate::Resettable for Spi1Ctl0Spec {}
