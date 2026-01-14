#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0055_6160],
    adc0_svt_fifodata: Adc0SvtFifodata,
    _reserved1: [u8; 0x011c],
    adc0_svt_memres: [Adc0SvtMemres; 12],
}
impl RegisterBlock {
    #[doc = "0x556160 - FIFO Data Register"]
    #[inline(always)]
    pub const fn adc0_svt_fifodata(&self) -> &Adc0SvtFifodata {
        &self.adc0_svt_fifodata
    }
    #[doc = "0x556280..0x5562b0 - Memory Result Register"]
    #[inline(always)]
    pub const fn adc0_svt_memres(&self, n: usize) -> &Adc0SvtMemres {
        &self.adc0_svt_memres[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x556280..0x5562b0 - Memory Result Register"]
    #[inline(always)]
    pub fn adc0_svt_memres_iter(&self) -> impl Iterator<Item = &Adc0SvtMemres> {
        self.adc0_svt_memres.iter()
    }
}
#[doc = "ADC0_SVT_FIFODATA (r) register accessor: FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_svt_fifodata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_svt_fifodata`] module"]
#[doc(alias = "ADC0_SVT_FIFODATA")]
pub type Adc0SvtFifodata = crate::Reg<adc0_svt_fifodata::Adc0SvtFifodataSpec>;
#[doc = "FIFO Data Register"]
pub mod adc0_svt_fifodata;
#[doc = "ADC0_SVT_MEMRES (r) register accessor: Memory Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_svt_memres::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_svt_memres`] module"]
#[doc(alias = "ADC0_SVT_MEMRES")]
pub type Adc0SvtMemres = crate::Reg<adc0_svt_memres::Adc0SvtMemresSpec>;
#[doc = "Memory Result Register"]
pub mod adc0_svt_memres;
