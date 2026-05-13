#[repr(C)]
#[doc = "EVENTLP_PUBCFG_FSUB\\[%s\\]"]
#[doc(alias = "EVENTLP_PUBCFG_FSUB")]
pub struct EventlpPubcfgFsub {
    eventlp_pubcfg_fsub_port: [EventlpPubcfgFsubPort; 25],
}
impl EventlpPubcfgFsub {
    #[doc = "0x00..0x64 - Subscriber channel ID register"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_fsub_port(&self, n: usize) -> &EventlpPubcfgFsubPort {
        &self.eventlp_pubcfg_fsub_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x64 - Subscriber channel ID register"]
    #[inline(always)]
    pub fn eventlp_pubcfg_fsub_port_iter(&self) -> impl Iterator<Item = &EventlpPubcfgFsubPort> {
        self.eventlp_pubcfg_fsub_port.iter()
    }
}
#[doc = "EVENTLP_PUBCFG_FSUB_PORT (rw) register accessor: Subscriber channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_fsub_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_fsub_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_fsub_port`] module"]
#[doc(alias = "EVENTLP_PUBCFG_FSUB_PORT")]
pub type EventlpPubcfgFsubPort = crate::Reg<eventlp_pubcfg_fsub_port::EventlpPubcfgFsubPortSpec>;
#[doc = "Subscriber channel ID register"]
pub mod eventlp_pubcfg_fsub_port;
