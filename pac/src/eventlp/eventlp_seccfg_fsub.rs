#[repr(C)]
#[doc = "EVENTLP_SECCFG_FSUB\\[%s\\]"]
#[doc(alias = "EVENTLP_SECCFG_FSUB")]
pub struct EventlpSeccfgFsub {
    eventlp_seccfg_fsub_port: [EventlpSeccfgFsubPort; 25],
}
impl EventlpSeccfgFsub {
    #[doc = "0x00..0x19 - Subscriber channel ID register"]
    #[inline(always)]
    pub const fn eventlp_seccfg_fsub_port(&self, n: usize) -> &EventlpSeccfgFsubPort {
        &self.eventlp_seccfg_fsub_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x19 - Subscriber channel ID register"]
    #[inline(always)]
    pub fn eventlp_seccfg_fsub_port_iter(&self) -> impl Iterator<Item = &EventlpSeccfgFsubPort> {
        self.eventlp_seccfg_fsub_port.iter()
    }
}
#[doc = "EVENTLP_SECCFG_FSUB_PORT (rw) register accessor: Subscriber channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_fsub_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_fsub_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_fsub_port`] module"]
#[doc(alias = "EVENTLP_SECCFG_FSUB_PORT")]
pub type EventlpSeccfgFsubPort = crate::Reg<eventlp_seccfg_fsub_port::EventlpSeccfgFsubPortSpec>;
#[doc = "Subscriber channel ID register"]
pub mod eventlp_seccfg_fsub_port;
