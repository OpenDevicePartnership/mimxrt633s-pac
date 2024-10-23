#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    caplength_chipid: CaplengthChipid,
    hcsparams: Hcsparams,
    hccparams: Hccparams,
    fladj_frindex: FladjFrindex,
    atlptd: Atlptd,
    isoptd: Isoptd,
    intptd: Intptd,
    datapayload: Datapayload,
    usbcmd: Usbcmd,
    usbsts: Usbsts,
    usbintr: Usbintr,
    portsc1: Portsc1,
    atlptdd: Atlptdd,
    atlptds: Atlptds,
    isoptdd: Isoptdd,
    isoptds: Isoptds,
    intptdd: Intptdd,
    intptds: Intptds,
    lastptd: Lastptd,
    _reserved19: [u8; 0x04],
    portmode: Portmode,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    #[inline(always)]
    pub const fn caplength_chipid(&self) -> &CaplengthChipid {
        &self.caplength_chipid
    }
    #[doc = "0x04 - Host Controller Structural Parameters"]
    #[inline(always)]
    pub const fn hcsparams(&self) -> &Hcsparams {
        &self.hcsparams
    }
    #[doc = "0x08 - Host Controller Capability Parameters"]
    #[inline(always)]
    pub const fn hccparams(&self) -> &Hccparams {
        &self.hccparams
    }
    #[doc = "0x0c - Frame Length Adjustment"]
    #[inline(always)]
    pub const fn fladj_frindex(&self) -> &FladjFrindex {
        &self.fladj_frindex
    }
    #[doc = "0x10 - Memory base address where ATL PTD0 is stored"]
    #[inline(always)]
    pub const fn atlptd(&self) -> &Atlptd {
        &self.atlptd
    }
    #[doc = "0x14 - Memory base address where ISO PTD0 is stored"]
    #[inline(always)]
    pub const fn isoptd(&self) -> &Isoptd {
        &self.isoptd
    }
    #[doc = "0x18 - Memory base address where INT PTD0 is stored"]
    #[inline(always)]
    pub const fn intptd(&self) -> &Intptd {
        &self.intptd
    }
    #[doc = "0x1c - Memory base address that indicates the start of the data payload buffers"]
    #[inline(always)]
    pub const fn datapayload(&self) -> &Datapayload {
        &self.datapayload
    }
    #[doc = "0x20 - USB Command register"]
    #[inline(always)]
    pub const fn usbcmd(&self) -> &Usbcmd {
        &self.usbcmd
    }
    #[doc = "0x24 - USB Interrupt Status register"]
    #[inline(always)]
    pub const fn usbsts(&self) -> &Usbsts {
        &self.usbsts
    }
    #[doc = "0x28 - USB Interrupt Enable register"]
    #[inline(always)]
    pub const fn usbintr(&self) -> &Usbintr {
        &self.usbintr
    }
    #[doc = "0x2c - Port Status and Control register"]
    #[inline(always)]
    pub const fn portsc1(&self) -> &Portsc1 {
        &self.portsc1
    }
    #[doc = "0x30 - Done map for each ATL PTD"]
    #[inline(always)]
    pub const fn atlptdd(&self) -> &Atlptdd {
        &self.atlptdd
    }
    #[doc = "0x34 - Skip map for each ATL PTD"]
    #[inline(always)]
    pub const fn atlptds(&self) -> &Atlptds {
        &self.atlptds
    }
    #[doc = "0x38 - Done map for each ISO PTD"]
    #[inline(always)]
    pub const fn isoptdd(&self) -> &Isoptdd {
        &self.isoptdd
    }
    #[doc = "0x3c - Skip map for each ISO PTD"]
    #[inline(always)]
    pub const fn isoptds(&self) -> &Isoptds {
        &self.isoptds
    }
    #[doc = "0x40 - Done map for each INT PTD"]
    #[inline(always)]
    pub const fn intptdd(&self) -> &Intptdd {
        &self.intptdd
    }
    #[doc = "0x44 - Skip map for each INT PTD"]
    #[inline(always)]
    pub const fn intptds(&self) -> &Intptds {
        &self.intptds
    }
    #[doc = "0x48 - Marks the last PTD in the list for ISO, INT and ATL"]
    #[inline(always)]
    pub const fn lastptd(&self) -> &Lastptd {
        &self.lastptd
    }
    #[doc = "0x50 - Controls the port if it is attached to the host block or the device block"]
    #[inline(always)]
    pub const fn portmode(&self) -> &Portmode {
        &self.portmode
    }
}
#[doc = "CAPLENGTH_CHIPID (r) register accessor: This register contains the offset value towards the start of the operational register space and the version number of the IP block\n\nYou can [`read`](crate::Reg::read) this register and get [`caplength_chipid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caplength_chipid`]
module"]
#[doc(alias = "CAPLENGTH_CHIPID")]
pub type CaplengthChipid = crate::Reg<caplength_chipid::CaplengthChipidSpec>;
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub mod caplength_chipid;
#[doc = "HCSPARAMS (r) register accessor: Host Controller Structural Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsparams`]
module"]
#[doc(alias = "HCSPARAMS")]
pub type Hcsparams = crate::Reg<hcsparams::HcsparamsSpec>;
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "HCCPARAMS (r) register accessor: Host Controller Capability Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccparams`]
module"]
#[doc(alias = "HCCPARAMS")]
pub type Hccparams = crate::Reg<hccparams::HccparamsSpec>;
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "FLADJ_FRINDEX (rw) register accessor: Frame Length Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`fladj_frindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fladj_frindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fladj_frindex`]
module"]
#[doc(alias = "FLADJ_FRINDEX")]
pub type FladjFrindex = crate::Reg<fladj_frindex::FladjFrindexSpec>;
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "ATLPTD (rw) register accessor: Memory base address where ATL PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`atlptd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atlptd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atlptd`]
module"]
#[doc(alias = "ATLPTD")]
pub type Atlptd = crate::Reg<atlptd::AtlptdSpec>;
#[doc = "Memory base address where ATL PTD0 is stored"]
pub mod atlptd;
#[doc = "ISOPTD (rw) register accessor: Memory base address where ISO PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoptd`]
module"]
#[doc(alias = "ISOPTD")]
pub type Isoptd = crate::Reg<isoptd::IsoptdSpec>;
#[doc = "Memory base address where ISO PTD0 is stored"]
pub mod isoptd;
#[doc = "INTPTD (rw) register accessor: Memory base address where INT PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`intptd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intptd`]
module"]
#[doc(alias = "INTPTD")]
pub type Intptd = crate::Reg<intptd::IntptdSpec>;
#[doc = "Memory base address where INT PTD0 is stored"]
pub mod intptd;
#[doc = "DATAPAYLOAD (rw) register accessor: Memory base address that indicates the start of the data payload buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`datapayload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datapayload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datapayload`]
module"]
#[doc(alias = "DATAPAYLOAD")]
pub type Datapayload = crate::Reg<datapayload::DatapayloadSpec>;
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub mod datapayload;
#[doc = "USBCMD (rw) register accessor: USB Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcmd`]
module"]
#[doc(alias = "USBCMD")]
pub type Usbcmd = crate::Reg<usbcmd::UsbcmdSpec>;
#[doc = "USB Command register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: USB Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsts`]
module"]
#[doc(alias = "USBSTS")]
pub type Usbsts = crate::Reg<usbsts::UsbstsSpec>;
#[doc = "USB Interrupt Status register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: USB Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintr`]
module"]
#[doc(alias = "USBINTR")]
pub type Usbintr = crate::Reg<usbintr::UsbintrSpec>;
#[doc = "USB Interrupt Enable register"]
pub mod usbintr;
#[doc = "PORTSC1 (rw) register accessor: Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`portsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portsc1`]
module"]
#[doc(alias = "PORTSC1")]
pub type Portsc1 = crate::Reg<portsc1::Portsc1Spec>;
#[doc = "Port Status and Control register"]
pub mod portsc1;
#[doc = "ATLPTDD (rw) register accessor: Done map for each ATL PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`atlptdd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atlptdd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atlptdd`]
module"]
#[doc(alias = "ATLPTDD")]
pub type Atlptdd = crate::Reg<atlptdd::AtlptddSpec>;
#[doc = "Done map for each ATL PTD"]
pub mod atlptdd;
#[doc = "ATLPTDS (rw) register accessor: Skip map for each ATL PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`atlptds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atlptds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atlptds`]
module"]
#[doc(alias = "ATLPTDS")]
pub type Atlptds = crate::Reg<atlptds::AtlptdsSpec>;
#[doc = "Skip map for each ATL PTD"]
pub mod atlptds;
#[doc = "ISOPTDD (rw) register accessor: Done map for each ISO PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptdd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptdd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoptdd`]
module"]
#[doc(alias = "ISOPTDD")]
pub type Isoptdd = crate::Reg<isoptdd::IsoptddSpec>;
#[doc = "Done map for each ISO PTD"]
pub mod isoptdd;
#[doc = "ISOPTDS (rw) register accessor: Skip map for each ISO PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoptds`]
module"]
#[doc(alias = "ISOPTDS")]
pub type Isoptds = crate::Reg<isoptds::IsoptdsSpec>;
#[doc = "Skip map for each ISO PTD"]
pub mod isoptds;
#[doc = "INTPTDD (rw) register accessor: Done map for each INT PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`intptdd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptdd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intptdd`]
module"]
#[doc(alias = "INTPTDD")]
pub type Intptdd = crate::Reg<intptdd::IntptddSpec>;
#[doc = "Done map for each INT PTD"]
pub mod intptdd;
#[doc = "INTPTDS (rw) register accessor: Skip map for each INT PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`intptds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intptds`]
module"]
#[doc(alias = "INTPTDS")]
pub type Intptds = crate::Reg<intptds::IntptdsSpec>;
#[doc = "Skip map for each INT PTD"]
pub mod intptds;
#[doc = "LASTPTD (rw) register accessor: Marks the last PTD in the list for ISO, INT and ATL\n\nYou can [`read`](crate::Reg::read) this register and get [`lastptd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lastptd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lastptd`]
module"]
#[doc(alias = "LASTPTD")]
pub type Lastptd = crate::Reg<lastptd::LastptdSpec>;
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub mod lastptd;
#[doc = "PORTMODE (rw) register accessor: Controls the port if it is attached to the host block or the device block\n\nYou can [`read`](crate::Reg::read) this register and get [`portmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portmode`]
module"]
#[doc(alias = "PORTMODE")]
pub type Portmode = crate::Reg<portmode::PortmodeSpec>;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
