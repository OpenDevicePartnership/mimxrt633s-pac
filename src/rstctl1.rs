#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sysrststat: Sysrststat,
    _reserved1: [u8; 0x0c],
    prstctl0: Prstctl0,
    prstctl1: Prstctl1,
    prstctl2: Prstctl2,
    _reserved4: [u8; 0x24],
    prstctl0_set: Prstctl0Set,
    prstctl1_set: Prstctl1Set,
    prstctl2_set: Prstctl2Set,
    _reserved7: [u8; 0x24],
    prstctl0_clr: Prstctl0Clr,
    prstctl1_clr: Prstctl1Clr,
    prstctl2_clr: Prstctl2Clr,
}
impl RegisterBlock {
    #[doc = "0x00 - system reset status register"]
    #[inline(always)]
    pub const fn sysrststat(&self) -> &Sysrststat {
        &self.sysrststat
    }
    #[doc = "0x10 - peripheral reset control register 0"]
    #[inline(always)]
    pub const fn prstctl0(&self) -> &Prstctl0 {
        &self.prstctl0
    }
    #[doc = "0x14 - peripheral reset control register 1"]
    #[inline(always)]
    pub const fn prstctl1(&self) -> &Prstctl1 {
        &self.prstctl1
    }
    #[doc = "0x18 - peripheral reset control register 2"]
    #[inline(always)]
    pub const fn prstctl2(&self) -> &Prstctl2 {
        &self.prstctl2
    }
    #[doc = "0x40 - peripheral reset set register 0"]
    #[inline(always)]
    pub const fn prstctl0_set(&self) -> &Prstctl0Set {
        &self.prstctl0_set
    }
    #[doc = "0x44 - peripheral reset set register 1"]
    #[inline(always)]
    pub const fn prstctl1_set(&self) -> &Prstctl1Set {
        &self.prstctl1_set
    }
    #[doc = "0x48 - peripheral reset set register 2"]
    #[inline(always)]
    pub const fn prstctl2_set(&self) -> &Prstctl2Set {
        &self.prstctl2_set
    }
    #[doc = "0x70 - peripheral reset clear register 0"]
    #[inline(always)]
    pub const fn prstctl0_clr(&self) -> &Prstctl0Clr {
        &self.prstctl0_clr
    }
    #[doc = "0x74 - peripheral reset clear register 1"]
    #[inline(always)]
    pub const fn prstctl1_clr(&self) -> &Prstctl1Clr {
        &self.prstctl1_clr
    }
    #[doc = "0x78 - peripheral reset clear register 2"]
    #[inline(always)]
    pub const fn prstctl2_clr(&self) -> &Prstctl2Clr {
        &self.prstctl2_clr
    }
}
#[doc = "SYSRSTSTAT (rw) register accessor: system reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrststat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrststat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrststat`]
module"]
#[doc(alias = "SYSRSTSTAT")]
pub type Sysrststat = crate::Reg<sysrststat::SysrststatSpec>;
#[doc = "system reset status register"]
pub mod sysrststat;
#[doc = "PRSTCTL0 (rw) register accessor: peripheral reset control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl0`]
module"]
#[doc(alias = "PRSTCTL0")]
pub type Prstctl0 = crate::Reg<prstctl0::Prstctl0Spec>;
#[doc = "peripheral reset control register 0"]
pub mod prstctl0;
#[doc = "PRSTCTL1 (rw) register accessor: peripheral reset control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl1`]
module"]
#[doc(alias = "PRSTCTL1")]
pub type Prstctl1 = crate::Reg<prstctl1::Prstctl1Spec>;
#[doc = "peripheral reset control register 1"]
pub mod prstctl1;
#[doc = "PRSTCTL2 (rw) register accessor: peripheral reset control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl2`]
module"]
#[doc(alias = "PRSTCTL2")]
pub type Prstctl2 = crate::Reg<prstctl2::Prstctl2Spec>;
#[doc = "peripheral reset control register 2"]
pub mod prstctl2;
#[doc = "PRSTCTL0_SET (w) register accessor: peripheral reset set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl0_set`]
module"]
#[doc(alias = "PRSTCTL0_SET")]
pub type Prstctl0Set = crate::Reg<prstctl0_set::Prstctl0SetSpec>;
#[doc = "peripheral reset set register 0"]
pub mod prstctl0_set;
#[doc = "PRSTCTL1_SET (w) register accessor: peripheral reset set register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl1_set`]
module"]
#[doc(alias = "PRSTCTL1_SET")]
pub type Prstctl1Set = crate::Reg<prstctl1_set::Prstctl1SetSpec>;
#[doc = "peripheral reset set register 1"]
pub mod prstctl1_set;
#[doc = "PRSTCTL2_SET (w) register accessor: peripheral reset set register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl2_set`]
module"]
#[doc(alias = "PRSTCTL2_SET")]
pub type Prstctl2Set = crate::Reg<prstctl2_set::Prstctl2SetSpec>;
#[doc = "peripheral reset set register 2"]
pub mod prstctl2_set;
#[doc = "PRSTCTL0_CLR (w) register accessor: peripheral reset clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl0_clr`]
module"]
#[doc(alias = "PRSTCTL0_CLR")]
pub type Prstctl0Clr = crate::Reg<prstctl0_clr::Prstctl0ClrSpec>;
#[doc = "peripheral reset clear register 0"]
pub mod prstctl0_clr;
#[doc = "PRSTCTL1_CLR (w) register accessor: peripheral reset clear register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl1_clr`]
module"]
#[doc(alias = "PRSTCTL1_CLR")]
pub type Prstctl1Clr = crate::Reg<prstctl1_clr::Prstctl1ClrSpec>;
#[doc = "peripheral reset clear register 1"]
pub mod prstctl1_clr;
#[doc = "PRSTCTL2_CLR (w) register accessor: peripheral reset clear register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstctl2_clr`]
module"]
#[doc(alias = "PRSTCTL2_CLR")]
pub type Prstctl2Clr = crate::Reg<prstctl2_clr::Prstctl2ClrSpec>;
#[doc = "peripheral reset clear register 2"]
pub mod prstctl2_clr;
