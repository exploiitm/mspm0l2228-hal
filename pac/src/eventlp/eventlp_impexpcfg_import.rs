#[repr(C)]
#[doc = "EVENTLP_IMPEXPCFG_IMPORT\\[%s\\]"]
#[doc(alias = "EVENTLP_IMPEXPCFG_IMPORT")]
pub struct EventlpImpexpcfgImport {
    eventlp_impexpcfg_import_port: EventlpImpexpcfgImportPort,
}
impl EventlpImpexpcfgImport {
    #[doc = "0x00 - Import channel ID registe"]
    #[inline(always)]
    pub const fn eventlp_impexpcfg_import_port(&self) -> &EventlpImpexpcfgImportPort {
        &self.eventlp_impexpcfg_import_port
    }
}
#[doc = "EVENTLP_IMPEXPCFG_IMPORT_PORT (rw) register accessor: Import channel ID registe\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_impexpcfg_import_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_impexpcfg_import_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_impexpcfg_import_port`] module"]
#[doc(alias = "EVENTLP_IMPEXPCFG_IMPORT_PORT")]
pub type EventlpImpexpcfgImportPort =
    crate::Reg<eventlp_impexpcfg_import_port::EventlpImpexpcfgImportPortSpec>;
#[doc = "Import channel ID registe"]
pub mod eventlp_impexpcfg_import_port;
