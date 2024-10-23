#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    ccr: Ccr,
    clcr: Clcr,
    csar: Csar,
    ccvr: Ccvr,
}
impl RegisterBlock {
    #[doc = "0x800 - Cache control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x804 - Cache line control register"]
    #[inline(always)]
    pub const fn clcr(&self) -> &Clcr {
        &self.clcr
    }
    #[doc = "0x808 - Cache search address register"]
    #[inline(always)]
    pub const fn csar(&self) -> &Csar {
        &self.csar
    }
    #[doc = "0x80c - Cache read/write value register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &Ccvr {
        &self.ccvr
    }
}
#[doc = "CCR (rw) register accessor: Cache control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Cache control register"]
pub mod ccr;
#[doc = "CLCR (rw) register accessor: Cache line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clcr`]
module"]
#[doc(alias = "CLCR")]
pub type Clcr = crate::Reg<clcr::ClcrSpec>;
#[doc = "Cache line control register"]
pub mod clcr;
#[doc = "CSAR (rw) register accessor: Cache search address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csar`]
module"]
#[doc(alias = "CSAR")]
pub type Csar = crate::Reg<csar::CsarSpec>;
#[doc = "Cache search address register"]
pub mod csar;
#[doc = "CCVR (rw) register accessor: Cache read/write value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`]
module"]
#[doc(alias = "CCVR")]
pub type Ccvr = crate::Reg<ccvr::CcvrSpec>;
#[doc = "Cache read/write value register"]
pub mod ccvr;
