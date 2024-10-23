#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: PCFG1, PCFG2, PSTAT"]
#[doc(alias = "SECCHANNEL")]
pub struct Secchannel {
    pcfg1: Pcfg1,
    pcfg2: Pcfg2,
    pstat: Pstat,
}
impl Secchannel {
    #[doc = "0x00 - Configuration register 1 for channel pair"]
    #[inline(always)]
    pub const fn pcfg1(&self) -> &Pcfg1 {
        &self.pcfg1
    }
    #[doc = "0x04 - Configuration register 2 for channel pair"]
    #[inline(always)]
    pub const fn pcfg2(&self) -> &Pcfg2 {
        &self.pcfg2
    }
    #[doc = "0x08 - Status register for channel pair"]
    #[inline(always)]
    pub const fn pstat(&self) -> &Pstat {
        &self.pstat
    }
}
#[doc = "PCFG1 (rw) register accessor: Configuration register 1 for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfg1`]
module"]
#[doc(alias = "PCFG1")]
pub type Pcfg1 = crate::Reg<pcfg1::Pcfg1Spec>;
#[doc = "Configuration register 1 for channel pair"]
pub mod pcfg1;
#[doc = "PCFG2 (rw) register accessor: Configuration register 2 for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfg2`]
module"]
#[doc(alias = "PCFG2")]
pub type Pcfg2 = crate::Reg<pcfg2::Pcfg2Spec>;
#[doc = "Configuration register 2 for channel pair"]
pub mod pcfg2;
#[doc = "PSTAT (rw) register accessor: Status register for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstat`]
module"]
#[doc(alias = "PSTAT")]
pub type Pstat = crate::Reg<pstat::PstatSpec>;
#[doc = "Status register for channel pair"]
pub mod pstat;
