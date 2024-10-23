#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: EV_STATE, EV_CTRL"]
#[doc(alias = "EV")]
pub struct Ev {
    ev_state: EvState,
    ev_ctrl: EvCtrl,
}
impl Ev {
    #[doc = "0x00 - SCT event state register 0"]
    #[inline(always)]
    pub const fn ev_state(&self) -> &EvState {
        &self.ev_state
    }
    #[doc = "0x04 - SCT event control register 0"]
    #[inline(always)]
    pub const fn ev_ctrl(&self) -> &EvCtrl {
        &self.ev_ctrl
    }
}
#[doc = "EV_STATE (rw) register accessor: SCT event state register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_state`]
module"]
#[doc(alias = "EV_STATE")]
pub type EvState = crate::Reg<ev_state::EvStateSpec>;
#[doc = "SCT event state register 0"]
pub mod ev_state;
#[doc = "EV_CTRL (rw) register accessor: SCT event control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_ctrl`]
module"]
#[doc(alias = "EV_CTRL")]
pub type EvCtrl = crate::Reg<ev_ctrl::EvCtrlSpec>;
#[doc = "SCT event control register 0"]
pub mod ev_ctrl;
