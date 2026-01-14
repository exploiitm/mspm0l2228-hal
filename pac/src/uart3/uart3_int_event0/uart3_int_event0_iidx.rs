#[doc = "Register `UART3_INT_EVENT0_IIDX` reader"]
pub type R = crate::R<Uart3IntEvent0IidxSpec>;
#[doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: UART receive time-out interrupt; Interrupt Flag: RT; Interrupt Priority: Highest"]
    Rtfg = 1,
    #[doc = "2: UART framing error interrupt; Interrupt Flag: FE"]
    Fefg = 2,
    #[doc = "3: UART parity error interrupt; Interrupt Flag: PE"]
    Pefg = 3,
    #[doc = "4: UART break error interrupt; Interrupt Flag: BE"]
    Befg = 4,
    #[doc = "5: UART receive overrun error interrupt; Interrupt Flag: OE"]
    Oefg = 5,
    #[doc = "6: Negative edge on UARTxRXD interrupt; Interrupt Flag: RXNE"]
    Rxne = 6,
    #[doc = "7: Positive edge on UARTxRXD interrupt; Interrupt Flag: RXPE"]
    Rxpe = 7,
    #[doc = "8: LIN capture 0 / match interrupt; Interrupt Flag: LINC0"]
    Linc0 = 8,
    #[doc = "9: LIN capture 1 interrupt; Interrupt Flag: LINC1"]
    Linc1 = 9,
    #[doc = "10: LIN hardware counter overflow interrupt; Interrupt Flag: LINOVF"]
    Linovf = 10,
    #[doc = "11: UART receive interrupt; Interrupt Flag: RX"]
    Rxifg = 11,
    #[doc = "12: UART transmit interrupt; Interrupt Flag: TX"]
    Txifg = 12,
    #[doc = "13: UART end of transmission interrupt (transmit serializer empty); Interrupt Flag: EOT"]
    Eot = 13,
    #[doc = "14: 9-bit mode address match interrupt; Interrupt Flag: MODE_9B"]
    Mode9b = 14,
    #[doc = "15: UART Clear to Send Modem interrupt; Interrupt Flag: CTS"]
    Cts = 15,
    #[doc = "16: DMA DONE on RX"]
    DmaDoneRx = 16,
    #[doc = "17: DMA DONE on TX"]
    DmaDoneTx = 17,
    #[doc = "18: Noise Error Event"]
    NerrEvt = 18,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Rtfg),
            2 => Some(Stat::Fefg),
            3 => Some(Stat::Pefg),
            4 => Some(Stat::Befg),
            5 => Some(Stat::Oefg),
            6 => Some(Stat::Rxne),
            7 => Some(Stat::Rxpe),
            8 => Some(Stat::Linc0),
            9 => Some(Stat::Linc1),
            10 => Some(Stat::Linovf),
            11 => Some(Stat::Rxifg),
            12 => Some(Stat::Txifg),
            13 => Some(Stat::Eot),
            14 => Some(Stat::Mode9b),
            15 => Some(Stat::Cts),
            16 => Some(Stat::DmaDoneRx),
            17 => Some(Stat::DmaDoneTx),
            18 => Some(Stat::NerrEvt),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "UART receive time-out interrupt; Interrupt Flag: RT; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_rtfg(&self) -> bool {
        *self == Stat::Rtfg
    }
    #[doc = "UART framing error interrupt; Interrupt Flag: FE"]
    #[inline(always)]
    pub fn is_fefg(&self) -> bool {
        *self == Stat::Fefg
    }
    #[doc = "UART parity error interrupt; Interrupt Flag: PE"]
    #[inline(always)]
    pub fn is_pefg(&self) -> bool {
        *self == Stat::Pefg
    }
    #[doc = "UART break error interrupt; Interrupt Flag: BE"]
    #[inline(always)]
    pub fn is_befg(&self) -> bool {
        *self == Stat::Befg
    }
    #[doc = "UART receive overrun error interrupt; Interrupt Flag: OE"]
    #[inline(always)]
    pub fn is_oefg(&self) -> bool {
        *self == Stat::Oefg
    }
    #[doc = "Negative edge on UARTxRXD interrupt; Interrupt Flag: RXNE"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == Stat::Rxne
    }
    #[doc = "Positive edge on UARTxRXD interrupt; Interrupt Flag: RXPE"]
    #[inline(always)]
    pub fn is_rxpe(&self) -> bool {
        *self == Stat::Rxpe
    }
    #[doc = "LIN capture 0 / match interrupt; Interrupt Flag: LINC0"]
    #[inline(always)]
    pub fn is_linc0(&self) -> bool {
        *self == Stat::Linc0
    }
    #[doc = "LIN capture 1 interrupt; Interrupt Flag: LINC1"]
    #[inline(always)]
    pub fn is_linc1(&self) -> bool {
        *self == Stat::Linc1
    }
    #[doc = "LIN hardware counter overflow interrupt; Interrupt Flag: LINOVF"]
    #[inline(always)]
    pub fn is_linovf(&self) -> bool {
        *self == Stat::Linovf
    }
    #[doc = "UART receive interrupt; Interrupt Flag: RX"]
    #[inline(always)]
    pub fn is_rxifg(&self) -> bool {
        *self == Stat::Rxifg
    }
    #[doc = "UART transmit interrupt; Interrupt Flag: TX"]
    #[inline(always)]
    pub fn is_txifg(&self) -> bool {
        *self == Stat::Txifg
    }
    #[doc = "UART end of transmission interrupt (transmit serializer empty); Interrupt Flag: EOT"]
    #[inline(always)]
    pub fn is_eot(&self) -> bool {
        *self == Stat::Eot
    }
    #[doc = "9-bit mode address match interrupt; Interrupt Flag: MODE_9B"]
    #[inline(always)]
    pub fn is_mode_9b(&self) -> bool {
        *self == Stat::Mode9b
    }
    #[doc = "UART Clear to Send Modem interrupt; Interrupt Flag: CTS"]
    #[inline(always)]
    pub fn is_cts(&self) -> bool {
        *self == Stat::Cts
    }
    #[doc = "DMA DONE on RX"]
    #[inline(always)]
    pub fn is_dma_done_rx(&self) -> bool {
        *self == Stat::DmaDoneRx
    }
    #[doc = "DMA DONE on TX"]
    #[inline(always)]
    pub fn is_dma_done_tx(&self) -> bool {
        *self == Stat::DmaDoneTx
    }
    #[doc = "Noise Error Event"]
    #[inline(always)]
    pub fn is_nerr_evt(&self) -> bool {
        *self == Stat::NerrEvt
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart3IntEvent0IidxSpec;
impl crate::RegisterSpec for Uart3IntEvent0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart3_int_event0_iidx::R`](R) reader structure"]
impl crate::Readable for Uart3IntEvent0IidxSpec {}
#[doc = "`reset()` method sets UART3_INT_EVENT0_IIDX to value 0"]
impl crate::Resettable for Uart3IntEvent0IidxSpec {}
