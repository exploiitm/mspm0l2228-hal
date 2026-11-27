#[doc = "Register `SPI0_CTL1` reader"]
pub type R = crate::R<Spi0Ctl1Spec>;
#[doc = "Register `SPI0_CTL1` writer"]
pub type W = crate::W<Spi0Ctl1Spec>;
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable module function"]
    Disable = 0,
    #[doc = "1: Enable module function"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - SPI enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable module function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable module function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - SPI enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable module function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable module function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "0: Disable loopback mode"]
    Disable = 0,
    #[doc = "1: Enable loopback mode"]
    Enable = 1,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop back mode"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            false => Lbm::Disable,
            true => Lbm::Enable,
        }
    }
    #[doc = "Disable loopback mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbm::Disable
    }
    #[doc = "Enable loopback mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbm::Enable
    }
}
#[doc = "Field `LBM` writer - Loop back mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable loopback mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Disable)
    }
    #[doc = "Enable loopback mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Enable)
    }
}
#[doc = "Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cp {
    #[doc = "0: Select Peripheral mode"]
    Disable = 0,
    #[doc = "1: Select Controller Mode"]
    Enable = 1,
}
impl From<Cp> for bool {
    #[inline(always)]
    fn from(variant: Cp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CP` reader - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
pub type CpR = crate::BitReader<Cp>;
impl CpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cp {
        match self.bits {
            false => Cp::Disable,
            true => Cp::Enable,
        }
    }
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cp::Disable
    }
    #[doc = "Select Controller Mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cp::Enable
    }
}
#[doc = "Field `CP` writer - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
pub type CpW<'a, REG> = crate::BitWriter<'a, REG, Cp>;
impl<'a, REG> CpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::Disable)
    }
    #[doc = "Select Controller Mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::Enable)
    }
}
#[doc = "Peripheral-mode: Data output disabled This bit is relevant only in Peripheral mode. In multiple-peripheral system topologies, SPI controller can broadcast a message to all peripherals, while only one peripheral drives the line. POD can be used by the SPI peripheral to disable driving data on the line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pod {
    #[doc = "0: SPI can drive the MISO output in peripheral mode."]
    Disable = 0,
    #[doc = "1: SPI cannot drive the MISO output in peripheral mode."]
    Enable = 1,
}
impl From<Pod> for bool {
    #[inline(always)]
    fn from(variant: Pod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POD` reader - Peripheral-mode: Data output disabled This bit is relevant only in Peripheral mode. In multiple-peripheral system topologies, SPI controller can broadcast a message to all peripherals, while only one peripheral drives the line. POD can be used by the SPI peripheral to disable driving data on the line."]
pub type PodR = crate::BitReader<Pod>;
impl PodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pod {
        match self.bits {
            false => Pod::Disable,
            true => Pod::Enable,
        }
    }
    #[doc = "SPI can drive the MISO output in peripheral mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pod::Disable
    }
    #[doc = "SPI cannot drive the MISO output in peripheral mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pod::Enable
    }
}
#[doc = "Field `POD` writer - Peripheral-mode: Data output disabled This bit is relevant only in Peripheral mode. In multiple-peripheral system topologies, SPI controller can broadcast a message to all peripherals, while only one peripheral drives the line. POD can be used by the SPI peripheral to disable driving data on the line."]
pub type PodW<'a, REG> = crate::BitWriter<'a, REG, Pod>;
impl<'a, REG> PodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI can drive the MISO output in peripheral mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Disable)
    }
    #[doc = "SPI cannot drive the MISO output in peripheral mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Enable)
    }
}
#[doc = "MSB first select. Controls the direction of the receive and transmit shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msb {
    #[doc = "0: LSB first"]
    Disable = 0,
    #[doc = "1: MSB first"]
    Enable = 1,
}
impl From<Msb> for bool {
    #[inline(always)]
    fn from(variant: Msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSB` reader - MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MsbR = crate::BitReader<Msb>;
impl MsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msb {
        match self.bits {
            false => Msb::Disable,
            true => Msb::Enable,
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Msb::Disable
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Msb::Enable
    }
}
#[doc = "Field `MSB` writer - MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG, Msb>;
impl<'a, REG> MsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Disable)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Enable)
    }
}
#[doc = "Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pren {
    #[doc = "0: Disable Parity receive function"]
    Disable = 0,
    #[doc = "1: Enable Parity receive function"]
    Enable = 1,
}
impl From<Pren> for bool {
    #[inline(always)]
    fn from(variant: Pren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREN` reader - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
pub type PrenR = crate::BitReader<Pren>;
impl PrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pren {
        match self.bits {
            false => Pren::Disable,
            true => Pren::Enable,
        }
    }
    #[doc = "Disable Parity receive function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pren::Disable
    }
    #[doc = "Enable Parity receive function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pren::Enable
    }
}
#[doc = "Field `PREN` writer - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
pub type PrenW<'a, REG> = crate::BitWriter<'a, REG, Pren>;
impl<'a, REG> PrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Parity receive function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::Disable)
    }
    #[doc = "Enable Parity receive function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::Enable)
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pes {
    #[doc = "0: Odd Parity mode"]
    Disable = 0,
    #[doc = "1: Even Parity mode"]
    Enable = 1,
}
impl From<Pes> for bool {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - Even Parity Select"]
pub type PesR = crate::BitReader<Pes>;
impl PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            false => Pes::Disable,
            true => Pes::Enable,
        }
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pes::Disable
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pes::Enable
    }
}
#[doc = "Field `PES` writer - Even Parity Select"]
pub type PesW<'a, REG> = crate::BitWriter<'a, REG, Pes>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Disable)
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Enable)
    }
}
#[doc = "Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pten {
    #[doc = "0: Parity transmission is disabled"]
    Disable = 0,
    #[doc = "1: Parity transmission is enabled"]
    Enable = 1,
}
impl From<Pten> for bool {
    #[inline(always)]
    fn from(variant: Pten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEN` reader - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
pub type PtenR = crate::BitReader<Pten>;
impl PtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pten {
        match self.bits {
            false => Pten::Disable,
            true => Pten::Enable,
        }
    }
    #[doc = "Parity transmission is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pten::Disable
    }
    #[doc = "Parity transmission is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pten::Enable
    }
}
#[doc = "Field `PTEN` writer - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
pub type PtenW<'a, REG> = crate::BitWriter<'a, REG, Pten>;
impl<'a, REG> PtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity transmission is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pten::Disable)
    }
    #[doc = "Parity transmission is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pten::Enable)
    }
}
#[doc = "Command/Data Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdenable {
    #[doc = "0: CS3 is used for Chip Select"]
    Disable = 0,
    #[doc = "1: CS3 is used as CD signal"]
    Enable = 1,
}
impl From<Cdenable> for bool {
    #[inline(always)]
    fn from(variant: Cdenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDENABLE` reader - Command/Data Mode enable"]
pub type CdenableR = crate::BitReader<Cdenable>;
impl CdenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdenable {
        match self.bits {
            false => Cdenable::Disable,
            true => Cdenable::Enable,
        }
    }
    #[doc = "CS3 is used for Chip Select"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cdenable::Disable
    }
    #[doc = "CS3 is used as CD signal"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cdenable::Enable
    }
}
#[doc = "Field `CDENABLE` writer - Command/Data Mode enable"]
pub type CdenableW<'a, REG> = crate::BitWriter<'a, REG, Cdenable>;
impl<'a, REG> CdenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CS3 is used for Chip Select"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdenable::Disable)
    }
    #[doc = "CS3 is used as CD signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdenable::Enable)
    }
}
#[doc = "Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdmode {
    #[doc = "0: Manual mode: Data"]
    Data = 0,
    #[doc = "15: Manual mode: Command"]
    Command = 15,
}
impl From<Cdmode> for u8 {
    #[inline(always)]
    fn from(variant: Cdmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdmode {
    type Ux = u8;
}
impl crate::IsEnum for Cdmode {}
#[doc = "Field `CDMODE` reader - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
pub type CdmodeR = crate::FieldReader<Cdmode>;
impl CdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdmode> {
        match self.bits {
            0 => Some(Cdmode::Data),
            15 => Some(Cdmode::Command),
            _ => None,
        }
    }
    #[doc = "Manual mode: Data"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Cdmode::Data
    }
    #[doc = "Manual mode: Command"]
    #[inline(always)]
    pub fn is_command(&self) -> bool {
        *self == Cdmode::Command
    }
}
#[doc = "Field `CDMODE` writer - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
pub type CdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdmode>;
impl<'a, REG> CdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual mode: Data"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Cdmode::Data)
    }
    #[doc = "Manual mode: Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut crate::W<REG> {
        self.variant(Cdmode::Command)
    }
}
#[doc = "Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repeattx {
    #[doc = "0: REPEATTX disable"]
    Disable = 0,
}
impl From<Repeattx> for u8 {
    #[inline(always)]
    fn from(variant: Repeattx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repeattx {
    type Ux = u8;
}
impl crate::IsEnum for Repeattx {}
#[doc = "Field `REPEATTX` reader - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type RepeattxR = crate::FieldReader<Repeattx>;
impl RepeattxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Repeattx> {
        match self.bits {
            0 => Some(Repeattx::Disable),
            _ => None,
        }
    }
    #[doc = "REPEATTX disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Repeattx::Disable
    }
}
#[doc = "Field `REPEATTX` writer - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type RepeattxW<'a, REG> = crate::FieldWriter<'a, REG, 8, Repeattx>;
impl<'a, REG> RepeattxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REPEATTX disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Repeattx::Disable)
    }
}
#[doc = "Field `RXTIMEOUT` reader - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type RxtimeoutR = crate::FieldReader;
#[doc = "Field `RXTIMEOUT` writer - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type RxtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loop back mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled This bit is relevant only in Peripheral mode. In multiple-peripheral system topologies, SPI controller can broadcast a message to all peripherals, while only one peripheral drives the line. POD can be used by the SPI peripheral to disable driving data on the line."]
    #[inline(always)]
    pub fn pod(&self) -> PodR {
        PodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn pren(&self) -> PrenR {
        PrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
    #[inline(always)]
    pub fn pten(&self) -> PtenR {
        PtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Command/Data Mode enable"]
    #[inline(always)]
    pub fn cdenable(&self) -> CdenableR {
        CdenableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
    #[inline(always)]
    pub fn cdmode(&self) -> CdmodeR {
        CdmodeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    pub fn repeattx(&self) -> RepeattxR {
        RepeattxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtimeout(&self) -> RxtimeoutR {
        RxtimeoutR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Spi0Ctl1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Loop back mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Spi0Ctl1Spec> {
        LbmW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
    #[inline(always)]
    pub fn cp(&mut self) -> CpW<'_, Spi0Ctl1Spec> {
        CpW::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled This bit is relevant only in Peripheral mode. In multiple-peripheral system topologies, SPI controller can broadcast a message to all peripherals, while only one peripheral drives the line. POD can be used by the SPI peripheral to disable driving data on the line."]
    #[inline(always)]
    pub fn pod(&mut self) -> PodW<'_, Spi0Ctl1Spec> {
        PodW::new(self, 3)
    }
    #[doc = "Bit 4 - MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn msb(&mut self) -> MsbW<'_, Spi0Ctl1Spec> {
        MsbW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn pren(&mut self) -> PrenW<'_, Spi0Ctl1Spec> {
        PrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn pes(&mut self) -> PesW<'_, Spi0Ctl1Spec> {
        PesW::new(self, 6)
    }
    #[doc = "Bit 8 - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
    #[inline(always)]
    pub fn pten(&mut self) -> PtenW<'_, Spi0Ctl1Spec> {
        PtenW::new(self, 8)
    }
    #[doc = "Bit 11 - Command/Data Mode enable"]
    #[inline(always)]
    pub fn cdenable(&mut self) -> CdenableW<'_, Spi0Ctl1Spec> {
        CdenableW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
    #[inline(always)]
    pub fn cdmode(&mut self) -> CdmodeW<'_, Spi0Ctl1Spec> {
        CdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    pub fn repeattx(&mut self) -> RepeattxW<'_, Spi0Ctl1Spec> {
        RepeattxW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtimeout(&mut self) -> RxtimeoutW<'_, Spi0Ctl1Spec> {
        RxtimeoutW::new(self, 24)
    }
}
#[doc = "SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0Ctl1Spec;
impl crate::RegisterSpec for Spi0Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_ctl1::R`](R) reader structure"]
impl crate::Readable for Spi0Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0_ctl1::W`](W) writer structure"]
impl crate::Writable for Spi0Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_CTL1 to value 0x04"]
impl crate::Resettable for Spi0Ctl1Spec {
    const RESET_VALUE: u32 = 0x04;
}
