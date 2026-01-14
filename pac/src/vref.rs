#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    vref_gprcm: [VrefGprcm; 1],
    _reserved1: [u8; 0x07e8],
    vref_clkdiv: VrefClkdiv,
    _reserved2: [u8; 0x04],
    vref_clksel: VrefClksel,
    _reserved3: [u8; 0xf4],
    vref_ctl0: VrefCtl0,
    vref_ctl1: VrefCtl1,
    vref_ctl2: VrefCtl2,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - VREF_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn vref_gprcm(&self, n: usize) -> &VrefGprcm {
        &self.vref_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - VREF_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn vref_gprcm_iter(&self) -> impl Iterator<Item = &VrefGprcm> {
        self.vref_gprcm.iter()
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn vref_clkdiv(&self) -> &VrefClkdiv {
        &self.vref_clkdiv
    }
    #[doc = "0x1008 - Clock Selection"]
    #[inline(always)]
    pub const fn vref_clksel(&self) -> &VrefClksel {
        &self.vref_clksel
    }
    #[doc = "0x1100 - Control 0"]
    #[inline(always)]
    pub const fn vref_ctl0(&self) -> &VrefCtl0 {
        &self.vref_ctl0
    }
    #[doc = "0x1104 - Control 1"]
    #[inline(always)]
    pub const fn vref_ctl1(&self) -> &VrefCtl1 {
        &self.vref_ctl1
    }
    #[doc = "0x1108 - Control 2"]
    #[inline(always)]
    pub const fn vref_ctl2(&self) -> &VrefCtl2 {
        &self.vref_ctl2
    }
}
#[doc = "VREF_GPRCM\\[%s\\]"]
pub use self::vref_gprcm::VrefGprcm;
#[doc = r"Cluster"]
#[doc = "VREF_GPRCM\\[%s\\]"]
pub mod vref_gprcm;
#[doc = "VREF_CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_clkdiv`] module"]
#[doc(alias = "VREF_CLKDIV")]
pub type VrefClkdiv = crate::Reg<vref_clkdiv::VrefClkdivSpec>;
#[doc = "Clock Divider"]
pub mod vref_clkdiv;
#[doc = "VREF_CLKSEL (rw) register accessor: Clock Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_clksel`] module"]
#[doc(alias = "VREF_CLKSEL")]
pub type VrefClksel = crate::Reg<vref_clksel::VrefClkselSpec>;
#[doc = "Clock Selection"]
pub mod vref_clksel;
#[doc = "VREF_CTL0 (rw) register accessor: Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_ctl0`] module"]
#[doc(alias = "VREF_CTL0")]
pub type VrefCtl0 = crate::Reg<vref_ctl0::VrefCtl0Spec>;
#[doc = "Control 0"]
pub mod vref_ctl0;
#[doc = "VREF_CTL1 (rw) register accessor: Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_ctl1`] module"]
#[doc(alias = "VREF_CTL1")]
pub type VrefCtl1 = crate::Reg<vref_ctl1::VrefCtl1Spec>;
#[doc = "Control 1"]
pub mod vref_ctl1;
#[doc = "VREF_CTL2 (rw) register accessor: Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_ctl2`] module"]
#[doc(alias = "VREF_CTL2")]
pub type VrefCtl2 = crate::Reg<vref_ctl2::VrefCtl2Spec>;
#[doc = "Control 2"]
pub mod vref_ctl2;
