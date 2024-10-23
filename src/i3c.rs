#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mconfig: Mconfig,
    sconfig: Sconfig,
    sstatus: Sstatus,
    sctrl: Sctrl,
    sintset: Sintset,
    sintclr: Sintclr,
    sintmasked: Sintmasked,
    serrwarn: Serrwarn,
    sdmactrl: Sdmactrl,
    _reserved9: [u8; 0x08],
    sdatactrl: Sdatactrl,
    swdatab: Swdatab,
    swdatabe: Swdatabe,
    swdatah: Swdatah,
    swdatahe: Swdatahe,
    srdatab: Srdatab,
    _reserved15: [u8; 0x04],
    srdatah: Srdatah,
    _reserved16: [u8; 0x14],
    scapabilities: Scapabilities,
    sdynaddr: Sdynaddr,
    smaxlimits: Smaxlimits,
    sidpartno: Sidpartno,
    sidext: Sidext,
    svendorid: Svendorid,
    stcclock: Stcclock,
    smsgmapaddr: Smsgmapaddr,
    _reserved24: [u8; 0x04],
    mctrl: Mctrl,
    mstatus: Mstatus,
    mibirules: Mibirules,
    mintset: Mintset,
    mintclr: Mintclr,
    mintmasked: Mintmasked,
    merrwarn: Merrwarn,
    mdmactrl: Mdmactrl,
    _reserved32: [u8; 0x08],
    mdatactrl: Mdatactrl,
    mwdatab: Mwdatab,
    mwdatabe: Mwdatabe,
    mwdatah: Mwdatah,
    mwdatahe: Mwdatahe,
    mrdatab: Mrdatab,
    _reserved38: [u8; 0x04],
    mrdatah: Mrdatah,
    _reserved39: [u8; 0x04],
    _reserved_39_mwmsg_sdr: [u8; 0x04],
    mrmsg_sdr: MrmsgSdr,
    _reserved_41_mwmsg_ddr: [u8; 0x04],
    mrmsg_ddr: MrmsgDdr,
    _reserved43: [u8; 0x04],
    mdynaddr: Mdynaddr,
    _reserved44: [u8; 0x0f14],
    sid: Sid,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    #[inline(always)]
    pub const fn mconfig(&self) -> &Mconfig {
        &self.mconfig
    }
    #[doc = "0x04 - Slave Configuration Register"]
    #[inline(always)]
    pub const fn sconfig(&self) -> &Sconfig {
        &self.sconfig
    }
    #[doc = "0x08 - Slave Status Register"]
    #[inline(always)]
    pub const fn sstatus(&self) -> &Sstatus {
        &self.sstatus
    }
    #[doc = "0x0c - Slave Control Register"]
    #[inline(always)]
    pub const fn sctrl(&self) -> &Sctrl {
        &self.sctrl
    }
    #[doc = "0x10 - Slave Interrupt Set Register"]
    #[inline(always)]
    pub const fn sintset(&self) -> &Sintset {
        &self.sintset
    }
    #[doc = "0x14 - Slave Interrupt Clear Register"]
    #[inline(always)]
    pub const fn sintclr(&self) -> &Sintclr {
        &self.sintclr
    }
    #[doc = "0x18 - Slave Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sintmasked(&self) -> &Sintmasked {
        &self.sintmasked
    }
    #[doc = "0x1c - Slave Errors and Warnings Register"]
    #[inline(always)]
    pub const fn serrwarn(&self) -> &Serrwarn {
        &self.serrwarn
    }
    #[doc = "0x20 - Slave DMA Control Register"]
    #[inline(always)]
    pub const fn sdmactrl(&self) -> &Sdmactrl {
        &self.sdmactrl
    }
    #[doc = "0x2c - Slave Data Control Register"]
    #[inline(always)]
    pub const fn sdatactrl(&self) -> &Sdatactrl {
        &self.sdatactrl
    }
    #[doc = "0x30 - Slave Write Data Byte Register"]
    #[inline(always)]
    pub const fn swdatab(&self) -> &Swdatab {
        &self.swdatab
    }
    #[doc = "0x34 - Slave Write Data Byte End"]
    #[inline(always)]
    pub const fn swdatabe(&self) -> &Swdatabe {
        &self.swdatabe
    }
    #[doc = "0x38 - Slave Write Data Half-word Register"]
    #[inline(always)]
    pub const fn swdatah(&self) -> &Swdatah {
        &self.swdatah
    }
    #[doc = "0x3c - Slave Write Data Half-word End Register"]
    #[inline(always)]
    pub const fn swdatahe(&self) -> &Swdatahe {
        &self.swdatahe
    }
    #[doc = "0x40 - Slave Read Data Byte Register"]
    #[inline(always)]
    pub const fn srdatab(&self) -> &Srdatab {
        &self.srdatab
    }
    #[doc = "0x48 - Slave Read Data Half-word Register"]
    #[inline(always)]
    pub const fn srdatah(&self) -> &Srdatah {
        &self.srdatah
    }
    #[doc = "0x60 - Slave Capabilities Register"]
    #[inline(always)]
    pub const fn scapabilities(&self) -> &Scapabilities {
        &self.scapabilities
    }
    #[doc = "0x64 - Slave Dynamic Address Register"]
    #[inline(always)]
    pub const fn sdynaddr(&self) -> &Sdynaddr {
        &self.sdynaddr
    }
    #[doc = "0x68 - Slave Maximum Limits Register"]
    #[inline(always)]
    pub const fn smaxlimits(&self) -> &Smaxlimits {
        &self.smaxlimits
    }
    #[doc = "0x6c - Slave ID Part Number Register"]
    #[inline(always)]
    pub const fn sidpartno(&self) -> &Sidpartno {
        &self.sidpartno
    }
    #[doc = "0x70 - Slave ID Extension Register"]
    #[inline(always)]
    pub const fn sidext(&self) -> &Sidext {
        &self.sidext
    }
    #[doc = "0x74 - Slave Vendor ID Register"]
    #[inline(always)]
    pub const fn svendorid(&self) -> &Svendorid {
        &self.svendorid
    }
    #[doc = "0x78 - Slave Time Control Clock Register"]
    #[inline(always)]
    pub const fn stcclock(&self) -> &Stcclock {
        &self.stcclock
    }
    #[doc = "0x7c - Slave Message-Mapped Address Register"]
    #[inline(always)]
    pub const fn smsgmapaddr(&self) -> &Smsgmapaddr {
        &self.smsgmapaddr
    }
    #[doc = "0x84 - Master Main Control Register"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x88 - Master Status Register"]
    #[inline(always)]
    pub const fn mstatus(&self) -> &Mstatus {
        &self.mstatus
    }
    #[doc = "0x8c - Master In-band Interrupt Registry and Rules Register"]
    #[inline(always)]
    pub const fn mibirules(&self) -> &Mibirules {
        &self.mibirules
    }
    #[doc = "0x90 - Master Interrupt Set Register"]
    #[inline(always)]
    pub const fn mintset(&self) -> &Mintset {
        &self.mintset
    }
    #[doc = "0x94 - Master Interrupt Clear Register"]
    #[inline(always)]
    pub const fn mintclr(&self) -> &Mintclr {
        &self.mintclr
    }
    #[doc = "0x98 - Master Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mintmasked(&self) -> &Mintmasked {
        &self.mintmasked
    }
    #[doc = "0x9c - Master Errors and Warnings Register"]
    #[inline(always)]
    pub const fn merrwarn(&self) -> &Merrwarn {
        &self.merrwarn
    }
    #[doc = "0xa0 - Master DMA Control Register"]
    #[inline(always)]
    pub const fn mdmactrl(&self) -> &Mdmactrl {
        &self.mdmactrl
    }
    #[doc = "0xac - Master Data Control Register"]
    #[inline(always)]
    pub const fn mdatactrl(&self) -> &Mdatactrl {
        &self.mdatactrl
    }
    #[doc = "0xb0 - Master Write Data Byte Register"]
    #[inline(always)]
    pub const fn mwdatab(&self) -> &Mwdatab {
        &self.mwdatab
    }
    #[doc = "0xb4 - Master Write Data Byte End Register"]
    #[inline(always)]
    pub const fn mwdatabe(&self) -> &Mwdatabe {
        &self.mwdatabe
    }
    #[doc = "0xb8 - Master Write Data Half-word Register"]
    #[inline(always)]
    pub const fn mwdatah(&self) -> &Mwdatah {
        &self.mwdatah
    }
    #[doc = "0xbc - Master Write Data Byte End Register"]
    #[inline(always)]
    pub const fn mwdatahe(&self) -> &Mwdatahe {
        &self.mwdatahe
    }
    #[doc = "0xc0 - Master Read Data Byte Register"]
    #[inline(always)]
    pub const fn mrdatab(&self) -> &Mrdatab {
        &self.mrdatab
    }
    #[doc = "0xc8 - Master Read Data Half-word Register"]
    #[inline(always)]
    pub const fn mrdatah(&self) -> &Mrdatah {
        &self.mrdatah
    }
    #[doc = "0xd0 - Master Write Message Data in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_data(&self) -> &MwmsgSdrData {
        unsafe { &*(self as *const Self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - Master Write Message in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_control(&self) -> &MwmsgSdrControl {
        unsafe { &*(self as *const Self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd4 - Master Read Message in SDR mode"]
    #[inline(always)]
    pub const fn mrmsg_sdr(&self) -> &MrmsgSdr {
        &self.mrmsg_sdr
    }
    #[doc = "0xd8 - Master Write Message Data in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_data(&self) -> &MwmsgDdrData {
        unsafe { &*(self as *const Self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Master Write Message in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_control(&self) -> &MwmsgDdrControl {
        unsafe { &*(self as *const Self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xdc - Master Read Message in DDR mode"]
    #[inline(always)]
    pub const fn mrmsg_ddr(&self) -> &MrmsgDdr {
        &self.mrmsg_ddr
    }
    #[doc = "0xe4 - Master Dynamic Address Register"]
    #[inline(always)]
    pub const fn mdynaddr(&self) -> &Mdynaddr {
        &self.mdynaddr
    }
    #[doc = "0xffc - Slave Module ID Register"]
    #[inline(always)]
    pub const fn sid(&self) -> &Sid {
        &self.sid
    }
}
#[doc = "MCONFIG (rw) register accessor: Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mconfig`]
module"]
#[doc(alias = "MCONFIG")]
pub type Mconfig = crate::Reg<mconfig::MconfigSpec>;
#[doc = "Master Configuration Register"]
pub mod mconfig;
#[doc = "SCONFIG (rw) register accessor: Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sconfig`]
module"]
#[doc(alias = "SCONFIG")]
pub type Sconfig = crate::Reg<sconfig::SconfigSpec>;
#[doc = "Slave Configuration Register"]
pub mod sconfig;
#[doc = "SSTATUS (rw) register accessor: Slave Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatus`]
module"]
#[doc(alias = "SSTATUS")]
pub type Sstatus = crate::Reg<sstatus::SstatusSpec>;
#[doc = "Slave Status Register"]
pub mod sstatus;
#[doc = "SCTRL (rw) register accessor: Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctrl`]
module"]
#[doc(alias = "SCTRL")]
pub type Sctrl = crate::Reg<sctrl::SctrlSpec>;
#[doc = "Slave Control Register"]
pub mod sctrl;
#[doc = "SINTSET (rw) register accessor: Slave Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintset`]
module"]
#[doc(alias = "SINTSET")]
pub type Sintset = crate::Reg<sintset::SintsetSpec>;
#[doc = "Slave Interrupt Set Register"]
pub mod sintset;
#[doc = "SINTCLR (rw) register accessor: Slave Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintclr`]
module"]
#[doc(alias = "SINTCLR")]
pub type Sintclr = crate::Reg<sintclr::SintclrSpec>;
#[doc = "Slave Interrupt Clear Register"]
pub mod sintclr;
#[doc = "SINTMASKED (r) register accessor: Slave Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintmasked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sintmasked`]
module"]
#[doc(alias = "SINTMASKED")]
pub type Sintmasked = crate::Reg<sintmasked::SintmaskedSpec>;
#[doc = "Slave Interrupt Mask Register"]
pub mod sintmasked;
#[doc = "SERRWARN (rw) register accessor: Slave Errors and Warnings Register\n\nYou can [`read`](crate::Reg::read) this register and get [`serrwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serrwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serrwarn`]
module"]
#[doc(alias = "SERRWARN")]
pub type Serrwarn = crate::Reg<serrwarn::SerrwarnSpec>;
#[doc = "Slave Errors and Warnings Register"]
pub mod serrwarn;
#[doc = "SDMACTRL (rw) register accessor: Slave DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmactrl`]
module"]
#[doc(alias = "SDMACTRL")]
pub type Sdmactrl = crate::Reg<sdmactrl::SdmactrlSpec>;
#[doc = "Slave DMA Control Register"]
pub mod sdmactrl;
#[doc = "SDATACTRL (rw) register accessor: Slave Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdatactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdatactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdatactrl`]
module"]
#[doc(alias = "SDATACTRL")]
pub type Sdatactrl = crate::Reg<sdatactrl::SdatactrlSpec>;
#[doc = "Slave Data Control Register"]
pub mod sdatactrl;
#[doc = "SWDATAB (w) register accessor: Slave Write Data Byte Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatab::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdatab`]
module"]
#[doc(alias = "SWDATAB")]
pub type Swdatab = crate::Reg<swdatab::SwdatabSpec>;
#[doc = "Slave Write Data Byte Register"]
pub mod swdatab;
#[doc = "SWDATABE (w) register accessor: Slave Write Data Byte End\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatabe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdatabe`]
module"]
#[doc(alias = "SWDATABE")]
pub type Swdatabe = crate::Reg<swdatabe::SwdatabeSpec>;
#[doc = "Slave Write Data Byte End"]
pub mod swdatabe;
#[doc = "SWDATAH (w) register accessor: Slave Write Data Half-word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatah::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdatah`]
module"]
#[doc(alias = "SWDATAH")]
pub type Swdatah = crate::Reg<swdatah::SwdatahSpec>;
#[doc = "Slave Write Data Half-word Register"]
pub mod swdatah;
#[doc = "SWDATAHE (w) register accessor: Slave Write Data Half-word End Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatahe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdatahe`]
module"]
#[doc(alias = "SWDATAHE")]
pub type Swdatahe = crate::Reg<swdatahe::SwdataheSpec>;
#[doc = "Slave Write Data Half-word End Register"]
pub mod swdatahe;
#[doc = "SRDATAB (r) register accessor: Slave Read Data Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srdatab::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdatab`]
module"]
#[doc(alias = "SRDATAB")]
pub type Srdatab = crate::Reg<srdatab::SrdatabSpec>;
#[doc = "Slave Read Data Byte Register"]
pub mod srdatab;
#[doc = "SRDATAH (r) register accessor: Slave Read Data Half-word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srdatah::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdatah`]
module"]
#[doc(alias = "SRDATAH")]
pub type Srdatah = crate::Reg<srdatah::SrdatahSpec>;
#[doc = "Slave Read Data Half-word Register"]
pub mod srdatah;
#[doc = "SCAPABILITIES (r) register accessor: Slave Capabilities Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scapabilities::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scapabilities`]
module"]
#[doc(alias = "SCAPABILITIES")]
pub type Scapabilities = crate::Reg<scapabilities::ScapabilitiesSpec>;
#[doc = "Slave Capabilities Register"]
pub mod scapabilities;
#[doc = "SDYNADDR (rw) register accessor: Slave Dynamic Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdynaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdynaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdynaddr`]
module"]
#[doc(alias = "SDYNADDR")]
pub type Sdynaddr = crate::Reg<sdynaddr::SdynaddrSpec>;
#[doc = "Slave Dynamic Address Register"]
pub mod sdynaddr;
#[doc = "SMAXLIMITS (rw) register accessor: Slave Maximum Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smaxlimits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smaxlimits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smaxlimits`]
module"]
#[doc(alias = "SMAXLIMITS")]
pub type Smaxlimits = crate::Reg<smaxlimits::SmaxlimitsSpec>;
#[doc = "Slave Maximum Limits Register"]
pub mod smaxlimits;
#[doc = "SIDPARTNO (rw) register accessor: Slave ID Part Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidpartno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidpartno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidpartno`]
module"]
#[doc(alias = "SIDPARTNO")]
pub type Sidpartno = crate::Reg<sidpartno::SidpartnoSpec>;
#[doc = "Slave ID Part Number Register"]
pub mod sidpartno;
#[doc = "SIDEXT (rw) register accessor: Slave ID Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidext`]
module"]
#[doc(alias = "SIDEXT")]
pub type Sidext = crate::Reg<sidext::SidextSpec>;
#[doc = "Slave ID Extension Register"]
pub mod sidext;
#[doc = "SVENDORID (rw) register accessor: Slave Vendor ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`svendorid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svendorid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svendorid`]
module"]
#[doc(alias = "SVENDORID")]
pub type Svendorid = crate::Reg<svendorid::SvendoridSpec>;
#[doc = "Slave Vendor ID Register"]
pub mod svendorid;
#[doc = "STCCLOCK (rw) register accessor: Slave Time Control Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcclock`]
module"]
#[doc(alias = "STCCLOCK")]
pub type Stcclock = crate::Reg<stcclock::StcclockSpec>;
#[doc = "Slave Time Control Clock Register"]
pub mod stcclock;
#[doc = "SMSGMAPADDR (r) register accessor: Slave Message-Mapped Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smsgmapaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smsgmapaddr`]
module"]
#[doc(alias = "SMSGMAPADDR")]
pub type Smsgmapaddr = crate::Reg<smsgmapaddr::SmsgmapaddrSpec>;
#[doc = "Slave Message-Mapped Address Register"]
pub mod smsgmapaddr;
#[doc = "MCTRL (rw) register accessor: Master Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Master Main Control Register"]
pub mod mctrl;
#[doc = "MSTATUS (rw) register accessor: Master Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstatus`]
module"]
#[doc(alias = "MSTATUS")]
pub type Mstatus = crate::Reg<mstatus::MstatusSpec>;
#[doc = "Master Status Register"]
pub mod mstatus;
#[doc = "MIBIRULES (rw) register accessor: Master In-band Interrupt Registry and Rules Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mibirules::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mibirules::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mibirules`]
module"]
#[doc(alias = "MIBIRULES")]
pub type Mibirules = crate::Reg<mibirules::MibirulesSpec>;
#[doc = "Master In-band Interrupt Registry and Rules Register"]
pub mod mibirules;
#[doc = "MINTSET (rw) register accessor: Master Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintset`]
module"]
#[doc(alias = "MINTSET")]
pub type Mintset = crate::Reg<mintset::MintsetSpec>;
#[doc = "Master Interrupt Set Register"]
pub mod mintset;
#[doc = "MINTCLR (w) register accessor: Master Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintclr`]
module"]
#[doc(alias = "MINTCLR")]
pub type Mintclr = crate::Reg<mintclr::MintclrSpec>;
#[doc = "Master Interrupt Clear Register"]
pub mod mintclr;
#[doc = "MINTMASKED (r) register accessor: Master Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintmasked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintmasked`]
module"]
#[doc(alias = "MINTMASKED")]
pub type Mintmasked = crate::Reg<mintmasked::MintmaskedSpec>;
#[doc = "Master Interrupt Mask Register"]
pub mod mintmasked;
#[doc = "MERRWARN (rw) register accessor: Master Errors and Warnings Register\n\nYou can [`read`](crate::Reg::read) this register and get [`merrwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`merrwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@merrwarn`]
module"]
#[doc(alias = "MERRWARN")]
pub type Merrwarn = crate::Reg<merrwarn::MerrwarnSpec>;
#[doc = "Master Errors and Warnings Register"]
pub mod merrwarn;
#[doc = "MDMACTRL (rw) register accessor: Master DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmactrl`]
module"]
#[doc(alias = "MDMACTRL")]
pub type Mdmactrl = crate::Reg<mdmactrl::MdmactrlSpec>;
#[doc = "Master DMA Control Register"]
pub mod mdmactrl;
#[doc = "MDATACTRL (rw) register accessor: Master Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdatactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdatactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdatactrl`]
module"]
#[doc(alias = "MDATACTRL")]
pub type Mdatactrl = crate::Reg<mdatactrl::MdatactrlSpec>;
#[doc = "Master Data Control Register"]
pub mod mdatactrl;
#[doc = "MWDATAB (w) register accessor: Master Write Data Byte Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatab::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatab`]
module"]
#[doc(alias = "MWDATAB")]
pub type Mwdatab = crate::Reg<mwdatab::MwdatabSpec>;
#[doc = "Master Write Data Byte Register"]
pub mod mwdatab;
#[doc = "MWDATABE (w) register accessor: Master Write Data Byte End Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatabe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatabe`]
module"]
#[doc(alias = "MWDATABE")]
pub type Mwdatabe = crate::Reg<mwdatabe::MwdatabeSpec>;
#[doc = "Master Write Data Byte End Register"]
pub mod mwdatabe;
#[doc = "MWDATAH (w) register accessor: Master Write Data Half-word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatah::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatah`]
module"]
#[doc(alias = "MWDATAH")]
pub type Mwdatah = crate::Reg<mwdatah::MwdatahSpec>;
#[doc = "Master Write Data Half-word Register"]
pub mod mwdatah;
#[doc = "MWDATAHE (w) register accessor: Master Write Data Byte End Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatahe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatahe`]
module"]
#[doc(alias = "MWDATAHE")]
pub type Mwdatahe = crate::Reg<mwdatahe::MwdataheSpec>;
#[doc = "Master Write Data Byte End Register"]
pub mod mwdatahe;
#[doc = "MRDATAB (r) register accessor: Master Read Data Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatab::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdatab`]
module"]
#[doc(alias = "MRDATAB")]
pub type Mrdatab = crate::Reg<mrdatab::MrdatabSpec>;
#[doc = "Master Read Data Byte Register"]
pub mod mrdatab;
#[doc = "MRDATAH (r) register accessor: Master Read Data Half-word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatah::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdatah`]
module"]
#[doc(alias = "MRDATAH")]
pub type Mrdatah = crate::Reg<mrdatah::MrdatahSpec>;
#[doc = "Master Read Data Half-word Register"]
pub mod mrdatah;
#[doc = "MWMSG_SDR_CONTROL (w) register accessor: Master Write Message in SDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_sdr_control::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwmsg_sdr_control`]
module"]
#[doc(alias = "MWMSG_SDR_CONTROL")]
pub type MwmsgSdrControl = crate::Reg<mwmsg_sdr_control::MwmsgSdrControlSpec>;
#[doc = "Master Write Message in SDR mode"]
pub mod mwmsg_sdr_control;
#[doc = "MWMSG_SDR_DATA (w) register accessor: Master Write Message Data in SDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_sdr_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwmsg_sdr_data`]
module"]
#[doc(alias = "MWMSG_SDR_DATA")]
pub type MwmsgSdrData = crate::Reg<mwmsg_sdr_data::MwmsgSdrDataSpec>;
#[doc = "Master Write Message Data in SDR mode"]
pub mod mwmsg_sdr_data;
#[doc = "MRMSG_SDR (r) register accessor: Master Read Message in SDR mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mrmsg_sdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrmsg_sdr`]
module"]
#[doc(alias = "MRMSG_SDR")]
pub type MrmsgSdr = crate::Reg<mrmsg_sdr::MrmsgSdrSpec>;
#[doc = "Master Read Message in SDR mode"]
pub mod mrmsg_sdr;
#[doc = "MWMSG_DDR_CONTROL (w) register accessor: Master Write Message in DDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr_control::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwmsg_ddr_control`]
module"]
#[doc(alias = "MWMSG_DDR_CONTROL")]
pub type MwmsgDdrControl = crate::Reg<mwmsg_ddr_control::MwmsgDdrControlSpec>;
#[doc = "Master Write Message in DDR mode"]
pub mod mwmsg_ddr_control;
#[doc = "MWMSG_DDR_DATA (w) register accessor: Master Write Message Data in DDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwmsg_ddr_data`]
module"]
#[doc(alias = "MWMSG_DDR_DATA")]
pub type MwmsgDdrData = crate::Reg<mwmsg_ddr_data::MwmsgDdrDataSpec>;
#[doc = "Master Write Message Data in DDR mode"]
pub mod mwmsg_ddr_data;
#[doc = "MRMSG_DDR (rw) register accessor: Master Read Message in DDR mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mrmsg_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrmsg_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrmsg_ddr`]
module"]
#[doc(alias = "MRMSG_DDR")]
pub type MrmsgDdr = crate::Reg<mrmsg_ddr::MrmsgDdrSpec>;
#[doc = "Master Read Message in DDR mode"]
pub mod mrmsg_ddr;
#[doc = "MDYNADDR (rw) register accessor: Master Dynamic Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdynaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdynaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdynaddr`]
module"]
#[doc(alias = "MDYNADDR")]
pub type Mdynaddr = crate::Reg<mdynaddr::MdynaddrSpec>;
#[doc = "Master Dynamic Address Register"]
pub mod mdynaddr;
#[doc = "SID (r) register accessor: Slave Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sid`]
module"]
#[doc(alias = "SID")]
pub type Sid = crate::Reg<sid::SidSpec>;
#[doc = "Slave Module ID Register"]
pub mod sid;
