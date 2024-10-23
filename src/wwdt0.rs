#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mod_: Mod,
    tc: Tc,
    feed: Feed,
    tv: Tv,
    _reserved4: [u8; 0x04],
    warnint: Warnint,
    window: Window,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
    #[doc = "0x04 - Watchdog timer constant register. This 24-bit register determines the time-out value."]
    #[inline(always)]
    pub const fn tc(&self) -> &Tc {
        &self.tc
    }
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
    #[inline(always)]
    pub const fn feed(&self) -> &Feed {
        &self.feed
    }
    #[doc = "0x0c - Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    #[inline(always)]
    pub const fn tv(&self) -> &Tv {
        &self.tv
    }
    #[doc = "0x14 - Watchdog Warning Interrupt compare value."]
    #[inline(always)]
    pub const fn warnint(&self) -> &Warnint {
        &self.warnint
    }
    #[doc = "0x18 - Watchdog Window compare value."]
    #[inline(always)]
    pub const fn window(&self) -> &Window {
        &self.window
    }
}
#[doc = "MOD (rw) register accessor: Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "TC (rw) register accessor: Watchdog timer constant register. This 24-bit register determines the time-out value.\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`]
module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
pub mod tc;
#[doc = "FEED (w) register accessor: Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feed`]
module"]
#[doc(alias = "FEED")]
pub type Feed = crate::Reg<feed::FeedSpec>;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
pub mod feed;
#[doc = "TV (r) register accessor: Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`tv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv`]
module"]
#[doc(alias = "TV")]
pub type Tv = crate::Reg<tv::TvSpec>;
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "WARNINT (rw) register accessor: Watchdog Warning Interrupt compare value.\n\nYou can [`read`](crate::Reg::read) this register and get [`warnint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warnint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@warnint`]
module"]
#[doc(alias = "WARNINT")]
pub type Warnint = crate::Reg<warnint::WarnintSpec>;
#[doc = "Watchdog Warning Interrupt compare value."]
pub mod warnint;
#[doc = "WINDOW (rw) register accessor: Watchdog Window compare value.\n\nYou can [`read`](crate::Reg::read) this register and get [`window::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`window::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@window`]
module"]
#[doc(alias = "WINDOW")]
pub type Window = crate::Reg<window::WindowSpec>;
#[doc = "Watchdog Window compare value."]
pub mod window;
