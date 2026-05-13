#[doc = "Register `I2C2_CCTR` reader"]
pub type R = crate::R<I2c2CctrSpec>;
#[doc = "Register `I2C2_CCTR` writer"]
pub type W = crate::W<I2c2CctrSpec>;
#[doc = "I2C Controller Enable and start transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Burstrun {
    #[doc = "0: In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    Disable = 0,
    #[doc = "1: The Controller is able to transmit or receive data."]
    Enable = 1,
}
impl From<Burstrun> for bool {
    #[inline(always)]
    fn from(variant: Burstrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTRUN` reader - I2C Controller Enable and start transaction"]
pub type BurstrunR = crate::BitReader<Burstrun>;
impl BurstrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstrun {
        match self.bits {
            false => Burstrun::Disable,
            true => Burstrun::Enable,
        }
    }
    #[doc = "In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Burstrun::Disable
    }
    #[doc = "The Controller is able to transmit or receive data."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Burstrun::Enable
    }
}
#[doc = "Field `BURSTRUN` writer - I2C Controller Enable and start transaction"]
pub type BurstrunW<'a, REG> = crate::BitWriter<'a, REG, Burstrun>;
impl<'a, REG> BurstrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In standard mode, this encoding means the Controller is unable to transmit or receive data."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Burstrun::Disable)
    }
    #[doc = "The Controller is able to transmit or receive data."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Burstrun::Enable)
    }
}
#[doc = "Generate START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: The controller does not generate the START condition."]
    Disable = 0,
    #[doc = "1: The controller generates the START or repeated START condition."]
    Enable = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Generate START"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::Disable,
            true => Start::Enable,
        }
    }
    #[doc = "The controller does not generate the START condition."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Start::Disable
    }
    #[doc = "The controller generates the START or repeated START condition."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Start::Enable
    }
}
#[doc = "Field `START` writer - Generate START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The controller does not generate the START condition."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Disable)
    }
    #[doc = "The controller generates the START or repeated START condition."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Enable)
    }
}
#[doc = "Generate STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: The controller does not generate the STOP condition."]
    Disable = 0,
    #[doc = "1: The controller generates the STOP condition."]
    Enable = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Generate STOP"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::Disable,
            true => Stop::Enable,
        }
    }
    #[doc = "The controller does not generate the STOP condition."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stop::Disable
    }
    #[doc = "The controller generates the STOP condition."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stop::Enable
    }
}
#[doc = "Field `STOP` writer - Generate STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The controller does not generate the STOP condition."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Disable)
    }
    #[doc = "The controller generates the STOP condition."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Enable)
    }
}
#[doc = "Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "0: The last received data byte of a transaction is not acknowledged automatically by the Controller."]
    Disable = 0,
    #[doc = "1: The last received data byte of a transaction is acknowledged automatically by the Controller."]
    Enable = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK."]
pub type AckR = crate::BitReader<Ack>;
impl AckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ack {
        match self.bits {
            false => Ack::Disable,
            true => Ack::Enable,
        }
    }
    #[doc = "The last received data byte of a transaction is not acknowledged automatically by the Controller."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ack::Disable
    }
    #[doc = "The last received data byte of a transaction is acknowledged automatically by the Controller."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ack::Enable
    }
}
#[doc = "Field `ACK` writer - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK."]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The last received data byte of a transaction is not acknowledged automatically by the Controller."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Disable)
    }
    #[doc = "The last received data byte of a transaction is acknowledged automatically by the Controller."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Enable)
    }
}
#[doc = "Controller ACK overrride Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cackoen {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    Enable = 1,
}
impl From<Cackoen> for bool {
    #[inline(always)]
    fn from(variant: Cackoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACKOEN` reader - Controller ACK overrride Enable"]
pub type CackoenR = crate::BitReader<Cackoen>;
impl CackoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cackoen {
        match self.bits {
            false => Cackoen::Disable,
            true => Cackoen::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cackoen::Disable
    }
    #[doc = "When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cackoen::Enable
    }
}
#[doc = "Field `CACKOEN` writer - Controller ACK overrride Enable"]
pub type CackoenW<'a, REG> = crate::BitWriter<'a, REG, Cackoen>;
impl<'a, REG> CackoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cackoen::Disable)
    }
    #[doc = "When 1 and the Controller is receiving data and the number of bytes indicated in MBLEN have been received, the state machine will generate an rxdone interrupt and wait at the start of the ACK for FW to indicate if an ACK or NACK should be sent. The ACK or NACK is selected by writing the MCTR register and setting ACK accordingly. The other fields in this register can also be written at this time to continue on with the transaction. If a NACK is sent the state machine will automatically send a Stop."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cackoen::Enable)
    }
}
#[doc = "Read on TX Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdOnTxempty {
    #[doc = "0: No special behavior"]
    Disable = 0,
    #[doc = "1: When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    Enable = 1,
}
impl From<RdOnTxempty> for bool {
    #[inline(always)]
    fn from(variant: RdOnTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_ON_TXEMPTY` reader - Read on TX Empty"]
pub type RdOnTxemptyR = crate::BitReader<RdOnTxempty>;
impl RdOnTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdOnTxempty {
        match self.bits {
            false => RdOnTxempty::Disable,
            true => RdOnTxempty::Enable,
        }
    }
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RdOnTxempty::Disable
    }
    #[doc = "When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RdOnTxempty::Enable
    }
}
#[doc = "Field `RD_ON_TXEMPTY` writer - Read on TX Empty"]
pub type RdOnTxemptyW<'a, REG> = crate::BitWriter<'a, REG, RdOnTxempty>;
impl<'a, REG> RdOnTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No special behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RdOnTxempty::Disable)
    }
    #[doc = "When 1 the Controller will transmit all bytes from the TX FIFO before continuing with the programmed Burst Run Read. If the DIR is not set to Read in the MSA then this bit is ignored. The Start must be set in the MCTR for proper I2C protocol. The Controller will first send the Start Condition, I2C Address with R/W bit set to write, before sending the bytes in the TX FIFO. When the TX FIFO is empty, the I2C transaction will continue as programmed in MTCR and MSA without sending a Stop Condition. This is intended to be used to perform simple I2C command based reads transition that will complete after initiating them without having to get an interrupt to turn the bus around."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RdOnTxempty::Enable)
    }
}
#[doc = "Field `CBLEN` reader - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
pub type CblenR = crate::FieldReader<u16>;
#[doc = "Field `CBLEN` writer - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
pub type CblenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - I2C Controller Enable and start transaction"]
    #[inline(always)]
    pub fn burstrun(&self) -> BurstrunR {
        BurstrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controller ACK overrride Enable"]
    #[inline(always)]
    pub fn cackoen(&self) -> CackoenR {
        CackoenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn rd_on_txempty(&self) -> RdOnTxemptyR {
        RdOnTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:27 - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
    #[inline(always)]
    pub fn cblen(&self) -> CblenR {
        CblenR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Controller Enable and start transaction"]
    #[inline(always)]
    pub fn burstrun(&mut self) -> BurstrunW<'_, I2c2CctrSpec> {
        BurstrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, I2c2CctrSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, I2c2CctrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK."]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, I2c2CctrSpec> {
        AckW::new(self, 3)
    }
    #[doc = "Bit 4 - Controller ACK overrride Enable"]
    #[inline(always)]
    pub fn cackoen(&mut self) -> CackoenW<'_, I2c2CctrSpec> {
        CackoenW::new(self, 4)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn rd_on_txempty(&mut self) -> RdOnTxemptyW<'_, I2c2CctrSpec> {
        RdOnTxemptyW::new(self, 5)
    }
    #[doc = "Bits 16:27 - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
    #[inline(always)]
    pub fn cblen(&mut self) -> CblenW<'_, I2c2CctrSpec> {
        CblenW::new(self, 16)
    }
}
#[doc = "I2C Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_cctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_cctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2CctrSpec;
impl crate::RegisterSpec for I2c2CctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_cctr::R`](R) reader structure"]
impl crate::Readable for I2c2CctrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c2_cctr::W`](W) writer structure"]
impl crate::Writable for I2c2CctrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_CCTR to value 0"]
impl crate::Resettable for I2c2CctrSpec {}
