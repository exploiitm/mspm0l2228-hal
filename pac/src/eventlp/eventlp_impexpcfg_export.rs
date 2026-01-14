#[repr(C)]
#[doc = "EVENTLP_IMPEXPCFG_EXPORT\\[%s\\]"]
#[doc(alias = "EVENTLP_IMPEXPCFG_EXPORT")]
pub struct EventlpImpexpcfgExport {
    eventlp_impexpcfg_export_port: EventlpImpexpcfgExportPort,
}
impl EventlpImpexpcfgExport {
    #[doc = "0x00 - Export channel ID register"]
    #[inline(always)]
    pub const fn eventlp_impexpcfg_export_port(&self) -> &EventlpImpexpcfgExportPort {
        &self.eventlp_impexpcfg_export_port
    }
}
#[doc = "EVENTLP_IMPEXPCFG_EXPORT_PORT (rw) register accessor: Export channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_impexpcfg_export_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_impexpcfg_export_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_impexpcfg_export_port`] module"]
#[doc(alias = "EVENTLP_IMPEXPCFG_EXPORT_PORT")]
pub type EventlpImpexpcfgExportPort =
    crate::Reg<eventlp_impexpcfg_export_port::EventlpImpexpcfgExportPortSpec>;
#[doc = "Export channel ID register"]
pub mod eventlp_impexpcfg_export_port;
