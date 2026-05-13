#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    iomux_pincm: [IomuxPincm; 75],
}
impl RegisterBlock {
    #[doc = "0x04..0x130 - Pin Control Management Register in SECCFG region"]
    #[inline(always)]
    pub const fn iomux_pincm(&self, n: usize) -> &IomuxPincm {
        &self.iomux_pincm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x130 - Pin Control Management Register in SECCFG region"]
    #[inline(always)]
    pub fn iomux_pincm_iter(&self) -> impl Iterator<Item = &IomuxPincm> {
        self.iomux_pincm.iter()
    }
}
#[doc = "IOMUX_PINCM (rw) register accessor: Pin Control Management Register in SECCFG region\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_pincm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_pincm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_pincm`] module"]
#[doc(alias = "IOMUX_PINCM")]
pub type IomuxPincm = crate::Reg<iomux_pincm::IomuxPincmSpec>;
#[doc = "Pin Control Management Register in SECCFG region"]
pub mod iomux_pincm;
