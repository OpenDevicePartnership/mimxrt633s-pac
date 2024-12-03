#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    cfg1: Cfg1,
    cfg2: Cfg2,
    stat: Stat,
    _reserved3: [u8; 0x10],
    div: Div,
    secchannel: (),
    _reserved5: [u8; 0x01e0],
    fifocfg: Fifocfg,
    fifostat: Fifostat,
    fifotrig: Fifotrig,
    _reserved8: [u8; 0x04],
    fifointenset: Fifointenset,
    fifointenclr: Fifointenclr,
    fifointstat: Fifointstat,
    _reserved11: [u8; 0x04],
    fifowr: Fifowr,
    fifowr48h: Fifowr48h,
    _reserved13: [u8; 0x08],
    fiford: Fiford,
    fiford48h: Fiford48h,
    _reserved15: [u8; 0x08],
    fifordnopop: Fifordnopop,
    fiford48hnopop: Fiford48hnopop,
    fifosize: Fifosize,
    _reserved18: [u8; 0x01b0],
    id: Id,
}
impl RegisterBlock {
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0xc08 - Status register for the primary channel pair."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0xc20..0xc44 - Array of registers: PCFG1, PCFG2, PSTAT"]
    #[inline(always)]
    pub const fn secchannel(&self, n: usize) -> &Secchannel {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3104)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc20..0xc44 - Array of registers: PCFG1, PCFG2, PSTAT"]
    #[inline(always)]
    pub fn secchannel_iter(&self) -> impl Iterator<Item = &Secchannel> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3104)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0xe00 - FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn fifocfg(&self) -> &Fifocfg {
        &self.fifocfg
    }
    #[doc = "0xe04 - FIFO status register."]
    #[inline(always)]
    pub const fn fifostat(&self) -> &Fifostat {
        &self.fifostat
    }
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn fifotrig(&self) -> &Fifotrig {
        &self.fifotrig
    }
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn fifointenset(&self) -> &Fifointenset {
        &self.fifointenset
    }
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn fifointenclr(&self) -> &Fifointenclr {
        &self.fifointenclr
    }
    #[doc = "0xe18 - FIFO interrupt status register."]
    #[inline(always)]
    pub const fn fifointstat(&self) -> &Fifointstat {
        &self.fifointstat
    }
    #[doc = "0xe20 - FIFO write data."]
    #[inline(always)]
    pub const fn fifowr(&self) -> &Fifowr {
        &self.fifowr
    }
    #[doc = "0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fifowr48h(&self) -> &Fifowr48h {
        &self.fifowr48h
    }
    #[doc = "0xe30 - FIFO read data."]
    #[inline(always)]
    pub const fn fiford(&self) -> &Fiford {
        &self.fiford
    }
    #[doc = "0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fiford48h(&self) -> &Fiford48h {
        &self.fiford48h
    }
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn fifordnopop(&self) -> &Fifordnopop {
        &self.fifordnopop
    }
    #[doc = "0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    #[inline(always)]
    pub const fn fiford48hnopop(&self) -> &Fiford48hnopop {
        &self.fiford48hnopop
    }
    #[doc = "0xe48 - FIFO size register"]
    #[inline(always)]
    pub const fn fifosize(&self) -> &Fifosize {
        &self.fifosize
    }
    #[doc = "0xffc - I2S Module identification"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CFG1 (rw) register accessor: Configuration register 1 for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Configuration register 1 for the primary channel pair."]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: Configuration register 2 for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "Configuration register 2 for the primary channel pair."]
pub mod cfg2;
#[doc = "STAT (rw) register accessor: Status register for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register for the primary channel pair."]
pub mod stat;
#[doc = "DIV (rw) register accessor: Clock divider, used by all channel pairs.\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Clock divider, used by all channel pairs."]
pub mod div;
#[doc = "Array of registers: PCFG1, PCFG2, PSTAT"]
pub use self::secchannel::Secchannel;
#[doc = r"Cluster"]
#[doc = "Array of registers: PCFG1, PCFG2, PSTAT"]
pub mod secchannel;
#[doc = "FIFOCFG (rw) register accessor: FIFO configuration and enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocfg`]
module"]
#[doc(alias = "FIFOCFG")]
pub type Fifocfg = crate::Reg<fifocfg::FifocfgSpec>;
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: FIFO status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifostat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifostat`]
module"]
#[doc(alias = "FIFOSTAT")]
pub type Fifostat = crate::Reg<fifostat::FifostatSpec>;
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: FIFO trigger settings for interrupt and DMA request.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifotrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifotrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifotrig`]
module"]
#[doc(alias = "FIFOTRIG")]
pub type Fifotrig = crate::Reg<fifotrig::FifotrigSpec>;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: FIFO interrupt enable set (enable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointenset`]
module"]
#[doc(alias = "FIFOINTENSET")]
pub type Fifointenset = crate::Reg<fifointenset::FifointensetSpec>;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: FIFO interrupt enable clear (disable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointenclr`]
module"]
#[doc(alias = "FIFOINTENCLR")]
pub type Fifointenclr = crate::Reg<fifointenclr::FifointenclrSpec>;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: FIFO interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointstat`]
module"]
#[doc(alias = "FIFOINTSTAT")]
pub type Fifointstat = crate::Reg<fifointstat::FifointstatSpec>;
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFOWR (w) register accessor: FIFO write data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifowr`]
module"]
#[doc(alias = "FIFOWR")]
pub type Fifowr = crate::Reg<fifowr::FifowrSpec>;
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFOWR48H (w) register accessor: FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr48h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifowr48h`]
module"]
#[doc(alias = "FIFOWR48H")]
pub type Fifowr48h = crate::Reg<fifowr48h::Fifowr48hSpec>;
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fifowr48h;
#[doc = "FIFORD (r) register accessor: FIFO read data.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiford`]
module"]
#[doc(alias = "FIFORD")]
pub type Fiford = crate::Reg<fiford::FifordSpec>;
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFORD48H (r) register accessor: FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford48h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiford48h`]
module"]
#[doc(alias = "FIFORD48H")]
pub type Fiford48h = crate::Reg<fiford48h::Fiford48hSpec>;
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48h;
#[doc = "FIFORDNOPOP (r) register accessor: FIFO data read with no FIFO pop.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifordnopop::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifordnopop`]
module"]
#[doc(alias = "FIFORDNOPOP")]
pub type Fifordnopop = crate::Reg<fifordnopop::FifordnopopSpec>;
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFORD48HNOPOP (r) register accessor: FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford48hnopop::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiford48hnopop`]
module"]
#[doc(alias = "FIFORD48HNOPOP")]
pub type Fiford48hnopop = crate::Reg<fiford48hnopop::Fiford48hnopopSpec>;
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48hnopop;
#[doc = "FIFOSIZE (rw) register accessor: FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifosize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifosize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifosize`]
module"]
#[doc(alias = "FIFOSIZE")]
pub type Fifosize = crate::Reg<fifosize::FifosizeSpec>;
#[doc = "FIFO size register"]
pub mod fifosize;
#[doc = "ID (r) register accessor: I2S Module identification\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "I2S Module identification"]
pub mod id;
