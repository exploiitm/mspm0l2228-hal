#[doc = "Register `SPI0_IFLS` reader"]
pub type R = crate::R<Spi0IflsSpec>;
#[doc = "Register `SPI0_IFLS` writer"]
pub type W = crate::W<Spi0IflsSpec>;
#[doc = "SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "0: Reserved"]
    LvlOff = 0,
    #[doc = "1: TX FIFO <= 3/4 empty"]
    Lvl3_4 = 1,
    #[doc = "2: TX FIFO <= 1/2 empty (default)"]
    Lvl1_2 = 2,
    #[doc = "3: TX FIFO <= 1/4 empty"]
    Lvl1_4 = 3,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "5: TX FIFO is empty"]
    LvlEmpty = 5,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "7: Trigger when TX FIFO has >= 1 frame free Should be used with DMA"]
    Level1 = 7,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txiflsel {
        match self.bits {
            0 => Txiflsel::LvlOff,
            1 => Txiflsel::Lvl3_4,
            2 => Txiflsel::Lvl1_2,
            3 => Txiflsel::Lvl1_4,
            4 => Txiflsel::LvlRes4,
            5 => Txiflsel::LvlEmpty,
            6 => Txiflsel::LvlRes6,
            7 => Txiflsel::Level1,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Txiflsel::LvlOff
    }
    #[doc = "TX FIFO <= 3/4 empty"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Txiflsel::Lvl3_4
    }
    #[doc = "TX FIFO <= 1/2 empty (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Txiflsel::Lvl1_2
    }
    #[doc = "TX FIFO <= 1/4 empty"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Txiflsel::Lvl1_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Txiflsel::LvlRes4
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == Txiflsel::LvlEmpty
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Txiflsel::LvlRes6
    }
    #[doc = "Trigger when TX FIFO has >= 1 frame free Should be used with DMA"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Txiflsel::Level1
    }
}
#[doc = "Field `TXIFLSEL` writer - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel, crate::Safe>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlOff)
    }
    #[doc = "TX FIFO <= 3/4 empty"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl3_4)
    }
    #[doc = "TX FIFO <= 1/2 empty (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_2)
    }
    #[doc = "TX FIFO <= 1/4 empty"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlRes4)
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlEmpty)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlRes6)
    }
    #[doc = "Trigger when TX FIFO has >= 1 frame free Should be used with DMA"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Level1)
    }
}
#[doc = "SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "0: Reserved"]
    LvlOff = 0,
    #[doc = "1: RX FIFO >= 1/4 full"]
    Lvl1_4 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    Lvl1_2 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    Lvl3_4 = 3,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "5: RX FIFO is full"]
    LvlFull = 5,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "7: Trigger when RX FIFO contains >= 1 frame"]
    Level1 = 7,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxiflsel {
        match self.bits {
            0 => Rxiflsel::LvlOff,
            1 => Rxiflsel::Lvl1_4,
            2 => Rxiflsel::Lvl1_2,
            3 => Rxiflsel::Lvl3_4,
            4 => Rxiflsel::LvlRes4,
            5 => Rxiflsel::LvlFull,
            6 => Rxiflsel::LvlRes6,
            7 => Rxiflsel::Level1,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Rxiflsel::LvlOff
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Rxiflsel::Lvl1_4
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Rxiflsel::Lvl1_2
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Rxiflsel::Lvl3_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Rxiflsel::LvlRes4
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == Rxiflsel::LvlFull
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Rxiflsel::LvlRes6
    }
    #[doc = "Trigger when RX FIFO contains >= 1 frame"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Rxiflsel::Level1
    }
}
#[doc = "Field `RXIFLSEL` writer - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel, crate::Safe>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlOff)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_4)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_2)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl3_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlRes4)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlFull)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlRes6)
    }
    #[doc = "Trigger when RX FIFO contains >= 1 frame"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Level1)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, Spi0IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, Spi0IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0IflsSpec;
impl crate::RegisterSpec for Spi0IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_ifls::R`](R) reader structure"]
impl crate::Readable for Spi0IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0_ifls::W`](W) writer structure"]
impl crate::Writable for Spi0IflsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0_IFLS to value 0x12"]
impl crate::Resettable for Spi0IflsSpec {
    const RESET_VALUE: u32 = 0x12;
}
