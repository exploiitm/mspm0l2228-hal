#[repr(C)]
#[doc = "EVENTLP_SECCFG_IMPORT\\[%s\\]"]
#[doc(alias = "EVENTLP_SECCFG_IMPORT")]
pub struct EventlpSeccfgImport {
    eventlp_seccfg_import_port: EventlpSeccfgImportPort,
}
impl EventlpSeccfgImport {
    #[doc = "0x00 - Import channel ID registe"]
    #[inline(always)]
    pub const fn eventlp_seccfg_import_port(&self) -> &EventlpSeccfgImportPort {
        &self.eventlp_seccfg_import_port
    }
}
#[doc = "EVENTLP_SECCFG_IMPORT_PORT (rw) register accessor: Import channel ID registe\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_import_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_import_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_import_port`] module"]
#[doc(alias = "EVENTLP_SECCFG_IMPORT_PORT")]
pub type EventlpSeccfgImportPort =
    crate::Reg<eventlp_seccfg_import_port::EventlpSeccfgImportPortSpec>;
#[doc = "Import channel ID registe"]
pub mod eventlp_seccfg_import_port;
