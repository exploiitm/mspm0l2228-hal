#[doc = "Register `LFSS_GEN_EVENT_ICLR` writer"]
pub type W = crate::W<LfssGenEventIclrSpec>;
#[doc = "RTC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdy {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rtcrdy> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` writer - RTC ready"]
pub type RtcrdyW<'a, REG> = crate::BitWriter<'a, REG, Rtcrdy>;
impl<'a, REG> RtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdy::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdy::Clr)
    }
}
#[doc = "RTC time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctev {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rtctev> for bool {
    #[inline(always)]
    fn from(variant: Rtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTEV` writer - RTC time event"]
pub type RtctevW<'a, REG> = crate::BitWriter<'a, REG, Rtctev>;
impl<'a, REG> RtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Clr)
    }
}
#[doc = "RTC alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca1 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rtca1> for bool {
    #[inline(always)]
    fn from(variant: Rtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCA1` writer - RTC alarm 1"]
pub type Rtca1W<'a, REG> = crate::BitWriter<'a, REG, Rtca1>;
impl<'a, REG> Rtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca1::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca1::Clr)
    }
}
#[doc = "RTC alarm 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtca2 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rtca2> for bool {
    #[inline(always)]
    fn from(variant: Rtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCA2` writer - RTC alarm 2"]
pub type Rtca2W<'a, REG> = crate::BitWriter<'a, REG, Rtca2>;
impl<'a, REG> Rtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca2::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtca2::Clr)
    }
}
#[doc = "RTC prescale timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0ps {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rt0ps> for bool {
    #[inline(always)]
    fn from(variant: Rt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PS` writer - RTC prescale timer 0"]
pub type Rt0psW<'a, REG> = crate::BitWriter<'a, REG, Rt0ps>;
impl<'a, REG> Rt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ps::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ps::Clr)
    }
}
#[doc = "RTC prescale timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1ps {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rt1ps> for bool {
    #[inline(always)]
    fn from(variant: Rt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PS` writer - RTC prescale timer 1"]
pub type Rt1psW<'a, REG> = crate::BitWriter<'a, REG, Rt1ps>;
impl<'a, REG> Rt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ps::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ps::Clr)
    }
}
#[doc = "RTC prescale timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt2ps {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Rt2ps> for bool {
    #[inline(always)]
    fn from(variant: Rt2ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT2PS` writer - RTC prescale timer 2"]
pub type Rt2psW<'a, REG> = crate::BitWriter<'a, REG, Rt2ps>;
impl<'a, REG> Rt2psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2ps::Clr)
    }
}
#[doc = "Time stamp event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsevt {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tsevt> for bool {
    #[inline(always)]
    fn from(variant: Tsevt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEVT` writer - Time stamp event"]
pub type TsevtW<'a, REG> = crate::BitWriter<'a, REG, Tsevt>;
impl<'a, REG> TsevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tsevt::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tsevt::Clr)
    }
}
#[doc = "Tamper I/O 0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio0 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio0> for bool {
    #[inline(always)]
    fn from(variant: Tio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO0` writer - Tamper I/O 0 event"]
pub type Tio0W<'a, REG> = crate::BitWriter<'a, REG, Tio0>;
impl<'a, REG> Tio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio0::Clr)
    }
}
#[doc = "Tamper I/O 1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio1 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio1> for bool {
    #[inline(always)]
    fn from(variant: Tio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO1` writer - Tamper I/O 1 event"]
pub type Tio1W<'a, REG> = crate::BitWriter<'a, REG, Tio1>;
impl<'a, REG> Tio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio1::Clr)
    }
}
#[doc = "Tamper I/O 2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio2 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio2> for bool {
    #[inline(always)]
    fn from(variant: Tio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO2` writer - Tamper I/O 2 event"]
pub type Tio2W<'a, REG> = crate::BitWriter<'a, REG, Tio2>;
impl<'a, REG> Tio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio2::Clr)
    }
}
#[doc = "Tamper I/O 3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio3 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio3> for bool {
    #[inline(always)]
    fn from(variant: Tio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO3` writer - Tamper I/O 3 event"]
pub type Tio3W<'a, REG> = crate::BitWriter<'a, REG, Tio3>;
impl<'a, REG> Tio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio3::Clr)
    }
}
#[doc = "Tamper I/O 4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio4 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio4> for bool {
    #[inline(always)]
    fn from(variant: Tio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO4` writer - Tamper I/O 4 event"]
pub type Tio4W<'a, REG> = crate::BitWriter<'a, REG, Tio4>;
impl<'a, REG> Tio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio4::Clr)
    }
}
#[doc = "Tamper I/O 5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio5 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio5> for bool {
    #[inline(always)]
    fn from(variant: Tio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO5` writer - Tamper I/O 5 event"]
pub type Tio5W<'a, REG> = crate::BitWriter<'a, REG, Tio5>;
impl<'a, REG> Tio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio5::Clr)
    }
}
#[doc = "Tamper I/O 6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio6 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio6> for bool {
    #[inline(always)]
    fn from(variant: Tio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO6` writer - Tamper I/O 6 event"]
pub type Tio6W<'a, REG> = crate::BitWriter<'a, REG, Tio6>;
impl<'a, REG> Tio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio6::Clr)
    }
}
#[doc = "Tamper I/O 7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio7 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio7> for bool {
    #[inline(always)]
    fn from(variant: Tio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO7` writer - Tamper I/O 7 event"]
pub type Tio7W<'a, REG> = crate::BitWriter<'a, REG, Tio7>;
impl<'a, REG> Tio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio7::Clr)
    }
}
#[doc = "Tamper I/O 8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio8 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio8> for bool {
    #[inline(always)]
    fn from(variant: Tio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO8` writer - Tamper I/O 8 event"]
pub type Tio8W<'a, REG> = crate::BitWriter<'a, REG, Tio8>;
impl<'a, REG> Tio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio8::Clr)
    }
}
#[doc = "Tamper I/O 9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio9 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio9> for bool {
    #[inline(always)]
    fn from(variant: Tio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO9` writer - Tamper I/O 9 event"]
pub type Tio9W<'a, REG> = crate::BitWriter<'a, REG, Tio9>;
impl<'a, REG> Tio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio9::Clr)
    }
}
#[doc = "Tamper I/O 10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio10 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio10> for bool {
    #[inline(always)]
    fn from(variant: Tio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO10` writer - Tamper I/O 10 event"]
pub type Tio10W<'a, REG> = crate::BitWriter<'a, REG, Tio10>;
impl<'a, REG> Tio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio10::Clr)
    }
}
#[doc = "Tamper I/O 11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio11 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio11> for bool {
    #[inline(always)]
    fn from(variant: Tio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO11` writer - Tamper I/O 11 event"]
pub type Tio11W<'a, REG> = crate::BitWriter<'a, REG, Tio11>;
impl<'a, REG> Tio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio11::Clr)
    }
}
#[doc = "Tamper I/O 12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio12 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio12> for bool {
    #[inline(always)]
    fn from(variant: Tio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO12` writer - Tamper I/O 12 event"]
pub type Tio12W<'a, REG> = crate::BitWriter<'a, REG, Tio12>;
impl<'a, REG> Tio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio12::Clr)
    }
}
#[doc = "Tamper I/O 13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio13 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio13> for bool {
    #[inline(always)]
    fn from(variant: Tio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO13` writer - Tamper I/O 13 event"]
pub type Tio13W<'a, REG> = crate::BitWriter<'a, REG, Tio13>;
impl<'a, REG> Tio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio13::Clr)
    }
}
#[doc = "Tamper I/O 14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio14 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio14> for bool {
    #[inline(always)]
    fn from(variant: Tio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO14` writer - Tamper I/O 14 event"]
pub type Tio14W<'a, REG> = crate::BitWriter<'a, REG, Tio14>;
impl<'a, REG> Tio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio14::Clr)
    }
}
#[doc = "Tamper I/O 15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tio15 {
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
    #[doc = "1: Clear interrupt"]
    Clr = 1,
}
impl From<Tio15> for bool {
    #[inline(always)]
    fn from(variant: Tio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIO15` writer - Tamper I/O 15 event"]
pub type Tio15W<'a, REG> = crate::BitWriter<'a, REG, Tio15>;
impl<'a, REG> Tio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::NoEffect)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tio15::Clr)
    }
}
impl W {
    #[doc = "Bit 0 - RTC ready"]
    #[inline(always)]
    pub fn rtcrdy(&mut self) -> RtcrdyW<'_, LfssGenEventIclrSpec> {
        RtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RtctevW<'_, LfssGenEventIclrSpec> {
        RtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC alarm 1"]
    #[inline(always)]
    pub fn rtca1(&mut self) -> Rtca1W<'_, LfssGenEventIclrSpec> {
        Rtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC alarm 2"]
    #[inline(always)]
    pub fn rtca2(&mut self) -> Rtca2W<'_, LfssGenEventIclrSpec> {
        Rtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC prescale timer 0"]
    #[inline(always)]
    pub fn rt0ps(&mut self) -> Rt0psW<'_, LfssGenEventIclrSpec> {
        Rt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC prescale timer 1"]
    #[inline(always)]
    pub fn rt1ps(&mut self) -> Rt1psW<'_, LfssGenEventIclrSpec> {
        Rt1psW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC prescale timer 2"]
    #[inline(always)]
    pub fn rt2ps(&mut self) -> Rt2psW<'_, LfssGenEventIclrSpec> {
        Rt2psW::new(self, 6)
    }
    #[doc = "Bit 7 - Time stamp event"]
    #[inline(always)]
    pub fn tsevt(&mut self) -> TsevtW<'_, LfssGenEventIclrSpec> {
        TsevtW::new(self, 7)
    }
    #[doc = "Bit 8 - Tamper I/O 0 event"]
    #[inline(always)]
    pub fn tio0(&mut self) -> Tio0W<'_, LfssGenEventIclrSpec> {
        Tio0W::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper I/O 1 event"]
    #[inline(always)]
    pub fn tio1(&mut self) -> Tio1W<'_, LfssGenEventIclrSpec> {
        Tio1W::new(self, 9)
    }
    #[doc = "Bit 10 - Tamper I/O 2 event"]
    #[inline(always)]
    pub fn tio2(&mut self) -> Tio2W<'_, LfssGenEventIclrSpec> {
        Tio2W::new(self, 10)
    }
    #[doc = "Bit 11 - Tamper I/O 3 event"]
    #[inline(always)]
    pub fn tio3(&mut self) -> Tio3W<'_, LfssGenEventIclrSpec> {
        Tio3W::new(self, 11)
    }
    #[doc = "Bit 12 - Tamper I/O 4 event"]
    #[inline(always)]
    pub fn tio4(&mut self) -> Tio4W<'_, LfssGenEventIclrSpec> {
        Tio4W::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper I/O 5 event"]
    #[inline(always)]
    pub fn tio5(&mut self) -> Tio5W<'_, LfssGenEventIclrSpec> {
        Tio5W::new(self, 13)
    }
    #[doc = "Bit 14 - Tamper I/O 6 event"]
    #[inline(always)]
    pub fn tio6(&mut self) -> Tio6W<'_, LfssGenEventIclrSpec> {
        Tio6W::new(self, 14)
    }
    #[doc = "Bit 15 - Tamper I/O 7 event"]
    #[inline(always)]
    pub fn tio7(&mut self) -> Tio7W<'_, LfssGenEventIclrSpec> {
        Tio7W::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper I/O 8 event"]
    #[inline(always)]
    pub fn tio8(&mut self) -> Tio8W<'_, LfssGenEventIclrSpec> {
        Tio8W::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper I/O 9 event"]
    #[inline(always)]
    pub fn tio9(&mut self) -> Tio9W<'_, LfssGenEventIclrSpec> {
        Tio9W::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper I/O 10 event"]
    #[inline(always)]
    pub fn tio10(&mut self) -> Tio10W<'_, LfssGenEventIclrSpec> {
        Tio10W::new(self, 18)
    }
    #[doc = "Bit 19 - Tamper I/O 11 event"]
    #[inline(always)]
    pub fn tio11(&mut self) -> Tio11W<'_, LfssGenEventIclrSpec> {
        Tio11W::new(self, 19)
    }
    #[doc = "Bit 20 - Tamper I/O 12 event"]
    #[inline(always)]
    pub fn tio12(&mut self) -> Tio12W<'_, LfssGenEventIclrSpec> {
        Tio12W::new(self, 20)
    }
    #[doc = "Bit 21 - Tamper I/O 13 event"]
    #[inline(always)]
    pub fn tio13(&mut self) -> Tio13W<'_, LfssGenEventIclrSpec> {
        Tio13W::new(self, 21)
    }
    #[doc = "Bit 22 - Tamper I/O 14 event"]
    #[inline(always)]
    pub fn tio14(&mut self) -> Tio14W<'_, LfssGenEventIclrSpec> {
        Tio14W::new(self, 22)
    }
    #[doc = "Bit 23 - Tamper I/O 15 event"]
    #[inline(always)]
    pub fn tio15(&mut self) -> Tio15W<'_, LfssGenEventIclrSpec> {
        Tio15W::new(self, 23)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_gen_event_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssGenEventIclrSpec;
impl crate::RegisterSpec for LfssGenEventIclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lfss_gen_event_iclr::W`](W) writer structure"]
impl crate::Writable for LfssGenEventIclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_GEN_EVENT_ICLR to value 0"]
impl crate::Resettable for LfssGenEventIclrSpec {}
