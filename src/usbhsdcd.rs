#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    clock: Clock,
    status: Status,
    signal_override: SignalOverride,
    timer0: Timer0,
    timer1: Timer1,
    _reserved_6_timer2_bc1: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Clock register"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Signal Override Register"]
    #[inline(always)]
    pub const fn signal_override(&self) -> &SignalOverride {
        &self.signal_override
    }
    #[doc = "0x10 - TIMER0 register"]
    #[inline(always)]
    pub const fn timer0(&self) -> &Timer0 {
        &self.timer0
    }
    #[doc = "0x14 - TIMER1 register"]
    #[inline(always)]
    pub const fn timer1(&self) -> &Timer1 {
        &self.timer1
    }
    #[doc = "0x18 - TIMER2_BC12 register"]
    #[inline(always)]
    pub const fn timer2_bc12(&self) -> &Timer2Bc12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIMER2_BC11 register"]
    #[inline(always)]
    pub const fn timer2_bc11(&self) -> &Timer2Bc11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
}
#[doc = "CONTROL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control register"]
pub mod control;
#[doc = "CLOCK (rw) register accessor: Clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "Clock register"]
pub mod clock;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "SIGNAL_OVERRIDE (rw) register accessor: Signal Override Register\n\nYou can [`read`](crate::Reg::read) this register and get [`signal_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`signal_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@signal_override`]
module"]
#[doc(alias = "SIGNAL_OVERRIDE")]
pub type SignalOverride = crate::Reg<signal_override::SignalOverrideSpec>;
#[doc = "Signal Override Register"]
pub mod signal_override;
#[doc = "TIMER0 (rw) register accessor: TIMER0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0`]
module"]
#[doc(alias = "TIMER0")]
pub type Timer0 = crate::Reg<timer0::Timer0Spec>;
#[doc = "TIMER0 register"]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: TIMER1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1`]
module"]
#[doc(alias = "TIMER1")]
pub type Timer1 = crate::Reg<timer1::Timer1Spec>;
#[doc = "TIMER1 register"]
pub mod timer1;
#[doc = "TIMER2_BC11 (rw) register accessor: TIMER2_BC11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_bc11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_bc11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_bc11`]
module"]
#[doc(alias = "TIMER2_BC11")]
pub type Timer2Bc11 = crate::Reg<timer2_bc11::Timer2Bc11Spec>;
#[doc = "TIMER2_BC11 register"]
pub mod timer2_bc11;
#[doc = "TIMER2_BC12 (rw) register accessor: TIMER2_BC12 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_bc12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_bc12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_bc12`]
module"]
#[doc(alias = "TIMER2_BC12")]
pub type Timer2Bc12 = crate::Reg<timer2_bc12::Timer2Bc12Spec>;
#[doc = "TIMER2_BC12 register"]
pub mod timer2_bc12;
