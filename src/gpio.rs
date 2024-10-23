#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    b: [B; 8],
    _reserved1: [u8; 0x0f00],
    w: [W; 8],
    _reserved2: [u8; 0x0c00],
    dir: [Dir; 8],
    _reserved3: [u8; 0x60],
    mask: [Mask; 8],
    _reserved4: [u8; 0x60],
    pin: [Pin; 8],
    _reserved5: [u8; 0x60],
    mpin: [Mpin; 8],
    _reserved6: [u8; 0x60],
    set: [Set; 8],
    _reserved7: [u8; 0x60],
    clr: [Clr; 8],
    _reserved8: [u8; 0x60],
    not: [Not; 8],
    _reserved9: [u8; 0x60],
    dirset: [Dirset; 8],
    _reserved10: [u8; 0x60],
    dirclr: [Dirclr; 8],
    _reserved11: [u8; 0x60],
    dirnot: [Dirnot; 8],
    _reserved12: [u8; 0x60],
    intena: [Intena; 8],
    _reserved13: [u8; 0x60],
    intenb: [Intenb; 8],
    _reserved14: [u8; 0x60],
    intpol: [Intpol; 8],
    _reserved15: [u8; 0x60],
    intedg: [Intedg; 8],
    _reserved16: [u8; 0x60],
    intstata: [Intstata; 8],
    _reserved17: [u8; 0x60],
    intstatb: [Intstatb; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x100 - Array of registers: B_\\[%s\\]"]
    #[inline(always)]
    pub const fn b(&self, n: usize) -> &B {
        &self.b[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x100 - Array of registers: B_\\[%s\\]"]
    #[inline(always)]
    pub fn b_iter(&self) -> impl Iterator<Item = &B> {
        self.b.iter()
    }
    #[doc = "0x1000..0x1400 - Array of registers: W_\\[%s\\]"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1400 - Array of registers: W_\\[%s\\]"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0x2000..0x2020 - Direction registers"]
    #[inline(always)]
    pub const fn dir(&self, n: usize) -> &Dir {
        &self.dir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2020 - Direction registers"]
    #[inline(always)]
    pub fn dir_iter(&self) -> impl Iterator<Item = &Dir> {
        self.dir.iter()
    }
    #[doc = "0x2080..0x20a0 - Mask register"]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        &self.mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2080..0x20a0 - Mask register"]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        self.mask.iter()
    }
    #[doc = "0x2100..0x2120 - Port pin register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &Pin {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2100..0x2120 - Port pin register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &Pin> {
        self.pin.iter()
    }
    #[doc = "0x2180..0x21a0 - Masked port register"]
    #[inline(always)]
    pub const fn mpin(&self, n: usize) -> &Mpin {
        &self.mpin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2180..0x21a0 - Masked port register"]
    #[inline(always)]
    pub fn mpin_iter(&self) -> impl Iterator<Item = &Mpin> {
        self.mpin.iter()
    }
    #[doc = "0x2200..0x2220 - Write: Set register for port Read: output bits for port"]
    #[inline(always)]
    pub const fn set(&self, n: usize) -> &Set {
        &self.set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2200..0x2220 - Write: Set register for port Read: output bits for port"]
    #[inline(always)]
    pub fn set_iter(&self) -> impl Iterator<Item = &Set> {
        self.set.iter()
    }
    #[doc = "0x2280..0x22a0 - Clear port"]
    #[inline(always)]
    pub const fn clr(&self, n: usize) -> &Clr {
        &self.clr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2280..0x22a0 - Clear port"]
    #[inline(always)]
    pub fn clr_iter(&self) -> impl Iterator<Item = &Clr> {
        self.clr.iter()
    }
    #[doc = "0x2300..0x2320 - Toggle port"]
    #[inline(always)]
    pub const fn not(&self, n: usize) -> &Not {
        &self.not[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2300..0x2320 - Toggle port"]
    #[inline(always)]
    pub fn not_iter(&self) -> impl Iterator<Item = &Not> {
        self.not.iter()
    }
    #[doc = "0x2380..0x23a0 - Set pin direction bits for port"]
    #[inline(always)]
    pub const fn dirset(&self, n: usize) -> &Dirset {
        &self.dirset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2380..0x23a0 - Set pin direction bits for port"]
    #[inline(always)]
    pub fn dirset_iter(&self) -> impl Iterator<Item = &Dirset> {
        self.dirset.iter()
    }
    #[doc = "0x2400..0x2420 - Clear pin direction bits for port"]
    #[inline(always)]
    pub const fn dirclr(&self, n: usize) -> &Dirclr {
        &self.dirclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2400..0x2420 - Clear pin direction bits for port"]
    #[inline(always)]
    pub fn dirclr_iter(&self) -> impl Iterator<Item = &Dirclr> {
        self.dirclr.iter()
    }
    #[doc = "0x2480..0x24a0 - Toggle pin direction bits for port"]
    #[inline(always)]
    pub const fn dirnot(&self, n: usize) -> &Dirnot {
        &self.dirnot[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2480..0x24a0 - Toggle pin direction bits for port"]
    #[inline(always)]
    pub fn dirnot_iter(&self) -> impl Iterator<Item = &Dirnot> {
        self.dirnot.iter()
    }
    #[doc = "0x2500..0x2520 - interrupt A enable control register"]
    #[inline(always)]
    pub const fn intena(&self, n: usize) -> &Intena {
        &self.intena[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2500..0x2520 - interrupt A enable control register"]
    #[inline(always)]
    pub fn intena_iter(&self) -> impl Iterator<Item = &Intena> {
        self.intena.iter()
    }
    #[doc = "0x2580..0x25a0 - interrupt B enable control register"]
    #[inline(always)]
    pub const fn intenb(&self, n: usize) -> &Intenb {
        &self.intenb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2580..0x25a0 - interrupt B enable control register"]
    #[inline(always)]
    pub fn intenb_iter(&self) -> impl Iterator<Item = &Intenb> {
        self.intenb.iter()
    }
    #[doc = "0x2600..0x2620 - interupt polarity control register"]
    #[inline(always)]
    pub const fn intpol(&self, n: usize) -> &Intpol {
        &self.intpol[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2600..0x2620 - interupt polarity control register"]
    #[inline(always)]
    pub fn intpol_iter(&self) -> impl Iterator<Item = &Intpol> {
        self.intpol.iter()
    }
    #[doc = "0x2680..0x26a0 - choose edge or level for interrupt"]
    #[inline(always)]
    pub const fn intedg(&self, n: usize) -> &Intedg {
        &self.intedg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2680..0x26a0 - choose edge or level for interrupt"]
    #[inline(always)]
    pub fn intedg_iter(&self) -> impl Iterator<Item = &Intedg> {
        self.intedg.iter()
    }
    #[doc = "0x2700..0x2720 - interrupt status for interrupt A"]
    #[inline(always)]
    pub const fn intstata(&self, n: usize) -> &Intstata {
        &self.intstata[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2700..0x2720 - interrupt status for interrupt A"]
    #[inline(always)]
    pub fn intstata_iter(&self) -> impl Iterator<Item = &Intstata> {
        self.intstata.iter()
    }
    #[doc = "0x2780..0x27a0 - interrupt status for interrupt B"]
    #[inline(always)]
    pub const fn intstatb(&self, n: usize) -> &Intstatb {
        &self.intstatb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2780..0x27a0 - interrupt status for interrupt B"]
    #[inline(always)]
    pub fn intstatb_iter(&self) -> impl Iterator<Item = &Intstatb> {
        self.intstatb.iter()
    }
}
#[doc = "Array of registers: B_\\[%s\\]"]
pub use self::b::B;
#[doc = r"Cluster"]
#[doc = "Array of registers: B_\\[%s\\]"]
pub mod b;
#[doc = "Array of registers: W_\\[%s\\]"]
pub use self::w::W;
#[doc = r"Cluster"]
#[doc = "Array of registers: W_\\[%s\\]"]
pub mod w;
#[doc = "DIR (rw) register accessor: Direction registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Direction registers"]
pub mod dir;
#[doc = "MASK (rw) register accessor: Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Mask register"]
pub mod mask;
#[doc = "PIN (rw) register accessor: Port pin register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`]
module"]
#[doc(alias = "PIN")]
pub type Pin = crate::Reg<pin::PinSpec>;
#[doc = "Port pin register"]
pub mod pin;
#[doc = "MPIN (rw) register accessor: Masked port register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpin`]
module"]
#[doc(alias = "MPIN")]
pub type Mpin = crate::Reg<mpin::MpinSpec>;
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "SET (rw) register accessor: Write: Set register for port Read: output bits for port\n\nYou can [`read`](crate::Reg::read) this register and get [`set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "CLR (w) register accessor: Clear port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Clear port"]
pub mod clr;
#[doc = "NOT (w) register accessor: Toggle port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`not::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@not`]
module"]
#[doc(alias = "NOT")]
pub type Not = crate::Reg<not::NotSpec>;
#[doc = "Toggle port"]
pub mod not;
#[doc = "DIRSET (w) register accessor: Set pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`]
module"]
#[doc(alias = "DIRSET")]
pub type Dirset = crate::Reg<dirset::DirsetSpec>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR (w) register accessor: Clear pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`]
module"]
#[doc(alias = "DIRCLR")]
pub type Dirclr = crate::Reg<dirclr::DirclrSpec>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT (w) register accessor: Toggle pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirnot::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirnot`]
module"]
#[doc(alias = "DIRNOT")]
pub type Dirnot = crate::Reg<dirnot::DirnotSpec>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
#[doc = "INTENA (rw) register accessor: interrupt A enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intena`]
module"]
#[doc(alias = "INTENA")]
pub type Intena = crate::Reg<intena::IntenaSpec>;
#[doc = "interrupt A enable control register"]
pub mod intena;
#[doc = "INTENB (rw) register accessor: interrupt B enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenb`]
module"]
#[doc(alias = "INTENB")]
pub type Intenb = crate::Reg<intenb::IntenbSpec>;
#[doc = "interrupt B enable control register"]
pub mod intenb;
#[doc = "INTPOL (rw) register accessor: interupt polarity control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpol`]
module"]
#[doc(alias = "INTPOL")]
pub type Intpol = crate::Reg<intpol::IntpolSpec>;
#[doc = "interupt polarity control register"]
pub mod intpol;
#[doc = "INTEDG (rw) register accessor: choose edge or level for interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intedg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intedg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intedg`]
module"]
#[doc(alias = "INTEDG")]
pub type Intedg = crate::Reg<intedg::IntedgSpec>;
#[doc = "choose edge or level for interrupt"]
pub mod intedg;
#[doc = "INTSTATA (rw) register accessor: interrupt status for interrupt A\n\nYou can [`read`](crate::Reg::read) this register and get [`intstata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstata`]
module"]
#[doc(alias = "INTSTATA")]
pub type Intstata = crate::Reg<intstata::IntstataSpec>;
#[doc = "interrupt status for interrupt A"]
pub mod intstata;
#[doc = "INTSTATB (rw) register accessor: interrupt status for interrupt B\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatb`]
module"]
#[doc(alias = "INTSTATB")]
pub type Intstatb = crate::Reg<intstatb::IntstatbSpec>;
#[doc = "interrupt status for interrupt B"]
pub mod intstatb;
