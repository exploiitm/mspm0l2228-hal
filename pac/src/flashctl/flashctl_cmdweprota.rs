#[doc = "Register `FLASHCTL_CMDWEPROTA` reader"]
pub type R = crate::R<FlashctlCmdweprotaSpec>;
#[doc = "Register `FLASHCTL_CMDWEPROTA` writer"]
pub type W = crate::W<FlashctlCmdweprotaSpec>;
#[doc = "Field `VAL` reader - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FlashctlCmdweprotaSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Write Erase Protect A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl_cmdweprota::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl_cmdweprota::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashctlCmdweprotaSpec;
impl crate::RegisterSpec for FlashctlCmdweprotaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashctl_cmdweprota::R`](R) reader structure"]
impl crate::Readable for FlashctlCmdweprotaSpec {}
#[doc = "`write(|w| ..)` method takes [`flashctl_cmdweprota::W`](W) writer structure"]
impl crate::Writable for FlashctlCmdweprotaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCTL_CMDWEPROTA to value 0"]
impl crate::Resettable for FlashctlCmdweprotaSpec {}
