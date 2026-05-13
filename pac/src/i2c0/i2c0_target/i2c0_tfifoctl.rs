#[doc = "Register `I2C0_TFIFOCTL` reader"]
pub type R = crate::R<I2c0TfifoctlSpec>;
#[doc = "Register `I2C0_TFIFOCTL` writer"]
pub type W = crate::W<I2c0TfifoctlSpec>;
#[doc = "TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txtrig {
    #[doc = "4: Trigger when TX FIFO contains 4 byte"]
    Level4 = 4,
    #[doc = "5: Trigger when TX FIFO contains 5 byte"]
    Level5 = 5,
    #[doc = "6: Trigger when TX FIFO contains 6 byte"]
    Level6 = 6,
    #[doc = "7: Trigger when TX FIFO contains 7 byte"]
    Level7 = 7,
}
impl From<Txtrig> for u8 {
    #[inline(always)]
    fn from(variant: Txtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txtrig {
    type Ux = u8;
}
impl crate::IsEnum for Txtrig {}
#[doc = "Field `TXTRIG` reader - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type TxtrigR = crate::FieldReader<Txtrig>;
impl TxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txtrig> {
        match self.bits {
            4 => Some(Txtrig::Level4),
            5 => Some(Txtrig::Level5),
            6 => Some(Txtrig::Level6),
            7 => Some(Txtrig::Level7),
            _ => None,
        }
    }
    #[doc = "Trigger when TX FIFO contains 4 byte"]
    #[inline(always)]
    pub fn is_level_4(&self) -> bool {
        *self == Txtrig::Level4
    }
    #[doc = "Trigger when TX FIFO contains 5 byte"]
    #[inline(always)]
    pub fn is_level_5(&self) -> bool {
        *self == Txtrig::Level5
    }
    #[doc = "Trigger when TX FIFO contains 6 byte"]
    #[inline(always)]
    pub fn is_level_6(&self) -> bool {
        *self == Txtrig::Level6
    }
    #[doc = "Trigger when TX FIFO contains 7 byte"]
    #[inline(always)]
    pub fn is_level_7(&self) -> bool {
        *self == Txtrig::Level7
    }
}
#[doc = "Field `TXTRIG` writer - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type TxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txtrig>;
impl<'a, REG> TxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when TX FIFO contains 4 byte"]
    #[inline(always)]
    pub fn level_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Level4)
    }
    #[doc = "Trigger when TX FIFO contains 5 byte"]
    #[inline(always)]
    pub fn level_5(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Level5)
    }
    #[doc = "Trigger when TX FIFO contains 6 byte"]
    #[inline(always)]
    pub fn level_6(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Level6)
    }
    #[doc = "Trigger when TX FIFO contains 7 byte"]
    #[inline(always)]
    pub fn level_7(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Level7)
    }
}
#[doc = "TX FIFO Flush Setting this bit will Flush the TX FIFO. Before clearing this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txflush {
    #[doc = "0: Do not Flush FIFO"]
    Noflush = 0,
    #[doc = "1: Flush FIFO"]
    Flush = 1,
}
impl From<Txflush> for bool {
    #[inline(always)]
    fn from(variant: Txflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` reader - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before clearing this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type TxflushR = crate::BitReader<Txflush>;
impl TxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txflush {
        match self.bits {
            false => Txflush::Noflush,
            true => Txflush::Flush,
        }
    }
    #[doc = "Do not Flush FIFO"]
    #[inline(always)]
    pub fn is_noflush(&self) -> bool {
        *self == Txflush::Noflush
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Txflush::Flush
    }
}
#[doc = "Field `TXFLUSH` writer - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before clearing this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type TxflushW<'a, REG> = crate::BitWriter<'a, REG, Txflush>;
impl<'a, REG> TxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not Flush FIFO"]
    #[inline(always)]
    pub fn noflush(self) -> &'a mut crate::W<REG> {
        self.variant(Txflush::Noflush)
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Txflush::Flush)
    }
}
#[doc = "RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtrig {
    #[doc = "4: Trigger when RX FIFO contains >= 5 byte"]
    Level5 = 4,
    #[doc = "5: Trigger when RX FIFO contains >= 6 byte"]
    Level6 = 5,
    #[doc = "6: Trigger when RX FIFO contains >= 7 byte"]
    Level7 = 6,
    #[doc = "7: Trigger when RX FIFO contains >= 8 byte"]
    Level8 = 7,
}
impl From<Rxtrig> for u8 {
    #[inline(always)]
    fn from(variant: Rxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtrig {
    type Ux = u8;
}
impl crate::IsEnum for Rxtrig {}
#[doc = "Field `RXTRIG` reader - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type RxtrigR = crate::FieldReader<Rxtrig>;
impl RxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxtrig> {
        match self.bits {
            4 => Some(Rxtrig::Level5),
            5 => Some(Rxtrig::Level6),
            6 => Some(Rxtrig::Level7),
            7 => Some(Rxtrig::Level8),
            _ => None,
        }
    }
    #[doc = "Trigger when RX FIFO contains >= 5 byte"]
    #[inline(always)]
    pub fn is_level_5(&self) -> bool {
        *self == Rxtrig::Level5
    }
    #[doc = "Trigger when RX FIFO contains >= 6 byte"]
    #[inline(always)]
    pub fn is_level_6(&self) -> bool {
        *self == Rxtrig::Level6
    }
    #[doc = "Trigger when RX FIFO contains >= 7 byte"]
    #[inline(always)]
    pub fn is_level_7(&self) -> bool {
        *self == Rxtrig::Level7
    }
    #[doc = "Trigger when RX FIFO contains >= 8 byte"]
    #[inline(always)]
    pub fn is_level_8(&self) -> bool {
        *self == Rxtrig::Level8
    }
}
#[doc = "Field `RXTRIG` writer - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type RxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxtrig>;
impl<'a, REG> RxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when RX FIFO contains >= 5 byte"]
    #[inline(always)]
    pub fn level_5(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Level5)
    }
    #[doc = "Trigger when RX FIFO contains >= 6 byte"]
    #[inline(always)]
    pub fn level_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Level6)
    }
    #[doc = "Trigger when RX FIFO contains >= 7 byte"]
    #[inline(always)]
    pub fn level_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Level7)
    }
    #[doc = "Trigger when RX FIFO contains >= 8 byte"]
    #[inline(always)]
    pub fn level_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Level8)
    }
}
#[doc = "RX FIFO Flush Setting this bit will Flush the RX FIFO. Before clearing this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxflush {
    #[doc = "0: Do not Flush FIFO"]
    Noflush = 0,
    #[doc = "1: Flush FIFO"]
    Flush = 1,
}
impl From<Rxflush> for bool {
    #[inline(always)]
    fn from(variant: Rxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` reader - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before clearing this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type RxflushR = crate::BitReader<Rxflush>;
impl RxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxflush {
        match self.bits {
            false => Rxflush::Noflush,
            true => Rxflush::Flush,
        }
    }
    #[doc = "Do not Flush FIFO"]
    #[inline(always)]
    pub fn is_noflush(&self) -> bool {
        *self == Rxflush::Noflush
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Rxflush::Flush
    }
}
#[doc = "Field `RXFLUSH` writer - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before clearing this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type RxflushW<'a, REG> = crate::BitWriter<'a, REG, Rxflush>;
impl<'a, REG> RxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not Flush FIFO"]
    #[inline(always)]
    pub fn noflush(self) -> &'a mut crate::W<REG> {
        self.variant(Rxflush::Noflush)
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Rxflush::Flush)
    }
}
impl R {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn txtrig(&self) -> TxtrigR {
        TxtrigR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before clearing this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn txflush(&self) -> TxflushR {
        TxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn rxtrig(&self) -> RxtrigR {
        RxtrigR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before clearing this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn rxflush(&self) -> RxflushR {
        RxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TxtrigW<'_, I2c0TfifoctlSpec> {
        TxtrigW::new(self, 0)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before clearing this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn txflush(&mut self) -> TxflushW<'_, I2c0TfifoctlSpec> {
        TxflushW::new(self, 7)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RxtrigW<'_, I2c0TfifoctlSpec> {
        RxtrigW::new(self, 8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before clearing this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RxflushW<'_, I2c0TfifoctlSpec> {
        RxflushW::new(self, 15)
    }
}
#[doc = "I2C Target FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_tfifoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_tfifoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0TfifoctlSpec;
impl crate::RegisterSpec for I2c0TfifoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_tfifoctl::R`](R) reader structure"]
impl crate::Readable for I2c0TfifoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_tfifoctl::W`](W) writer structure"]
impl crate::Writable for I2c0TfifoctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_TFIFOCTL to value 0"]
impl crate::Resettable for I2c0TfifoctlSpec {}
