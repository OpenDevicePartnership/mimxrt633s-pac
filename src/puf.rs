#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    keyindex: Keyindex,
    keysize: Keysize,
    _reserved3: [u8; 0x14],
    stat: Stat,
    _reserved4: [u8; 0x04],
    allow: Allow,
    _reserved5: [u8; 0x14],
    keyinput: Keyinput,
    codeinput: Codeinput,
    codeoutput: Codeoutput,
    _reserved8: [u8; 0x14],
    keyoutindex: Keyoutindex,
    keyoutput: Keyoutput,
    _reserved10: [u8; 0x74],
    ifstat: Ifstat,
    _reserved11: [u8; 0x1c],
    version: Version,
    inten: Inten,
    intstat: Intstat,
    pwrctrl: Pwrctrl,
    cfg: Cfg,
    _reserved16: [u8; 0xf0],
    keylock: Keylock,
    keyenable: Keyenable,
    keyreset: Keyreset,
    idxblk_l: IdxblkL,
    idxblk_h_dp: IdxblkHDp,
    keymask: [Keymask; 2],
    _reserved22: [u8; 0x38],
    idxblk_h: IdxblkH,
    idxblk_l_dp: IdxblkLDp,
}
impl RegisterBlock {
    #[doc = "0x00 - PUF Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - PUF Key Index"]
    #[inline(always)]
    pub const fn keyindex(&self) -> &Keyindex {
        &self.keyindex
    }
    #[doc = "0x08 - PUF Key Size"]
    #[inline(always)]
    pub const fn keysize(&self) -> &Keysize {
        &self.keysize
    }
    #[doc = "0x20 - PUF Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x28 - PUF Allow"]
    #[inline(always)]
    pub const fn allow(&self) -> &Allow {
        &self.allow
    }
    #[doc = "0x40 - PUF Key Input"]
    #[inline(always)]
    pub const fn keyinput(&self) -> &Keyinput {
        &self.keyinput
    }
    #[doc = "0x44 - PUF Code Input"]
    #[inline(always)]
    pub const fn codeinput(&self) -> &Codeinput {
        &self.codeinput
    }
    #[doc = "0x48 - PUF Code Output"]
    #[inline(always)]
    pub const fn codeoutput(&self) -> &Codeoutput {
        &self.codeoutput
    }
    #[doc = "0x60 - PUF Key Output Index"]
    #[inline(always)]
    pub const fn keyoutindex(&self) -> &Keyoutindex {
        &self.keyoutindex
    }
    #[doc = "0x64 - PUF Key Output"]
    #[inline(always)]
    pub const fn keyoutput(&self) -> &Keyoutput {
        &self.keyoutput
    }
    #[doc = "0xdc - PUF Interface Status and Clear"]
    #[inline(always)]
    pub const fn ifstat(&self) -> &Ifstat {
        &self.ifstat
    }
    #[doc = "0xfc - PUF Version"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x100 - PUF Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x104 - PUF Interrupt Status"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x108 - PUF Power Control"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    #[doc = "0x10c - PUF Configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x200 - Key Lock"]
    #[inline(always)]
    pub const fn keylock(&self) -> &Keylock {
        &self.keylock
    }
    #[doc = "0x204 - Key Enable"]
    #[inline(always)]
    pub const fn keyenable(&self) -> &Keyenable {
        &self.keyenable
    }
    #[doc = "0x208 - Key Reset"]
    #[inline(always)]
    pub const fn keyreset(&self) -> &Keyreset {
        &self.keyreset
    }
    #[doc = "0x20c - Index Block Low"]
    #[inline(always)]
    pub const fn idxblk_l(&self) -> &IdxblkL {
        &self.idxblk_l
    }
    #[doc = "0x210 - Index Block High Duplicate"]
    #[inline(always)]
    pub const fn idxblk_h_dp(&self) -> &IdxblkHDp {
        &self.idxblk_h_dp
    }
    #[doc = "0x214..0x21c - Key Mask x"]
    #[inline(always)]
    pub const fn keymask(&self, n: usize) -> &Keymask {
        &self.keymask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x214..0x21c - Key Mask x"]
    #[inline(always)]
    pub fn keymask_iter(&self) -> impl Iterator<Item = &Keymask> {
        self.keymask.iter()
    }
    #[doc = "0x254 - Index Block High"]
    #[inline(always)]
    pub const fn idxblk_h(&self) -> &IdxblkH {
        &self.idxblk_h
    }
    #[doc = "0x258 - Index Block Low Duplicate"]
    #[inline(always)]
    pub const fn idxblk_l_dp(&self) -> &IdxblkLDp {
        &self.idxblk_l_dp
    }
}
#[doc = "CTRL (rw) register accessor: PUF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "PUF Control"]
pub mod ctrl;
#[doc = "KEYINDEX (rw) register accessor: PUF Key Index\n\nYou can [`read`](crate::Reg::read) this register and get [`keyindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyindex`]
module"]
#[doc(alias = "KEYINDEX")]
pub type Keyindex = crate::Reg<keyindex::KeyindexSpec>;
#[doc = "PUF Key Index"]
pub mod keyindex;
#[doc = "KEYSIZE (rw) register accessor: PUF Key Size\n\nYou can [`read`](crate::Reg::read) this register and get [`keysize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keysize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keysize`]
module"]
#[doc(alias = "KEYSIZE")]
pub type Keysize = crate::Reg<keysize::KeysizeSpec>;
#[doc = "PUF Key Size"]
pub mod keysize;
#[doc = "STAT (r) register accessor: PUF Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "PUF Status"]
pub mod stat;
#[doc = "ALLOW (r) register accessor: PUF Allow\n\nYou can [`read`](crate::Reg::read) this register and get [`allow::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@allow`]
module"]
#[doc(alias = "ALLOW")]
pub type Allow = crate::Reg<allow::AllowSpec>;
#[doc = "PUF Allow"]
pub mod allow;
#[doc = "KEYINPUT (w) register accessor: PUF Key Input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyinput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyinput`]
module"]
#[doc(alias = "KEYINPUT")]
pub type Keyinput = crate::Reg<keyinput::KeyinputSpec>;
#[doc = "PUF Key Input"]
pub mod keyinput;
#[doc = "CODEINPUT (w) register accessor: PUF Code Input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codeinput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codeinput`]
module"]
#[doc(alias = "CODEINPUT")]
pub type Codeinput = crate::Reg<codeinput::CodeinputSpec>;
#[doc = "PUF Code Input"]
pub mod codeinput;
#[doc = "CODEOUTPUT (r) register accessor: PUF Code Output\n\nYou can [`read`](crate::Reg::read) this register and get [`codeoutput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codeoutput`]
module"]
#[doc(alias = "CODEOUTPUT")]
pub type Codeoutput = crate::Reg<codeoutput::CodeoutputSpec>;
#[doc = "PUF Code Output"]
pub mod codeoutput;
#[doc = "KEYOUTINDEX (r) register accessor: PUF Key Output Index\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutindex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyoutindex`]
module"]
#[doc(alias = "KEYOUTINDEX")]
pub type Keyoutindex = crate::Reg<keyoutindex::KeyoutindexSpec>;
#[doc = "PUF Key Output Index"]
pub mod keyoutindex;
#[doc = "KEYOUTPUT (r) register accessor: PUF Key Output\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyoutput`]
module"]
#[doc(alias = "KEYOUTPUT")]
pub type Keyoutput = crate::Reg<keyoutput::KeyoutputSpec>;
#[doc = "PUF Key Output"]
pub mod keyoutput;
#[doc = "IFSTAT (rw) register accessor: PUF Interface Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ifstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifstat`]
module"]
#[doc(alias = "IFSTAT")]
pub type Ifstat = crate::Reg<ifstat::IfstatSpec>;
#[doc = "PUF Interface Status and Clear"]
pub mod ifstat;
#[doc = "VERSION (r) register accessor: PUF Version\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "PUF Version"]
pub mod version;
#[doc = "INTEN (rw) register accessor: PUF Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: PUF Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "PUF Interrupt Status"]
pub mod intstat;
#[doc = "PWRCTRL (rw) register accessor: PUF Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`]
module"]
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PwrctrlSpec>;
#[doc = "PUF Power Control"]
pub mod pwrctrl;
#[doc = "CFG (rw) register accessor: PUF Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "PUF Configuration"]
pub mod cfg;
#[doc = "KEYLOCK (rw) register accessor: Key Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keylock`]
module"]
#[doc(alias = "KEYLOCK")]
pub type Keylock = crate::Reg<keylock::KeylockSpec>;
#[doc = "Key Lock"]
pub mod keylock;
#[doc = "KEYENABLE (rw) register accessor: Key Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`keyenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyenable`]
module"]
#[doc(alias = "KEYENABLE")]
pub type Keyenable = crate::Reg<keyenable::KeyenableSpec>;
#[doc = "Key Enable"]
pub mod keyenable;
#[doc = "KEYRESET (w) register accessor: Key Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyreset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyreset`]
module"]
#[doc(alias = "KEYRESET")]
pub type Keyreset = crate::Reg<keyreset::KeyresetSpec>;
#[doc = "Key Reset"]
pub mod keyreset;
#[doc = "IDXBLK_L (rw) register accessor: Index Block Low\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_l`]
module"]
#[doc(alias = "IDXBLK_L")]
pub type IdxblkL = crate::Reg<idxblk_l::IdxblkLSpec>;
#[doc = "Index Block Low"]
pub mod idxblk_l;
#[doc = "IDXBLK_H_DP (rw) register accessor: Index Block High Duplicate\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_h_dp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_h_dp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_h_dp`]
module"]
#[doc(alias = "IDXBLK_H_DP")]
pub type IdxblkHDp = crate::Reg<idxblk_h_dp::IdxblkHDpSpec>;
#[doc = "Index Block High Duplicate"]
pub mod idxblk_h_dp;
#[doc = "KEYMASK (w) register accessor: Key Mask x\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keymask::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keymask`]
module"]
#[doc(alias = "KEYMASK")]
pub type Keymask = crate::Reg<keymask::KeymaskSpec>;
#[doc = "Key Mask x"]
pub mod keymask;
#[doc = "IDXBLK_H (rw) register accessor: Index Block High\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_h`]
module"]
#[doc(alias = "IDXBLK_H")]
pub type IdxblkH = crate::Reg<idxblk_h::IdxblkHSpec>;
#[doc = "Index Block High"]
pub mod idxblk_h;
#[doc = "IDXBLK_L_DP (rw) register accessor: Index Block Low Duplicate\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_l_dp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_l_dp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idxblk_l_dp`]
module"]
#[doc(alias = "IDXBLK_L_DP")]
pub type IdxblkLDp = crate::Reg<idxblk_l_dp::IdxblkLDpSpec>;
#[doc = "Index Block Low Duplicate"]
pub mod idxblk_l_dp;
