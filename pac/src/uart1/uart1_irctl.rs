#[doc = "Register `UART1_IRCTL` reader"]
pub type R = crate::R<Uart1IrctlSpec>;
#[doc = "Register `UART1_IRCTL` writer"]
pub type W = crate::W<Uart1IrctlSpec>;
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iren {
    #[doc = "0: IrDA encoder/decoder disabled"]
    Disable = 0,
    #[doc = "1: IrDA encoder/decoder enabled"]
    Enable = 1,
}
impl From<Iren> for bool {
    #[inline(always)]
    fn from(variant: Iren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA encoder/decoder enable"]
pub type IrenR = crate::BitReader<Iren>;
impl IrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iren {
        match self.bits {
            false => Iren::Disable,
            true => Iren::Enable,
        }
    }
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Iren::Disable
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Iren::Enable
    }
}
#[doc = "Field `IREN` writer - IrDA encoder/decoder enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG, Iren>;
impl<'a, REG> IrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::Disable)
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::Enable)
    }
}
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irtxclk {
    #[doc = "0: IrDA encode data is based on the functional clock."]
    Bitclk = 0,
    #[doc = "1: IrDA encode data is based on the Baud Rate clock< when select 8x oversampling, the IRTXPL cycle should less 8; when select 16x oversampling, the IRTXPL cycle should less 16."]
    Brclk = 1,
}
impl From<Irtxclk> for bool {
    #[inline(always)]
    fn from(variant: Irtxclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRTXCLK` reader - IrDA transmit pulse clock select"]
pub type IrtxclkR = crate::BitReader<Irtxclk>;
impl IrtxclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irtxclk {
        match self.bits {
            false => Irtxclk::Bitclk,
            true => Irtxclk::Brclk,
        }
    }
    #[doc = "IrDA encode data is based on the functional clock."]
    #[inline(always)]
    pub fn is_bitclk(&self) -> bool {
        *self == Irtxclk::Bitclk
    }
    #[doc = "IrDA encode data is based on the Baud Rate clock< when select 8x oversampling, the IRTXPL cycle should less 8; when select 16x oversampling, the IRTXPL cycle should less 16."]
    #[inline(always)]
    pub fn is_brclk(&self) -> bool {
        *self == Irtxclk::Brclk
    }
}
#[doc = "Field `IRTXCLK` writer - IrDA transmit pulse clock select"]
pub type IrtxclkW<'a, REG> = crate::BitWriter<'a, REG, Irtxclk>;
impl<'a, REG> IrtxclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA encode data is based on the functional clock."]
    #[inline(always)]
    pub fn bitclk(self) -> &'a mut crate::W<REG> {
        self.variant(Irtxclk::Bitclk)
    }
    #[doc = "IrDA encode data is based on the Baud Rate clock< when select 8x oversampling, the IRTXPL cycle should less 8; when select 16x oversampling, the IRTXPL cycle should less 16."]
    #[inline(always)]
    pub fn brclk(self) -> &'a mut crate::W<REG> {
        self.variant(Irtxclk::Brclk)
    }
}
#[doc = "Field `IRTXPL` reader - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\] (IRTXCLK = functional clock of the UART)"]
pub type IrtxplR = crate::FieldReader;
#[doc = "Field `IRTXPL` writer - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\] (IRTXCLK = functional clock of the UART)"]
pub type IrtxplW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irrxpl {
    #[doc = "0: IrDA transceiver delivers a high pulse when a light pulse is seen"]
    High = 0,
    #[doc = "1: IrDA transceiver delivers a low pulse when a light pulse is seen"]
    Low = 1,
}
impl From<Irrxpl> for bool {
    #[inline(always)]
    fn from(variant: Irrxpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRRXPL` reader - IrDA receive input UCAxRXD polarity"]
pub type IrrxplR = crate::BitReader<Irrxpl>;
impl IrrxplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irrxpl {
        match self.bits {
            false => Irrxpl::High,
            true => Irrxpl::Low,
        }
    }
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Irrxpl::High
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Irrxpl::Low
    }
}
#[doc = "Field `IRRXPL` writer - IrDA receive input UCAxRXD polarity"]
pub type IrrxplW<'a, REG> = crate::BitWriter<'a, REG, Irrxpl>;
impl<'a, REG> IrrxplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Irrxpl::High)
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Irrxpl::Low)
    }
}
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn irtxclk(&self) -> IrtxclkR {
        IrtxclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\] (IRTXCLK = functional clock of the UART)"]
    #[inline(always)]
    pub fn irtxpl(&self) -> IrtxplR {
        IrtxplR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn irrxpl(&self) -> IrrxplR {
        IrrxplR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, Uart1IrctlSpec> {
        IrenW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn irtxclk(&mut self) -> IrtxclkW<'_, Uart1IrctlSpec> {
        IrtxclkW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\] (IRTXCLK = functional clock of the UART)"]
    #[inline(always)]
    pub fn irtxpl(&mut self) -> IrtxplW<'_, Uart1IrctlSpec> {
        IrtxplW::new(self, 2)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn irrxpl(&mut self) -> IrrxplW<'_, Uart1IrctlSpec> {
        IrrxplW::new(self, 9)
    }
}
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_irctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_irctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart1IrctlSpec;
impl crate::RegisterSpec for Uart1IrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_irctl::R`](R) reader structure"]
impl crate::Readable for Uart1IrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart1_irctl::W`](W) writer structure"]
impl crate::Writable for Uart1IrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_IRCTL to value 0"]
impl crate::Resettable for Uart1IrctlSpec {}
