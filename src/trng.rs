#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mctl: Mctl,
    scmisc: Scmisc,
    pkrrng: Pkrrng,
    _reserved_3_pkrsq: [u8; 0x04],
    sdctl: Sdctl,
    _reserved_5_sblim: [u8; 0x04],
    frqmin: Frqmin,
    _reserved_7_frqcnt: [u8; 0x04],
    _reserved_8_scmc: [u8; 0x04],
    _reserved_9_scr: [u8; 0x04],
    _reserved_10_scr: [u8; 0x04],
    _reserved_11_scr: [u8; 0x04],
    _reserved_12_scr: [u8; 0x04],
    _reserved_13_scr: [u8; 0x04],
    _reserved_14_scr: [u8; 0x04],
    status: Status,
    ent: [Ent; 16],
    pkrcnt10: Pkrcnt10,
    pkrcnt32: Pkrcnt32,
    pkrcnt54: Pkrcnt54,
    pkrcnt76: Pkrcnt76,
    pkrcnt98: Pkrcnt98,
    pkrcntba: Pkrcntba,
    pkrcntdc: Pkrcntdc,
    pkrcntfe: Pkrcntfe,
    sec_cfg: SecCfg,
    int_ctrl: IntCtrl,
    int_mask: IntMask,
    int_status: IntStatus,
    _reserved29: [u8; 0x40],
    vid1: Vid1,
    vid2: Vid2,
}
impl RegisterBlock {
    #[doc = "0x00 - Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn mctl(&self) -> &Mctl {
        &self.mctl
    }
    #[doc = "0x04 - Statistical Check Miscellaneous Register"]
    #[inline(always)]
    pub const fn scmisc(&self) -> &Scmisc {
        &self.scmisc
    }
    #[doc = "0x08 - Poker Range Register"]
    #[inline(always)]
    pub const fn pkrrng(&self) -> &Pkrrng {
        &self.pkrrng
    }
    #[doc = "0x0c - Poker Square Calculation Result Register"]
    #[inline(always)]
    pub const fn pkrsq(&self) -> &Pkrsq {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Poker Maximum Limit Register"]
    #[inline(always)]
    pub const fn pkrmax(&self) -> &Pkrmax {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Seed Control Register"]
    #[inline(always)]
    pub const fn sdctl(&self) -> &Sdctl {
        &self.sdctl
    }
    #[doc = "0x14 - Total Samples Register"]
    #[inline(always)]
    pub const fn totsam(&self) -> &Totsam {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Sparse Bit Limit Register"]
    #[inline(always)]
    pub const fn sblim(&self) -> &Sblim {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Frequency Count Minimum Limit Register"]
    #[inline(always)]
    pub const fn frqmin(&self) -> &Frqmin {
        &self.frqmin
    }
    #[doc = "0x1c - Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub const fn frqmax(&self) -> &Frqmax {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Frequency Count Register"]
    #[inline(always)]
    pub const fn frqcnt(&self) -> &Frqcnt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub const fn scml(&self) -> &Scml {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub const fn scmc(&self) -> &Scmc {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub const fn scr1l(&self) -> &Scr1l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub const fn scr1c(&self) -> &Scr1c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub const fn scr2l(&self) -> &Scr2l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub const fn scr2c(&self) -> &Scr2c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub const fn scr3l(&self) -> &Scr3l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub const fn scr3c(&self) -> &Scr3c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub const fn scr4l(&self) -> &Scr4l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub const fn scr4c(&self) -> &Scr4c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub const fn scr5l(&self) -> &Scr5l {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub const fn scr5c(&self) -> &Scr5c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub const fn scr6pl(&self) -> &Scr6pl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub const fn scr6pc(&self) -> &Scr6pc {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x40..0x80 - Entropy Read Register"]
    #[inline(always)]
    pub const fn ent(&self, n: usize) -> &Ent {
        &self.ent[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Entropy Read Register"]
    #[inline(always)]
    pub fn ent_iter(&self) -> impl Iterator<Item = &Ent> {
        self.ent.iter()
    }
    #[doc = "0x80 - Statistical Check Poker Count 1 and 0 Register"]
    #[inline(always)]
    pub const fn pkrcnt10(&self) -> &Pkrcnt10 {
        &self.pkrcnt10
    }
    #[doc = "0x84 - Statistical Check Poker Count 3 and 2 Register"]
    #[inline(always)]
    pub const fn pkrcnt32(&self) -> &Pkrcnt32 {
        &self.pkrcnt32
    }
    #[doc = "0x88 - Statistical Check Poker Count 5 and 4 Register"]
    #[inline(always)]
    pub const fn pkrcnt54(&self) -> &Pkrcnt54 {
        &self.pkrcnt54
    }
    #[doc = "0x8c - Statistical Check Poker Count 7 and 6 Register"]
    #[inline(always)]
    pub const fn pkrcnt76(&self) -> &Pkrcnt76 {
        &self.pkrcnt76
    }
    #[doc = "0x90 - Statistical Check Poker Count 9 and 8 Register"]
    #[inline(always)]
    pub const fn pkrcnt98(&self) -> &Pkrcnt98 {
        &self.pkrcnt98
    }
    #[doc = "0x94 - Statistical Check Poker Count B and A Register"]
    #[inline(always)]
    pub const fn pkrcntba(&self) -> &Pkrcntba {
        &self.pkrcntba
    }
    #[doc = "0x98 - Statistical Check Poker Count D and C Register"]
    #[inline(always)]
    pub const fn pkrcntdc(&self) -> &Pkrcntdc {
        &self.pkrcntdc
    }
    #[doc = "0x9c - Statistical Check Poker Count F and E Register"]
    #[inline(always)]
    pub const fn pkrcntfe(&self) -> &Pkrcntfe {
        &self.pkrcntfe
    }
    #[doc = "0xa0 - Security Configuration Register"]
    #[inline(always)]
    pub const fn sec_cfg(&self) -> &SecCfg {
        &self.sec_cfg
    }
    #[doc = "0xa4 - Interrupt Control Register"]
    #[inline(always)]
    pub const fn int_ctrl(&self) -> &IntCtrl {
        &self.int_ctrl
    }
    #[doc = "0xa8 - Mask Register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0xac - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0xf0 - Version ID Register (MS)"]
    #[inline(always)]
    pub const fn vid1(&self) -> &Vid1 {
        &self.vid1
    }
    #[doc = "0xf4 - Version ID Register (LS)"]
    #[inline(always)]
    pub const fn vid2(&self) -> &Vid2 {
        &self.vid2
    }
}
#[doc = "MCTL (rw) register accessor: Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctl`]
module"]
#[doc(alias = "MCTL")]
pub type Mctl = crate::Reg<mctl::MctlSpec>;
#[doc = "Miscellaneous Control Register"]
pub mod mctl;
#[doc = "SCMISC (rw) register accessor: Statistical Check Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmisc`]
module"]
#[doc(alias = "SCMISC")]
pub type Scmisc = crate::Reg<scmisc::ScmiscSpec>;
#[doc = "Statistical Check Miscellaneous Register"]
pub mod scmisc;
#[doc = "PKRRNG (rw) register accessor: Poker Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrrng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkrrng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrrng`]
module"]
#[doc(alias = "PKRRNG")]
pub type Pkrrng = crate::Reg<pkrrng::PkrrngSpec>;
#[doc = "Poker Range Register"]
pub mod pkrrng;
#[doc = "PKRMAX (rw) register accessor: Poker Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkrmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrmax`]
module"]
#[doc(alias = "PKRMAX")]
pub type Pkrmax = crate::Reg<pkrmax::PkrmaxSpec>;
#[doc = "Poker Maximum Limit Register"]
pub mod pkrmax;
#[doc = "PKRSQ (r) register accessor: Poker Square Calculation Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrsq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrsq`]
module"]
#[doc(alias = "PKRSQ")]
pub type Pkrsq = crate::Reg<pkrsq::PkrsqSpec>;
#[doc = "Poker Square Calculation Result Register"]
pub mod pkrsq;
#[doc = "SDCTL (rw) register accessor: Seed Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdctl`]
module"]
#[doc(alias = "SDCTL")]
pub type Sdctl = crate::Reg<sdctl::SdctlSpec>;
#[doc = "Seed Control Register"]
pub mod sdctl;
#[doc = "SBLIM (rw) register accessor: Sparse Bit Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sblim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sblim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sblim`]
module"]
#[doc(alias = "SBLIM")]
pub type Sblim = crate::Reg<sblim::SblimSpec>;
#[doc = "Sparse Bit Limit Register"]
pub mod sblim;
#[doc = "TOTSAM (r) register accessor: Total Samples Register\n\nYou can [`read`](crate::Reg::read) this register and get [`totsam::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@totsam`]
module"]
#[doc(alias = "TOTSAM")]
pub type Totsam = crate::Reg<totsam::TotsamSpec>;
#[doc = "Total Samples Register"]
pub mod totsam;
#[doc = "FRQMIN (rw) register accessor: Frequency Count Minimum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frqmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frqmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frqmin`]
module"]
#[doc(alias = "FRQMIN")]
pub type Frqmin = crate::Reg<frqmin::FrqminSpec>;
#[doc = "Frequency Count Minimum Limit Register"]
pub mod frqmin;
#[doc = "FRQCNT (r) register accessor: Frequency Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frqcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frqcnt`]
module"]
#[doc(alias = "FRQCNT")]
pub type Frqcnt = crate::Reg<frqcnt::FrqcntSpec>;
#[doc = "Frequency Count Register"]
pub mod frqcnt;
#[doc = "FRQMAX (rw) register accessor: Frequency Count Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frqmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frqmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frqmax`]
module"]
#[doc(alias = "FRQMAX")]
pub type Frqmax = crate::Reg<frqmax::FrqmaxSpec>;
#[doc = "Frequency Count Maximum Limit Register"]
pub mod frqmax;
#[doc = "SCMC (r) register accessor: Statistical Check Monobit Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmc`]
module"]
#[doc(alias = "SCMC")]
pub type Scmc = crate::Reg<scmc::ScmcSpec>;
#[doc = "Statistical Check Monobit Count Register"]
pub mod scmc;
#[doc = "SCML (rw) register accessor: Statistical Check Monobit Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scml`]
module"]
#[doc(alias = "SCML")]
pub type Scml = crate::Reg<scml::ScmlSpec>;
#[doc = "Statistical Check Monobit Limit Register"]
pub mod scml;
#[doc = "SCR1C (r) register accessor: Statistical Check Run Length 1 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr1c`]
module"]
#[doc(alias = "SCR1C")]
pub type Scr1c = crate::Reg<scr1c::Scr1cSpec>;
#[doc = "Statistical Check Run Length 1 Count Register"]
pub mod scr1c;
#[doc = "SCR1L (rw) register accessor: Statistical Check Run Length 1 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr1l`]
module"]
#[doc(alias = "SCR1L")]
pub type Scr1l = crate::Reg<scr1l::Scr1lSpec>;
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod scr1l;
#[doc = "SCR2C (r) register accessor: Statistical Check Run Length 2 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr2c`]
module"]
#[doc(alias = "SCR2C")]
pub type Scr2c = crate::Reg<scr2c::Scr2cSpec>;
#[doc = "Statistical Check Run Length 2 Count Register"]
pub mod scr2c;
#[doc = "SCR2L (rw) register accessor: Statistical Check Run Length 2 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr2l`]
module"]
#[doc(alias = "SCR2L")]
pub type Scr2l = crate::Reg<scr2l::Scr2lSpec>;
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod scr2l;
#[doc = "SCR3C (r) register accessor: Statistical Check Run Length 3 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr3c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr3c`]
module"]
#[doc(alias = "SCR3C")]
pub type Scr3c = crate::Reg<scr3c::Scr3cSpec>;
#[doc = "Statistical Check Run Length 3 Count Register"]
pub mod scr3c;
#[doc = "SCR3L (rw) register accessor: Statistical Check Run Length 3 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr3l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr3l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr3l`]
module"]
#[doc(alias = "SCR3L")]
pub type Scr3l = crate::Reg<scr3l::Scr3lSpec>;
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod scr3l;
#[doc = "SCR4C (r) register accessor: Statistical Check Run Length 4 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr4c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr4c`]
module"]
#[doc(alias = "SCR4C")]
pub type Scr4c = crate::Reg<scr4c::Scr4cSpec>;
#[doc = "Statistical Check Run Length 4 Count Register"]
pub mod scr4c;
#[doc = "SCR4L (rw) register accessor: Statistical Check Run Length 4 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr4l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr4l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr4l`]
module"]
#[doc(alias = "SCR4L")]
pub type Scr4l = crate::Reg<scr4l::Scr4lSpec>;
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub mod scr4l;
#[doc = "SCR5C (r) register accessor: Statistical Check Run Length 5 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr5c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr5c`]
module"]
#[doc(alias = "SCR5C")]
pub type Scr5c = crate::Reg<scr5c::Scr5cSpec>;
#[doc = "Statistical Check Run Length 5 Count Register"]
pub mod scr5c;
#[doc = "SCR5L (rw) register accessor: Statistical Check Run Length 5 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr5l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr5l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr5l`]
module"]
#[doc(alias = "SCR5L")]
pub type Scr5l = crate::Reg<scr5l::Scr5lSpec>;
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub mod scr5l;
#[doc = "SCR6PC (r) register accessor: Statistical Check Run Length 6+ Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr6pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr6pc`]
module"]
#[doc(alias = "SCR6PC")]
pub type Scr6pc = crate::Reg<scr6pc::Scr6pcSpec>;
#[doc = "Statistical Check Run Length 6+ Count Register"]
pub mod scr6pc;
#[doc = "SCR6PL (rw) register accessor: Statistical Check Run Length 6+ Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr6pl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr6pl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr6pl`]
module"]
#[doc(alias = "SCR6PL")]
pub type Scr6pl = crate::Reg<scr6pl::Scr6plSpec>;
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub mod scr6pl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ENT (r) register accessor: Entropy Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ent::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ent`]
module"]
#[doc(alias = "ENT")]
pub type Ent = crate::Reg<ent::EntSpec>;
#[doc = "Entropy Read Register"]
pub mod ent;
#[doc = "PKRCNT10 (r) register accessor: Statistical Check Poker Count 1 and 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcnt10`]
module"]
#[doc(alias = "PKRCNT10")]
pub type Pkrcnt10 = crate::Reg<pkrcnt10::Pkrcnt10Spec>;
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub mod pkrcnt10;
#[doc = "PKRCNT32 (r) register accessor: Statistical Check Poker Count 3 and 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcnt32`]
module"]
#[doc(alias = "PKRCNT32")]
pub type Pkrcnt32 = crate::Reg<pkrcnt32::Pkrcnt32Spec>;
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub mod pkrcnt32;
#[doc = "PKRCNT54 (r) register accessor: Statistical Check Poker Count 5 and 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt54::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcnt54`]
module"]
#[doc(alias = "PKRCNT54")]
pub type Pkrcnt54 = crate::Reg<pkrcnt54::Pkrcnt54Spec>;
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub mod pkrcnt54;
#[doc = "PKRCNT76 (r) register accessor: Statistical Check Poker Count 7 and 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt76::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcnt76`]
module"]
#[doc(alias = "PKRCNT76")]
pub type Pkrcnt76 = crate::Reg<pkrcnt76::Pkrcnt76Spec>;
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub mod pkrcnt76;
#[doc = "PKRCNT98 (r) register accessor: Statistical Check Poker Count 9 and 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcnt98::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcnt98`]
module"]
#[doc(alias = "PKRCNT98")]
pub type Pkrcnt98 = crate::Reg<pkrcnt98::Pkrcnt98Spec>;
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub mod pkrcnt98;
#[doc = "PKRCNTBA (r) register accessor: Statistical Check Poker Count B and A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcntba::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcntba`]
module"]
#[doc(alias = "PKRCNTBA")]
pub type Pkrcntba = crate::Reg<pkrcntba::PkrcntbaSpec>;
#[doc = "Statistical Check Poker Count B and A Register"]
pub mod pkrcntba;
#[doc = "PKRCNTDC (r) register accessor: Statistical Check Poker Count D and C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcntdc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcntdc`]
module"]
#[doc(alias = "PKRCNTDC")]
pub type Pkrcntdc = crate::Reg<pkrcntdc::PkrcntdcSpec>;
#[doc = "Statistical Check Poker Count D and C Register"]
pub mod pkrcntdc;
#[doc = "PKRCNTFE (r) register accessor: Statistical Check Poker Count F and E Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrcntfe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkrcntfe`]
module"]
#[doc(alias = "PKRCNTFE")]
pub type Pkrcntfe = crate::Reg<pkrcntfe::PkrcntfeSpec>;
#[doc = "Statistical Check Poker Count F and E Register"]
pub mod pkrcntfe;
#[doc = "SEC_CFG (rw) register accessor: Security Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_cfg`]
module"]
#[doc(alias = "SEC_CFG")]
pub type SecCfg = crate::Reg<sec_cfg::SecCfgSpec>;
#[doc = "Security Configuration Register"]
pub mod sec_cfg;
#[doc = "INT_CTRL (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ctrl`]
module"]
#[doc(alias = "INT_CTRL")]
pub type IntCtrl = crate::Reg<int_ctrl::IntCtrlSpec>;
#[doc = "Interrupt Control Register"]
pub mod int_ctrl;
#[doc = "INT_MASK (rw) register accessor: Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Mask Register"]
pub mod int_mask;
#[doc = "INT_STATUS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "VID1 (r) register accessor: Version ID Register (MS)\n\nYou can [`read`](crate::Reg::read) this register and get [`vid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid1`]
module"]
#[doc(alias = "VID1")]
pub type Vid1 = crate::Reg<vid1::Vid1Spec>;
#[doc = "Version ID Register (MS)"]
pub mod vid1;
#[doc = "VID2 (r) register accessor: Version ID Register (LS)\n\nYou can [`read`](crate::Reg::read) this register and get [`vid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid2`]
module"]
#[doc(alias = "VID2")]
pub type Vid2 = crate::Reg<vid2::Vid2Spec>;
#[doc = "Version ID Register (LS)"]
pub mod vid2;
