#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    wuc_fsub_0: WucFsub0,
    wuc_fsub_1: WucFsub1,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Port 0"]
    #[inline(always)]
    pub const fn wuc_fsub_0(&self) -> &WucFsub0 {
        &self.wuc_fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn wuc_fsub_1(&self) -> &WucFsub1 {
        &self.wuc_fsub_1
    }
}
#[doc = "WUC_FSUB_0 (rw) register accessor: Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wuc_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuc_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuc_fsub_0`] module"]
#[doc(alias = "WUC_FSUB_0")]
pub type WucFsub0 = crate::Reg<wuc_fsub_0::WucFsub0Spec>;
#[doc = "Subscriber Port 0"]
pub mod wuc_fsub_0;
#[doc = "WUC_FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wuc_fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuc_fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuc_fsub_1`] module"]
#[doc(alias = "WUC_FSUB_1")]
pub type WucFsub1 = crate::Reg<wuc_fsub_1::WucFsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod wuc_fsub_1;
