#[doc = "Register `LFSS_TIOCTL[%s]` reader"]
pub type R = crate::R<LfssTioctlSpec>;
#[doc = "Register `LFSS_TIOCTL[%s]` writer"]
pub type W = crate::W<LfssTioctlSpec>;
#[doc = "tamper I/O is controlled by SoC IOMUX module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iomux {
    #[doc = "0: The tamper I/O is controlled by the IOMUX of the SoC and does allow assignment to a peripheral function. In the case the main supply (VDD) is lost, this I/O will go into a Hi-Z state."]
    Iomux = 0,
    #[doc = "1: The tamper I/O is controlled by the TIOCTL register and stays functional during loss of the main supply (VDD)."]
    Tamper = 1,
}
impl From<Iomux> for bool {
    #[inline(always)]
    fn from(variant: Iomux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOMUX` reader - tamper I/O is controlled by SoC IOMUX module"]
pub type IomuxR = crate::BitReader<Iomux>;
impl IomuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iomux {
        match self.bits {
            false => Iomux::Iomux,
            true => Iomux::Tamper,
        }
    }
    #[doc = "The tamper I/O is controlled by the IOMUX of the SoC and does allow assignment to a peripheral function. In the case the main supply (VDD) is lost, this I/O will go into a Hi-Z state."]
    #[inline(always)]
    pub fn is_iomux(&self) -> bool {
        *self == Iomux::Iomux
    }
    #[doc = "The tamper I/O is controlled by the TIOCTL register and stays functional during loss of the main supply (VDD)."]
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == Iomux::Tamper
    }
}
#[doc = "Field `IOMUX` writer - tamper I/O is controlled by SoC IOMUX module"]
pub type IomuxW<'a, REG> = crate::BitWriter<'a, REG, Iomux>;
impl<'a, REG> IomuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The tamper I/O is controlled by the IOMUX of the SoC and does allow assignment to a peripheral function. In the case the main supply (VDD) is lost, this I/O will go into a Hi-Z state."]
    #[inline(always)]
    pub fn iomux(self) -> &'a mut crate::W<REG> {
        self.variant(Iomux::Iomux)
    }
    #[doc = "The tamper I/O is controlled by the TIOCTL register and stays functional during loss of the main supply (VDD)."]
    #[inline(always)]
    pub fn tamper(self) -> &'a mut crate::W<REG> {
        self.variant(Iomux::Tamper)
    }
}
#[doc = "Selects the source for TOUT control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Toutsel {
    #[doc = "0: The TOUT register is the source for TOUT"]
    Tout = 0,
    #[doc = "1: The LFCLK is the source for TOUT"]
    Lfclkext = 1,
    #[doc = "2: The heart beat generatore is the source for TOUT"]
    Heartbeat = 2,
    #[doc = "3: The time stamp event status is the source for TOUT"]
    Tsevtstat = 3,
}
impl From<Toutsel> for u8 {
    #[inline(always)]
    fn from(variant: Toutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Toutsel {
    type Ux = u8;
}
impl crate::IsEnum for Toutsel {}
#[doc = "Field `TOUTSEL` reader - Selects the source for TOUT control"]
pub type ToutselR = crate::FieldReader<Toutsel>;
impl ToutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toutsel {
        match self.bits {
            0 => Toutsel::Tout,
            1 => Toutsel::Lfclkext,
            2 => Toutsel::Heartbeat,
            3 => Toutsel::Tsevtstat,
            _ => unreachable!(),
        }
    }
    #[doc = "The TOUT register is the source for TOUT"]
    #[inline(always)]
    pub fn is_tout(&self) -> bool {
        *self == Toutsel::Tout
    }
    #[doc = "The LFCLK is the source for TOUT"]
    #[inline(always)]
    pub fn is_lfclkext(&self) -> bool {
        *self == Toutsel::Lfclkext
    }
    #[doc = "The heart beat generatore is the source for TOUT"]
    #[inline(always)]
    pub fn is_heartbeat(&self) -> bool {
        *self == Toutsel::Heartbeat
    }
    #[doc = "The time stamp event status is the source for TOUT"]
    #[inline(always)]
    pub fn is_tsevtstat(&self) -> bool {
        *self == Toutsel::Tsevtstat
    }
}
#[doc = "Field `TOUTSEL` writer - Selects the source for TOUT control"]
pub type ToutselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Toutsel, crate::Safe>;
impl<'a, REG> ToutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The TOUT register is the source for TOUT"]
    #[inline(always)]
    pub fn tout(self) -> &'a mut crate::W<REG> {
        self.variant(Toutsel::Tout)
    }
    #[doc = "The LFCLK is the source for TOUT"]
    #[inline(always)]
    pub fn lfclkext(self) -> &'a mut crate::W<REG> {
        self.variant(Toutsel::Lfclkext)
    }
    #[doc = "The heart beat generatore is the source for TOUT"]
    #[inline(always)]
    pub fn heartbeat(self) -> &'a mut crate::W<REG> {
        self.variant(Toutsel::Heartbeat)
    }
    #[doc = "The time stamp event status is the source for TOUT"]
    #[inline(always)]
    pub fn tsevtstat(self) -> &'a mut crate::W<REG> {
        self.variant(Toutsel::Tsevtstat)
    }
}
#[doc = "Enables and configures edge detection polarity for TIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    #[doc = "0: Edge detection disabled"]
    Disable = 0,
    #[doc = "1: Detects rising edge of input event"]
    Rise = 1,
    #[doc = "2: Detects falling edge of input event"]
    Fall = 2,
    #[doc = "3: Detects both rising and falling edge of input event"]
    Both = 3,
}
impl From<Polarity> for u8 {
    #[inline(always)]
    fn from(variant: Polarity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity {
    type Ux = u8;
}
impl crate::IsEnum for Polarity {}
#[doc = "Field `POLARITY` reader - Enables and configures edge detection polarity for TIO"]
pub type PolarityR = crate::FieldReader<Polarity>;
impl PolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity {
        match self.bits {
            0 => Polarity::Disable,
            1 => Polarity::Rise,
            2 => Polarity::Fall,
            3 => Polarity::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Polarity::Disable
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Polarity::Rise
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Polarity::Fall
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Polarity::Both
    }
}
#[doc = "Field `POLARITY` writer - Enables and configures edge detection polarity for TIO"]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity, crate::Safe>;
impl<'a, REG> PolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Edge detection disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Disable)
    }
    #[doc = "Detects rising edge of input event"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Rise)
    }
    #[doc = "Detects falling edge of input event"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Fall)
    }
    #[doc = "Detects both rising and falling edge of input event"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Both)
    }
}
#[doc = "Programmable counter length of digital glitch filter for TIO0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren {
    #[doc = "0: no filter on the tamper I/O beyond CDC synchronization sample"]
    NoFlt = 0,
    #[doc = "1: 1 FLCLK minimum sample (30us)"]
    Flt1 = 1,
    #[doc = "2: 3 LFCLK minimum sample (100us)"]
    Flt2 = 2,
    #[doc = "3: 6 LFCLK minimum sample (200us)"]
    Flt3 = 3,
}
impl From<Filteren> for u8 {
    #[inline(always)]
    fn from(variant: Filteren) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren {
    type Ux = u8;
}
impl crate::IsEnum for Filteren {}
#[doc = "Field `FILTEREN` reader - Programmable counter length of digital glitch filter for TIO0"]
pub type FilterenR = crate::FieldReader<Filteren>;
impl FilterenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren {
        match self.bits {
            0 => Filteren::NoFlt,
            1 => Filteren::Flt1,
            2 => Filteren::Flt2,
            3 => Filteren::Flt3,
            _ => unreachable!(),
        }
    }
    #[doc = "no filter on the tamper I/O beyond CDC synchronization sample"]
    #[inline(always)]
    pub fn is_no_flt(&self) -> bool {
        *self == Filteren::NoFlt
    }
    #[doc = "1 FLCLK minimum sample (30us)"]
    #[inline(always)]
    pub fn is_flt_1(&self) -> bool {
        *self == Filteren::Flt1
    }
    #[doc = "3 LFCLK minimum sample (100us)"]
    #[inline(always)]
    pub fn is_flt_2(&self) -> bool {
        *self == Filteren::Flt2
    }
    #[doc = "6 LFCLK minimum sample (200us)"]
    #[inline(always)]
    pub fn is_flt_3(&self) -> bool {
        *self == Filteren::Flt3
    }
}
#[doc = "Field `FILTEREN` writer - Programmable counter length of digital glitch filter for TIO0"]
pub type FilterenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren, crate::Safe>;
impl<'a, REG> FilterenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no filter on the tamper I/O beyond CDC synchronization sample"]
    #[inline(always)]
    pub fn no_flt(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren::NoFlt)
    }
    #[doc = "1 FLCLK minimum sample (30us)"]
    #[inline(always)]
    pub fn flt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren::Flt1)
    }
    #[doc = "3 LFCLK minimum sample (100us)"]
    #[inline(always)]
    pub fn flt_2(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren::Flt2)
    }
    #[doc = "6 LFCLK minimum sample (200us)"]
    #[inline(always)]
    pub fn flt_3(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren::Flt3)
    }
}
#[doc = "pull down enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipd {
    #[doc = "0: The pull-down function is disabled."]
    Disable = 0,
    #[doc = "1: The pull-down function is enabled."]
    Enable = 1,
}
impl From<Pipd> for bool {
    #[inline(always)]
    fn from(variant: Pipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPD` reader - pull down enable"]
pub type PipdR = crate::BitReader<Pipd>;
impl PipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipd {
        match self.bits {
            false => Pipd::Disable,
            true => Pipd::Enable,
        }
    }
    #[doc = "The pull-down function is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pipd::Disable
    }
    #[doc = "The pull-down function is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pipd::Enable
    }
}
#[doc = "Field `PIPD` writer - pull down enable"]
pub type PipdW<'a, REG> = crate::BitWriter<'a, REG, Pipd>;
impl<'a, REG> PipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pull-down function is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::Disable)
    }
    #[doc = "The pull-down function is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipd::Enable)
    }
}
#[doc = "pull up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipu {
    #[doc = "0: The pull-up function is disabled."]
    Disable = 0,
    #[doc = "1: The pull-up function is enabled."]
    Enable = 1,
}
impl From<Pipu> for bool {
    #[inline(always)]
    fn from(variant: Pipu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIPU` reader - pull up enable"]
pub type PipuR = crate::BitReader<Pipu>;
impl PipuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pipu {
        match self.bits {
            false => Pipu::Disable,
            true => Pipu::Enable,
        }
    }
    #[doc = "The pull-up function is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pipu::Disable
    }
    #[doc = "The pull-up function is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pipu::Enable
    }
}
#[doc = "Field `PIPU` writer - pull up enable"]
pub type PipuW<'a, REG> = crate::BitWriter<'a, REG, Pipu>;
impl<'a, REG> PipuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pull-up function is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::Disable)
    }
    #[doc = "The pull-up function is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pipu::Enable)
    }
}
#[doc = "input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inena {
    #[doc = "0: The input path is disabled."]
    Disable = 0,
    #[doc = "1: The input path is enabled."]
    Enable = 1,
}
impl From<Inena> for bool {
    #[inline(always)]
    fn from(variant: Inena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INENA` reader - input enable"]
pub type InenaR = crate::BitReader<Inena>;
impl InenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inena {
        match self.bits {
            false => Inena::Disable,
            true => Inena::Enable,
        }
    }
    #[doc = "The input path is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inena::Disable
    }
    #[doc = "The input path is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inena::Enable
    }
}
#[doc = "Field `INENA` writer - input enable"]
pub type InenaW<'a, REG> = crate::BitWriter<'a, REG, Inena>;
impl<'a, REG> InenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input path is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Disable)
    }
    #[doc = "The input path is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inena::Enable)
    }
}
#[doc = "Output inversion enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outinv {
    #[doc = "0: The output inversion is disabled."]
    Disbale = 0,
    #[doc = "1: The output inversion is enabled."]
    Enable = 1,
}
impl From<Outinv> for bool {
    #[inline(always)]
    fn from(variant: Outinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTINV` reader - Output inversion enable"]
pub type OutinvR = crate::BitReader<Outinv>;
impl OutinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outinv {
        match self.bits {
            false => Outinv::Disbale,
            true => Outinv::Enable,
        }
    }
    #[doc = "The output inversion is disabled."]
    #[inline(always)]
    pub fn is_disbale(&self) -> bool {
        *self == Outinv::Disbale
    }
    #[doc = "The output inversion is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Outinv::Enable
    }
}
#[doc = "Field `OUTINV` writer - Output inversion enable"]
pub type OutinvW<'a, REG> = crate::BitWriter<'a, REG, Outinv>;
impl<'a, REG> OutinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output inversion is disabled."]
    #[inline(always)]
    pub fn disbale(self) -> &'a mut crate::W<REG> {
        self.variant(Outinv::Disbale)
    }
    #[doc = "The output inversion is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Outinv::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - tamper I/O is controlled by SoC IOMUX module"]
    #[inline(always)]
    pub fn iomux(&self) -> IomuxR {
        IomuxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Selects the source for TOUT control"]
    #[inline(always)]
    pub fn toutsel(&self) -> ToutselR {
        ToutselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for TIO"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for TIO0"]
    #[inline(always)]
    pub fn filteren(&self) -> FilterenR {
        FilterenR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - pull down enable"]
    #[inline(always)]
    pub fn pipd(&self) -> PipdR {
        PipdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pull up enable"]
    #[inline(always)]
    pub fn pipu(&self) -> PipuR {
        PipuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - input enable"]
    #[inline(always)]
    pub fn inena(&self) -> InenaR {
        InenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output inversion enable"]
    #[inline(always)]
    pub fn outinv(&self) -> OutinvR {
        OutinvR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tamper I/O is controlled by SoC IOMUX module"]
    #[inline(always)]
    pub fn iomux(&mut self) -> IomuxW<'_, LfssTioctlSpec> {
        IomuxW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Selects the source for TOUT control"]
    #[inline(always)]
    pub fn toutsel(&mut self) -> ToutselW<'_, LfssTioctlSpec> {
        ToutselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for TIO"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, LfssTioctlSpec> {
        PolarityW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for TIO0"]
    #[inline(always)]
    pub fn filteren(&mut self) -> FilterenW<'_, LfssTioctlSpec> {
        FilterenW::new(self, 12)
    }
    #[doc = "Bit 16 - pull down enable"]
    #[inline(always)]
    pub fn pipd(&mut self) -> PipdW<'_, LfssTioctlSpec> {
        PipdW::new(self, 16)
    }
    #[doc = "Bit 17 - pull up enable"]
    #[inline(always)]
    pub fn pipu(&mut self) -> PipuW<'_, LfssTioctlSpec> {
        PipuW::new(self, 17)
    }
    #[doc = "Bit 18 - input enable"]
    #[inline(always)]
    pub fn inena(&mut self) -> InenaW<'_, LfssTioctlSpec> {
        InenaW::new(self, 18)
    }
    #[doc = "Bit 19 - Output inversion enable"]
    #[inline(always)]
    pub fn outinv(&mut self) -> OutinvW<'_, LfssTioctlSpec> {
        OutinvW::new(self, 19)
    }
}
#[doc = "Tamper I/O Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_tioctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_tioctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssTioctlSpec;
impl crate::RegisterSpec for LfssTioctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_tioctl::R`](R) reader structure"]
impl crate::Readable for LfssTioctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_tioctl::W`](W) writer structure"]
impl crate::Writable for LfssTioctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_TIOCTL[%s] to value 0"]
impl crate::Resettable for LfssTioctlSpec {}
