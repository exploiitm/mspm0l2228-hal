#[repr(C)]
#[doc = "EVENTLP_SECCFG_FPUB\\[%s\\]"]
#[doc(alias = "EVENTLP_SECCFG_FPUB")]
pub struct EventlpSeccfgFpub {
    eventlp_seccfg_fpub_port: [EventlpSeccfgFpubPort; 21],
}
impl EventlpSeccfgFpub {
    #[doc = "0x00..0x15 - Publisher channel ID register"]
    #[inline(always)]
    pub const fn eventlp_seccfg_fpub_port(&self, n: usize) -> &EventlpSeccfgFpubPort {
        &self.eventlp_seccfg_fpub_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x15 - Publisher channel ID register"]
    #[inline(always)]
    pub fn eventlp_seccfg_fpub_port_iter(&self) -> impl Iterator<Item = &EventlpSeccfgFpubPort> {
        self.eventlp_seccfg_fpub_port.iter()
    }
}
#[doc = "EVENTLP_SECCFG_FPUB_PORT (rw) register accessor: Publisher channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_fpub_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_fpub_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_fpub_port`] module"]
#[doc(alias = "EVENTLP_SECCFG_FPUB_PORT")]
pub type EventlpSeccfgFpubPort = crate::Reg<eventlp_seccfg_fpub_port::EventlpSeccfgFpubPortSpec>;
#[doc = "Publisher channel ID register"]
pub mod eventlp_seccfg_fpub_port;
