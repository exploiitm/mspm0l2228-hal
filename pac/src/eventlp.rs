#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xf8],
    eventlp_pubcfg_desc_ex: EventlpPubcfgDescEx,
    eventlp_pubcfg_desc: EventlpPubcfgDesc,
    eventlp_pubcfg_fsub: [EventlpPubcfgFsub; 1],
    _reserved3: [u8; 0x019c],
    eventlp_pubcfg_fpub: [EventlpPubcfgFpub; 1],
    _reserved4: [u8; 0x01ac],
    eventlp_pubcfg_export: [EventlpPubcfgExport; 1],
    _reserved5: [u8; 0x01fc],
    eventlp_pubcfg_import: [EventlpPubcfgImport; 1],
    _reserved6: [u8; 0x01fc],
    eventlp_pubcfg_cpu_connect: [EventlpPubcfgCpuConnect; 1],
    _reserved7: [u8; 0x078c],
    eventlp_seccfg_desc_ex: EventlpSeccfgDescEx,
    eventlp_seccfg_desc: EventlpSeccfgDesc,
    eventlp_seccfg_fsub: [EventlpSeccfgFsub; 1],
    _reserved10: [u8; 0x67],
    eventlp_seccfg_fpub: [EventlpSeccfgFpub; 1],
    _reserved11: [u8; 0x6b],
    eventlp_seccfg_export: [EventlpSeccfgExport; 1],
    _reserved12: [u8; 0x7f],
    eventlp_seccfg_import: [EventlpSeccfgImport; 1],
    _reserved13: [u8; 0x7f],
    eventlp_seccfg_cpu_connect: [EventlpSeccfgCpuConnect; 1],
    _reserved14: [u8; 0xe5],
    eventlp_ctl: EventlpCtl,
    _reserved15: [u8; 0x07ff],
    eventlp_lmgmt_sftydiag: [EventlpLmgmtSftydiag; 1],
    eventlp_impexpcfg_export: [EventlpImpexpcfgExport; 1],
    _reserved17: [u8; 0x01fc],
    eventlp_impexpcfg_import: [EventlpImpexpcfgImport; 1],
}
impl RegisterBlock {
    #[doc = "0xf8 - Extended Module Description"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_desc_ex(&self) -> &EventlpPubcfgDescEx {
        &self.eventlp_pubcfg_desc_ex
    }
    #[doc = "0xfc - Module Description"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_desc(&self) -> &EventlpPubcfgDesc {
        &self.eventlp_pubcfg_desc
    }
    #[doc = "0x100..0x164 - EVENTLP_PUBCFG_FSUB\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_fsub(&self, n: usize) -> &EventlpPubcfgFsub {
        &self.eventlp_pubcfg_fsub[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x164 - EVENTLP_PUBCFG_FSUB\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_pubcfg_fsub_iter(&self) -> impl Iterator<Item = &EventlpPubcfgFsub> {
        self.eventlp_pubcfg_fsub.iter()
    }
    #[doc = "0x300..0x354 - EVENTLP_PUBCFG_FPUB\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_fpub(&self, n: usize) -> &EventlpPubcfgFpub {
        &self.eventlp_pubcfg_fpub[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x354 - EVENTLP_PUBCFG_FPUB\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_pubcfg_fpub_iter(&self) -> impl Iterator<Item = &EventlpPubcfgFpub> {
        self.eventlp_pubcfg_fpub.iter()
    }
    #[doc = "0x500 - EVENTLP_PUBCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_export(&self, n: usize) -> &EventlpPubcfgExport {
        &self.eventlp_pubcfg_export[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500 - EVENTLP_PUBCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_pubcfg_export_iter(&self) -> impl Iterator<Item = &EventlpPubcfgExport> {
        self.eventlp_pubcfg_export.iter()
    }
    #[doc = "0x700 - EVENTLP_PUBCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_import(&self, n: usize) -> &EventlpPubcfgImport {
        &self.eventlp_pubcfg_import[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700 - EVENTLP_PUBCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_pubcfg_import_iter(&self) -> impl Iterator<Item = &EventlpPubcfgImport> {
        self.eventlp_pubcfg_import.iter()
    }
    #[doc = "0x900..0x96c - EVENTLP_PUBCFG_CPU_CONNECT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_cpu_connect(&self, n: usize) -> &EventlpPubcfgCpuConnect {
        &self.eventlp_pubcfg_cpu_connect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x96c - EVENTLP_PUBCFG_CPU_CONNECT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_pubcfg_cpu_connect_iter(
        &self,
    ) -> impl Iterator<Item = &EventlpPubcfgCpuConnect> {
        self.eventlp_pubcfg_cpu_connect.iter()
    }
    #[doc = "0x10f8 - Extended Module Description"]
    #[inline(always)]
    pub const fn eventlp_seccfg_desc_ex(&self) -> &EventlpSeccfgDescEx {
        &self.eventlp_seccfg_desc_ex
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn eventlp_seccfg_desc(&self) -> &EventlpSeccfgDesc {
        &self.eventlp_seccfg_desc
    }
    #[doc = "0x1100..0x1119 - EVENTLP_SECCFG_FSUB\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_seccfg_fsub(&self, n: usize) -> &EventlpSeccfgFsub {
        &self.eventlp_seccfg_fsub[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1100..0x1119 - EVENTLP_SECCFG_FSUB\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_seccfg_fsub_iter(&self) -> impl Iterator<Item = &EventlpSeccfgFsub> {
        self.eventlp_seccfg_fsub.iter()
    }
    #[doc = "0x1180..0x1195 - EVENTLP_SECCFG_FPUB\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_seccfg_fpub(&self, n: usize) -> &EventlpSeccfgFpub {
        &self.eventlp_seccfg_fpub[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1180..0x1195 - EVENTLP_SECCFG_FPUB\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_seccfg_fpub_iter(&self) -> impl Iterator<Item = &EventlpSeccfgFpub> {
        self.eventlp_seccfg_fpub.iter()
    }
    #[doc = "0x1200 - EVENTLP_SECCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_seccfg_export(&self, n: usize) -> &EventlpSeccfgExport {
        &self.eventlp_seccfg_export[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1200 - EVENTLP_SECCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_seccfg_export_iter(&self) -> impl Iterator<Item = &EventlpSeccfgExport> {
        self.eventlp_seccfg_export.iter()
    }
    #[doc = "0x1280 - EVENTLP_SECCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_seccfg_import(&self, n: usize) -> &EventlpSeccfgImport {
        &self.eventlp_seccfg_import[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1280 - EVENTLP_SECCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_seccfg_import_iter(&self) -> impl Iterator<Item = &EventlpSeccfgImport> {
        self.eventlp_seccfg_import.iter()
    }
    #[doc = "0x1300..0x131b - EVENTLP_SECCFG_CPU_CONNECT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_seccfg_cpu_connect(&self, n: usize) -> &EventlpSeccfgCpuConnect {
        &self.eventlp_seccfg_cpu_connect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1300..0x131b - EVENTLP_SECCFG_CPU_CONNECT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_seccfg_cpu_connect_iter(
        &self,
    ) -> impl Iterator<Item = &EventlpSeccfgCpuConnect> {
        self.eventlp_seccfg_cpu_connect.iter()
    }
    #[doc = "0x1400 - Event Manager control register"]
    #[inline(always)]
    pub const fn eventlp_ctl(&self) -> &EventlpCtl {
        &self.eventlp_ctl
    }
    #[doc = "0x1c00..0x2000 - EVENTLP_LMGMT_SFTYDIAG\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_lmgmt_sftydiag(&self, n: usize) -> &EventlpLmgmtSftydiag {
        &self.eventlp_lmgmt_sftydiag[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c00..0x2000 - EVENTLP_LMGMT_SFTYDIAG\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_lmgmt_sftydiag_iter(&self) -> impl Iterator<Item = &EventlpLmgmtSftydiag> {
        self.eventlp_lmgmt_sftydiag.iter()
    }
    #[doc = "0x2000 - EVENTLP_IMPEXPCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_impexpcfg_export(&self, n: usize) -> &EventlpImpexpcfgExport {
        &self.eventlp_impexpcfg_export[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000 - EVENTLP_IMPEXPCFG_EXPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_impexpcfg_export_iter(&self) -> impl Iterator<Item = &EventlpImpexpcfgExport> {
        self.eventlp_impexpcfg_export.iter()
    }
    #[doc = "0x2200 - EVENTLP_IMPEXPCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub const fn eventlp_impexpcfg_import(&self, n: usize) -> &EventlpImpexpcfgImport {
        &self.eventlp_impexpcfg_import[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2200 - EVENTLP_IMPEXPCFG_IMPORT\\[%s\\]"]
    #[inline(always)]
    pub fn eventlp_impexpcfg_import_iter(&self) -> impl Iterator<Item = &EventlpImpexpcfgImport> {
        self.eventlp_impexpcfg_import.iter()
    }
}
#[doc = "EVENTLP_PUBCFG_DESC_EX (r) register accessor: Extended Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_desc_ex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_desc_ex`] module"]
#[doc(alias = "EVENTLP_PUBCFG_DESC_EX")]
pub type EventlpPubcfgDescEx = crate::Reg<eventlp_pubcfg_desc_ex::EventlpPubcfgDescExSpec>;
#[doc = "Extended Module Description"]
pub mod eventlp_pubcfg_desc_ex;
#[doc = "EVENTLP_PUBCFG_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_desc`] module"]
#[doc(alias = "EVENTLP_PUBCFG_DESC")]
pub type EventlpPubcfgDesc = crate::Reg<eventlp_pubcfg_desc::EventlpPubcfgDescSpec>;
#[doc = "Module Description"]
pub mod eventlp_pubcfg_desc;
#[doc = "EVENTLP_PUBCFG_FSUB\\[%s\\]"]
pub use self::eventlp_pubcfg_fsub::EventlpPubcfgFsub;
#[doc = r"Cluster"]
#[doc = "EVENTLP_PUBCFG_FSUB\\[%s\\]"]
pub mod eventlp_pubcfg_fsub;
#[doc = "EVENTLP_PUBCFG_FPUB\\[%s\\]"]
pub use self::eventlp_pubcfg_fpub::EventlpPubcfgFpub;
#[doc = r"Cluster"]
#[doc = "EVENTLP_PUBCFG_FPUB\\[%s\\]"]
pub mod eventlp_pubcfg_fpub;
#[doc = "EVENTLP_PUBCFG_EXPORT\\[%s\\]"]
pub use self::eventlp_pubcfg_export::EventlpPubcfgExport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_PUBCFG_EXPORT\\[%s\\]"]
pub mod eventlp_pubcfg_export;
#[doc = "EVENTLP_PUBCFG_IMPORT\\[%s\\]"]
pub use self::eventlp_pubcfg_import::EventlpPubcfgImport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_PUBCFG_IMPORT\\[%s\\]"]
pub mod eventlp_pubcfg_import;
#[doc = "EVENTLP_PUBCFG_CPU_CONNECT\\[%s\\]"]
pub use self::eventlp_pubcfg_cpu_connect::EventlpPubcfgCpuConnect;
#[doc = r"Cluster"]
#[doc = "EVENTLP_PUBCFG_CPU_CONNECT\\[%s\\]"]
pub mod eventlp_pubcfg_cpu_connect;
#[doc = "EVENTLP_SECCFG_DESC_EX (r) register accessor: Extended Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_desc_ex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_desc_ex`] module"]
#[doc(alias = "EVENTLP_SECCFG_DESC_EX")]
pub type EventlpSeccfgDescEx = crate::Reg<eventlp_seccfg_desc_ex::EventlpSeccfgDescExSpec>;
#[doc = "Extended Module Description"]
pub mod eventlp_seccfg_desc_ex;
#[doc = "EVENTLP_SECCFG_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_desc`] module"]
#[doc(alias = "EVENTLP_SECCFG_DESC")]
pub type EventlpSeccfgDesc = crate::Reg<eventlp_seccfg_desc::EventlpSeccfgDescSpec>;
#[doc = "Module Description"]
pub mod eventlp_seccfg_desc;
#[doc = "EVENTLP_SECCFG_FSUB\\[%s\\]"]
pub use self::eventlp_seccfg_fsub::EventlpSeccfgFsub;
#[doc = r"Cluster"]
#[doc = "EVENTLP_SECCFG_FSUB\\[%s\\]"]
pub mod eventlp_seccfg_fsub;
#[doc = "EVENTLP_SECCFG_FPUB\\[%s\\]"]
pub use self::eventlp_seccfg_fpub::EventlpSeccfgFpub;
#[doc = r"Cluster"]
#[doc = "EVENTLP_SECCFG_FPUB\\[%s\\]"]
pub mod eventlp_seccfg_fpub;
#[doc = "EVENTLP_SECCFG_EXPORT\\[%s\\]"]
pub use self::eventlp_seccfg_export::EventlpSeccfgExport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_SECCFG_EXPORT\\[%s\\]"]
pub mod eventlp_seccfg_export;
#[doc = "EVENTLP_SECCFG_IMPORT\\[%s\\]"]
pub use self::eventlp_seccfg_import::EventlpSeccfgImport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_SECCFG_IMPORT\\[%s\\]"]
pub mod eventlp_seccfg_import;
#[doc = "EVENTLP_SECCFG_CPU_CONNECT\\[%s\\]"]
pub use self::eventlp_seccfg_cpu_connect::EventlpSeccfgCpuConnect;
#[doc = r"Cluster"]
#[doc = "EVENTLP_SECCFG_CPU_CONNECT\\[%s\\]"]
pub mod eventlp_seccfg_cpu_connect;
#[doc = "EVENTLP_CTL (rw) register accessor: Event Manager control register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_ctl`] module"]
#[doc(alias = "EVENTLP_CTL")]
pub type EventlpCtl = crate::Reg<eventlp_ctl::EventlpCtlSpec>;
#[doc = "Event Manager control register"]
pub mod eventlp_ctl;
#[doc = "EVENTLP_LMGMT_SFTYDIAG\\[%s\\]"]
pub use self::eventlp_lmgmt_sftydiag::EventlpLmgmtSftydiag;
#[doc = r"Cluster"]
#[doc = "EVENTLP_LMGMT_SFTYDIAG\\[%s\\]"]
pub mod eventlp_lmgmt_sftydiag;
#[doc = "EVENTLP_IMPEXPCFG_EXPORT\\[%s\\]"]
pub use self::eventlp_impexpcfg_export::EventlpImpexpcfgExport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_IMPEXPCFG_EXPORT\\[%s\\]"]
pub mod eventlp_impexpcfg_export;
#[doc = "EVENTLP_IMPEXPCFG_IMPORT\\[%s\\]"]
pub use self::eventlp_impexpcfg_import::EventlpImpexpcfgImport;
#[doc = r"Cluster"]
#[doc = "EVENTLP_IMPEXPCFG_IMPORT\\[%s\\]"]
pub mod eventlp_impexpcfg_import;
