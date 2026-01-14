#[repr(C)]
#[doc = "EVENTLP_PUBCFG_CPU_CONNECT\\[%s\\]"]
#[doc(alias = "EVENTLP_PUBCFG_CPU_CONNECT")]
pub struct EventlpPubcfgCpuConnect {
    eventlp_pubcfg_cpu_num: [EventlpPubcfgCpuNum; 27],
}
impl EventlpPubcfgCpuConnect {
    #[doc = "0x00..0x6c - CPU connect register"]
    #[inline(always)]
    pub const fn eventlp_pubcfg_cpu_num(&self, n: usize) -> &EventlpPubcfgCpuNum {
        &self.eventlp_pubcfg_cpu_num[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x6c - CPU connect register"]
    #[inline(always)]
    pub fn eventlp_pubcfg_cpu_num_iter(&self) -> impl Iterator<Item = &EventlpPubcfgCpuNum> {
        self.eventlp_pubcfg_cpu_num.iter()
    }
}
#[doc = "EVENTLP_PUBCFG_CPU_NUM (rw) register accessor: CPU connect register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_pubcfg_cpu_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_pubcfg_cpu_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_pubcfg_cpu_num`] module"]
#[doc(alias = "EVENTLP_PUBCFG_CPU_NUM")]
pub type EventlpPubcfgCpuNum = crate::Reg<eventlp_pubcfg_cpu_num::EventlpPubcfgCpuNumSpec>;
#[doc = "CPU connect register"]
pub mod eventlp_pubcfg_cpu_num;
