#[doc = "Register `UART1_IFLS` reader"]
pub type R = crate::R<Uart1IflsSpec>;
#[doc = "Register `UART1_IFLS` writer"]
pub type W = crate::W<Uart1IflsSpec>;
#[doc = "UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "1: TX FIFO <= 3/4 empty"]
    Lvl3_4 = 1,
    #[doc = "2: TX FIFO <= 1/2 empty (default)"]
    Lvl1_2 = 2,
    #[doc = "3: TX FIFO <= 1/4 empty"]
    Lvl1_4 = 3,
    #[doc = "5: TX FIFO is empty"]
    LvlEmpty = 5,
    #[doc = "7: TX FIFO >= 1 entry free Note: esp. required for DMA Trigger"]
    Lvl1 = 7,
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
#[doc = "Field `TXIFLSEL` reader - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            1 => Some(Txiflsel::Lvl3_4),
            2 => Some(Txiflsel::Lvl1_2),
            3 => Some(Txiflsel::Lvl1_4),
            5 => Some(Txiflsel::LvlEmpty),
            7 => Some(Txiflsel::Lvl1),
            _ => None,
        }
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
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == Txiflsel::LvlEmpty
    }
    #[doc = "TX FIFO >= 1 entry free Note: esp. required for DMA Trigger"]
    #[inline(always)]
    pub fn is_lvl_1(&self) -> bool {
        *self == Txiflsel::Lvl1
    }
}
#[doc = "Field `TXIFLSEL` writer - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlEmpty)
    }
    #[doc = "TX FIFO >= 1 entry free Note: esp. required for DMA Trigger"]
    #[inline(always)]
    pub fn lvl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1)
    }
}
#[doc = "UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "0: RX FIFO >= 1/4 full Note: For ULP Domain"]
    Lvl1_4Res = 0,
    #[doc = "1: RX FIFO >= 1/4 full"]
    Lvl1_4 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    Lvl1_2 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    Lvl3_4 = 3,
    #[doc = "4: RX FIFO is full Note: For ULP Domain"]
    LvlFullRes = 4,
    #[doc = "5: RX FIFO is full"]
    LvlFull = 5,
    #[doc = "7: RX FIFO >= 1 entry available Note: esp. required for DMA Trigger"]
    Lvl1 = 7,
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
#[doc = "Field `RXIFLSEL` reader - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            0 => Some(Rxiflsel::Lvl1_4Res),
            1 => Some(Rxiflsel::Lvl1_4),
            2 => Some(Rxiflsel::Lvl1_2),
            3 => Some(Rxiflsel::Lvl3_4),
            4 => Some(Rxiflsel::LvlFullRes),
            5 => Some(Rxiflsel::LvlFull),
            7 => Some(Rxiflsel::Lvl1),
            _ => None,
        }
    }
    #[doc = "RX FIFO >= 1/4 full Note: For ULP Domain"]
    #[inline(always)]
    pub fn is_lvl_1_4_res(&self) -> bool {
        *self == Rxiflsel::Lvl1_4Res
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
    #[doc = "RX FIFO is full Note: For ULP Domain"]
    #[inline(always)]
    pub fn is_lvl_full_res(&self) -> bool {
        *self == Rxiflsel::LvlFullRes
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == Rxiflsel::LvlFull
    }
    #[doc = "RX FIFO >= 1 entry available Note: esp. required for DMA Trigger"]
    #[inline(always)]
    pub fn is_lvl_1(&self) -> bool {
        *self == Rxiflsel::Lvl1
    }
}
#[doc = "Field `RXIFLSEL` writer - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RX FIFO >= 1/4 full Note: For ULP Domain"]
    #[inline(always)]
    pub fn lvl_1_4_res(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_4Res)
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
    #[doc = "RX FIFO is full Note: For ULP Domain"]
    #[inline(always)]
    pub fn lvl_full_res(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlFullRes)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlFull)
    }
    #[doc = "RX FIFO >= 1 entry available Note: esp. required for DMA Trigger"]
    #[inline(always)]
    pub fn lvl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1)
    }
}
#[doc = "Field `RXTOSEL` reader - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
pub type RxtoselR = crate::FieldReader;
#[doc = "Field `RXTOSEL` writer - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
pub type RxtoselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtosel(&self) -> RxtoselR {
        RxtoselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, Uart1IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, Uart1IflsSpec> {
        RxiflselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtosel(&mut self) -> RxtoselW<'_, Uart1IflsSpec> {
        RxtoselW::new(self, 8)
    }
}
#[doc = "UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IflsSpec;
impl crate::RegisterSpec for Uart1IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_ifls::R`](R) reader structure"]
impl crate::Readable for Uart1IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_ifls::W`](W) writer structure"]
impl crate::Writable for Uart1IflsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_IFLS to value 0x22"]
impl crate::Resettable for Uart1IflsSpec {
    const RESET_VALUE: u32 = 0x22;
}
