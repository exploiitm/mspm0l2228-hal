#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    aesadv_gprcm: [AesadvGprcm; 1],
    _reserved1: [u8; 0x0800],
    aesadv_pdbgctl: AesadvPdbgctl,
    _reserved2: [u8; 0x04],
    aesadv_int_event0: [AesadvIntEvent0; 1],
    _reserved3: [u8; 0x04],
    aesadv_int_event1: [AesadvIntEvent1; 1],
    _reserved4: [u8; 0x04],
    aesadv_int_event2: [AesadvIntEvent2; 1],
    _reserved5: [u8; 0x34],
    aesadv_evt_mode: AesadvEvtMode,
    _reserved6: [u8; 0x1c],
    aesadv_gcmccm_tag0: AesadvGcmccmTag0,
    aesadv_gcmccm_tag1: AesadvGcmccmTag1,
    aesadv_gcmccm_tag2: AesadvGcmccmTag2,
    aesadv_gcmccm_tag3: AesadvGcmccmTag3,
    aesadv_ghash_h0: AesadvGhashH0,
    aesadv_ghash_h1: AesadvGhashH1,
    aesadv_ghash_h2: AesadvGhashH2,
    aesadv_ghash_h3: AesadvGhashH3,
    aesadv_key0: AesadvKey0,
    aesadv_key1: AesadvKey1,
    aesadv_key2: AesadvKey2,
    aesadv_key3: AesadvKey3,
    aesadv_key4: AesadvKey4,
    aesadv_key5: AesadvKey5,
    aesadv_key6: AesadvKey6,
    aesadv_key7: AesadvKey7,
    aesadv_iv0: AesadvIv0,
    aesadv_iv1: AesadvIv1,
    aesadv_iv2: AesadvIv2,
    aesadv_iv3: AesadvIv3,
    aesadv_ctrl: AesadvCtrl,
    aesadv_c_length_0: AesadvCLength0,
    aesadv_c_length_1: AesadvCLength1,
    aesadv_aad_length: AesadvAadLength,
    aesadv_data0: AesadvData0,
    aesadv_data1: AesadvData1,
    aesadv_data2: AesadvData2,
    aesadv_data3: AesadvData3,
    aesadv_tag0: AesadvTag0,
    aesadv_tag1: AesadvTag1,
    aesadv_tag2: AesadvTag2,
    aesadv_tag3: AesadvTag3,
    aesadv_status: AesadvStatus,
    aesadv_data_in: AesadvDataIn,
    aesadv_data_out: AesadvDataOut,
    _reserved41: [u8; 0x44],
    aesadv_force_in_av: AesadvForceInAv,
    aesadv_ccm_aln_wrd: AesadvCcmAlnWrd,
    aesadv_blk_cnt0: AesadvBlkCnt0,
    aesadv_blk_cnt1: AesadvBlkCnt1,
    _reserved45: [u8; 0x14],
    aesadv_dma_hs: AesadvDmaHs,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - AESADV_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn aesadv_gprcm(&self, n: usize) -> &AesadvGprcm {
        &self.aesadv_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - AESADV_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn aesadv_gprcm_iter(&self) -> impl Iterator<Item = &AesadvGprcm> {
        self.aesadv_gprcm.iter()
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn aesadv_pdbgctl(&self) -> &AesadvPdbgctl {
        &self.aesadv_pdbgctl
    }
    #[doc = "0x1020..0x104c - AESADV_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn aesadv_int_event0(&self, n: usize) -> &AesadvIntEvent0 {
        &self.aesadv_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - AESADV_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn aesadv_int_event0_iter(&self) -> impl Iterator<Item = &AesadvIntEvent0> {
        self.aesadv_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - AESADV_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn aesadv_int_event1(&self, n: usize) -> &AesadvIntEvent1 {
        &self.aesadv_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - AESADV_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn aesadv_int_event1_iter(&self) -> impl Iterator<Item = &AesadvIntEvent1> {
        self.aesadv_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - AESADV_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn aesadv_int_event2(&self, n: usize) -> &AesadvIntEvent2 {
        &self.aesadv_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - AESADV_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn aesadv_int_event2_iter(&self) -> impl Iterator<Item = &AesadvIntEvent2> {
        self.aesadv_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn aesadv_evt_mode(&self) -> &AesadvEvtMode {
        &self.aesadv_evt_mode
    }
    #[doc = "0x1100 - CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)"]
    #[inline(always)]
    pub const fn aesadv_gcmccm_tag0(&self) -> &AesadvGcmccmTag0 {
        &self.aesadv_gcmccm_tag0
    }
    #[doc = "0x1104 - CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
    #[inline(always)]
    pub const fn aesadv_gcmccm_tag1(&self) -> &AesadvGcmccmTag1 {
        &self.aesadv_gcmccm_tag1
    }
    #[doc = "0x1108 - CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
    #[inline(always)]
    pub const fn aesadv_gcmccm_tag2(&self) -> &AesadvGcmccmTag2 {
        &self.aesadv_gcmccm_tag2
    }
    #[doc = "0x110c - CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)"]
    #[inline(always)]
    pub const fn aesadv_gcmccm_tag3(&self) -> &AesadvGcmccmTag3 {
        &self.aesadv_gcmccm_tag3
    }
    #[doc = "0x1110 - CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)"]
    #[inline(always)]
    pub const fn aesadv_ghash_h0(&self) -> &AesadvGhashH0 {
        &self.aesadv_ghash_h0
    }
    #[doc = "0x1114 - CCM &amp; CBC-MAC second key / GCM Hash Key input"]
    #[inline(always)]
    pub const fn aesadv_ghash_h1(&self) -> &AesadvGhashH1 {
        &self.aesadv_ghash_h1
    }
    #[doc = "0x1118 - CCM &amp; CBC-MAC second key / GCM Hash Key input"]
    #[inline(always)]
    pub const fn aesadv_ghash_h2(&self) -> &AesadvGhashH2 {
        &self.aesadv_ghash_h2
    }
    #[doc = "0x111c - CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)"]
    #[inline(always)]
    pub const fn aesadv_ghash_h3(&self) -> &AesadvGhashH3 {
        &self.aesadv_ghash_h3
    }
    #[doc = "0x1120 - KEY (LSW)"]
    #[inline(always)]
    pub const fn aesadv_key0(&self) -> &AesadvKey0 {
        &self.aesadv_key0
    }
    #[doc = "0x1124 - KEY"]
    #[inline(always)]
    pub const fn aesadv_key1(&self) -> &AesadvKey1 {
        &self.aesadv_key1
    }
    #[doc = "0x1128 - KEY"]
    #[inline(always)]
    pub const fn aesadv_key2(&self) -> &AesadvKey2 {
        &self.aesadv_key2
    }
    #[doc = "0x112c - KEY"]
    #[inline(always)]
    pub const fn aesadv_key3(&self) -> &AesadvKey3 {
        &self.aesadv_key3
    }
    #[doc = "0x1130 - KEY"]
    #[inline(always)]
    pub const fn aesadv_key4(&self) -> &AesadvKey4 {
        &self.aesadv_key4
    }
    #[doc = "0x1134 - KEY"]
    #[inline(always)]
    pub const fn aesadv_key5(&self) -> &AesadvKey5 {
        &self.aesadv_key5
    }
    #[doc = "0x1138 - KEY"]
    #[inline(always)]
    pub const fn aesadv_key6(&self) -> &AesadvKey6 {
        &self.aesadv_key6
    }
    #[doc = "0x113c - KEY (MSW)"]
    #[inline(always)]
    pub const fn aesadv_key7(&self) -> &AesadvKey7 {
        &self.aesadv_key7
    }
    #[doc = "0x1140 - IV (LSW)"]
    #[inline(always)]
    pub const fn aesadv_iv0(&self) -> &AesadvIv0 {
        &self.aesadv_iv0
    }
    #[doc = "0x1144 - IV"]
    #[inline(always)]
    pub const fn aesadv_iv1(&self) -> &AesadvIv1 {
        &self.aesadv_iv1
    }
    #[doc = "0x1148 - IV"]
    #[inline(always)]
    pub const fn aesadv_iv2(&self) -> &AesadvIv2 {
        &self.aesadv_iv2
    }
    #[doc = "0x114c - IV"]
    #[inline(always)]
    pub const fn aesadv_iv3(&self) -> &AesadvIv3 {
        &self.aesadv_iv3
    }
    #[doc = "0x1150 - Input/Output Buffer Control and Mode selection"]
    #[inline(always)]
    pub const fn aesadv_ctrl(&self) -> &AesadvCtrl {
        &self.aesadv_ctrl
    }
    #[doc = "0x1154 - Crypto data length (LSW)"]
    #[inline(always)]
    pub const fn aesadv_c_length_0(&self) -> &AesadvCLength0 {
        &self.aesadv_c_length_0
    }
    #[doc = "0x1158 - Crypto data length (MSW)"]
    #[inline(always)]
    pub const fn aesadv_c_length_1(&self) -> &AesadvCLength1 {
        &self.aesadv_c_length_1
    }
    #[doc = "0x115c - AAD Data Length"]
    #[inline(always)]
    pub const fn aesadv_aad_length(&self) -> &AesadvAadLength {
        &self.aesadv_aad_length
    }
    #[doc = "0x1160 - Data input (LSW) / Data output (LSW)"]
    #[inline(always)]
    pub const fn aesadv_data0(&self) -> &AesadvData0 {
        &self.aesadv_data0
    }
    #[doc = "0x1164 - Data input / Data output"]
    #[inline(always)]
    pub const fn aesadv_data1(&self) -> &AesadvData1 {
        &self.aesadv_data1
    }
    #[doc = "0x1168 - Data input / Data output"]
    #[inline(always)]
    pub const fn aesadv_data2(&self) -> &AesadvData2 {
        &self.aesadv_data2
    }
    #[doc = "0x116c - Data input (LSW) / Data output (MSW)"]
    #[inline(always)]
    pub const fn aesadv_data3(&self) -> &AesadvData3 {
        &self.aesadv_data3
    }
    #[doc = "0x1170 - Hash result (LSW)"]
    #[inline(always)]
    pub const fn aesadv_tag0(&self) -> &AesadvTag0 {
        &self.aesadv_tag0
    }
    #[doc = "0x1174 - Hash result"]
    #[inline(always)]
    pub const fn aesadv_tag1(&self) -> &AesadvTag1 {
        &self.aesadv_tag1
    }
    #[doc = "0x1178 - Hash result"]
    #[inline(always)]
    pub const fn aesadv_tag2(&self) -> &AesadvTag2 {
        &self.aesadv_tag2
    }
    #[doc = "0x117c - Hash result (MSW)"]
    #[inline(always)]
    pub const fn aesadv_tag3(&self) -> &AesadvTag3 {
        &self.aesadv_tag3
    }
    #[doc = "0x1180 - Status"]
    #[inline(always)]
    pub const fn aesadv_status(&self) -> &AesadvStatus {
        &self.aesadv_status
    }
    #[doc = "0x1184 - Data in alias register"]
    #[inline(always)]
    pub const fn aesadv_data_in(&self) -> &AesadvDataIn {
        &self.aesadv_data_in
    }
    #[doc = "0x1188 - Data out alias register"]
    #[inline(always)]
    pub const fn aesadv_data_out(&self) -> &AesadvDataOut {
        &self.aesadv_data_out
    }
    #[doc = "0x11d0 - Data control register for input data"]
    #[inline(always)]
    pub const fn aesadv_force_in_av(&self) -> &AesadvForceInAv {
        &self.aesadv_force_in_av
    }
    #[doc = "0x11d4 - AES-CCM AAD alignment data word"]
    #[inline(always)]
    pub const fn aesadv_ccm_aln_wrd(&self) -> &AesadvCcmAlnWrd {
        &self.aesadv_ccm_aln_wrd
    }
    #[doc = "0x11d8 - Internal block counter (LSW)"]
    #[inline(always)]
    pub const fn aesadv_blk_cnt0(&self) -> &AesadvBlkCnt0 {
        &self.aesadv_blk_cnt0
    }
    #[doc = "0x11dc - Internal block counter (MSW)"]
    #[inline(always)]
    pub const fn aesadv_blk_cnt1(&self) -> &AesadvBlkCnt1 {
        &self.aesadv_blk_cnt1
    }
    #[doc = "0x11f4 - Control register for DMA handshaking"]
    #[inline(always)]
    pub const fn aesadv_dma_hs(&self) -> &AesadvDmaHs {
        &self.aesadv_dma_hs
    }
}
#[doc = "AESADV_GPRCM\\[%s\\]"]
pub use self::aesadv_gprcm::AesadvGprcm;
#[doc = r"Cluster"]
#[doc = "AESADV_GPRCM\\[%s\\]"]
pub mod aesadv_gprcm;
#[doc = "AESADV_PDBGCTL (r) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_pdbgctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_pdbgctl`] module"]
#[doc(alias = "AESADV_PDBGCTL")]
pub type AesadvPdbgctl = crate::Reg<aesadv_pdbgctl::AesadvPdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod aesadv_pdbgctl;
#[doc = "AESADV_INT_EVENT0\\[%s\\]"]
pub use self::aesadv_int_event0::AesadvIntEvent0;
#[doc = r"Cluster"]
#[doc = "AESADV_INT_EVENT0\\[%s\\]"]
pub mod aesadv_int_event0;
#[doc = "AESADV_INT_EVENT1\\[%s\\]"]
pub use self::aesadv_int_event1::AesadvIntEvent1;
#[doc = r"Cluster"]
#[doc = "AESADV_INT_EVENT1\\[%s\\]"]
pub mod aesadv_int_event1;
#[doc = "AESADV_INT_EVENT2\\[%s\\]"]
pub use self::aesadv_int_event2::AesadvIntEvent2;
#[doc = r"Cluster"]
#[doc = "AESADV_INT_EVENT2\\[%s\\]"]
pub mod aesadv_int_event2;
#[doc = "AESADV_EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_evt_mode`] module"]
#[doc(alias = "AESADV_EVT_MODE")]
pub type AesadvEvtMode = crate::Reg<aesadv_evt_mode::AesadvEvtModeSpec>;
#[doc = "Event Mode"]
pub mod aesadv_evt_mode;
#[doc = "AESADV_GCMCCM_TAG0 (w) register accessor: CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_gcmccm_tag0`] module"]
#[doc(alias = "AESADV_GCMCCM_TAG0")]
pub type AesadvGcmccmTag0 = crate::Reg<aesadv_gcmccm_tag0::AesadvGcmccmTag0Spec>;
#[doc = "CBC-MAC third key (LSW) / GCM &amp; CCM Intermediate TAG (LSW)"]
pub mod aesadv_gcmccm_tag0;
#[doc = "AESADV_GCMCCM_TAG1 (w) register accessor: CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_gcmccm_tag1`] module"]
#[doc(alias = "AESADV_GCMCCM_TAG1")]
pub type AesadvGcmccmTag1 = crate::Reg<aesadv_gcmccm_tag1::AesadvGcmccmTag1Spec>;
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
pub mod aesadv_gcmccm_tag1;
#[doc = "AESADV_GCMCCM_TAG2 (w) register accessor: CBC-MAC third key / GCM &amp; CCM Intermediate TAG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_gcmccm_tag2`] module"]
#[doc(alias = "AESADV_GCMCCM_TAG2")]
pub type AesadvGcmccmTag2 = crate::Reg<aesadv_gcmccm_tag2::AesadvGcmccmTag2Spec>;
#[doc = "CBC-MAC third key / GCM &amp; CCM Intermediate TAG"]
pub mod aesadv_gcmccm_tag2;
#[doc = "AESADV_GCMCCM_TAG3 (w) register accessor: CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_gcmccm_tag3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_gcmccm_tag3`] module"]
#[doc(alias = "AESADV_GCMCCM_TAG3")]
pub type AesadvGcmccmTag3 = crate::Reg<aesadv_gcmccm_tag3::AesadvGcmccmTag3Spec>;
#[doc = "CBC-MAC third key (MSW) / GCM &amp; CCM Intermediate TAG (MSW)"]
pub mod aesadv_gcmccm_tag3;
#[doc = "AESADV_GHASH_H0 (w) register accessor: CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ghash_h0`] module"]
#[doc(alias = "AESADV_GHASH_H0")]
pub type AesadvGhashH0 = crate::Reg<aesadv_ghash_h0::AesadvGhashH0Spec>;
#[doc = "CCM &amp; CBC-MAC second key (LSW) / GCM Hash Key input (LSW)"]
pub mod aesadv_ghash_h0;
#[doc = "AESADV_GHASH_H1 (w) register accessor: CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ghash_h1`] module"]
#[doc(alias = "AESADV_GHASH_H1")]
pub type AesadvGhashH1 = crate::Reg<aesadv_ghash_h1::AesadvGhashH1Spec>;
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input"]
pub mod aesadv_ghash_h1;
#[doc = "AESADV_GHASH_H2 (w) register accessor: CCM &amp; CBC-MAC second key / GCM Hash Key input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ghash_h2`] module"]
#[doc(alias = "AESADV_GHASH_H2")]
pub type AesadvGhashH2 = crate::Reg<aesadv_ghash_h2::AesadvGhashH2Spec>;
#[doc = "CCM &amp; CBC-MAC second key / GCM Hash Key input"]
pub mod aesadv_ghash_h2;
#[doc = "AESADV_GHASH_H3 (w) register accessor: CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ghash_h3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ghash_h3`] module"]
#[doc(alias = "AESADV_GHASH_H3")]
pub type AesadvGhashH3 = crate::Reg<aesadv_ghash_h3::AesadvGhashH3Spec>;
#[doc = "CCM &amp; CBC-MAC second key (MSW) / GCM Hash Key input (MSW)"]
pub mod aesadv_ghash_h3;
#[doc = "AESADV_KEY0 (w) register accessor: KEY (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key0`] module"]
#[doc(alias = "AESADV_KEY0")]
pub type AesadvKey0 = crate::Reg<aesadv_key0::AesadvKey0Spec>;
#[doc = "KEY (LSW)"]
pub mod aesadv_key0;
#[doc = "AESADV_KEY1 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key1`] module"]
#[doc(alias = "AESADV_KEY1")]
pub type AesadvKey1 = crate::Reg<aesadv_key1::AesadvKey1Spec>;
#[doc = "KEY"]
pub mod aesadv_key1;
#[doc = "AESADV_KEY2 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key2`] module"]
#[doc(alias = "AESADV_KEY2")]
pub type AesadvKey2 = crate::Reg<aesadv_key2::AesadvKey2Spec>;
#[doc = "KEY"]
pub mod aesadv_key2;
#[doc = "AESADV_KEY3 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key3`] module"]
#[doc(alias = "AESADV_KEY3")]
pub type AesadvKey3 = crate::Reg<aesadv_key3::AesadvKey3Spec>;
#[doc = "KEY"]
pub mod aesadv_key3;
#[doc = "AESADV_KEY4 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key4`] module"]
#[doc(alias = "AESADV_KEY4")]
pub type AesadvKey4 = crate::Reg<aesadv_key4::AesadvKey4Spec>;
#[doc = "KEY"]
pub mod aesadv_key4;
#[doc = "AESADV_KEY5 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key5`] module"]
#[doc(alias = "AESADV_KEY5")]
pub type AesadvKey5 = crate::Reg<aesadv_key5::AesadvKey5Spec>;
#[doc = "KEY"]
pub mod aesadv_key5;
#[doc = "AESADV_KEY6 (w) register accessor: KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key6`] module"]
#[doc(alias = "AESADV_KEY6")]
pub type AesadvKey6 = crate::Reg<aesadv_key6::AesadvKey6Spec>;
#[doc = "KEY"]
pub mod aesadv_key6;
#[doc = "AESADV_KEY7 (w) register accessor: KEY (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_key7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_key7`] module"]
#[doc(alias = "AESADV_KEY7")]
pub type AesadvKey7 = crate::Reg<aesadv_key7::AesadvKey7Spec>;
#[doc = "KEY (MSW)"]
pub mod aesadv_key7;
#[doc = "AESADV_IV0 (rw) register accessor: IV (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_iv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_iv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_iv0`] module"]
#[doc(alias = "AESADV_IV0")]
pub type AesadvIv0 = crate::Reg<aesadv_iv0::AesadvIv0Spec>;
#[doc = "IV (LSW)"]
pub mod aesadv_iv0;
#[doc = "AESADV_IV1 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_iv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_iv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_iv1`] module"]
#[doc(alias = "AESADV_IV1")]
pub type AesadvIv1 = crate::Reg<aesadv_iv1::AesadvIv1Spec>;
#[doc = "IV"]
pub mod aesadv_iv1;
#[doc = "AESADV_IV2 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_iv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_iv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_iv2`] module"]
#[doc(alias = "AESADV_IV2")]
pub type AesadvIv2 = crate::Reg<aesadv_iv2::AesadvIv2Spec>;
#[doc = "IV"]
pub mod aesadv_iv2;
#[doc = "AESADV_IV3 (rw) register accessor: IV\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_iv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_iv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_iv3`] module"]
#[doc(alias = "AESADV_IV3")]
pub type AesadvIv3 = crate::Reg<aesadv_iv3::AesadvIv3Spec>;
#[doc = "IV"]
pub mod aesadv_iv3;
#[doc = "AESADV_CTRL (rw) register accessor: Input/Output Buffer Control and Mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ctrl`] module"]
#[doc(alias = "AESADV_CTRL")]
pub type AesadvCtrl = crate::Reg<aesadv_ctrl::AesadvCtrlSpec>;
#[doc = "Input/Output Buffer Control and Mode selection"]
pub mod aesadv_ctrl;
#[doc = "AESADV_C_LENGTH_0 (w) register accessor: Crypto data length (LSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_c_length_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_c_length_0`] module"]
#[doc(alias = "AESADV_C_LENGTH_0")]
pub type AesadvCLength0 = crate::Reg<aesadv_c_length_0::AesadvCLength0Spec>;
#[doc = "Crypto data length (LSW)"]
pub mod aesadv_c_length_0;
#[doc = "AESADV_C_LENGTH_1 (w) register accessor: Crypto data length (MSW)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_c_length_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_c_length_1`] module"]
#[doc(alias = "AESADV_C_LENGTH_1")]
pub type AesadvCLength1 = crate::Reg<aesadv_c_length_1::AesadvCLength1Spec>;
#[doc = "Crypto data length (MSW)"]
pub mod aesadv_c_length_1;
#[doc = "AESADV_AAD_LENGTH (w) register accessor: AAD Data Length\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_aad_length::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_aad_length`] module"]
#[doc(alias = "AESADV_AAD_LENGTH")]
pub type AesadvAadLength = crate::Reg<aesadv_aad_length::AesadvAadLengthSpec>;
#[doc = "AAD Data Length"]
pub mod aesadv_aad_length;
#[doc = "AESADV_DATA0 (rw) register accessor: Data input (LSW) / Data output (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data0`] module"]
#[doc(alias = "AESADV_DATA0")]
pub type AesadvData0 = crate::Reg<aesadv_data0::AesadvData0Spec>;
#[doc = "Data input (LSW) / Data output (LSW)"]
pub mod aesadv_data0;
#[doc = "AESADV_DATA1 (rw) register accessor: Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data1`] module"]
#[doc(alias = "AESADV_DATA1")]
pub type AesadvData1 = crate::Reg<aesadv_data1::AesadvData1Spec>;
#[doc = "Data input / Data output"]
pub mod aesadv_data1;
#[doc = "AESADV_DATA2 (rw) register accessor: Data input / Data output\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data2`] module"]
#[doc(alias = "AESADV_DATA2")]
pub type AesadvData2 = crate::Reg<aesadv_data2::AesadvData2Spec>;
#[doc = "Data input / Data output"]
pub mod aesadv_data2;
#[doc = "AESADV_DATA3 (rw) register accessor: Data input (LSW) / Data output (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data3`] module"]
#[doc(alias = "AESADV_DATA3")]
pub type AesadvData3 = crate::Reg<aesadv_data3::AesadvData3Spec>;
#[doc = "Data input (LSW) / Data output (MSW)"]
pub mod aesadv_data3;
#[doc = "AESADV_TAG0 (r) register accessor: Hash result (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_tag0`] module"]
#[doc(alias = "AESADV_TAG0")]
pub type AesadvTag0 = crate::Reg<aesadv_tag0::AesadvTag0Spec>;
#[doc = "Hash result (LSW)"]
pub mod aesadv_tag0;
#[doc = "AESADV_TAG1 (r) register accessor: Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_tag1`] module"]
#[doc(alias = "AESADV_TAG1")]
pub type AesadvTag1 = crate::Reg<aesadv_tag1::AesadvTag1Spec>;
#[doc = "Hash result"]
pub mod aesadv_tag1;
#[doc = "AESADV_TAG2 (r) register accessor: Hash result\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_tag2`] module"]
#[doc(alias = "AESADV_TAG2")]
pub type AesadvTag2 = crate::Reg<aesadv_tag2::AesadvTag2Spec>;
#[doc = "Hash result"]
pub mod aesadv_tag2;
#[doc = "AESADV_TAG3 (r) register accessor: Hash result (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_tag3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_tag3`] module"]
#[doc(alias = "AESADV_TAG3")]
pub type AesadvTag3 = crate::Reg<aesadv_tag3::AesadvTag3Spec>;
#[doc = "Hash result (MSW)"]
pub mod aesadv_tag3;
#[doc = "AESADV_STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_status`] module"]
#[doc(alias = "AESADV_STATUS")]
pub type AesadvStatus = crate::Reg<aesadv_status::AesadvStatusSpec>;
#[doc = "Status"]
pub mod aesadv_status;
#[doc = "AESADV_DATA_IN (w) register accessor: Data in alias register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_data_in::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data_in`] module"]
#[doc(alias = "AESADV_DATA_IN")]
pub type AesadvDataIn = crate::Reg<aesadv_data_in::AesadvDataInSpec>;
#[doc = "Data in alias register"]
pub mod aesadv_data_in;
#[doc = "AESADV_DATA_OUT (r) register accessor: Data out alias register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_data_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_data_out`] module"]
#[doc(alias = "AESADV_DATA_OUT")]
pub type AesadvDataOut = crate::Reg<aesadv_data_out::AesadvDataOutSpec>;
#[doc = "Data out alias register"]
pub mod aesadv_data_out;
#[doc = "AESADV_FORCE_IN_AV (w) register accessor: Data control register for input data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_force_in_av::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_force_in_av`] module"]
#[doc(alias = "AESADV_FORCE_IN_AV")]
pub type AesadvForceInAv = crate::Reg<aesadv_force_in_av::AesadvForceInAvSpec>;
#[doc = "Data control register for input data"]
pub mod aesadv_force_in_av;
#[doc = "AESADV_CCM_ALN_WRD (rw) register accessor: AES-CCM AAD alignment data word\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_ccm_aln_wrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_ccm_aln_wrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_ccm_aln_wrd`] module"]
#[doc(alias = "AESADV_CCM_ALN_WRD")]
pub type AesadvCcmAlnWrd = crate::Reg<aesadv_ccm_aln_wrd::AesadvCcmAlnWrdSpec>;
#[doc = "AES-CCM AAD alignment data word"]
pub mod aesadv_ccm_aln_wrd;
#[doc = "AESADV_BLK_CNT0 (rw) register accessor: Internal block counter (LSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_blk_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_blk_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_blk_cnt0`] module"]
#[doc(alias = "AESADV_BLK_CNT0")]
pub type AesadvBlkCnt0 = crate::Reg<aesadv_blk_cnt0::AesadvBlkCnt0Spec>;
#[doc = "Internal block counter (LSW)"]
pub mod aesadv_blk_cnt0;
#[doc = "AESADV_BLK_CNT1 (rw) register accessor: Internal block counter (MSW)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_blk_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_blk_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_blk_cnt1`] module"]
#[doc(alias = "AESADV_BLK_CNT1")]
pub type AesadvBlkCnt1 = crate::Reg<aesadv_blk_cnt1::AesadvBlkCnt1Spec>;
#[doc = "Internal block counter (MSW)"]
pub mod aesadv_blk_cnt1;
#[doc = "AESADV_DMA_HS (rw) register accessor: Control register for DMA handshaking\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadv_dma_hs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadv_dma_hs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadv_dma_hs`] module"]
#[doc(alias = "AESADV_DMA_HS")]
pub type AesadvDmaHs = crate::Reg<aesadv_dma_hs::AesadvDmaHsSpec>;
#[doc = "Control register for DMA handshaking"]
pub mod aesadv_dma_hs;
