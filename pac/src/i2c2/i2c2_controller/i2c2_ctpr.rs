#[doc = "Register `I2C2_CTPR` reader"]
pub type R = crate::R<I2c2CtprSpec>;
#[doc = "Register `I2C2_CTPR` writer"]
pub type W = crate::W<I2c2CtprSpec>;
#[doc = "Field `TPR` reader - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type TprR = crate::FieldReader;
#[doc = "Field `TPR` writer - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
pub type TprW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TprR {
        TprR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period This field is used in the equation to configure SCL_PERIOD : SCL_PERIOD = (1 + TPR ) * (SCL_LP + SCL_HP ) * INT_CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the Timer Period register value (range of 1 to 127). SCL_LP is the SCL Low period (fixed at 6). SCL_HP is the SCL High period (fixed at 4). CLK_PRD is the functional clock period in ns."]
    #[inline(always)]
    pub fn tpr(&mut self) -> TprW<'_, I2c2CtprSpec> {
        TprW::new(self, 0)
    }
}
#[doc = "I2C Controller Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2_ctpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2_ctpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c2CtprSpec;
impl crate::RegisterSpec for I2c2CtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c2_ctpr::R`](R) reader structure"]
impl crate::Readable for I2c2CtprSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c2_ctpr::W`](W) writer structure"]
impl crate::Writable for I2c2CtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C2_CTPR to value 0x01"]
impl crate::Resettable for I2c2CtprSpec {
    const RESET_VALUE: u32 = 0x01;
}
