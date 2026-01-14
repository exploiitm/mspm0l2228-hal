#[repr(C)]
#[doc = "EVENTLP_PUBCFG_EXPORT\\[%s\\]"]
#[doc(alias = "EVENTLP_PUBCFG_EXPORT")]
pub struct EventlpPubcfgExport {
    eventlp_pubcfg_export_port: EventlpPubcfgExportPort,
}
impl EventlpPubcfgExport {
    #[doc = "0x00 - Export channel ID register"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_export_port(&self) -> &EventlpPubcfgExportPort {
        &self.eventlp_pubcfg_export_port
    }
}
#[doc = "EVENTLP_PUBCFG_EXPORT_PORT (rw) register accessor: Export channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_export_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_export_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_export_port`] module"]
#[doc(alias = "EVENTLP_PUBCFG_EXPORT_PORT")]
pub type EventlpPubcfgExportPort =
    crate::Reg<eventlp_pubcfg_export_port::EventlpPubcfgExportPortSpec>;
#[doc = "Export channel ID register"]
pub mod eventlp_pubcfg_export_port;
