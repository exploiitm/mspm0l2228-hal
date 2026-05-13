#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    crcp0_gprcm: [Crcp0Gprcm; 1],
    _reserved1: [u8; 0x08e4],
    crcp0_desc: Crcp0Desc,
    crcp0_crcctrl: Crcp0Crcctrl,
    crcp0_crcseed: Crcp0Crcseed,
    crcp0_crcin: Crcp0Crcin,
    crcp0_crcout: Crcp0Crcout,
    crcp0_crcpoly: Crcp0Crcpoly,
    _reserved7: [u8; 0x06ec],
    crcp0_crcin_idx: [Crcp0CrcinIdx; 512],
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - CRCP0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn crcp0_gprcm(&self, n: usize) -> &Crcp0Gprcm {
        &self.crcp0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - CRCP0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn crcp0_gprcm_iter(&self) -> impl Iterator<Item = &Crcp0Gprcm> {
        self.crcp0_gprcm.iter()
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn crcp0_desc(&self) -> &Crcp0Desc {
        &self.crcp0_desc
    }
    #[doc = "0x1100 - CRC Control Register"]
    #[inline(always)]
    pub const fn crcp0_crcctrl(&self) -> &Crcp0Crcctrl {
        &self.crcp0_crcctrl
    }
    #[doc = "0x1104 - CRC Seed Register"]
    #[inline(always)]
    pub const fn crcp0_crcseed(&self) -> &Crcp0Crcseed {
        &self.crcp0_crcseed
    }
    #[doc = "0x1108 - CRC Input Data Register"]
    #[inline(always)]
    pub const fn crcp0_crcin(&self) -> &Crcp0Crcin {
        &self.crcp0_crcin
    }
    #[doc = "0x110c - CRC Output Result Register"]
    #[inline(always)]
    pub const fn crcp0_crcout(&self) -> &Crcp0Crcout {
        &self.crcp0_crcout
    }
    #[doc = "0x1110 - CRC Polynomial configuration register"]
    #[inline(always)]
    pub const fn crcp0_crcpoly(&self) -> &Crcp0Crcpoly {
        &self.crcp0_crcpoly
    }
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub const fn crcp0_crcin_idx(&self, n: usize) -> &Crcp0CrcinIdx {
        &self.crcp0_crcin_idx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub fn crcp0_crcin_idx_iter(&self) -> impl Iterator<Item = &Crcp0CrcinIdx> {
        self.crcp0_crcin_idx.iter()
    }
}
#[doc = "CRCP0_GPRCM\\[%s\\]"]
pub use self::crcp0_gprcm::Crcp0Gprcm;
#[doc = r"Cluster"]
#[doc = "CRCP0_GPRCM\\[%s\\]"]
pub mod crcp0_gprcm;
#[doc = "CRCP0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_desc`] module"]
#[doc(alias = "CRCP0_DESC")]
pub type Crcp0Desc = crate::Reg<crcp0_desc::Crcp0DescSpec>;
#[doc = "Module Description"]
pub mod crcp0_desc;
#[doc = "CRCP0_CRCCTRL (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcctrl`] module"]
#[doc(alias = "CRCP0_CRCCTRL")]
pub type Crcp0Crcctrl = crate::Reg<crcp0_crcctrl::Crcp0CrcctrlSpec>;
#[doc = "CRC Control Register"]
pub mod crcp0_crcctrl;
#[doc = "CRCP0_CRCSEED (w) register accessor: CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcseed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcseed`] module"]
#[doc(alias = "CRCP0_CRCSEED")]
pub type Crcp0Crcseed = crate::Reg<crcp0_crcseed::Crcp0CrcseedSpec>;
#[doc = "CRC Seed Register"]
pub mod crcp0_crcseed;
#[doc = "CRCP0_CRCIN (w) register accessor: CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcin`] module"]
#[doc(alias = "CRCP0_CRCIN")]
pub type Crcp0Crcin = crate::Reg<crcp0_crcin::Crcp0CrcinSpec>;
#[doc = "CRC Input Data Register"]
pub mod crcp0_crcin;
#[doc = "CRCP0_CRCOUT (r) register accessor: CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_crcout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcout`] module"]
#[doc(alias = "CRCP0_CRCOUT")]
pub type Crcp0Crcout = crate::Reg<crcp0_crcout::Crcp0CrcoutSpec>;
#[doc = "CRC Output Result Register"]
pub mod crcp0_crcout;
#[doc = "CRCP0_CRCPOLY (rw) register accessor: CRC Polynomial configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcp0_crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcpoly`] module"]
#[doc(alias = "CRCP0_CRCPOLY")]
pub type Crcp0Crcpoly = crate::Reg<crcp0_crcpoly::Crcp0CrcpolySpec>;
#[doc = "CRC Polynomial configuration register"]
pub mod crcp0_crcpoly;
#[doc = "CRCP0_CRCIN_IDX (w) register accessor: CRC Input Data Array Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcp0_crcin_idx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcp0_crcin_idx`] module"]
#[doc(alias = "CRCP0_CRCIN_IDX")]
pub type Crcp0CrcinIdx = crate::Reg<crcp0_crcin_idx::Crcp0CrcinIdxSpec>;
#[doc = "CRC Input Data Array Register"]
pub mod crcp0_crcin_idx;
