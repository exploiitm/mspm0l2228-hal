#[repr(C)]
#[doc = "EVENTLP_PUBCFG_IMPORT\\[%s\\]"]
#[doc(alias = "EVENTLP_PUBCFG_IMPORT")]
pub struct EventlpPubcfgImport {
    eventlp_pubcfg_import_port: EventlpPubcfgImportPort,
}
impl EventlpPubcfgImport {
    #[doc = "0x00 - Import channel ID registe"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_import_port(&self) -> &EventlpPubcfgImportPort {
        &self.eventlp_pubcfg_import_port
    }
}
#[doc = "EVENTLP_PUBCFG_IMPORT_PORT (rw) register accessor: Import channel ID registe\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_import_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_import_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_import_port`] module"]
#[doc(alias = "EVENTLP_PUBCFG_IMPORT_PORT")]
pub type EventlpPubcfgImportPort =
    crate::Reg<eventlp_pubcfg_import_port::EventlpPubcfgImportPortSpec>;
#[doc = "Import channel ID registe"]
pub mod eventlp_pubcfg_import_port;
