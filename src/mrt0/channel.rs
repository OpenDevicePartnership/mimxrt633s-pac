#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    intval: Intval,
    timer: Timer,
    ctrl: Ctrl,
    stat: Stat,
}
impl Channel {
    #[doc = "0x00 - MRT Time interval value register. This value is loaded into the TIMER register."]
    #[inline(always)]
    pub const fn intval(&self) -> &Intval {
        &self.intval
    }
    #[doc = "0x04 - MRT Timer register. This register reads the value of the down-counter."]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x08 - MRT Control register. This register controls the MRT modes."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - MRT Status register."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "INTVAL (rw) register accessor: MRT Time interval value register. This value is loaded into the TIMER register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intval`]
module"]
#[doc(alias = "INTVAL")]
pub type Intval = crate::Reg<intval::IntvalSpec>;
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
pub mod intval;
#[doc = "TIMER (r) register accessor: MRT Timer register. This register reads the value of the down-counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`]
module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
pub mod timer;
#[doc = "CTRL (rw) register accessor: MRT Control register. This register controls the MRT modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "MRT Control register. This register controls the MRT modes."]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: MRT Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "MRT Status register."]
pub mod stat;
