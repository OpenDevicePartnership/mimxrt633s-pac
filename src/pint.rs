#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    isel: Isel,
    ienr: Ienr,
    sienr: Sienr,
    cienr: Cienr,
    ienf: Ienf,
    sienf: Sienf,
    cienf: Cienf,
    rise: Rise,
    fall: Fall,
    ist: Ist,
    pmctrl: Pmctrl,
    pmsrc: Pmsrc,
    pmcfg: Pmcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    #[inline(always)]
    pub const fn isel(&self) -> &Isel {
        &self.isel
    }
    #[doc = "0x04 - Pin interrupt level or rising edge interrupt enable register"]
    #[inline(always)]
    pub const fn ienr(&self) -> &Ienr {
        &self.ienr
    }
    #[doc = "0x08 - Pin interrupt level or rising edge interrupt set register"]
    #[inline(always)]
    pub const fn sienr(&self) -> &Sienr {
        &self.sienr
    }
    #[doc = "0x0c - Pin interrupt level (rising edge interrupt) clear register"]
    #[inline(always)]
    pub const fn cienr(&self) -> &Cienr {
        &self.cienr
    }
    #[doc = "0x10 - Pin interrupt active level or falling edge interrupt enable register"]
    #[inline(always)]
    pub const fn ienf(&self) -> &Ienf {
        &self.ienf
    }
    #[doc = "0x14 - Pin interrupt active level or falling edge interrupt set register"]
    #[inline(always)]
    pub const fn sienf(&self) -> &Sienf {
        &self.sienf
    }
    #[doc = "0x18 - Pin interrupt active level or falling edge interrupt clear register"]
    #[inline(always)]
    pub const fn cienf(&self) -> &Cienf {
        &self.cienf
    }
    #[doc = "0x1c - Pin interrupt rising edge register"]
    #[inline(always)]
    pub const fn rise(&self) -> &Rise {
        &self.rise
    }
    #[doc = "0x20 - Pin interrupt falling edge register"]
    #[inline(always)]
    pub const fn fall(&self) -> &Fall {
        &self.fall
    }
    #[doc = "0x24 - Pin interrupt status register"]
    #[inline(always)]
    pub const fn ist(&self) -> &Ist {
        &self.ist
    }
    #[doc = "0x28 - Pattern match interrupt control register"]
    #[inline(always)]
    pub const fn pmctrl(&self) -> &Pmctrl {
        &self.pmctrl
    }
    #[doc = "0x2c - Pattern match interrupt bit-slice source register"]
    #[inline(always)]
    pub const fn pmsrc(&self) -> &Pmsrc {
        &self.pmsrc
    }
    #[doc = "0x30 - Pattern match interrupt bit slice configuration register"]
    #[inline(always)]
    pub const fn pmcfg(&self) -> &Pmcfg {
        &self.pmcfg
    }
}
#[doc = "ISEL (rw) register accessor: Pin Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`isel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isel`]
module"]
#[doc(alias = "ISEL")]
pub type Isel = crate::Reg<isel::IselSpec>;
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "IENR (rw) register accessor: Pin interrupt level or rising edge interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienr`]
module"]
#[doc(alias = "IENR")]
pub type Ienr = crate::Reg<ienr::IenrSpec>;
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
pub mod ienr;
#[doc = "SIENR (w) register accessor: Pin interrupt level or rising edge interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sienr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sienr`]
module"]
#[doc(alias = "SIENR")]
pub type Sienr = crate::Reg<sienr::SienrSpec>;
#[doc = "Pin interrupt level or rising edge interrupt set register"]
pub mod sienr;
#[doc = "CIENR (w) register accessor: Pin interrupt level (rising edge interrupt) clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cienr`]
module"]
#[doc(alias = "CIENR")]
pub type Cienr = crate::Reg<cienr::CienrSpec>;
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
pub mod cienr;
#[doc = "IENF (rw) register accessor: Pin interrupt active level or falling edge interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienf`]
module"]
#[doc(alias = "IENF")]
pub type Ienf = crate::Reg<ienf::IenfSpec>;
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
pub mod ienf;
#[doc = "SIENF (w) register accessor: Pin interrupt active level or falling edge interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sienf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sienf`]
module"]
#[doc(alias = "SIENF")]
pub type Sienf = crate::Reg<sienf::SienfSpec>;
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
pub mod sienf;
#[doc = "CIENF (w) register accessor: Pin interrupt active level or falling edge interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cienf`]
module"]
#[doc(alias = "CIENF")]
pub type Cienf = crate::Reg<cienf::CienfSpec>;
#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
pub mod cienf;
#[doc = "RISE (rw) register accessor: Pin interrupt rising edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rise`]
module"]
#[doc(alias = "RISE")]
pub type Rise = crate::Reg<rise::RiseSpec>;
#[doc = "Pin interrupt rising edge register"]
pub mod rise;
#[doc = "FALL (rw) register accessor: Pin interrupt falling edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fall`]
module"]
#[doc(alias = "FALL")]
pub type Fall = crate::Reg<fall::FallSpec>;
#[doc = "Pin interrupt falling edge register"]
pub mod fall;
#[doc = "IST (rw) register accessor: Pin interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ist::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ist::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ist`]
module"]
#[doc(alias = "IST")]
pub type Ist = crate::Reg<ist::IstSpec>;
#[doc = "Pin interrupt status register"]
pub mod ist;
#[doc = "PMCTRL (rw) register accessor: Pattern match interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmctrl`]
module"]
#[doc(alias = "PMCTRL")]
pub type Pmctrl = crate::Reg<pmctrl::PmctrlSpec>;
#[doc = "Pattern match interrupt control register"]
pub mod pmctrl;
#[doc = "PMSRC (rw) register accessor: Pattern match interrupt bit-slice source register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmsrc`]
module"]
#[doc(alias = "PMSRC")]
pub type Pmsrc = crate::Reg<pmsrc::PmsrcSpec>;
#[doc = "Pattern match interrupt bit-slice source register"]
pub mod pmsrc;
#[doc = "PMCFG (rw) register accessor: Pattern match interrupt bit slice configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcfg`]
module"]
#[doc(alias = "PMCFG")]
pub type Pmcfg = crate::Reg<pmcfg::PmcfgSpec>;
#[doc = "Pattern match interrupt bit slice configuration register"]
pub mod pmcfg;
