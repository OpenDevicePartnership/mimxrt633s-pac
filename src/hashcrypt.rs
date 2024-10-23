#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    intenset: Intenset,
    intenclr: Intenclr,
    memctrl: Memctrl,
    memaddr: Memaddr,
    _reserved6: [u8; 0x08],
    indata: Indata,
    alias: [Alias; 7],
    digest0: [Digest0; 8],
    _reserved9: [u8; 0x20],
    cryptcfg: Cryptcfg,
    config: Config,
    _reserved11: [u8; 0x04],
    lock: Lock,
    mask: [Mask; 4],
    reload: [Reload; 8],
    _reserved14: [u8; 0x10],
    prng_seed: PrngSeed,
    _reserved15: [u8; 0x04],
    prng_out: PrngOut,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register to enable and operate Hash and Crypto"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Indicates status of Hash peripheral."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Write 1 to enable interrupts; reads back with which are set."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - Write 1 to clear interrupts."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Setup Master to access memory (if available)"]
    #[inline(always)]
    pub const fn memctrl(&self) -> &Memctrl {
        &self.memctrl
    }
    #[doc = "0x14 - Address to start memory access from (if available)."]
    #[inline(always)]
    pub const fn memaddr(&self) -> &Memaddr {
        &self.memaddr
    }
    #[doc = "0x20 - Input of 16 words at a time to load up buffer."]
    #[inline(always)]
    pub const fn indata(&self) -> &Indata {
        &self.indata
    }
    #[doc = "0x24..0x40 - no description available"]
    #[inline(always)]
    pub const fn alias(&self, n: usize) -> &Alias {
        &self.alias[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x40 - no description available"]
    #[inline(always)]
    pub fn alias_iter(&self) -> impl Iterator<Item = &Alias> {
        self.alias.iter()
    }
    #[doc = "0x40..0x60 - no description available"]
    #[inline(always)]
    pub const fn digest0(&self, n: usize) -> &Digest0 {
        &self.digest0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - no description available"]
    #[inline(always)]
    pub fn digest0_iter(&self) -> impl Iterator<Item = &Digest0> {
        self.digest0.iter()
    }
    #[doc = "0x80 - Crypto settings for AES and Salsa and ChaCha"]
    #[inline(always)]
    pub const fn cryptcfg(&self) -> &Cryptcfg {
        &self.cryptcfg
    }
    #[doc = "0x84 - Returns the configuration of this block in this chip - indicates what services are available."]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x8c - Lock register allows locking to the current security level or unlocking by the lock holding level."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x90..0xa0 - no description available"]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        &self.mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xa0 - no description available"]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        self.mask.iter()
    }
    #[doc = "0xa0..0xc0 - no description available"]
    #[inline(always)]
    pub const fn reload(&self, n: usize) -> &Reload {
        &self.reload[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xc0 - no description available"]
    #[inline(always)]
    pub fn reload_iter(&self) -> impl Iterator<Item = &Reload> {
        self.reload.iter()
    }
    #[doc = "0xd0 - PRNG random seed input value used as an entropy source"]
    #[inline(always)]
    pub const fn prng_seed(&self) -> &PrngSeed {
        &self.prng_seed
    }
    #[doc = "0xd8 - PRNG software-accessable random output value"]
    #[inline(always)]
    pub const fn prng_out(&self) -> &PrngOut {
        &self.prng_out
    }
}
#[doc = "CTRL (rw) register accessor: Control register to enable and operate Hash and Crypto\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register to enable and operate Hash and Crypto"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Indicates status of Hash peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Indicates status of Hash peripheral."]
pub mod status;
#[doc = "INTENSET (rw) register accessor: Write 1 to enable interrupts; reads back with which are set.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Write 1 to clear interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Write 1 to clear interrupts."]
pub mod intenclr;
#[doc = "MEMCTRL (rw) register accessor: Setup Master to access memory (if available)\n\nYou can [`read`](crate::Reg::read) this register and get [`memctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctrl`]
module"]
#[doc(alias = "MEMCTRL")]
pub type Memctrl = crate::Reg<memctrl::MemctrlSpec>;
#[doc = "Setup Master to access memory (if available)"]
pub mod memctrl;
#[doc = "MEMADDR (rw) register accessor: Address to start memory access from (if available).\n\nYou can [`read`](crate::Reg::read) this register and get [`memaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memaddr`]
module"]
#[doc(alias = "MEMADDR")]
pub type Memaddr = crate::Reg<memaddr::MemaddrSpec>;
#[doc = "Address to start memory access from (if available)."]
pub mod memaddr;
#[doc = "INDATA (w) register accessor: Input of 16 words at a time to load up buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indata`]
module"]
#[doc(alias = "INDATA")]
pub type Indata = crate::Reg<indata::IndataSpec>;
#[doc = "Input of 16 words at a time to load up buffer."]
pub mod indata;
#[doc = "ALIAS (w) register accessor: no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alias::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alias`]
module"]
#[doc(alias = "ALIAS")]
pub type Alias = crate::Reg<alias::AliasSpec>;
#[doc = "no description available"]
pub mod alias;
#[doc = "DIGEST0 (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`digest0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digest0`]
module"]
#[doc(alias = "DIGEST0")]
pub type Digest0 = crate::Reg<digest0::Digest0Spec>;
#[doc = "no description available"]
pub mod digest0;
#[doc = "CRYPTCFG (rw) register accessor: Crypto settings for AES and Salsa and ChaCha\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryptcfg`]
module"]
#[doc(alias = "CRYPTCFG")]
pub type Cryptcfg = crate::Reg<cryptcfg::CryptcfgSpec>;
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
pub mod cryptcfg;
#[doc = "CONFIG (rw) register accessor: Returns the configuration of this block in this chip - indicates what services are available.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
pub mod config;
#[doc = "LOCK (rw) register accessor: Lock register allows locking to the current security level or unlocking by the lock holding level.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
pub mod lock;
#[doc = "MASK (w) register accessor: no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "no description available"]
pub mod mask;
#[doc = "RELOAD (w) register accessor: no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reload`]
module"]
#[doc(alias = "RELOAD")]
pub type Reload = crate::Reg<reload::ReloadSpec>;
#[doc = "no description available"]
pub mod reload;
#[doc = "PRNG_SEED (w) register accessor: PRNG random seed input value used as an entropy source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prng_seed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prng_seed`]
module"]
#[doc(alias = "PRNG_SEED")]
pub type PrngSeed = crate::Reg<prng_seed::PrngSeedSpec>;
#[doc = "PRNG random seed input value used as an entropy source"]
pub mod prng_seed;
#[doc = "PRNG_OUT (r) register accessor: PRNG software-accessable random output value\n\nYou can [`read`](crate::Reg::read) this register and get [`prng_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prng_out`]
module"]
#[doc(alias = "PRNG_OUT")]
pub type PrngOut = crate::Reg<prng_out::PrngOutSpec>;
#[doc = "PRNG software-accessable random output value"]
pub mod prng_out;
