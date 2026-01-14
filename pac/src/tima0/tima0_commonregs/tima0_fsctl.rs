#[doc = "Register `TIMA0_FSCTL` reader"]
pub type R = crate::R<Tima0FsctlSpec>;
#[doc = "Register `TIMA0_FSCTL` writer"]
pub type W = crate::W<Tima0FsctlSpec>;
#[doc = "This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fcen> for bool {
    #[inline(always)]
    fn from(variant: Fcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCEN` reader - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
pub type FcenR = crate::BitReader<Fcen>;
impl FcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcen {
        match self.bits {
            false => Fcen::Disable,
            true => Fcen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fcen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fcen::Enable
    }
}
#[doc = "Field `FCEN` writer - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
pub type FcenW<'a, REG> = crate::BitWriter<'a, REG, Fcen>;
impl<'a, REG> FcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fcen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fcen::Enable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fac0en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fac0en> for bool {
    #[inline(always)]
    fn from(variant: Fac0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAC0EN` reader - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
pub type Fac0enR = crate::BitReader<Fac0en>;
impl Fac0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fac0en {
        match self.bits {
            false => Fac0en::Disable,
            true => Fac0en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fac0en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fac0en::Enable
    }
}
#[doc = "Field `FAC0EN` writer - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
pub type Fac0enW<'a, REG> = crate::BitWriter<'a, REG, Fac0en>;
impl<'a, REG> Fac0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac0en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac0en::Enable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fac1en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fac1en> for bool {
    #[inline(always)]
    fn from(variant: Fac1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAC1EN` reader - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
pub type Fac1enR = crate::BitReader<Fac1en>;
impl Fac1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fac1en {
        match self.bits {
            false => Fac1en::Disable,
            true => Fac1en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fac1en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fac1en::Enable
    }
}
#[doc = "Field `FAC1EN` writer - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
pub type Fac1enW<'a, REG> = crate::BitWriter<'a, REG, Fac1en>;
impl<'a, REG> Fac1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac1en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac1en::Enable)
    }
}
#[doc = "This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fac2en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fac2en> for bool {
    #[inline(always)]
    fn from(variant: Fac2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAC2EN` reader - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
pub type Fac2enR = crate::BitReader<Fac2en>;
impl Fac2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fac2en {
        match self.bits {
            false => Fac2en::Disable,
            true => Fac2en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fac2en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fac2en::Enable
    }
}
#[doc = "Field `FAC2EN` writer - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
pub type Fac2enW<'a, REG> = crate::BitWriter<'a, REG, Fac2en>;
impl<'a, REG> Fac2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac2en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fac2en::Enable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fex0en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fex0en> for bool {
    #[inline(always)]
    fn from(variant: Fex0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEX0EN` reader - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex0enR = crate::BitReader<Fex0en>;
impl Fex0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fex0en {
        match self.bits {
            false => Fex0en::Disable,
            true => Fex0en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fex0en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fex0en::Enable
    }
}
#[doc = "Field `FEX0EN` writer - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex0enW<'a, REG> = crate::BitWriter<'a, REG, Fex0en>;
impl<'a, REG> Fex0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex0en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex0en::Enable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fex1en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fex1en> for bool {
    #[inline(always)]
    fn from(variant: Fex1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEX1EN` reader - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex1enR = crate::BitReader<Fex1en>;
impl Fex1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fex1en {
        match self.bits {
            false => Fex1en::Disable,
            true => Fex1en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fex1en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fex1en::Enable
    }
}
#[doc = "Field `FEX1EN` writer - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex1enW<'a, REG> = crate::BitWriter<'a, REG, Fex1en>;
impl<'a, REG> Fex1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex1en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex1en::Enable)
    }
}
#[doc = "This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fex2en {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fex2en> for bool {
    #[inline(always)]
    fn from(variant: Fex2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEX2EN` reader - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex2enR = crate::BitReader<Fex2en>;
impl Fex2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fex2en {
        match self.bits {
            false => Fex2en::Disable,
            true => Fex2en::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fex2en::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fex2en::Enable
    }
}
#[doc = "Field `FEX2EN` writer - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
pub type Fex2enW<'a, REG> = crate::BitWriter<'a, REG, Fex2en>;
impl<'a, REG> Fex2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex2en::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fex2en::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fcen(&self) -> FcenR {
        FcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac0en(&self) -> Fac0enR {
        Fac0enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac1en(&self) -> Fac1enR {
        Fac1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac2en(&self) -> Fac2enR {
        Fac2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex0en(&self) -> Fex0enR {
        Fex0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex1en(&self) -> Fex1enR {
        Fex1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex2en(&self) -> Fex2enR {
        Fex2enR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field controls whether the fault caused by the system clock fault is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fcen(&mut self) -> FcenW<'_, Tima0FsctlSpec> {
        FcenW::new(self, 0)
    }
    #[doc = "Bit 1 - This field controls whether the fault signal detected by the analog comparator0 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac0en(&mut self) -> Fac0enW<'_, Tima0FsctlSpec> {
        Fac0enW::new(self, 1)
    }
    #[doc = "Bit 2 - This field controls whether the fault signal detected by the analog comparator1 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac1en(&mut self) -> Fac1enW<'_, Tima0FsctlSpec> {
        Fac1enW::new(self, 2)
    }
    #[doc = "Bit 3 - This field controls whether the fault signal detected by the analog comparator2 is enable 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fac2en(&mut self) -> Fac2enW<'_, Tima0FsctlSpec> {
        Fac2enW::new(self, 3)
    }
    #[doc = "Bit 4 - This field controls whether the fault caused by external fault pin0 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex0en(&mut self) -> Fex0enW<'_, Tima0FsctlSpec> {
        Fex0enW::new(self, 4)
    }
    #[doc = "Bit 5 - This field controls whether the fault caused by external fault pin1 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex1en(&mut self) -> Fex1enW<'_, Tima0FsctlSpec> {
        Fex1enW::new(self, 5)
    }
    #[doc = "Bit 6 - This field controls whether the fault caused by external fault pin2 is enable. 0: DISABLE 1: ENABLE"]
    #[inline(always)]
    pub fn fex2en(&mut self) -> Fex2enW<'_, Tima0FsctlSpec> {
        Fex2enW::new(self, 6)
    }
}
#[doc = "Fault Source Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tima0_fsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tima0_fsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tima0FsctlSpec;
impl crate::RegisterSpec for Tima0FsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tima0_fsctl::R`](R) reader structure"]
impl crate::Readable for Tima0FsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tima0_fsctl::W`](W) writer structure"]
impl crate::Writable for Tima0FsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMA0_FSCTL to value 0"]
impl crate::Resettable for Tima0FsctlSpec {}
