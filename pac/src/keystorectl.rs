#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1100],
    keystorectl_cfg: KeystorectlCfg,
    keystorectl_keywr: KeystorectlKeywr,
    keystorectl_keyrd: KeystorectlKeyrd,
    keystorectl_status: KeystorectlStatus,
    keystorectl_keyin: KeystorectlKeyin,
}
impl RegisterBlock {
    #[doc = "0x1100 - Keystore configuration"]
    #[inline(always)]
    pub const fn keystorectl_cfg(&self) -> &KeystorectlCfg {
        &self.keystorectl_cfg
    }
    #[doc = "0x1104 - Key write configuration"]
    #[inline(always)]
    pub const fn keystorectl_keywr(&self) -> &KeystorectlKeywr {
        &self.keystorectl_keywr
    }
    #[doc = "0x1108 - Key read configuration"]
    #[inline(always)]
    pub const fn keystorectl_keyrd(&self) -> &KeystorectlKeyrd {
        &self.keystorectl_keyrd
    }
    #[doc = "0x110c - Status"]
    #[inline(always)]
    pub const fn keystorectl_status(&self) -> &KeystorectlStatus {
        &self.keystorectl_status
    }
    #[doc = "0x1110 - Input key"]
    #[inline(always)]
    pub const fn keystorectl_keyin(&self) -> &KeystorectlKeyin {
        &self.keystorectl_keyin
    }
}
#[doc = "KEYSTORECTL_CFG (rw) register accessor: Keystore configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keystorectl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keystorectl_cfg`] module"]
#[doc(alias = "KEYSTORECTL_CFG")]
pub type KeystorectlCfg = crate::Reg<keystorectl_cfg::KeystorectlCfgSpec>;
#[doc = "Keystore configuration"]
pub mod keystorectl_cfg;
#[doc = "KEYSTORECTL_KEYWR (rw) register accessor: Key write configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keystorectl_keywr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_keywr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keystorectl_keywr`] module"]
#[doc(alias = "KEYSTORECTL_KEYWR")]
pub type KeystorectlKeywr = crate::Reg<keystorectl_keywr::KeystorectlKeywrSpec>;
#[doc = "Key write configuration"]
pub mod keystorectl_keywr;
#[doc = "KEYSTORECTL_KEYRD (rw) register accessor: Key read configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`keystorectl_keyrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_keyrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keystorectl_keyrd`] module"]
#[doc(alias = "KEYSTORECTL_KEYRD")]
pub type KeystorectlKeyrd = crate::Reg<keystorectl_keyrd::KeystorectlKeyrdSpec>;
#[doc = "Key read configuration"]
pub mod keystorectl_keyrd;
#[doc = "KEYSTORECTL_STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`keystorectl_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keystorectl_status`] module"]
#[doc(alias = "KEYSTORECTL_STATUS")]
pub type KeystorectlStatus = crate::Reg<keystorectl_status::KeystorectlStatusSpec>;
#[doc = "Status"]
pub mod keystorectl_status;
#[doc = "KEYSTORECTL_KEYIN (w) register accessor: Input key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keystorectl_keyin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keystorectl_keyin`] module"]
#[doc(alias = "KEYSTORECTL_KEYIN")]
pub type KeystorectlKeyin = crate::Reg<keystorectl_keyin::KeystorectlKeyinSpec>;
#[doc = "Input key"]
pub mod keystorectl_keyin;
