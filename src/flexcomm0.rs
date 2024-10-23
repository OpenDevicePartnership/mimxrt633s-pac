#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0ff8],
    pselid: Pselid,
    pid: Pid,
}
impl RegisterBlock {
    #[doc = "0xff8 - Peripheral Select and Flexcomm ID register."]
    #[inline(always)]
    pub const fn pselid(&self) -> &Pselid {
        &self.pselid
    }
    #[doc = "0xffc - Peripheral identification register."]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
}
#[doc = "PSELID (rw) register accessor: Peripheral Select and Flexcomm ID register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselid`]
module"]
#[doc(alias = "PSELID")]
pub type Pselid = crate::Reg<pselid::PselidSpec>;
#[doc = "Peripheral Select and Flexcomm ID register."]
pub mod pselid;
#[doc = "PID (rw) register accessor: Peripheral identification register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "Peripheral identification register."]
pub mod pid;
