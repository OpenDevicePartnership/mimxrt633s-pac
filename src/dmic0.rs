#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: (),
    _reserved1: [u8; 0x0f00],
    chanen: Chanen,
    _reserved2: [u8; 0x0c],
    use2fs: Use2fs,
    global_sync_en: GlobalSyncEn,
    global_count_val: GlobalCountVal,
    decreset: Decreset,
    _reserved6: [u8; 0x60],
    hwvadgain: Hwvadgain,
    hwvadhpfs: Hwvadhpfs,
    hwvadst10: Hwvadst10,
    hwvadrstt: Hwvadrstt,
    hwvadthgn: Hwvadthgn,
    hwvadthgs: Hwvadthgs,
    hwvadlowz: Hwvadlowz,
}
impl RegisterBlock {
    #[doc = "0x00..0x4a0 - Array of registers: OSR, DIVHFCLK, PREAC2FSCOEF, PREAC4FSCOEF, GAINSHIFT, FIFO_CTRL, FIFO_STATUS, FIFO_DATA, PHY_CTRL, DC_CTRL"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x4a0 - Array of registers: OSR, DIVHFCLK, PREAC2FSCOEF, PREAC4FSCOEF, GAINSHIFT, FIFO_CTRL, FIFO_STATUS, FIFO_DATA, PHY_CTRL, DC_CTRL"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0xf00 - Channel Enable register"]
    #[inline(always)]
    pub const fn chanen(&self) -> &Chanen {
        &self.chanen
    }
    #[doc = "0xf10 - Use 2FS register"]
    #[inline(always)]
    pub const fn use2fs(&self) -> &Use2fs {
        &self.use2fs
    }
    #[doc = "0xf14 - global sync enable"]
    #[inline(always)]
    pub const fn global_sync_en(&self) -> &GlobalSyncEn {
        &self.global_sync_en
    }
    #[doc = "0xf18 - no description available"]
    #[inline(always)]
    pub const fn global_count_val(&self) -> &GlobalCountVal {
        &self.global_count_val
    }
    #[doc = "0xf1c - no description available"]
    #[inline(always)]
    pub const fn decreset(&self) -> &Decreset {
        &self.decreset
    }
    #[doc = "0xf80 - HWVAD input gain register"]
    #[inline(always)]
    pub const fn hwvadgain(&self) -> &Hwvadgain {
        &self.hwvadgain
    }
    #[doc = "0xf84 - HWVAD filter control register"]
    #[inline(always)]
    pub const fn hwvadhpfs(&self) -> &Hwvadhpfs {
        &self.hwvadhpfs
    }
    #[doc = "0xf88 - HWVAD control register"]
    #[inline(always)]
    pub const fn hwvadst10(&self) -> &Hwvadst10 {
        &self.hwvadst10
    }
    #[doc = "0xf8c - HWVAD filter reset register"]
    #[inline(always)]
    pub const fn hwvadrstt(&self) -> &Hwvadrstt {
        &self.hwvadrstt
    }
    #[doc = "0xf90 - HWVAD noise estimator gain register"]
    #[inline(always)]
    pub const fn hwvadthgn(&self) -> &Hwvadthgn {
        &self.hwvadthgn
    }
    #[doc = "0xf94 - HWVAD signal estimator gain register"]
    #[inline(always)]
    pub const fn hwvadthgs(&self) -> &Hwvadthgs {
        &self.hwvadthgs
    }
    #[doc = "0xf98 - HWVAD noise envelope estimator register"]
    #[inline(always)]
    pub const fn hwvadlowz(&self) -> &Hwvadlowz {
        &self.hwvadlowz
    }
}
#[doc = "Array of registers: OSR, DIVHFCLK, PREAC2FSCOEF, PREAC4FSCOEF, GAINSHIFT, FIFO_CTRL, FIFO_STATUS, FIFO_DATA, PHY_CTRL, DC_CTRL"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Array of registers: OSR, DIVHFCLK, PREAC2FSCOEF, PREAC4FSCOEF, GAINSHIFT, FIFO_CTRL, FIFO_STATUS, FIFO_DATA, PHY_CTRL, DC_CTRL"]
pub mod channel;
#[doc = "CHANEN (rw) register accessor: Channel Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chanen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chanen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chanen`]
module"]
#[doc(alias = "CHANEN")]
pub type Chanen = crate::Reg<chanen::ChanenSpec>;
#[doc = "Channel Enable register"]
pub mod chanen;
#[doc = "USE2FS (rw) register accessor: Use 2FS register\n\nYou can [`read`](crate::Reg::read) this register and get [`use2fs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`use2fs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@use2fs`]
module"]
#[doc(alias = "USE2FS")]
pub type Use2fs = crate::Reg<use2fs::Use2fsSpec>;
#[doc = "Use 2FS register"]
pub mod use2fs;
#[doc = "GLOBAL_SYNC_EN (rw) register accessor: global sync enable\n\nYou can [`read`](crate::Reg::read) this register and get [`global_sync_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global_sync_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@global_sync_en`]
module"]
#[doc(alias = "GLOBAL_SYNC_EN")]
pub type GlobalSyncEn = crate::Reg<global_sync_en::GlobalSyncEnSpec>;
#[doc = "global sync enable"]
pub mod global_sync_en;
#[doc = "GLOBAL_COUNT_VAL (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`global_count_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global_count_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@global_count_val`]
module"]
#[doc(alias = "GLOBAL_COUNT_VAL")]
pub type GlobalCountVal = crate::Reg<global_count_val::GlobalCountValSpec>;
#[doc = "no description available"]
pub mod global_count_val;
#[doc = "DECRESET (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`decreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decreset`]
module"]
#[doc(alias = "DECRESET")]
pub type Decreset = crate::Reg<decreset::DecresetSpec>;
#[doc = "no description available"]
pub mod decreset;
#[doc = "HWVADGAIN (rw) register accessor: HWVAD input gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadgain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadgain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadgain`]
module"]
#[doc(alias = "HWVADGAIN")]
pub type Hwvadgain = crate::Reg<hwvadgain::HwvadgainSpec>;
#[doc = "HWVAD input gain register"]
pub mod hwvadgain;
#[doc = "HWVADHPFS (rw) register accessor: HWVAD filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadhpfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadhpfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadhpfs`]
module"]
#[doc(alias = "HWVADHPFS")]
pub type Hwvadhpfs = crate::Reg<hwvadhpfs::HwvadhpfsSpec>;
#[doc = "HWVAD filter control register"]
pub mod hwvadhpfs;
#[doc = "HWVADST10 (rw) register accessor: HWVAD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadst10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadst10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadst10`]
module"]
#[doc(alias = "HWVADST10")]
pub type Hwvadst10 = crate::Reg<hwvadst10::Hwvadst10Spec>;
#[doc = "HWVAD control register"]
pub mod hwvadst10;
#[doc = "HWVADRSTT (rw) register accessor: HWVAD filter reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadrstt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadrstt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadrstt`]
module"]
#[doc(alias = "HWVADRSTT")]
pub type Hwvadrstt = crate::Reg<hwvadrstt::HwvadrsttSpec>;
#[doc = "HWVAD filter reset register"]
pub mod hwvadrstt;
#[doc = "HWVADTHGN (rw) register accessor: HWVAD noise estimator gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadthgn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadthgn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadthgn`]
module"]
#[doc(alias = "HWVADTHGN")]
pub type Hwvadthgn = crate::Reg<hwvadthgn::HwvadthgnSpec>;
#[doc = "HWVAD noise estimator gain register"]
pub mod hwvadthgn;
#[doc = "HWVADTHGS (rw) register accessor: HWVAD signal estimator gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadthgs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadthgs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadthgs`]
module"]
#[doc(alias = "HWVADTHGS")]
pub type Hwvadthgs = crate::Reg<hwvadthgs::HwvadthgsSpec>;
#[doc = "HWVAD signal estimator gain register"]
pub mod hwvadthgs;
#[doc = "HWVADLOWZ (r) register accessor: HWVAD noise envelope estimator register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadlowz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvadlowz`]
module"]
#[doc(alias = "HWVADLOWZ")]
pub type Hwvadlowz = crate::Reg<hwvadlowz::HwvadlowzSpec>;
#[doc = "HWVAD noise envelope estimator register"]
pub mod hwvadlowz;
