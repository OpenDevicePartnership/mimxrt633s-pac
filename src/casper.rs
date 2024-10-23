#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    _reserved2: [u8; 0x04],
    status: Status,
    intenset: Intenset,
    intenclr: Intenclr,
    intstat: Intstat,
    _reserved6: [u8; 0x04],
    areg: Areg,
    breg: Breg,
    creg: Creg,
    dreg: Dreg,
    res0: Res0,
    res1: Res1,
    res2: Res2,
    res3: Res3,
    _reserved14: [u8; 0x20],
    mask: Mask,
    remask: Remask,
    _reserved16: [u8; 0x18],
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - Contains the offsets of AB and CD in the RAM."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x04 - Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x0c - Indicates operational status and would contain the carry bit if used."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Sets interrupts"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - Clears interrupts"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x18 - Interrupt status bits (mask of INTENSET and STATUS)"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x20 - A register"]
    #[inline(always)]
    pub const fn areg(&self) -> &Areg {
        &self.areg
    }
    #[doc = "0x24 - B register"]
    #[inline(always)]
    pub const fn breg(&self) -> &Breg {
        &self.breg
    }
    #[doc = "0x28 - C register"]
    #[inline(always)]
    pub const fn creg(&self) -> &Creg {
        &self.creg
    }
    #[doc = "0x2c - D register"]
    #[inline(always)]
    pub const fn dreg(&self) -> &Dreg {
        &self.dreg
    }
    #[doc = "0x30 - Result register 0"]
    #[inline(always)]
    pub const fn res0(&self) -> &Res0 {
        &self.res0
    }
    #[doc = "0x34 - Result register 1"]
    #[inline(always)]
    pub const fn res1(&self) -> &Res1 {
        &self.res1
    }
    #[doc = "0x38 - Result register 2"]
    #[inline(always)]
    pub const fn res2(&self) -> &Res2 {
        &self.res2
    }
    #[doc = "0x3c - Result register 3"]
    #[inline(always)]
    pub const fn res3(&self) -> &Res3 {
        &self.res3
    }
    #[doc = "0x60 - Optional mask register"]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
    #[doc = "0x64 - Optional re-mask register"]
    #[inline(always)]
    pub const fn remask(&self) -> &Remask {
        &self.remask
    }
    #[doc = "0x80 - Security lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "CTRL0 (rw) register accessor: Contains the offsets of AB and CD in the RAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Contains the offsets of AB and CD in the RAM."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
pub mod ctrl1;
#[doc = "STATUS (rw) register accessor: Indicates operational status and would contain the carry bit if used.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Indicates operational status and would contain the carry bit if used."]
pub mod status;
#[doc = "INTENSET (rw) register accessor: Sets interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Sets interrupts"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Clears interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Clears interrupts"]
pub mod intenclr;
#[doc = "INTSTAT (rw) register accessor: Interrupt status bits (mask of INTENSET and STATUS)\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
pub mod intstat;
#[doc = "AREG (rw) register accessor: A register\n\nYou can [`read`](crate::Reg::read) this register and get [`areg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`areg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@areg`]
module"]
#[doc(alias = "AREG")]
pub type Areg = crate::Reg<areg::AregSpec>;
#[doc = "A register"]
pub mod areg;
#[doc = "BREG (rw) register accessor: B register\n\nYou can [`read`](crate::Reg::read) this register and get [`breg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`breg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@breg`]
module"]
#[doc(alias = "BREG")]
pub type Breg = crate::Reg<breg::BregSpec>;
#[doc = "B register"]
pub mod breg;
#[doc = "CREG (rw) register accessor: C register\n\nYou can [`read`](crate::Reg::read) this register and get [`creg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`creg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@creg`]
module"]
#[doc(alias = "CREG")]
pub type Creg = crate::Reg<creg::CregSpec>;
#[doc = "C register"]
pub mod creg;
#[doc = "DREG (rw) register accessor: D register\n\nYou can [`read`](crate::Reg::read) this register and get [`dreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dreg`]
module"]
#[doc(alias = "DREG")]
pub type Dreg = crate::Reg<dreg::DregSpec>;
#[doc = "D register"]
pub mod dreg;
#[doc = "RES0 (rw) register accessor: Result register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`res0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res0`]
module"]
#[doc(alias = "RES0")]
pub type Res0 = crate::Reg<res0::Res0Spec>;
#[doc = "Result register 0"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: Result register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res1`]
module"]
#[doc(alias = "RES1")]
pub type Res1 = crate::Reg<res1::Res1Spec>;
#[doc = "Result register 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: Result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res2`]
module"]
#[doc(alias = "RES2")]
pub type Res2 = crate::Reg<res2::Res2Spec>;
#[doc = "Result register 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: Result register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`res3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res3`]
module"]
#[doc(alias = "RES3")]
pub type Res3 = crate::Reg<res3::Res3Spec>;
#[doc = "Result register 3"]
pub mod res3;
#[doc = "MASK (rw) register accessor: Optional mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Optional mask register"]
pub mod mask;
#[doc = "REMASK (rw) register accessor: Optional re-mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`remask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remask`]
module"]
#[doc(alias = "REMASK")]
pub type Remask = crate::Reg<remask::RemaskSpec>;
#[doc = "Optional re-mask register"]
pub mod remask;
#[doc = "LOCK (rw) register accessor: Security lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Security lock register"]
pub mod lock;
