#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sct0_in_sel: [Sct0InSel; 7],
    _reserved1: [u8; 0xe4],
    pint_sel: [PintSel; 8],
    _reserved2: [u8; 0x20],
    dsp_int_sel: [DspIntSel; 27],
    _reserved3: [u8; 0x54],
    dmac0_itrig_sel: [Dmac0ItrigSel; 33],
    _reserved4: [u8; 0x7c],
    dmac0_otrig_sel: [Dmac0OtrigSel; 4],
    _reserved5: [u8; 0xf0],
    dmac1_itrig_sel: [Dmac1ItrigSel; 33],
    _reserved6: [u8; 0x7c],
    dmac1_otrig_sel: [Dmac1OtrigSel; 4],
    _reserved7: [u8; 0xf0],
    ct32bit_cap: [Ct32bitCap; 5],
    _reserved8: [u8; 0xb0],
    fmeasure_ch_sel: [FmeasureChSel; 2],
    _reserved9: [u8; 0x38],
    dmac0_req_ena0: Dmac0ReqEna0,
    dmac0_req_ena1: Dmac0ReqEna1,
    dmac0_req_ena0_set: Dmac0ReqEna0Set,
    dmac0_req_ena1_set: Dmac0ReqEna1Set,
    dmac0_req_ena0_clr: Dmac0ReqEna0Clr,
    dmac0_req_ena1_clr: Dmac0ReqEna1Clr,
    _reserved15: [u8; 0x08],
    dmac1_req_ena0: Dmac1ReqEna0,
    dmac1_req_ena1: Dmac1ReqEna1,
    dmac1_req_ena0_set: Dmac1ReqEna0Set,
    dmac1_req_ena1_set: Dmac1ReqEna1Set,
    dmac1_req_ena0_clr: Dmac1ReqEna0Clr,
    dmac1_req_ena1_clr: Dmac1ReqEna1Clr,
    _reserved21: [u8; 0x08],
    dmac0_itrig_ena0: Dmac0ItrigEna0,
    _reserved22: [u8; 0x04],
    dmac0_itrig_ena0_set: Dmac0ItrigEna0Set,
    _reserved23: [u8; 0x04],
    dmac0_itrig_ena0_clr: Dmac0ItrigEna0Clr,
    _reserved24: [u8; 0x0c],
    dmac1_itrig_ena0: Dmac1ItrigEna0,
    _reserved25: [u8; 0x04],
    dmac1_itrig_ena0_set: Dmac1ItrigEna0Set,
    _reserved26: [u8; 0x04],
    dmac1_itrig_ena0_clr: Dmac1ItrigEna0Clr,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - SCT Peripheral Input Multiplexers N"]
    #[inline(always)]
    pub const fn sct0_in_sel(&self, n: usize) -> &Sct0InSel {
        &self.sct0_in_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - SCT Peripheral Input Multiplexers N"]
    #[inline(always)]
    pub fn sct0_in_sel_iter(&self) -> impl Iterator<Item = &Sct0InSel> {
        self.sct0_in_sel.iter()
    }
    #[doc = "0x100..0x120 - GPIO Pin Input Multiplexer N"]
    #[inline(always)]
    pub const fn pint_sel(&self, n: usize) -> &PintSel {
        &self.pint_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - GPIO Pin Input Multiplexer N"]
    #[inline(always)]
    pub fn pint_sel_iter(&self) -> impl Iterator<Item = &PintSel> {
        self.pint_sel.iter()
    }
    #[doc = "0x140..0x1ac - DSP Interrupt Input Multiplexers N"]
    #[inline(always)]
    pub const fn dsp_int_sel(&self, n: usize) -> &DspIntSel {
        &self.dsp_int_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x1ac - DSP Interrupt Input Multiplexers N"]
    #[inline(always)]
    pub fn dsp_int_sel_iter(&self) -> impl Iterator<Item = &DspIntSel> {
        self.dsp_int_sel.iter()
    }
    #[doc = "0x200..0x284 - DMAC0 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac0_itrig_sel(&self, n: usize) -> &Dmac0ItrigSel {
        &self.dmac0_itrig_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x284 - DMAC0 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub fn dmac0_itrig_sel_iter(&self) -> impl Iterator<Item = &Dmac0ItrigSel> {
        self.dmac0_itrig_sel.iter()
    }
    #[doc = "0x300..0x310 - DMAC0 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac0_otrig_sel(&self, n: usize) -> &Dmac0OtrigSel {
        &self.dmac0_otrig_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x310 - DMAC0 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub fn dmac0_otrig_sel_iter(&self) -> impl Iterator<Item = &Dmac0OtrigSel> {
        self.dmac0_otrig_sel.iter()
    }
    #[doc = "0x400..0x484 - DMAC1 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac1_itrig_sel(&self, n: usize) -> &Dmac1ItrigSel {
        &self.dmac1_itrig_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x484 - DMAC1 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub fn dmac1_itrig_sel_iter(&self) -> impl Iterator<Item = &Dmac1ItrigSel> {
        self.dmac1_itrig_sel.iter()
    }
    #[doc = "0x500..0x510 - DMAC1 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac1_otrig_sel(&self, n: usize) -> &Dmac1OtrigSel {
        &self.dmac1_otrig_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x510 - DMAC1 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub fn dmac1_otrig_sel_iter(&self) -> impl Iterator<Item = &Dmac1OtrigSel> {
        self.dmac1_otrig_sel.iter()
    }
    #[doc = "0x600..0x650 - CT32BITn Counter Timer Capture Trigger Multiplexers"]
    #[inline(always)]
    pub const fn ct32bit_cap(&self, n: usize) -> &Ct32bitCap {
        &self.ct32bit_cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x650 - CT32BITn Counter Timer Capture Trigger Multiplexers"]
    #[inline(always)]
    pub fn ct32bit_cap_iter(&self) -> impl Iterator<Item = &Ct32bitCap> {
        self.ct32bit_cap.iter()
    }
    #[doc = "0x700..0x708 - Frequency Measurement Input Channel Multiplexers"]
    #[inline(always)]
    pub const fn fmeasure_ch_sel(&self, n: usize) -> &FmeasureChSel {
        &self.fmeasure_ch_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700..0x708 - Frequency Measurement Input Channel Multiplexers"]
    #[inline(always)]
    pub fn fmeasure_ch_sel_iter(&self) -> impl Iterator<Item = &FmeasureChSel> {
        self.fmeasure_ch_sel.iter()
    }
    #[doc = "0x740 - DMAC0 request enable 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0(&self) -> &Dmac0ReqEna0 {
        &self.dmac0_req_ena0
    }
    #[doc = "0x744 - DMAC0 request enable 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1(&self) -> &Dmac0ReqEna1 {
        &self.dmac0_req_ena1
    }
    #[doc = "0x748 - DMAC0 request enable set 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0_set(&self) -> &Dmac0ReqEna0Set {
        &self.dmac0_req_ena0_set
    }
    #[doc = "0x74c - DMAC0 request enable set 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1_set(&self) -> &Dmac0ReqEna1Set {
        &self.dmac0_req_ena1_set
    }
    #[doc = "0x750 - DMAC0 request enable clear 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0_clr(&self) -> &Dmac0ReqEna0Clr {
        &self.dmac0_req_ena0_clr
    }
    #[doc = "0x754 - DMAC0 request enable clear 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1_clr(&self) -> &Dmac0ReqEna1Clr {
        &self.dmac0_req_ena1_clr
    }
    #[doc = "0x760 - DMAC1 request enable 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0(&self) -> &Dmac1ReqEna0 {
        &self.dmac1_req_ena0
    }
    #[doc = "0x764 - DMAC1 request enable 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1(&self) -> &Dmac1ReqEna1 {
        &self.dmac1_req_ena1
    }
    #[doc = "0x768 - DMAC1 request enable set 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0_set(&self) -> &Dmac1ReqEna0Set {
        &self.dmac1_req_ena0_set
    }
    #[doc = "0x76c - DMAC1 request enable set 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1_set(&self) -> &Dmac1ReqEna1Set {
        &self.dmac1_req_ena1_set
    }
    #[doc = "0x770 - DMAC1 request enable clear 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0_clr(&self) -> &Dmac1ReqEna0Clr {
        &self.dmac1_req_ena0_clr
    }
    #[doc = "0x774 - DMAC1 request enable clear 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1_clr(&self) -> &Dmac1ReqEna1Clr {
        &self.dmac1_req_ena1_clr
    }
    #[doc = "0x780 - DMAC0 input trigger enable 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0(&self) -> &Dmac0ItrigEna0 {
        &self.dmac0_itrig_ena0
    }
    #[doc = "0x788 - DMAC0 input trigger enable set 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0_set(&self) -> &Dmac0ItrigEna0Set {
        &self.dmac0_itrig_ena0_set
    }
    #[doc = "0x790 - DMAC0 input trigger enable clear 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0_clr(&self) -> &Dmac0ItrigEna0Clr {
        &self.dmac0_itrig_ena0_clr
    }
    #[doc = "0x7a0 - DMAC1 input trigger enable 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0(&self) -> &Dmac1ItrigEna0 {
        &self.dmac1_itrig_ena0
    }
    #[doc = "0x7a8 - DMAC1 input trigger enable set 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0_set(&self) -> &Dmac1ItrigEna0Set {
        &self.dmac1_itrig_ena0_set
    }
    #[doc = "0x7b0 - DMAC1 input trigger enable clear 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0_clr(&self) -> &Dmac1ItrigEna0Clr {
        &self.dmac1_itrig_ena0_clr
    }
}
#[doc = "SCT0_IN_SEL (rw) register accessor: SCT Peripheral Input Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`sct0_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sct0_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sct0_in_sel`]
module"]
#[doc(alias = "SCT0_IN_SEL")]
pub type Sct0InSel = crate::Reg<sct0_in_sel::Sct0InSelSpec>;
#[doc = "SCT Peripheral Input Multiplexers N"]
pub mod sct0_in_sel;
#[doc = "PINT_SEL (rw) register accessor: GPIO Pin Input Multiplexer N\n\nYou can [`read`](crate::Reg::read) this register and get [`pint_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pint_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pint_sel`]
module"]
#[doc(alias = "PINT_SEL")]
pub type PintSel = crate::Reg<pint_sel::PintSelSpec>;
#[doc = "GPIO Pin Input Multiplexer N"]
pub mod pint_sel;
#[doc = "DSP_INT_SEL (rw) register accessor: DSP Interrupt Input Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_int_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_int_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_int_sel`]
module"]
#[doc(alias = "DSP_INT_SEL")]
pub type DspIntSel = crate::Reg<dsp_int_sel::DspIntSelSpec>;
#[doc = "DSP Interrupt Input Multiplexers N"]
pub mod dsp_int_sel;
#[doc = "DMAC0_ITRIG_SEL (rw) register accessor: DMAC0 Input Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_itrig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_itrig_sel`]
module"]
#[doc(alias = "DMAC0_ITRIG_SEL")]
pub type Dmac0ItrigSel = crate::Reg<dmac0_itrig_sel::Dmac0ItrigSelSpec>;
#[doc = "DMAC0 Input Trigger Multiplexers N"]
pub mod dmac0_itrig_sel;
#[doc = "DMAC0_OTRIG_SEL (rw) register accessor: DMAC0 Output Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_otrig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_otrig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_otrig_sel`]
module"]
#[doc(alias = "DMAC0_OTRIG_SEL")]
pub type Dmac0OtrigSel = crate::Reg<dmac0_otrig_sel::Dmac0OtrigSelSpec>;
#[doc = "DMAC0 Output Trigger Multiplexers N"]
pub mod dmac0_otrig_sel;
#[doc = "DMAC1_ITRIG_SEL (rw) register accessor: DMAC1 Input Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_itrig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_itrig_sel`]
module"]
#[doc(alias = "DMAC1_ITRIG_SEL")]
pub type Dmac1ItrigSel = crate::Reg<dmac1_itrig_sel::Dmac1ItrigSelSpec>;
#[doc = "DMAC1 Input Trigger Multiplexers N"]
pub mod dmac1_itrig_sel;
#[doc = "DMAC1_OTRIG_SEL (rw) register accessor: DMAC1 Output Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_otrig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_otrig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_otrig_sel`]
module"]
#[doc(alias = "DMAC1_OTRIG_SEL")]
pub type Dmac1OtrigSel = crate::Reg<dmac1_otrig_sel::Dmac1OtrigSelSpec>;
#[doc = "DMAC1 Output Trigger Multiplexers N"]
pub mod dmac1_otrig_sel;
#[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
pub use self::ct32bit_cap::Ct32bitCap;
#[doc = r"Cluster"]
#[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
pub mod ct32bit_cap;
#[doc = "FMEASURE_CH_SEL (rw) register accessor: Frequency Measurement Input Channel Multiplexers\n\nYou can [`read`](crate::Reg::read) this register and get [`fmeasure_ch_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmeasure_ch_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmeasure_ch_sel`]
module"]
#[doc(alias = "FMEASURE_CH_SEL")]
pub type FmeasureChSel = crate::Reg<fmeasure_ch_sel::FmeasureChSelSpec>;
#[doc = "Frequency Measurement Input Channel Multiplexers"]
pub mod fmeasure_ch_sel;
#[doc = "DMAC0_REQ_ENA0 (rw) register accessor: DMAC0 request enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_req_ena0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena0`]
module"]
#[doc(alias = "DMAC0_REQ_ENA0")]
pub type Dmac0ReqEna0 = crate::Reg<dmac0_req_ena0::Dmac0ReqEna0Spec>;
#[doc = "DMAC0 request enable 0"]
pub mod dmac0_req_ena0;
#[doc = "DMAC0_REQ_ENA1 (rw) register accessor: DMAC0 request enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_req_ena1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena1`]
module"]
#[doc(alias = "DMAC0_REQ_ENA1")]
pub type Dmac0ReqEna1 = crate::Reg<dmac0_req_ena1::Dmac0ReqEna1Spec>;
#[doc = "DMAC0 request enable 1"]
pub mod dmac0_req_ena1;
#[doc = "DMAC0_REQ_ENA0_SET (w) register accessor: DMAC0 request enable set 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena0_set`]
module"]
#[doc(alias = "DMAC0_REQ_ENA0_SET")]
pub type Dmac0ReqEna0Set = crate::Reg<dmac0_req_ena0_set::Dmac0ReqEna0SetSpec>;
#[doc = "DMAC0 request enable set 0"]
pub mod dmac0_req_ena0_set;
#[doc = "DMAC0_REQ_ENA1_SET (w) register accessor: DMAC0 request enable set 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena1_set`]
module"]
#[doc(alias = "DMAC0_REQ_ENA1_SET")]
pub type Dmac0ReqEna1Set = crate::Reg<dmac0_req_ena1_set::Dmac0ReqEna1SetSpec>;
#[doc = "DMAC0 request enable set 1"]
pub mod dmac0_req_ena1_set;
#[doc = "DMAC0_REQ_ENA0_CLR (w) register accessor: DMAC0 request enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena0_clr`]
module"]
#[doc(alias = "DMAC0_REQ_ENA0_CLR")]
pub type Dmac0ReqEna0Clr = crate::Reg<dmac0_req_ena0_clr::Dmac0ReqEna0ClrSpec>;
#[doc = "DMAC0 request enable clear 0"]
pub mod dmac0_req_ena0_clr;
#[doc = "DMAC0_REQ_ENA1_CLR (w) register accessor: DMAC0 request enable clear 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_req_ena1_clr`]
module"]
#[doc(alias = "DMAC0_REQ_ENA1_CLR")]
pub type Dmac0ReqEna1Clr = crate::Reg<dmac0_req_ena1_clr::Dmac0ReqEna1ClrSpec>;
#[doc = "DMAC0 request enable clear 1"]
pub mod dmac0_req_ena1_clr;
#[doc = "DMAC1_REQ_ENA0 (rw) register accessor: DMAC1 request enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_req_ena0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena0`]
module"]
#[doc(alias = "DMAC1_REQ_ENA0")]
pub type Dmac1ReqEna0 = crate::Reg<dmac1_req_ena0::Dmac1ReqEna0Spec>;
#[doc = "DMAC1 request enable 0"]
pub mod dmac1_req_ena0;
#[doc = "DMAC1_REQ_ENA1 (rw) register accessor: DMAC1 request enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_req_ena1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena1`]
module"]
#[doc(alias = "DMAC1_REQ_ENA1")]
pub type Dmac1ReqEna1 = crate::Reg<dmac1_req_ena1::Dmac1ReqEna1Spec>;
#[doc = "DMAC1 request enable 1"]
pub mod dmac1_req_ena1;
#[doc = "DMAC1_REQ_ENA0_SET (w) register accessor: DMAC1 request enable set 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena0_set`]
module"]
#[doc(alias = "DMAC1_REQ_ENA0_SET")]
pub type Dmac1ReqEna0Set = crate::Reg<dmac1_req_ena0_set::Dmac1ReqEna0SetSpec>;
#[doc = "DMAC1 request enable set 0"]
pub mod dmac1_req_ena0_set;
#[doc = "DMAC1_REQ_ENA1_SET (w) register accessor: DMAC1 request enable set 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena1_set`]
module"]
#[doc(alias = "DMAC1_REQ_ENA1_SET")]
pub type Dmac1ReqEna1Set = crate::Reg<dmac1_req_ena1_set::Dmac1ReqEna1SetSpec>;
#[doc = "DMAC1 request enable set 1"]
pub mod dmac1_req_ena1_set;
#[doc = "DMAC1_REQ_ENA0_CLR (w) register accessor: DMAC1 request enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena0_clr`]
module"]
#[doc(alias = "DMAC1_REQ_ENA0_CLR")]
pub type Dmac1ReqEna0Clr = crate::Reg<dmac1_req_ena0_clr::Dmac1ReqEna0ClrSpec>;
#[doc = "DMAC1 request enable clear 0"]
pub mod dmac1_req_ena0_clr;
#[doc = "DMAC1_REQ_ENA1_CLR (w) register accessor: DMAC1 request enable clear 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_req_ena1_clr`]
module"]
#[doc(alias = "DMAC1_REQ_ENA1_CLR")]
pub type Dmac1ReqEna1Clr = crate::Reg<dmac1_req_ena1_clr::Dmac1ReqEna1ClrSpec>;
#[doc = "DMAC1 request enable clear 1"]
pub mod dmac1_req_ena1_clr;
#[doc = "DMAC0_ITRIG_ENA0 (rw) register accessor: DMAC0 input trigger enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_itrig_ena0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_ena0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_itrig_ena0`]
module"]
#[doc(alias = "DMAC0_ITRIG_ENA0")]
pub type Dmac0ItrigEna0 = crate::Reg<dmac0_itrig_ena0::Dmac0ItrigEna0Spec>;
#[doc = "DMAC0 input trigger enable 0"]
pub mod dmac0_itrig_ena0;
#[doc = "DMAC0_ITRIG_ENA0_SET (w) register accessor: DMAC0 input trigger enable set 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_ena0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_itrig_ena0_set`]
module"]
#[doc(alias = "DMAC0_ITRIG_ENA0_SET")]
pub type Dmac0ItrigEna0Set = crate::Reg<dmac0_itrig_ena0_set::Dmac0ItrigEna0SetSpec>;
#[doc = "DMAC0 input trigger enable set 0"]
pub mod dmac0_itrig_ena0_set;
#[doc = "DMAC0_ITRIG_ENA0_CLR (w) register accessor: DMAC0 input trigger enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_ena0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac0_itrig_ena0_clr`]
module"]
#[doc(alias = "DMAC0_ITRIG_ENA0_CLR")]
pub type Dmac0ItrigEna0Clr = crate::Reg<dmac0_itrig_ena0_clr::Dmac0ItrigEna0ClrSpec>;
#[doc = "DMAC0 input trigger enable clear 0"]
pub mod dmac0_itrig_ena0_clr;
#[doc = "DMAC1_ITRIG_ENA0 (rw) register accessor: DMAC1 input trigger enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_itrig_ena0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_ena0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_itrig_ena0`]
module"]
#[doc(alias = "DMAC1_ITRIG_ENA0")]
pub type Dmac1ItrigEna0 = crate::Reg<dmac1_itrig_ena0::Dmac1ItrigEna0Spec>;
#[doc = "DMAC1 input trigger enable 0"]
pub mod dmac1_itrig_ena0;
#[doc = "DMAC1_ITRIG_ENA0_SET (w) register accessor: DMAC1 input trigger enable set 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_ena0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_itrig_ena0_set`]
module"]
#[doc(alias = "DMAC1_ITRIG_ENA0_SET")]
pub type Dmac1ItrigEna0Set = crate::Reg<dmac1_itrig_ena0_set::Dmac1ItrigEna0SetSpec>;
#[doc = "DMAC1 input trigger enable set 0"]
pub mod dmac1_itrig_ena0_set;
#[doc = "DMAC1_ITRIG_ENA0_CLR (w) register accessor: DMAC1 input trigger enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_ena0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac1_itrig_ena0_clr`]
module"]
#[doc(alias = "DMAC1_ITRIG_ENA0_CLR")]
pub type Dmac1ItrigEna0Clr = crate::Reg<dmac1_itrig_ena0_clr::Dmac1ItrigEna0ClrSpec>;
#[doc = "DMAC1 input trigger enable clear 0"]
pub mod dmac1_itrig_ena0_clr;
