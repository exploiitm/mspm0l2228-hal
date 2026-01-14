#[repr(C)]
#[doc = "EVENTLP_PUBCFG_FPUB\\[%s\\]"]
#[doc(alias = "EVENTLP_PUBCFG_FPUB")]
pub struct EventlpPubcfgFpub {
    eventlp_pubcfg_fpub_port: [EventlpPubcfgFpubPort; 21],
}
impl EventlpPubcfgFpub {
    #[doc = "0x00..0x54 - Publisher channel ID register"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_fpub_port(&self, n: usize) -> &EventlpPubcfgFpubPort {
        &self.eventlp_pubcfg_fpub_port[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x54 - Publisher channel ID register"]
    #[inline(always)]
    pub fn eventlp_pubcfg_fpub_port_iter(&self) -> impl Iterator<Item = &EventlpPubcfgFpubPort> {
        self.eventlp_pubcfg_fpub_port.iter()
    }
}
#[doc = "EVENTLP_PUBCFG_FPUB_PORT (rw) register accessor: Publisher channel ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_fpub_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_fpub_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_fpub_port`] module"]
#[doc(alias = "EVENTLP_PUBCFG_FPUB_PORT")]
pub type EventlpPubcfgFpubPort = crate::Reg<eventlp_pubcfg_fpub_port::EventlpPubcfgFpubPortSpec>;
#[doc = "Publisher channel ID register"]
pub mod eventlp_pubcfg_fpub_port;
