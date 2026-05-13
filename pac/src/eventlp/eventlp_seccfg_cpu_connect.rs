#[repr(C)]
#[doc = "EVENTLP_SECCFG_CPU_CONNECT\\[%s\\]"]
#[doc(alias = "EVENTLP_SECCFG_CPU_CONNECT")]
pub struct EventlpSeccfgCpuConnect {
    eventlp_seccfg_cpu_num: [EventlpSeccfgCpuNum; 27],
}
impl EventlpSeccfgCpuConnect {
    #[doc = "0x00..0x1b - CPU connect register"]
    #[inline(always)]
    pub const fn eventlp_seccfg_cpu_num(&self, n: usize) -> &EventlpSeccfgCpuNum {
        &self.eventlp_seccfg_cpu_num[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1b - CPU connect register"]
    #[inline(always)]
    pub fn eventlp_seccfg_cpu_num_iter(&self) -> impl Iterator<Item = &EventlpSeccfgCpuNum> {
        self.eventlp_seccfg_cpu_num.iter()
    }
}
#[doc = "EVENTLP_SECCFG_CPU_NUM (rw) register accessor: CPU connect register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventlp_seccfg_cpu_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventlp_seccfg_cpu_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventlp_seccfg_cpu_num`] module"]
#[doc(alias = "EVENTLP_SECCFG_CPU_NUM")]
pub type EventlpSeccfgCpuNum = crate::Reg<eventlp_seccfg_cpu_num::EventlpSeccfgCpuNumSpec>;
#[doc = "CPU connect register"]
pub mod eventlp_seccfg_cpu_num;
