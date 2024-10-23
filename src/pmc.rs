#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    status: Status,
    flags: Flags,
    ctrl: Ctrl,
    runctrl: Runctrl,
    sleepctrl: Sleepctrl,
    lvdcorectrl: Lvdcorectrl,
    _reserved6: [u8; 0x08],
    autowkup: Autowkup,
    pmiccfg: Pmiccfg,
    padvrange: Padvrange,
    memseqctrl: Memseqctrl,
}
impl RegisterBlock {
    #[doc = "0x04 - PMC status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Wakeup, interrupt, and reset flags"]
    #[inline(always)]
    pub const fn flags(&self) -> &Flags {
        &self.flags
    }
    #[doc = "0x0c - PMC control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - PMC controls used during run mode"]
    #[inline(always)]
    pub const fn runctrl(&self) -> &Runctrl {
        &self.runctrl
    }
    #[doc = "0x14 - PMC controls used during deep sleep mode"]
    #[inline(always)]
    pub const fn sleepctrl(&self) -> &Sleepctrl {
        &self.sleepctrl
    }
    #[doc = "0x18 - Active vddcore LVD monitor trip adjust"]
    #[inline(always)]
    pub const fn lvdcorectrl(&self) -> &Lvdcorectrl {
        &self.lvdcorectrl
    }
    #[doc = "0x24 - Automatic wakeup from deepsleep / deep powerdown modes"]
    #[inline(always)]
    pub const fn autowkup(&self) -> &Autowkup {
        &self.autowkup
    }
    #[doc = "0x28 - PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on"]
    #[inline(always)]
    pub const fn pmiccfg(&self) -> &Pmiccfg {
        &self.pmiccfg
    }
    #[doc = "0x2c - GPIO vdde range selection control"]
    #[inline(always)]
    pub const fn padvrange(&self) -> &Padvrange {
        &self.padvrange
    }
    #[doc = "0x30 - Memory Sequencer Control Register"]
    #[inline(always)]
    pub const fn memseqctrl(&self) -> &Memseqctrl {
        &self.memseqctrl
    }
}
#[doc = "STATUS (r) register accessor: PMC status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "PMC status"]
pub mod status;
#[doc = "FLAGS (rw) register accessor: Wakeup, interrupt, and reset flags\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flags`]
module"]
#[doc(alias = "FLAGS")]
pub type Flags = crate::Reg<flags::FlagsSpec>;
#[doc = "Wakeup, interrupt, and reset flags"]
pub mod flags;
#[doc = "CTRL (rw) register accessor: PMC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "PMC control register"]
pub mod ctrl;
#[doc = "RUNCTRL (rw) register accessor: PMC controls used during run mode\n\nYou can [`read`](crate::Reg::read) this register and get [`runctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`runctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@runctrl`]
module"]
#[doc(alias = "RUNCTRL")]
pub type Runctrl = crate::Reg<runctrl::RunctrlSpec>;
#[doc = "PMC controls used during run mode"]
pub mod runctrl;
#[doc = "SLEEPCTRL (rw) register accessor: PMC controls used during deep sleep mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepctrl`]
module"]
#[doc(alias = "SLEEPCTRL")]
pub type Sleepctrl = crate::Reg<sleepctrl::SleepctrlSpec>;
#[doc = "PMC controls used during deep sleep mode"]
pub mod sleepctrl;
#[doc = "LVDCORECTRL (rw) register accessor: Active vddcore LVD monitor trip adjust\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcorectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcorectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdcorectrl`]
module"]
#[doc(alias = "LVDCORECTRL")]
pub type Lvdcorectrl = crate::Reg<lvdcorectrl::LvdcorectrlSpec>;
#[doc = "Active vddcore LVD monitor trip adjust"]
pub mod lvdcorectrl;
#[doc = "AUTOWKUP (rw) register accessor: Automatic wakeup from deepsleep / deep powerdown modes\n\nYou can [`read`](crate::Reg::read) this register and get [`autowkup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autowkup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autowkup`]
module"]
#[doc(alias = "AUTOWKUP")]
pub type Autowkup = crate::Reg<autowkup::AutowkupSpec>;
#[doc = "Automatic wakeup from deepsleep / deep powerdown modes"]
pub mod autowkup;
#[doc = "PMICCFG (rw) register accessor: PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on\n\nYou can [`read`](crate::Reg::read) this register and get [`pmiccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmiccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmiccfg`]
module"]
#[doc(alias = "PMICCFG")]
pub type Pmiccfg = crate::Reg<pmiccfg::PmiccfgSpec>;
#[doc = "PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on"]
pub mod pmiccfg;
#[doc = "PADVRANGE (rw) register accessor: GPIO vdde range selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`padvrange::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padvrange::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padvrange`]
module"]
#[doc(alias = "PADVRANGE")]
pub type Padvrange = crate::Reg<padvrange::PadvrangeSpec>;
#[doc = "GPIO vdde range selection control"]
pub mod padvrange;
#[doc = "MEMSEQCTRL (rw) register accessor: Memory Sequencer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memseqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memseqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memseqctrl`]
module"]
#[doc(alias = "MEMSEQCTRL")]
pub type Memseqctrl = crate::Reg<memseqctrl::MemseqctrlSpec>;
#[doc = "Memory Sequencer Control Register"]
pub mod memseqctrl;
