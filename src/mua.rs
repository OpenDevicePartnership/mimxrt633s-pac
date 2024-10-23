#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver: Ver,
    par: Par,
    _reserved2: [u8; 0x18],
    tr: [Tr; 4],
    _reserved3: [u8; 0x10],
    rr: [Rr; 4],
    _reserved4: [u8; 0x10],
    sr: Sr,
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    #[inline(always)]
    pub const fn ver(&self) -> &Ver {
        &self.ver
    }
    #[doc = "0x04 - Use Parameter register to determine the parameter settings of MUA."]
    #[inline(always)]
    pub const fn par(&self) -> &Par {
        &self.par
    }
    #[doc = "0x20..0x30 - Transmit Register"]
    #[inline(always)]
    pub const fn tr(&self, n: usize) -> &Tr {
        &self.tr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Transmit Register"]
    #[inline(always)]
    pub fn tr_iter(&self) -> impl Iterator<Item = &Tr> {
        self.tr.iter()
    }
    #[doc = "0x40..0x50 - Receive Register."]
    #[inline(always)]
    pub const fn rr(&self, n: usize) -> &Rr {
        &self.rr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Receive Register."]
    #[inline(always)]
    pub fn rr_iter(&self) -> impl Iterator<Item = &Rr> {
        self.rr.iter()
    }
    #[doc = "0x60 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x64 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "VER (r) register accessor: Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver`]
module"]
#[doc(alias = "VER")]
pub type Ver = crate::Reg<ver::VerSpec>;
#[doc = "Version ID Register"]
pub mod ver;
#[doc = "PAR (r) register accessor: Use Parameter register to determine the parameter settings of MUA.\n\nYou can [`read`](crate::Reg::read) this register and get [`par::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`]
module"]
#[doc(alias = "PAR")]
pub type Par = crate::Reg<par::ParSpec>;
#[doc = "Use Parameter register to determine the parameter settings of MUA."]
pub mod par;
#[doc = "TR (rw) register accessor: Transmit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
#[doc(alias = "TR")]
pub type Tr = crate::Reg<tr::TrSpec>;
#[doc = "Transmit Register"]
pub mod tr;
#[doc = "RR (rw) register accessor: Receive Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`]
module"]
#[doc(alias = "RR")]
pub type Rr = crate::Reg<rr::RrSpec>;
#[doc = "Receive Register."]
pub mod rr;
#[doc = "SR (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
