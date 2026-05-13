#[doc = "Register `I2C1_CONTROLLER_I2CPECCTL` reader"]
pub type R = crate::R<I2c1ControllerI2cpecctlSpec>;
#[doc = "Register `I2C1_CONTROLLER_I2CPECCTL` writer"]
pub type W = crate::W<I2c1ControllerI2cpecctlSpec>;
#[doc = "Field `PECCNT` reader - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Controller use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Target Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the Controller_I2CPECCTL Register will clear the current PEC Byte Count in the Controller State Machine."]
pub type PeccntR = crate::FieldReader<u16>;
#[doc = "Field `PECCNT` writer - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Controller use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Target Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the Controller_I2CPECCTL Register will clear the current PEC Byte Count in the Controller State Machine."]
pub type PeccntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits except the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecen {
    #[doc = "0: PEC is disabled in Controller mode"]
    Disable = 0,
    #[doc = "1: PEC is enabled in Controller mode"]
    Enable = 1,
}
impl From<Pecen> for bool {
    #[inline(always)]
    fn from(variant: Pecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits except the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type PecenR = crate::BitReader<Pecen>;
impl PecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecen {
        match self.bits {
            false => Pecen::Disable,
            true => Pecen::Enable,
        }
    }
    #[doc = "PEC is disabled in Controller mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pecen::Disable
    }
    #[doc = "PEC is enabled in Controller mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pecen::Enable
    }
}
#[doc = "Field `PECEN` writer - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits except the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG, Pecen>;
impl<'a, REG> PecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC is disabled in Controller mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Disable)
    }
    #[doc = "PEC is enabled in Controller mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:8 - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Controller use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Target Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the Controller_I2CPECCTL Register will clear the current PEC Byte Count in the Controller State Machine."]
    #[inline(always)]
    pub fn peccnt(&self) -> PeccntR {
        PeccntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits except the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Controller use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Target Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the Controller_I2CPECCTL Register will clear the current PEC Byte Count in the Controller State Machine."]
    #[inline(always)]
    pub fn peccnt(&mut self) -> PeccntW<'_, I2c1ControllerI2cpecctlSpec> {
        PeccntW::new(self, 0)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits except the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, I2c1ControllerI2cpecctlSpec> {
        PecenW::new(self, 12)
    }
}
#[doc = "I2C Controller PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_controller_i2cpecctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_controller_i2cpecctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1ControllerI2cpecctlSpec;
impl crate::RegisterSpec for I2c1ControllerI2cpecctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_controller_i2cpecctl::R`](R) reader structure"]
impl crate::Readable for I2c1ControllerI2cpecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_controller_i2cpecctl::W`](W) writer structure"]
impl crate::Writable for I2c1ControllerI2cpecctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1_CONTROLLER_I2CPECCTL to value 0"]
impl crate::Resettable for I2c1ControllerI2cpecctlSpec {}
