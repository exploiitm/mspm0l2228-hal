#[doc = "Register `SPI1_CLKCTL` reader"]
pub type R = crate::R<Spi1ClkctlSpec>;
#[doc = "Register `SPI1_CLKCTL` writer"]
pub type W = crate::W<Spi1ClkctlSpec>;
#[doc = "Field `SCR` reader - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ScrR = crate::FieldReader<u16>;
#[doc = "Field `SCR` writer - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DSAMPLE` reader - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
pub type DsampleR = crate::FieldReader;
#[doc = "Field `DSAMPLE` writer - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
pub type DsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
    #[inline(always)]
    pub fn dsample(&self) -> DsampleR {
        DsampleR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, Spi1ClkctlSpec> {
        ScrW::new(self, 0)
    }
    #[doc = "Bits 28:31 - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
    #[inline(always)]
    pub fn dsample(&mut self) -> DsampleW<'_, Spi1ClkctlSpec> {
        DsampleW::new(self, 28)
    }
}
#[doc = "Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1ClkctlSpec;
impl crate::RegisterSpec for Spi1ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1_clkctl::R`](R) reader structure"]
impl crate::Readable for Spi1ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`spi1_clkctl::W`](W) writer structure"]
impl crate::Writable for Spi1ClkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1_CLKCTL to value 0"]
impl crate::Resettable for Spi1ClkctlSpec {}
