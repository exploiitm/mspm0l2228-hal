#[doc = "Register `LFSS_HEARTBEAT` reader"]
pub type R = crate::R<LfssHeartbeatSpec>;
#[doc = "Register `LFSS_HEARTBEAT` writer"]
pub type W = crate::W<LfssHeartbeatSpec>;
#[doc = "Heart beat interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hbinterval {
    #[doc = "0: Heart beat interval 0.125 sec"]
    Hbint0p125 = 0,
    #[doc = "1: Heart beat interval 0.25 sec"]
    Hbint0p25 = 1,
    #[doc = "2: Heart beat interval 0.5 sec"]
    Hbint0p5 = 2,
    #[doc = "3: Heart beat interval 1 sec"]
    Hbint1 = 3,
    #[doc = "4: Heart beat interval 2 sec"]
    Hbint2 = 4,
    #[doc = "5: Heart beat interval 4 sec"]
    Hbint4 = 5,
    #[doc = "6: Heart beat interval 8 sec"]
    Hbint8 = 6,
    #[doc = "7: Heart beat interval 16 sec"]
    Hbint16 = 7,
}
impl From<Hbinterval> for u8 {
    #[inline(always)]
    fn from(variant: Hbinterval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hbinterval {
    type Ux = u8;
}
impl crate::IsEnum for Hbinterval {}
#[doc = "Field `HBINTERVAL` reader - Heart beat interval"]
pub type HbintervalR = crate::FieldReader<Hbinterval>;
impl HbintervalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hbinterval {
        match self.bits {
            0 => Hbinterval::Hbint0p125,
            1 => Hbinterval::Hbint0p25,
            2 => Hbinterval::Hbint0p5,
            3 => Hbinterval::Hbint1,
            4 => Hbinterval::Hbint2,
            5 => Hbinterval::Hbint4,
            6 => Hbinterval::Hbint8,
            7 => Hbinterval::Hbint16,
            _ => unreachable!(),
        }
    }
    #[doc = "Heart beat interval 0.125 sec"]
    #[inline(always)]
    pub fn is_hbint0p125(&self) -> bool {
        *self == Hbinterval::Hbint0p125
    }
    #[doc = "Heart beat interval 0.25 sec"]
    #[inline(always)]
    pub fn is_hbint0p25(&self) -> bool {
        *self == Hbinterval::Hbint0p25
    }
    #[doc = "Heart beat interval 0.5 sec"]
    #[inline(always)]
    pub fn is_hbint0p5(&self) -> bool {
        *self == Hbinterval::Hbint0p5
    }
    #[doc = "Heart beat interval 1 sec"]
    #[inline(always)]
    pub fn is_hbint1(&self) -> bool {
        *self == Hbinterval::Hbint1
    }
    #[doc = "Heart beat interval 2 sec"]
    #[inline(always)]
    pub fn is_hbint2(&self) -> bool {
        *self == Hbinterval::Hbint2
    }
    #[doc = "Heart beat interval 4 sec"]
    #[inline(always)]
    pub fn is_hbint4(&self) -> bool {
        *self == Hbinterval::Hbint4
    }
    #[doc = "Heart beat interval 8 sec"]
    #[inline(always)]
    pub fn is_hbint8(&self) -> bool {
        *self == Hbinterval::Hbint8
    }
    #[doc = "Heart beat interval 16 sec"]
    #[inline(always)]
    pub fn is_hbint16(&self) -> bool {
        *self == Hbinterval::Hbint16
    }
}
#[doc = "Field `HBINTERVAL` writer - Heart beat interval"]
pub type HbintervalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hbinterval, crate::Safe>;
impl<'a, REG> HbintervalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Heart beat interval 0.125 sec"]
    #[inline(always)]
    pub fn hbint0p125(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint0p125)
    }
    #[doc = "Heart beat interval 0.25 sec"]
    #[inline(always)]
    pub fn hbint0p25(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint0p25)
    }
    #[doc = "Heart beat interval 0.5 sec"]
    #[inline(always)]
    pub fn hbint0p5(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint0p5)
    }
    #[doc = "Heart beat interval 1 sec"]
    #[inline(always)]
    pub fn hbint1(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint1)
    }
    #[doc = "Heart beat interval 2 sec"]
    #[inline(always)]
    pub fn hbint2(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint2)
    }
    #[doc = "Heart beat interval 4 sec"]
    #[inline(always)]
    pub fn hbint4(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint4)
    }
    #[doc = "Heart beat interval 8 sec"]
    #[inline(always)]
    pub fn hbint8(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint8)
    }
    #[doc = "Heart beat interval 16 sec"]
    #[inline(always)]
    pub fn hbint16(self) -> &'a mut crate::W<REG> {
        self.variant(Hbinterval::Hbint16)
    }
}
#[doc = "Heart beat interval width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hbwidth {
    #[doc = "0: Heart beat pulse width 1 msec"]
    Hbpwdth1 = 0,
    #[doc = "1: Heart beat pulse width 2 msec"]
    Hbpwdth2 = 1,
    #[doc = "2: Heart beat pulse width 4 msec"]
    Hbpwdth4 = 2,
    #[doc = "3: Heart beat pulse width 8 msec"]
    Hbpwdth8 = 3,
    #[doc = "4: Heart beat pulse width 16 msec"]
    Hbpwdth16 = 4,
    #[doc = "5: Heart beat pulse width 32 msec"]
    Hbpwdth32 = 5,
    #[doc = "6: Heart beat pulse width 64 msec"]
    Hbpwdth64 = 6,
    #[doc = "7: Heart beat pulse width 128 msec"]
    Hbpwdth128 = 7,
}
impl From<Hbwidth> for u8 {
    #[inline(always)]
    fn from(variant: Hbwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hbwidth {
    type Ux = u8;
}
impl crate::IsEnum for Hbwidth {}
#[doc = "Field `HBWIDTH` reader - Heart beat interval width"]
pub type HbwidthR = crate::FieldReader<Hbwidth>;
impl HbwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hbwidth {
        match self.bits {
            0 => Hbwidth::Hbpwdth1,
            1 => Hbwidth::Hbpwdth2,
            2 => Hbwidth::Hbpwdth4,
            3 => Hbwidth::Hbpwdth8,
            4 => Hbwidth::Hbpwdth16,
            5 => Hbwidth::Hbpwdth32,
            6 => Hbwidth::Hbpwdth64,
            7 => Hbwidth::Hbpwdth128,
            _ => unreachable!(),
        }
    }
    #[doc = "Heart beat pulse width 1 msec"]
    #[inline(always)]
    pub fn is_hbpwdth1(&self) -> bool {
        *self == Hbwidth::Hbpwdth1
    }
    #[doc = "Heart beat pulse width 2 msec"]
    #[inline(always)]
    pub fn is_hbpwdth2(&self) -> bool {
        *self == Hbwidth::Hbpwdth2
    }
    #[doc = "Heart beat pulse width 4 msec"]
    #[inline(always)]
    pub fn is_hbpwdth4(&self) -> bool {
        *self == Hbwidth::Hbpwdth4
    }
    #[doc = "Heart beat pulse width 8 msec"]
    #[inline(always)]
    pub fn is_hbpwdth8(&self) -> bool {
        *self == Hbwidth::Hbpwdth8
    }
    #[doc = "Heart beat pulse width 16 msec"]
    #[inline(always)]
    pub fn is_hbpwdth16(&self) -> bool {
        *self == Hbwidth::Hbpwdth16
    }
    #[doc = "Heart beat pulse width 32 msec"]
    #[inline(always)]
    pub fn is_hbpwdth32(&self) -> bool {
        *self == Hbwidth::Hbpwdth32
    }
    #[doc = "Heart beat pulse width 64 msec"]
    #[inline(always)]
    pub fn is_hbpwdth64(&self) -> bool {
        *self == Hbwidth::Hbpwdth64
    }
    #[doc = "Heart beat pulse width 128 msec"]
    #[inline(always)]
    pub fn is_hbpwdth128(&self) -> bool {
        *self == Hbwidth::Hbpwdth128
    }
}
#[doc = "Field `HBWIDTH` writer - Heart beat interval width"]
pub type HbwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hbwidth, crate::Safe>;
impl<'a, REG> HbwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Heart beat pulse width 1 msec"]
    #[inline(always)]
    pub fn hbpwdth1(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth1)
    }
    #[doc = "Heart beat pulse width 2 msec"]
    #[inline(always)]
    pub fn hbpwdth2(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth2)
    }
    #[doc = "Heart beat pulse width 4 msec"]
    #[inline(always)]
    pub fn hbpwdth4(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth4)
    }
    #[doc = "Heart beat pulse width 8 msec"]
    #[inline(always)]
    pub fn hbpwdth8(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth8)
    }
    #[doc = "Heart beat pulse width 16 msec"]
    #[inline(always)]
    pub fn hbpwdth16(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth16)
    }
    #[doc = "Heart beat pulse width 32 msec"]
    #[inline(always)]
    pub fn hbpwdth32(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth32)
    }
    #[doc = "Heart beat pulse width 64 msec"]
    #[inline(always)]
    pub fn hbpwdth64(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth64)
    }
    #[doc = "Heart beat pulse width 128 msec"]
    #[inline(always)]
    pub fn hbpwdth128(self) -> &'a mut crate::W<REG> {
        self.variant(Hbwidth::Hbpwdth128)
    }
}
#[doc = "Heart beat mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hbmode {
    #[doc = "0: Heart beat disabled"]
    HbDis = 0,
    #[doc = "1: Heart beat allways enabled"]
    HbAllways = 1,
    #[doc = "2: Heart beat enabled when time stamp event detected"]
    HbTs = 2,
    #[doc = "3: Heart beat when VDD has fail condition"]
    HbVddfail = 3,
}
impl From<Hbmode> for u8 {
    #[inline(always)]
    fn from(variant: Hbmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hbmode {
    type Ux = u8;
}
impl crate::IsEnum for Hbmode {}
#[doc = "Field `HBMODE` reader - Heart beat mode"]
pub type HbmodeR = crate::FieldReader<Hbmode>;
impl HbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hbmode {
        match self.bits {
            0 => Hbmode::HbDis,
            1 => Hbmode::HbAllways,
            2 => Hbmode::HbTs,
            3 => Hbmode::HbVddfail,
            _ => unreachable!(),
        }
    }
    #[doc = "Heart beat disabled"]
    #[inline(always)]
    pub fn is_hb_dis(&self) -> bool {
        *self == Hbmode::HbDis
    }
    #[doc = "Heart beat allways enabled"]
    #[inline(always)]
    pub fn is_hb_allways(&self) -> bool {
        *self == Hbmode::HbAllways
    }
    #[doc = "Heart beat enabled when time stamp event detected"]
    #[inline(always)]
    pub fn is_hb_ts(&self) -> bool {
        *self == Hbmode::HbTs
    }
    #[doc = "Heart beat when VDD has fail condition"]
    #[inline(always)]
    pub fn is_hb_vddfail(&self) -> bool {
        *self == Hbmode::HbVddfail
    }
}
#[doc = "Field `HBMODE` writer - Heart beat mode"]
pub type HbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hbmode, crate::Safe>;
impl<'a, REG> HbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Heart beat disabled"]
    #[inline(always)]
    pub fn hb_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hbmode::HbDis)
    }
    #[doc = "Heart beat allways enabled"]
    #[inline(always)]
    pub fn hb_allways(self) -> &'a mut crate::W<REG> {
        self.variant(Hbmode::HbAllways)
    }
    #[doc = "Heart beat enabled when time stamp event detected"]
    #[inline(always)]
    pub fn hb_ts(self) -> &'a mut crate::W<REG> {
        self.variant(Hbmode::HbTs)
    }
    #[doc = "Heart beat when VDD has fail condition"]
    #[inline(always)]
    pub fn hb_vddfail(self) -> &'a mut crate::W<REG> {
        self.variant(Hbmode::HbVddfail)
    }
}
impl R {
    #[doc = "Bits 0:2 - Heart beat interval"]
    #[inline(always)]
    pub fn hbinterval(&self) -> HbintervalR {
        HbintervalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Heart beat interval width"]
    #[inline(always)]
    pub fn hbwidth(&self) -> HbwidthR {
        HbwidthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Heart beat mode"]
    #[inline(always)]
    pub fn hbmode(&self) -> HbmodeR {
        HbmodeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Heart beat interval"]
    #[inline(always)]
    pub fn hbinterval(&mut self) -> HbintervalW<'_, LfssHeartbeatSpec> {
        HbintervalW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Heart beat interval width"]
    #[inline(always)]
    pub fn hbwidth(&mut self) -> HbwidthW<'_, LfssHeartbeatSpec> {
        HbwidthW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Heart beat mode"]
    #[inline(always)]
    pub fn hbmode(&mut self) -> HbmodeW<'_, LfssHeartbeatSpec> {
        HbmodeW::new(self, 16)
    }
}
#[doc = "Heartbeat Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfss_heartbeat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfss_heartbeat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfssHeartbeatSpec;
impl crate::RegisterSpec for LfssHeartbeatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfss_heartbeat::R`](R) reader structure"]
impl crate::Readable for LfssHeartbeatSpec {}
#[doc = "`write(|w| ..)` method takes [`lfss_heartbeat::W`](W) writer structure"]
impl crate::Writable for LfssHeartbeatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSS_HEARTBEAT to value 0"]
impl crate::Resettable for LfssHeartbeatSpec {}
