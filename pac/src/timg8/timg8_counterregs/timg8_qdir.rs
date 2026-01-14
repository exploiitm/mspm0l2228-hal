#[doc = "Register `TIMG8_QDIR` reader"]
pub type R = crate::R<Timg8QdirSpec>;
#[doc = "Direction of count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Down (Phase B leads Phase A)"]
    Down = 0,
    #[doc = "1: Up (Phase A leads Phase B)"]
    Up = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction of count"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Down,
            true => Dir::Up,
        }
    }
    #[doc = "Down (Phase B leads Phase A)"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Dir::Down
    }
    #[doc = "Up (Phase A leads Phase B)"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Dir::Up
    }
}
impl R {
    #[doc = "Bit 0 - Direction of count"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
}
#[doc = "Count Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg8_qdir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timg8QdirSpec;
impl crate::RegisterSpec for Timg8QdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg8_qdir::R`](R) reader structure"]
impl crate::Readable for Timg8QdirSpec {}
#[doc = "`reset()` method sets TIMG8_QDIR to value 0"]
impl crate::Resettable for Timg8QdirSpec {}
