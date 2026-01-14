#[doc = "Register `I2C0_CPU_INT_IIDX` reader"]
pub type R = crate::R<I2c0CpuIntIidxSpec>;
#[doc = "I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "0: No interrupt pending"]
    NoIntr = 0,
    #[doc = "1: Controller data received"]
    Crxdonefg = 1,
    #[doc = "2: Controller data transmitted"]
    Ctxdonefg = 2,
    #[doc = "3: Controller receive FIFO Trigger Level"]
    Crxfifotrg = 3,
    #[doc = "4: Controller transmit FIFO Trigger level"]
    Ctxfifotrg = 4,
    #[doc = "5: RX FIFO FULL Event/interrupt pending"]
    Crxfifofull = 5,
    #[doc = "6: Transmit FIFO/Buffer Empty Event/interrupt pending"]
    CtxEmpty = 6,
    #[doc = "8: Address/Data NACK"]
    Cnackfg = 8,
    #[doc = "9: Start Event"]
    Cstartfg = 9,
    #[doc = "10: Stop Event"]
    Cstopfg = 10,
    #[doc = "11: Arbitration Lost"]
    Carblostfg = 11,
    #[doc = "12: DMA DONE on Channel TX"]
    CdmaDoneTx = 12,
    #[doc = "13: DMA DONE on Channel RX"]
    CdmaDoneRx = 13,
    #[doc = "14: Controller PEC Receive Error Event"]
    CpecRxErr = 14,
    #[doc = "15: Timeout A Event"]
    Timeouta = 15,
    #[doc = "16: Timeout B Event"]
    Timeoutb = 16,
    #[doc = "17: Target Data Event"]
    Trxdonefg = 17,
    #[doc = "18: Target Data Event"]
    Ttxdonefg = 18,
    #[doc = "19: Target receive FIFO Trigger Level"]
    Trxfifotrg = 19,
    #[doc = "20: Target transmit FIFO Trigger level"]
    Ttxfifotrg = 20,
    #[doc = "21: RX FIFO FULL Event/interrupt pending"]
    Trxfifofull = 21,
    #[doc = "22: Transmit FIFO/Buffer Empty Event/interrupt pending"]
    Ttxempty = 22,
    #[doc = "23: Start Event"]
    Tstartfg = 23,
    #[doc = "24: Stop Event"]
    Tstopfg = 24,
    #[doc = "25: General Call Event"]
    Tgencall = 25,
    #[doc = "26: DMA DONE on Channel TX"]
    TdmaDoneTx = 26,
    #[doc = "27: DMA DONE on Channel RX"]
    TdmaDoneRx = 27,
    #[doc = "28: Target PEC receive error event"]
    TpecRxErr = 28,
    #[doc = "29: Target TX FIFO underflow"]
    TtxUnfl = 29,
    #[doc = "30: Target RX FIFO overflow event"]
    TrxOvfl = 30,
    #[doc = "31: Target arbitration lost event"]
    Tarblost = 31,
    #[doc = "32: Interrupt overflow event"]
    IntrOvfl = 32,
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
#[doc = "Field `STAT` reader - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            1 => Some(Stat::Crxdonefg),
            2 => Some(Stat::Ctxdonefg),
            3 => Some(Stat::Crxfifotrg),
            4 => Some(Stat::Ctxfifotrg),
            5 => Some(Stat::Crxfifofull),
            6 => Some(Stat::CtxEmpty),
            8 => Some(Stat::Cnackfg),
            9 => Some(Stat::Cstartfg),
            10 => Some(Stat::Cstopfg),
            11 => Some(Stat::Carblostfg),
            12 => Some(Stat::CdmaDoneTx),
            13 => Some(Stat::CdmaDoneRx),
            14 => Some(Stat::CpecRxErr),
            15 => Some(Stat::Timeouta),
            16 => Some(Stat::Timeoutb),
            17 => Some(Stat::Trxdonefg),
            18 => Some(Stat::Ttxdonefg),
            19 => Some(Stat::Trxfifotrg),
            20 => Some(Stat::Ttxfifotrg),
            21 => Some(Stat::Trxfifofull),
            22 => Some(Stat::Ttxempty),
            23 => Some(Stat::Tstartfg),
            24 => Some(Stat::Tstopfg),
            25 => Some(Stat::Tgencall),
            26 => Some(Stat::TdmaDoneTx),
            27 => Some(Stat::TdmaDoneRx),
            28 => Some(Stat::TpecRxErr),
            29 => Some(Stat::TtxUnfl),
            30 => Some(Stat::TrxOvfl),
            31 => Some(Stat::Tarblost),
            32 => Some(Stat::IntrOvfl),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "Controller data received"]
    #[inline(always)]
    pub fn is_crxdonefg(&self) -> bool {
        *self == Stat::Crxdonefg
    }
    #[doc = "Controller data transmitted"]
    #[inline(always)]
    pub fn is_ctxdonefg(&self) -> bool {
        *self == Stat::Ctxdonefg
    }
    #[doc = "Controller receive FIFO Trigger Level"]
    #[inline(always)]
    pub fn is_crxfifotrg(&self) -> bool {
        *self == Stat::Crxfifotrg
    }
    #[doc = "Controller transmit FIFO Trigger level"]
    #[inline(always)]
    pub fn is_ctxfifotrg(&self) -> bool {
        *self == Stat::Ctxfifotrg
    }
    #[doc = "RX FIFO FULL Event/interrupt pending"]
    #[inline(always)]
    pub fn is_crxfifofull(&self) -> bool {
        *self == Stat::Crxfifofull
    }
    #[doc = "Transmit FIFO/Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_ctx_empty(&self) -> bool {
        *self == Stat::CtxEmpty
    }
    #[doc = "Address/Data NACK"]
    #[inline(always)]
    pub fn is_cnackfg(&self) -> bool {
        *self == Stat::Cnackfg
    }
    #[doc = "Start Event"]
    #[inline(always)]
    pub fn is_cstartfg(&self) -> bool {
        *self == Stat::Cstartfg
    }
    #[doc = "Stop Event"]
    #[inline(always)]
    pub fn is_cstopfg(&self) -> bool {
        *self == Stat::Cstopfg
    }
    #[doc = "Arbitration Lost"]
    #[inline(always)]
    pub fn is_carblostfg(&self) -> bool {
        *self == Stat::Carblostfg
    }
    #[doc = "DMA DONE on Channel TX"]
    #[inline(always)]
    pub fn is_cdma_done_tx(&self) -> bool {
        *self == Stat::CdmaDoneTx
    }
    #[doc = "DMA DONE on Channel RX"]
    #[inline(always)]
    pub fn is_cdma_done_rx(&self) -> bool {
        *self == Stat::CdmaDoneRx
    }
    #[doc = "Controller PEC Receive Error Event"]
    #[inline(always)]
    pub fn is_cpec_rx_err(&self) -> bool {
        *self == Stat::CpecRxErr
    }
    #[doc = "Timeout A Event"]
    #[inline(always)]
    pub fn is_timeouta(&self) -> bool {
        *self == Stat::Timeouta
    }
    #[doc = "Timeout B Event"]
    #[inline(always)]
    pub fn is_timeoutb(&self) -> bool {
        *self == Stat::Timeoutb
    }
    #[doc = "Target Data Event"]
    #[inline(always)]
    pub fn is_trxdonefg(&self) -> bool {
        *self == Stat::Trxdonefg
    }
    #[doc = "Target Data Event"]
    #[inline(always)]
    pub fn is_ttxdonefg(&self) -> bool {
        *self == Stat::Ttxdonefg
    }
    #[doc = "Target receive FIFO Trigger Level"]
    #[inline(always)]
    pub fn is_trxfifotrg(&self) -> bool {
        *self == Stat::Trxfifotrg
    }
    #[doc = "Target transmit FIFO Trigger level"]
    #[inline(always)]
    pub fn is_ttxfifotrg(&self) -> bool {
        *self == Stat::Ttxfifotrg
    }
    #[doc = "RX FIFO FULL Event/interrupt pending"]
    #[inline(always)]
    pub fn is_trxfifofull(&self) -> bool {
        *self == Stat::Trxfifofull
    }
    #[doc = "Transmit FIFO/Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn is_ttxempty(&self) -> bool {
        *self == Stat::Ttxempty
    }
    #[doc = "Start Event"]
    #[inline(always)]
    pub fn is_tstartfg(&self) -> bool {
        *self == Stat::Tstartfg
    }
    #[doc = "Stop Event"]
    #[inline(always)]
    pub fn is_tstopfg(&self) -> bool {
        *self == Stat::Tstopfg
    }
    #[doc = "General Call Event"]
    #[inline(always)]
    pub fn is_tgencall(&self) -> bool {
        *self == Stat::Tgencall
    }
    #[doc = "DMA DONE on Channel TX"]
    #[inline(always)]
    pub fn is_tdma_done_tx(&self) -> bool {
        *self == Stat::TdmaDoneTx
    }
    #[doc = "DMA DONE on Channel RX"]
    #[inline(always)]
    pub fn is_tdma_done_rx(&self) -> bool {
        *self == Stat::TdmaDoneRx
    }
    #[doc = "Target PEC receive error event"]
    #[inline(always)]
    pub fn is_tpec_rx_err(&self) -> bool {
        *self == Stat::TpecRxErr
    }
    #[doc = "Target TX FIFO underflow"]
    #[inline(always)]
    pub fn is_ttx_unfl(&self) -> bool {
        *self == Stat::TtxUnfl
    }
    #[doc = "Target RX FIFO overflow event"]
    #[inline(always)]
    pub fn is_trx_ovfl(&self) -> bool {
        *self == Stat::TrxOvfl
    }
    #[doc = "Target arbitration lost event"]
    #[inline(always)]
    pub fn is_tarblost(&self) -> bool {
        *self == Stat::Tarblost
    }
    #[doc = "Interrupt overflow event"]
    #[inline(always)]
    pub fn is_intr_ovfl(&self) -> bool {
        *self == Stat::IntrOvfl
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_cpu_int_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0CpuIntIidxSpec;
impl crate::RegisterSpec for I2c0CpuIntIidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_cpu_int_iidx::R`](R) reader structure"]
impl crate::Readable for I2c0CpuIntIidxSpec {}
#[doc = "`reset()` method sets I2C0_CPU_INT_IIDX to value 0"]
impl crate::Resettable for I2c0CpuIntIidxSpec {}
