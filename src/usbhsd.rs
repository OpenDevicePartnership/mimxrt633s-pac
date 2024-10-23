#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    devcmdstat: Devcmdstat,
    info: Info,
    epliststart: Epliststart,
    databufstart: Databufstart,
    lpm: Lpm,
    epskip: Epskip,
    epinuse: Epinuse,
    epbufcfg: Epbufcfg,
    intstat: Intstat,
    inten: Inten,
    intsetstat: Intsetstat,
    _reserved11: [u8; 0x08],
    eptoggle: Eptoggle,
}
impl RegisterBlock {
    #[doc = "0x00 - USB Device Command/Status register"]
    #[inline(always)]
    pub const fn devcmdstat(&self) -> &Devcmdstat {
        &self.devcmdstat
    }
    #[doc = "0x04 - USB Info register"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x08 - USB EP Command/Status List start address"]
    #[inline(always)]
    pub const fn epliststart(&self) -> &Epliststart {
        &self.epliststart
    }
    #[doc = "0x0c - USB Data buffer start address"]
    #[inline(always)]
    pub const fn databufstart(&self) -> &Databufstart {
        &self.databufstart
    }
    #[doc = "0x10 - USB Link Power Management register"]
    #[inline(always)]
    pub const fn lpm(&self) -> &Lpm {
        &self.lpm
    }
    #[doc = "0x14 - USB Endpoint skip"]
    #[inline(always)]
    pub const fn epskip(&self) -> &Epskip {
        &self.epskip
    }
    #[doc = "0x18 - USB Endpoint Buffer in use"]
    #[inline(always)]
    pub const fn epinuse(&self) -> &Epinuse {
        &self.epinuse
    }
    #[doc = "0x1c - USB Endpoint Buffer Configuration register"]
    #[inline(always)]
    pub const fn epbufcfg(&self) -> &Epbufcfg {
        &self.epbufcfg
    }
    #[doc = "0x20 - USB interrupt status register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x24 - USB interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x28 - USB set interrupt status register"]
    #[inline(always)]
    pub const fn intsetstat(&self) -> &Intsetstat {
        &self.intsetstat
    }
    #[doc = "0x34 - USB Endpoint toggle register"]
    #[inline(always)]
    pub const fn eptoggle(&self) -> &Eptoggle {
        &self.eptoggle
    }
}
#[doc = "DEVCMDSTAT (rw) register accessor: USB Device Command/Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`devcmdstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcmdstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devcmdstat`]
module"]
#[doc(alias = "DEVCMDSTAT")]
pub type Devcmdstat = crate::Reg<devcmdstat::DevcmdstatSpec>;
#[doc = "USB Device Command/Status register"]
pub mod devcmdstat;
#[doc = "INFO (r) register accessor: USB Info register\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "USB Info register"]
pub mod info;
#[doc = "EPLISTSTART (rw) register accessor: USB EP Command/Status List start address\n\nYou can [`read`](crate::Reg::read) this register and get [`epliststart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epliststart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epliststart`]
module"]
#[doc(alias = "EPLISTSTART")]
pub type Epliststart = crate::Reg<epliststart::EpliststartSpec>;
#[doc = "USB EP Command/Status List start address"]
pub mod epliststart;
#[doc = "DATABUFSTART (rw) register accessor: USB Data buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`databufstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databufstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databufstart`]
module"]
#[doc(alias = "DATABUFSTART")]
pub type Databufstart = crate::Reg<databufstart::DatabufstartSpec>;
#[doc = "USB Data buffer start address"]
pub mod databufstart;
#[doc = "LPM (rw) register accessor: USB Link Power Management register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm`]
module"]
#[doc(alias = "LPM")]
pub type Lpm = crate::Reg<lpm::LpmSpec>;
#[doc = "USB Link Power Management register"]
pub mod lpm;
#[doc = "EPSKIP (rw) register accessor: USB Endpoint skip\n\nYou can [`read`](crate::Reg::read) this register and get [`epskip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epskip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epskip`]
module"]
#[doc(alias = "EPSKIP")]
pub type Epskip = crate::Reg<epskip::EpskipSpec>;
#[doc = "USB Endpoint skip"]
pub mod epskip;
#[doc = "EPINUSE (rw) register accessor: USB Endpoint Buffer in use\n\nYou can [`read`](crate::Reg::read) this register and get [`epinuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinuse`]
module"]
#[doc(alias = "EPINUSE")]
pub type Epinuse = crate::Reg<epinuse::EpinuseSpec>;
#[doc = "USB Endpoint Buffer in use"]
pub mod epinuse;
#[doc = "EPBUFCFG (rw) register accessor: USB Endpoint Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`epbufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epbufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epbufcfg`]
module"]
#[doc(alias = "EPBUFCFG")]
pub type Epbufcfg = crate::Reg<epbufcfg::EpbufcfgSpec>;
#[doc = "USB Endpoint Buffer Configuration register"]
pub mod epbufcfg;
#[doc = "INTSTAT (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "USB interrupt status register"]
pub mod intstat;
#[doc = "INTEN (rw) register accessor: USB interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "USB interrupt enable register"]
pub mod inten;
#[doc = "INTSETSTAT (rw) register accessor: USB set interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsetstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsetstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsetstat`]
module"]
#[doc(alias = "INTSETSTAT")]
pub type Intsetstat = crate::Reg<intsetstat::IntsetstatSpec>;
#[doc = "USB set interrupt status register"]
pub mod intsetstat;
#[doc = "EPTOGGLE (r) register accessor: USB Endpoint toggle register\n\nYou can [`read`](crate::Reg::read) this register and get [`eptoggle::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptoggle`]
module"]
#[doc(alias = "EPTOGGLE")]
pub type Eptoggle = crate::Reg<eptoggle::EptoggleSpec>;
#[doc = "USB Endpoint toggle register"]
pub mod eptoggle;
