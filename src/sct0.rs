#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    ctrl: Ctrl,
    limit: Limit,
    halt: Halt,
    stop: Stop,
    start: Start,
    _reserved6: [u8; 0x28],
    count: Count,
    state: State,
    input: Input,
    regmode: Regmode,
    output: Output,
    outputdirctrl: Outputdirctrl,
    res: Res,
    dmareq0: Dmareq0,
    dmareq1: Dmareq1,
    _reserved15: [u8; 0x8c],
    even: Even,
    evflag: Evflag,
    conen: Conen,
    conflag: Conflag,
    _reserved_19_cap0: [u8; 0x04],
    _reserved_20_cap1: [u8; 0x04],
    _reserved_21_cap2: [u8; 0x04],
    _reserved_22_cap3: [u8; 0x04],
    _reserved_23_cap4: [u8; 0x04],
    _reserved_24_cap5: [u8; 0x04],
    _reserved_25_cap6: [u8; 0x04],
    _reserved_26_cap7: [u8; 0x04],
    _reserved_27_cap8: [u8; 0x04],
    _reserved_28_cap9: [u8; 0x04],
    _reserved_29_cap10: [u8; 0x04],
    _reserved_30_cap11: [u8; 0x04],
    _reserved_31_cap12: [u8; 0x04],
    _reserved_32_cap13: [u8; 0x04],
    _reserved_33_cap14: [u8; 0x04],
    _reserved_34_cap15: [u8; 0x04],
    _reserved35: [u8; 0xc0],
    _reserved_35_capctrl0: [u8; 0x04],
    _reserved_36_capctrl1: [u8; 0x04],
    _reserved_37_capctrl2: [u8; 0x04],
    _reserved_38_capctrl3: [u8; 0x04],
    _reserved_39_capctrl4: [u8; 0x04],
    _reserved_40_capctrl5: [u8; 0x04],
    _reserved_41_capctrl6: [u8; 0x04],
    _reserved_42_capctrl7: [u8; 0x04],
    _reserved_43_capctrl8: [u8; 0x04],
    _reserved_44_capctrl9: [u8; 0x04],
    _reserved_45_capctrl10: [u8; 0x04],
    _reserved_46_capctrl11: [u8; 0x04],
    _reserved_47_capctrl12: [u8; 0x04],
    _reserved_48_capctrl13: [u8; 0x04],
    _reserved_49_capctrl14: [u8; 0x04],
    _reserved_50_capctrl15: [u8; 0x04],
    _reserved51: [u8; 0xc0],
    ev: [Ev; 16],
    _reserved52: [u8; 0x0180],
    out: [Out; 10],
}
impl RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - SCT control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - SCT limit event select register"]
    #[inline(always)]
    pub const fn limit(&self) -> &Limit {
        &self.limit
    }
    #[doc = "0x0c - SCT halt event select register"]
    #[inline(always)]
    pub const fn halt(&self) -> &Halt {
        &self.halt
    }
    #[doc = "0x10 - SCT stop event select register"]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x14 - SCT start event select register"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x40 - SCT counter register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x44 - SCT state register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x48 - SCT input register"]
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
    }
    #[doc = "0x4c - SCT match/capture mode register"]
    #[inline(always)]
    pub const fn regmode(&self) -> &Regmode {
        &self.regmode
    }
    #[doc = "0x50 - SCT output register"]
    #[inline(always)]
    pub const fn output(&self) -> &Output {
        &self.output
    }
    #[doc = "0x54 - SCT output counter direction control register"]
    #[inline(always)]
    pub const fn outputdirctrl(&self) -> &Outputdirctrl {
        &self.outputdirctrl
    }
    #[doc = "0x58 - SCT conflict resolution register"]
    #[inline(always)]
    pub const fn res(&self) -> &Res {
        &self.res
    }
    #[doc = "0x5c - SCT DMA request 0 register"]
    #[inline(always)]
    pub const fn dmareq0(&self) -> &Dmareq0 {
        &self.dmareq0
    }
    #[doc = "0x60 - SCT DMA request 1 register"]
    #[inline(always)]
    pub const fn dmareq1(&self) -> &Dmareq1 {
        &self.dmareq1
    }
    #[doc = "0xf0 - SCT event interrupt enable register"]
    #[inline(always)]
    pub const fn even(&self) -> &Even {
        &self.even
    }
    #[doc = "0xf4 - SCT event flag register"]
    #[inline(always)]
    pub const fn evflag(&self) -> &Evflag {
        &self.evflag
    }
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    #[inline(always)]
    pub const fn conen(&self) -> &Conen {
        &self.conen
    }
    #[doc = "0xfc - SCT conflict flag register"]
    #[inline(always)]
    pub const fn conflag(&self) -> &Conflag {
        &self.conflag
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match0(&self) -> &Match0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap0(&self) -> &Cap0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match1(&self) -> &Match1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap1(&self) -> &Cap1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match2(&self) -> &Match2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap2(&self) -> &Cap2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match3(&self) -> &Match3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap3(&self) -> &Cap3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match4(&self) -> &Match4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap4(&self) -> &Cap4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match5(&self) -> &Match5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap5(&self) -> &Cap5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match6(&self) -> &Match6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap6(&self) -> &Cap6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match7(&self) -> &Match7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap7(&self) -> &Cap7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match8(&self) -> &Match8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap8(&self) -> &Cap8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match9(&self) -> &Match9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap9(&self) -> &Cap9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x128 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match10(&self) -> &Match10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap10(&self) -> &Cap10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match11(&self) -> &Match11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap11(&self) -> &Cap11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x130 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match12(&self) -> &Match12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap12(&self) -> &Cap12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match13(&self) -> &Match13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap13(&self) -> &Cap13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match14(&self) -> &Match14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap14(&self) -> &Cap14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match15(&self) -> &Match15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap15(&self) -> &Cap15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel0(&self) -> &Matchrel0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl0(&self) -> &Capctrl0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel1(&self) -> &Matchrel1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl1(&self) -> &Capctrl1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel2(&self) -> &Matchrel2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl2(&self) -> &Capctrl2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel3(&self) -> &Matchrel3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl3(&self) -> &Capctrl3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel4(&self) -> &Matchrel4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl4(&self) -> &Capctrl4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel5(&self) -> &Matchrel5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl5(&self) -> &Capctrl5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel6(&self) -> &Matchrel6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl6(&self) -> &Capctrl6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel7(&self) -> &Matchrel7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl7(&self) -> &Capctrl7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel8(&self) -> &Matchrel8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl8(&self) -> &Capctrl8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel9(&self) -> &Matchrel9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl9(&self) -> &Capctrl9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x228 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel10(&self) -> &Matchrel10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl10(&self) -> &Capctrl10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x22c - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel11(&self) -> &Matchrel11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl11(&self) -> &Capctrl11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x230 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel12(&self) -> &Matchrel12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl12(&self) -> &Capctrl12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x234 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel13(&self) -> &Matchrel13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl13(&self) -> &Capctrl13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x238 - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel14(&self) -> &Matchrel14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl14(&self) -> &Capctrl14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x23c - SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel15(&self) -> &Matchrel15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl15(&self) -> &Capctrl15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x300..0x380 - Array of registers: EV_STATE, EV_CTRL"]
    #[inline(always)]
    pub const fn ev(&self, n: usize) -> &Ev {
        &self.ev[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x380 - Array of registers: EV_STATE, EV_CTRL"]
    #[inline(always)]
    pub fn ev_iter(&self) -> impl Iterator<Item = &Ev> {
        self.ev.iter()
    }
    #[doc = "0x500..0x550 - Array of registers: OUT_SET, OUT_CLR"]
    #[inline(always)]
    pub const fn out(&self, n: usize) -> &Out {
        &self.out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x550 - Array of registers: OUT_SET, OUT_CLR"]
    #[inline(always)]
    pub fn out_iter(&self) -> impl Iterator<Item = &Out> {
        self.out.iter()
    }
}
#[doc = "CONFIG (rw) register accessor: SCT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "CTRL (rw) register accessor: SCT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "LIMIT (rw) register accessor: SCT limit event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit`]
module"]
#[doc(alias = "LIMIT")]
pub type Limit = crate::Reg<limit::LimitSpec>;
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "HALT (rw) register accessor: SCT halt event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halt`]
module"]
#[doc(alias = "HALT")]
pub type Halt = crate::Reg<halt::HaltSpec>;
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "STOP (rw) register accessor: SCT stop event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "START (rw) register accessor: SCT start event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "COUNT (rw) register accessor: SCT counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "SCT counter register"]
pub mod count;
#[doc = "STATE (rw) register accessor: SCT state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "SCT state register"]
pub mod state;
#[doc = "INPUT (r) register accessor: SCT input register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::InputSpec>;
#[doc = "SCT input register"]
pub mod input;
#[doc = "REGMODE (rw) register accessor: SCT match/capture mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`regmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regmode`]
module"]
#[doc(alias = "REGMODE")]
pub type Regmode = crate::Reg<regmode::RegmodeSpec>;
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "OUTPUT (rw) register accessor: SCT output register\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "OUTPUT")]
pub type Output = crate::Reg<output::OutputSpec>;
#[doc = "SCT output register"]
pub mod output;
#[doc = "OUTPUTDIRCTRL (rw) register accessor: SCT output counter direction control register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputdirctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputdirctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputdirctrl`]
module"]
#[doc(alias = "OUTPUTDIRCTRL")]
pub type Outputdirctrl = crate::Reg<outputdirctrl::OutputdirctrlSpec>;
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "RES (rw) register accessor: SCT conflict resolution register\n\nYou can [`read`](crate::Reg::read) this register and get [`res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
#[doc(alias = "RES")]
pub type Res = crate::Reg<res::ResSpec>;
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "DMAREQ0 (rw) register accessor: SCT DMA request 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq0`]
module"]
#[doc(alias = "DMAREQ0")]
pub type Dmareq0 = crate::Reg<dmareq0::Dmareq0Spec>;
#[doc = "SCT DMA request 0 register"]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: SCT DMA request 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq1`]
module"]
#[doc(alias = "DMAREQ1")]
pub type Dmareq1 = crate::Reg<dmareq1::Dmareq1Spec>;
#[doc = "SCT DMA request 1 register"]
pub mod dmareq1;
#[doc = "EVEN (rw) register accessor: SCT event interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`even::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`even::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@even`]
module"]
#[doc(alias = "EVEN")]
pub type Even = crate::Reg<even::EvenSpec>;
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "EVFLAG (rw) register accessor: SCT event flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`evflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflag`]
module"]
#[doc(alias = "EVFLAG")]
pub type Evflag = crate::Reg<evflag::EvflagSpec>;
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "CONEN (rw) register accessor: SCT conflict interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`conen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conen`]
module"]
#[doc(alias = "CONEN")]
pub type Conen = crate::Reg<conen::ConenSpec>;
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "CONFLAG (rw) register accessor: SCT conflict flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`conflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conflag`]
module"]
#[doc(alias = "CONFLAG")]
pub type Conflag = crate::Reg<conflag::ConflagSpec>;
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "CAP0 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap0`]
module"]
#[doc(alias = "CAP0")]
pub type Cap0 = crate::Reg<cap0::Cap0Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap0;
#[doc = "MATCH0 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match0`]
module"]
#[doc(alias = "MATCH0")]
pub type Match0 = crate::Reg<match0::Match0Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match0;
#[doc = "CAP1 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap1`]
module"]
#[doc(alias = "CAP1")]
pub type Cap1 = crate::Reg<cap1::Cap1Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap1;
#[doc = "MATCH1 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match1`]
module"]
#[doc(alias = "MATCH1")]
pub type Match1 = crate::Reg<match1::Match1Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match1;
#[doc = "CAP2 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap2`]
module"]
#[doc(alias = "CAP2")]
pub type Cap2 = crate::Reg<cap2::Cap2Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap2;
#[doc = "MATCH2 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match2`]
module"]
#[doc(alias = "MATCH2")]
pub type Match2 = crate::Reg<match2::Match2Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match2;
#[doc = "CAP3 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap3`]
module"]
#[doc(alias = "CAP3")]
pub type Cap3 = crate::Reg<cap3::Cap3Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap3;
#[doc = "MATCH3 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match3`]
module"]
#[doc(alias = "MATCH3")]
pub type Match3 = crate::Reg<match3::Match3Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match3;
#[doc = "CAP4 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap4`]
module"]
#[doc(alias = "CAP4")]
pub type Cap4 = crate::Reg<cap4::Cap4Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap4;
#[doc = "MATCH4 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match4`]
module"]
#[doc(alias = "MATCH4")]
pub type Match4 = crate::Reg<match4::Match4Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match4;
#[doc = "CAP5 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap5`]
module"]
#[doc(alias = "CAP5")]
pub type Cap5 = crate::Reg<cap5::Cap5Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap5;
#[doc = "MATCH5 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match5`]
module"]
#[doc(alias = "MATCH5")]
pub type Match5 = crate::Reg<match5::Match5Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match5;
#[doc = "CAP6 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap6`]
module"]
#[doc(alias = "CAP6")]
pub type Cap6 = crate::Reg<cap6::Cap6Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap6;
#[doc = "MATCH6 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match6`]
module"]
#[doc(alias = "MATCH6")]
pub type Match6 = crate::Reg<match6::Match6Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match6;
#[doc = "CAP7 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap7`]
module"]
#[doc(alias = "CAP7")]
pub type Cap7 = crate::Reg<cap7::Cap7Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap7;
#[doc = "MATCH7 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match7`]
module"]
#[doc(alias = "MATCH7")]
pub type Match7 = crate::Reg<match7::Match7Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match7;
#[doc = "CAP8 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap8`]
module"]
#[doc(alias = "CAP8")]
pub type Cap8 = crate::Reg<cap8::Cap8Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap8;
#[doc = "MATCH8 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match8`]
module"]
#[doc(alias = "MATCH8")]
pub type Match8 = crate::Reg<match8::Match8Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match8;
#[doc = "CAP9 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap9`]
module"]
#[doc(alias = "CAP9")]
pub type Cap9 = crate::Reg<cap9::Cap9Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap9;
#[doc = "MATCH9 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match9`]
module"]
#[doc(alias = "MATCH9")]
pub type Match9 = crate::Reg<match9::Match9Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match9;
#[doc = "CAP10 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap10`]
module"]
#[doc(alias = "CAP10")]
pub type Cap10 = crate::Reg<cap10::Cap10Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap10;
#[doc = "MATCH10 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match10`]
module"]
#[doc(alias = "MATCH10")]
pub type Match10 = crate::Reg<match10::Match10Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match10;
#[doc = "CAP11 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap11`]
module"]
#[doc(alias = "CAP11")]
pub type Cap11 = crate::Reg<cap11::Cap11Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap11;
#[doc = "MATCH11 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match11`]
module"]
#[doc(alias = "MATCH11")]
pub type Match11 = crate::Reg<match11::Match11Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match11;
#[doc = "CAP12 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap12`]
module"]
#[doc(alias = "CAP12")]
pub type Cap12 = crate::Reg<cap12::Cap12Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap12;
#[doc = "MATCH12 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match12`]
module"]
#[doc(alias = "MATCH12")]
pub type Match12 = crate::Reg<match12::Match12Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match12;
#[doc = "CAP13 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap13`]
module"]
#[doc(alias = "CAP13")]
pub type Cap13 = crate::Reg<cap13::Cap13Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap13;
#[doc = "MATCH13 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match13`]
module"]
#[doc(alias = "MATCH13")]
pub type Match13 = crate::Reg<match13::Match13Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match13;
#[doc = "CAP14 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap14`]
module"]
#[doc(alias = "CAP14")]
pub type Cap14 = crate::Reg<cap14::Cap14Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap14;
#[doc = "MATCH14 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match14`]
module"]
#[doc(alias = "MATCH14")]
pub type Match14 = crate::Reg<match14::Match14Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match14;
#[doc = "CAP15 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap15`]
module"]
#[doc(alias = "CAP15")]
pub type Cap15 = crate::Reg<cap15::Cap15Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap15;
#[doc = "MATCH15 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match15`]
module"]
#[doc(alias = "MATCH15")]
pub type Match15 = crate::Reg<match15::Match15Spec>;
#[doc = "SCT match value register of match channels"]
pub mod match15;
#[doc = "CAPCTRL0 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl0`]
module"]
#[doc(alias = "CAPCTRL0")]
pub type Capctrl0 = crate::Reg<capctrl0::Capctrl0Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl0;
#[doc = "MATCHREL0 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel0`]
module"]
#[doc(alias = "MATCHREL0")]
pub type Matchrel0 = crate::Reg<matchrel0::Matchrel0Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel0;
#[doc = "CAPCTRL1 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl1`]
module"]
#[doc(alias = "CAPCTRL1")]
pub type Capctrl1 = crate::Reg<capctrl1::Capctrl1Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl1;
#[doc = "MATCHREL1 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel1`]
module"]
#[doc(alias = "MATCHREL1")]
pub type Matchrel1 = crate::Reg<matchrel1::Matchrel1Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel1;
#[doc = "CAPCTRL2 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl2`]
module"]
#[doc(alias = "CAPCTRL2")]
pub type Capctrl2 = crate::Reg<capctrl2::Capctrl2Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl2;
#[doc = "MATCHREL2 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel2`]
module"]
#[doc(alias = "MATCHREL2")]
pub type Matchrel2 = crate::Reg<matchrel2::Matchrel2Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel2;
#[doc = "CAPCTRL3 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl3`]
module"]
#[doc(alias = "CAPCTRL3")]
pub type Capctrl3 = crate::Reg<capctrl3::Capctrl3Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl3;
#[doc = "MATCHREL3 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel3`]
module"]
#[doc(alias = "MATCHREL3")]
pub type Matchrel3 = crate::Reg<matchrel3::Matchrel3Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel3;
#[doc = "CAPCTRL4 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl4`]
module"]
#[doc(alias = "CAPCTRL4")]
pub type Capctrl4 = crate::Reg<capctrl4::Capctrl4Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl4;
#[doc = "MATCHREL4 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel4`]
module"]
#[doc(alias = "MATCHREL4")]
pub type Matchrel4 = crate::Reg<matchrel4::Matchrel4Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel4;
#[doc = "CAPCTRL5 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl5`]
module"]
#[doc(alias = "CAPCTRL5")]
pub type Capctrl5 = crate::Reg<capctrl5::Capctrl5Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl5;
#[doc = "MATCHREL5 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel5`]
module"]
#[doc(alias = "MATCHREL5")]
pub type Matchrel5 = crate::Reg<matchrel5::Matchrel5Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel5;
#[doc = "CAPCTRL6 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl6`]
module"]
#[doc(alias = "CAPCTRL6")]
pub type Capctrl6 = crate::Reg<capctrl6::Capctrl6Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl6;
#[doc = "MATCHREL6 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel6`]
module"]
#[doc(alias = "MATCHREL6")]
pub type Matchrel6 = crate::Reg<matchrel6::Matchrel6Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel6;
#[doc = "CAPCTRL7 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl7`]
module"]
#[doc(alias = "CAPCTRL7")]
pub type Capctrl7 = crate::Reg<capctrl7::Capctrl7Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl7;
#[doc = "MATCHREL7 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel7`]
module"]
#[doc(alias = "MATCHREL7")]
pub type Matchrel7 = crate::Reg<matchrel7::Matchrel7Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel7;
#[doc = "CAPCTRL8 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl8`]
module"]
#[doc(alias = "CAPCTRL8")]
pub type Capctrl8 = crate::Reg<capctrl8::Capctrl8Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl8;
#[doc = "MATCHREL8 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel8`]
module"]
#[doc(alias = "MATCHREL8")]
pub type Matchrel8 = crate::Reg<matchrel8::Matchrel8Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel8;
#[doc = "CAPCTRL9 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl9`]
module"]
#[doc(alias = "CAPCTRL9")]
pub type Capctrl9 = crate::Reg<capctrl9::Capctrl9Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl9;
#[doc = "MATCHREL9 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel9`]
module"]
#[doc(alias = "MATCHREL9")]
pub type Matchrel9 = crate::Reg<matchrel9::Matchrel9Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel9;
#[doc = "CAPCTRL10 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl10`]
module"]
#[doc(alias = "CAPCTRL10")]
pub type Capctrl10 = crate::Reg<capctrl10::Capctrl10Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl10;
#[doc = "MATCHREL10 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel10`]
module"]
#[doc(alias = "MATCHREL10")]
pub type Matchrel10 = crate::Reg<matchrel10::Matchrel10Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel10;
#[doc = "CAPCTRL11 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl11`]
module"]
#[doc(alias = "CAPCTRL11")]
pub type Capctrl11 = crate::Reg<capctrl11::Capctrl11Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl11;
#[doc = "MATCHREL11 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel11`]
module"]
#[doc(alias = "MATCHREL11")]
pub type Matchrel11 = crate::Reg<matchrel11::Matchrel11Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel11;
#[doc = "CAPCTRL12 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl12`]
module"]
#[doc(alias = "CAPCTRL12")]
pub type Capctrl12 = crate::Reg<capctrl12::Capctrl12Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl12;
#[doc = "MATCHREL12 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel12`]
module"]
#[doc(alias = "MATCHREL12")]
pub type Matchrel12 = crate::Reg<matchrel12::Matchrel12Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel12;
#[doc = "CAPCTRL13 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl13`]
module"]
#[doc(alias = "CAPCTRL13")]
pub type Capctrl13 = crate::Reg<capctrl13::Capctrl13Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl13;
#[doc = "MATCHREL13 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel13`]
module"]
#[doc(alias = "MATCHREL13")]
pub type Matchrel13 = crate::Reg<matchrel13::Matchrel13Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel13;
#[doc = "CAPCTRL14 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl14`]
module"]
#[doc(alias = "CAPCTRL14")]
pub type Capctrl14 = crate::Reg<capctrl14::Capctrl14Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl14;
#[doc = "MATCHREL14 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel14`]
module"]
#[doc(alias = "MATCHREL14")]
pub type Matchrel14 = crate::Reg<matchrel14::Matchrel14Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel14;
#[doc = "CAPCTRL15 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl15`]
module"]
#[doc(alias = "CAPCTRL15")]
pub type Capctrl15 = crate::Reg<capctrl15::Capctrl15Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl15;
#[doc = "MATCHREL15 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`matchrel15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matchrel15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matchrel15`]
module"]
#[doc(alias = "MATCHREL15")]
pub type Matchrel15 = crate::Reg<matchrel15::Matchrel15Spec>;
#[doc = "SCT match reload value register"]
pub mod matchrel15;
#[doc = "Array of registers: EV_STATE, EV_CTRL"]
pub use self::ev::Ev;
#[doc = r"Cluster"]
#[doc = "Array of registers: EV_STATE, EV_CTRL"]
pub mod ev;
#[doc = "Array of registers: OUT_SET, OUT_CLR"]
pub use self::out::Out;
#[doc = r"Cluster"]
#[doc = "Array of registers: OUT_SET, OUT_CLR"]
pub mod out;
