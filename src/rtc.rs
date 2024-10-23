#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    match_: Match,
    count: Count,
    wake: Wake,
    subsec: Subsec,
    _reserved5: [u8; 0x2c],
    gpreg: [Gpreg; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - RTC match register"]
    #[inline(always)]
    pub const fn match_(&self) -> &Match {
        &self.match_
    }
    #[doc = "0x08 - RTC counter register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x0c - High-resolution/wake-up timer control register"]
    #[inline(always)]
    pub const fn wake(&self) -> &Wake {
        &self.wake
    }
    #[doc = "0x10 - RTC Sub-second Counter register"]
    #[inline(always)]
    pub const fn subsec(&self) -> &Subsec {
        &self.subsec
    }
    #[doc = "0x40..0x60 - General Purpose register"]
    #[inline(always)]
    pub const fn gpreg(&self, n: usize) -> &Gpreg {
        &self.gpreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - General Purpose register"]
    #[inline(always)]
    pub fn gpreg_iter(&self) -> impl Iterator<Item = &Gpreg> {
        self.gpreg.iter()
    }
}
#[doc = "CTRL (rw) register accessor: RTC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "MATCH (rw) register accessor: RTC match register\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "RTC match register"]
pub mod match_;
#[doc = "COUNT (rw) register accessor: RTC counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "RTC counter register"]
pub mod count;
#[doc = "WAKE (rw) register accessor: High-resolution/wake-up timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wake`]
module"]
#[doc(alias = "WAKE")]
pub type Wake = crate::Reg<wake::WakeSpec>;
#[doc = "High-resolution/wake-up timer control register"]
pub mod wake;
#[doc = "SUBSEC (rw) register accessor: RTC Sub-second Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`subsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsec`]
module"]
#[doc(alias = "SUBSEC")]
pub type Subsec = crate::Reg<subsec::SubsecSpec>;
#[doc = "RTC Sub-second Counter register"]
pub mod subsec;
#[doc = "GPREG (rw) register accessor: General Purpose register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg`]
module"]
#[doc(alias = "GPREG")]
pub type Gpreg = crate::Reg<gpreg::GpregSpec>;
#[doc = "General Purpose register"]
pub mod gpreg;
