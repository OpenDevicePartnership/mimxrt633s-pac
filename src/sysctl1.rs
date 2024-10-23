#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    mclkpindir: Mclkpindir,
    _reserved1: [u8; 0x1c],
    dspnmisrcsel: Dspnmisrcsel,
    _reserved2: [u8; 0x0c],
    fcctrlsel: [Fcctrlsel; 8],
    _reserved3: [u8; 0x20],
    sharedctrlset: [Sharedctrlset; 2],
    _reserved4: [u8; 0x0178],
    rxevpulsegen: Rxevpulsegen,
}
impl RegisterBlock {
    #[doc = "0x10 - mclk direction control"]
    #[inline(always)]
    pub const fn mclkpindir(&self) -> &Mclkpindir {
        &self.mclkpindir
    }
    #[doc = "0x30 - DSP NMI source selection"]
    #[inline(always)]
    pub const fn dspnmisrcsel(&self) -> &Dspnmisrcsel {
        &self.dspnmisrcsel
    }
    #[doc = "0x40..0x60 - flexcomm control selection N"]
    #[inline(always)]
    pub const fn fcctrlsel(&self, n: usize) -> &Fcctrlsel {
        &self.fcctrlsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - flexcomm control selection N"]
    #[inline(always)]
    pub fn fcctrlsel_iter(&self) -> impl Iterator<Item = &Fcctrlsel> {
        self.fcctrlsel.iter()
    }
    #[doc = "0x80..0x88 - shared control set N"]
    #[inline(always)]
    pub const fn sharedctrlset(&self, n: usize) -> &Sharedctrlset {
        &self.sharedctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - shared control set N"]
    #[inline(always)]
    pub fn sharedctrlset_iter(&self) -> impl Iterator<Item = &Sharedctrlset> {
        self.sharedctrlset.iter()
    }
    #[doc = "0x200 - RX Event Pulse Generator"]
    #[inline(always)]
    pub const fn rxevpulsegen(&self) -> &Rxevpulsegen {
        &self.rxevpulsegen
    }
}
#[doc = "MCLKPINDIR (rw) register accessor: mclk direction control\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkpindir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkpindir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkpindir`]
module"]
#[doc(alias = "MCLKPINDIR")]
pub type Mclkpindir = crate::Reg<mclkpindir::MclkpindirSpec>;
#[doc = "mclk direction control"]
pub mod mclkpindir;
#[doc = "DSPNMISRCSEL (rw) register accessor: DSP NMI source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dspnmisrcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspnmisrcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspnmisrcsel`]
module"]
#[doc(alias = "DSPNMISRCSEL")]
pub type Dspnmisrcsel = crate::Reg<dspnmisrcsel::DspnmisrcselSpec>;
#[doc = "DSP NMI source selection"]
pub mod dspnmisrcsel;
#[doc = "FCCTRLSEL (rw) register accessor: flexcomm control selection N\n\nYou can [`read`](crate::Reg::read) this register and get [`fcctrlsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcctrlsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcctrlsel`]
module"]
#[doc(alias = "FCCTRLSEL")]
pub type Fcctrlsel = crate::Reg<fcctrlsel::FcctrlselSpec>;
#[doc = "flexcomm control selection N"]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET (rw) register accessor: shared control set N\n\nYou can [`read`](crate::Reg::read) this register and get [`sharedctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharedctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharedctrlset`]
module"]
#[doc(alias = "SHAREDCTRLSET")]
pub type Sharedctrlset = crate::Reg<sharedctrlset::SharedctrlsetSpec>;
#[doc = "shared control set N"]
pub mod sharedctrlset;
#[doc = "RXEVPULSEGEN (rw) register accessor: RX Event Pulse Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`rxevpulsegen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxevpulsegen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxevpulsegen`]
module"]
#[doc(alias = "RXEVPULSEGEN")]
pub type Rxevpulsegen = crate::Reg<rxevpulsegen::RxevpulsegenSpec>;
#[doc = "RX Event Pulse Generator"]
pub mod rxevpulsegen;
