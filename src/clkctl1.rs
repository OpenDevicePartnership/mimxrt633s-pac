#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pscctl0: Pscctl0,
    pscctl1: Pscctl1,
    pscctl2: Pscctl2,
    _reserved3: [u8; 0x24],
    pscctl0_set: Pscctl0Set,
    pscctl1_set: Pscctl1Set,
    pscctl2_set: Pscctl2Set,
    _reserved6: [u8; 0x24],
    pscctl0_clr: Pscctl0Clr,
    pscctl1_clr: Pscctl1Clr,
    pscctl2_clr: Pscctl2Clr,
    _reserved9: [u8; 0x0184],
    audiopll0clksel: Audiopll0clksel,
    audiopll0ctl0: Audiopll0ctl0,
    _reserved11: [u8; 0x04],
    audiopll0locktimediv2: Audiopll0locktimediv2,
    audiopll0num: Audiopll0num,
    audiopll0denom: Audiopll0denom,
    audiopll0pfd: Audiopll0pfd,
    _reserved15: [u8; 0x24],
    audiopllclkdiv: Audiopllclkdiv,
    _reserved16: [u8; 0x01bc],
    dspcpuclkdiv: Dspcpuclkdiv,
    dspmainramclkdiv: Dspmainramclkdiv,
    _reserved18: [u8; 0x28],
    dspcpuclksela: Dspcpuclksela,
    dspcpuclkselb: Dspcpuclkselb,
    _reserved20: [u8; 0x48],
    oseventfclksel: Oseventfclksel,
    _reserved21: [u8; 0x7c],
    flexcomm: (),
    _reserved22: [u8; 0x01c0],
    frg14clksel: Frg14clksel,
    frg14ctl: Frg14ctl,
    fc14fclksel: Fc14fclksel,
    _reserved25: [u8; 0x14],
    frg15clksel: Frg15clksel,
    frg15ctl: Frg15ctl,
    fc15fclksel: Fc15fclksel,
    _reserved28: [u8; 0x10],
    frgpllclkdiv: Frgpllclkdiv,
    dmic0fclksel: Dmic0fclksel,
    dmic0fclkdiv: Dmic0fclkdiv,
    _reserved31: [u8; 0x18],
    ct32bitfclksel: [Ct32bitfclksel; 5],
    _reserved32: [u8; 0x0c],
    audiomclksel: Audiomclksel,
    audiomclkdiv: Audiomclkdiv,
    _reserved34: [u8; 0x18],
    clkoutsel0: Clkoutsel0,
    clkoutsel1: Clkoutsel1,
    clkoutdiv: Clkoutdiv,
    _reserved37: [u8; 0x14],
    i3c0fclksel: I3c0fclksel,
    i3c0fclkstcsel: I3c0fclkstcsel,
    i3c0fclkstcdiv: I3c0fclkstcdiv,
    i3c0fclksdiv: I3c0fclksdiv,
    i3c0fclkdiv: I3c0fclkdiv,
    _reserved42: [u8; 0x0c],
    wdt1fclksel: Wdt1fclksel,
    _reserved43: [u8; 0x1c],
    acmp0fclksel: Acmp0fclksel,
    acmp0fclkdiv: Acmp0fclkdiv,
}
impl RegisterBlock {
    #[doc = "0x10 - clock control register 0"]
    #[inline(always)]
    pub const fn pscctl0(&self) -> &Pscctl0 {
        &self.pscctl0
    }
    #[doc = "0x14 - clock control register 1"]
    #[inline(always)]
    pub const fn pscctl1(&self) -> &Pscctl1 {
        &self.pscctl1
    }
    #[doc = "0x18 - clock control register 2"]
    #[inline(always)]
    pub const fn pscctl2(&self) -> &Pscctl2 {
        &self.pscctl2
    }
    #[doc = "0x40 - clock set register 0"]
    #[inline(always)]
    pub const fn pscctl0_set(&self) -> &Pscctl0Set {
        &self.pscctl0_set
    }
    #[doc = "0x44 - clock set register 1"]
    #[inline(always)]
    pub const fn pscctl1_set(&self) -> &Pscctl1Set {
        &self.pscctl1_set
    }
    #[doc = "0x48 - clock set register 2"]
    #[inline(always)]
    pub const fn pscctl2_set(&self) -> &Pscctl2Set {
        &self.pscctl2_set
    }
    #[doc = "0x70 - clock clear register 0"]
    #[inline(always)]
    pub const fn pscctl0_clr(&self) -> &Pscctl0Clr {
        &self.pscctl0_clr
    }
    #[doc = "0x74 - clock clear register 1"]
    #[inline(always)]
    pub const fn pscctl1_clr(&self) -> &Pscctl1Clr {
        &self.pscctl1_clr
    }
    #[doc = "0x78 - clock clear register 2"]
    #[inline(always)]
    pub const fn pscctl2_clr(&self) -> &Pscctl2Clr {
        &self.pscctl2_clr
    }
    #[doc = "0x200 - audio pll0 clock selection"]
    #[inline(always)]
    pub const fn audiopll0clksel(&self) -> &Audiopll0clksel {
        &self.audiopll0clksel
    }
    #[doc = "0x204 - audio pll0 control0"]
    #[inline(always)]
    pub const fn audiopll0ctl0(&self) -> &Audiopll0ctl0 {
        &self.audiopll0ctl0
    }
    #[doc = "0x20c - audio pll0 lock time"]
    #[inline(always)]
    pub const fn audiopll0locktimediv2(&self) -> &Audiopll0locktimediv2 {
        &self.audiopll0locktimediv2
    }
    #[doc = "0x210 - audio pll0 number"]
    #[inline(always)]
    pub const fn audiopll0num(&self) -> &Audiopll0num {
        &self.audiopll0num
    }
    #[doc = "0x214 - Audio pll0 denom"]
    #[inline(always)]
    pub const fn audiopll0denom(&self) -> &Audiopll0denom {
        &self.audiopll0denom
    }
    #[doc = "0x218 - audio pll0 PFD"]
    #[inline(always)]
    pub const fn audiopll0pfd(&self) -> &Audiopll0pfd {
        &self.audiopll0pfd
    }
    #[doc = "0x240 - audio pll0 clock divider"]
    #[inline(always)]
    pub const fn audiopllclkdiv(&self) -> &Audiopllclkdiv {
        &self.audiopllclkdiv
    }
    #[doc = "0x400 - DSP cpu clock divider"]
    #[inline(always)]
    pub const fn dspcpuclkdiv(&self) -> &Dspcpuclkdiv {
        &self.dspcpuclkdiv
    }
    #[doc = "0x404 - DSP main ram clock divider"]
    #[inline(always)]
    pub const fn dspmainramclkdiv(&self) -> &Dspmainramclkdiv {
        &self.dspmainramclkdiv
    }
    #[doc = "0x430 - DSP clock selection A"]
    #[inline(always)]
    pub const fn dspcpuclksela(&self) -> &Dspcpuclksela {
        &self.dspcpuclksela
    }
    #[doc = "0x434 - DSP clock selection B"]
    #[inline(always)]
    pub const fn dspcpuclkselb(&self) -> &Dspcpuclkselb {
        &self.dspcpuclkselb
    }
    #[doc = "0x480 - OS EVENT clock selection"]
    #[inline(always)]
    pub const fn oseventfclksel(&self) -> &Oseventfclksel {
        &self.oseventfclksel
    }
    #[doc = "0x500..0x560 - flexcomm clock controller"]
    #[inline(always)]
    pub const fn flexcomm(&self, n: usize) -> &Flexcomm {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x560 - flexcomm clock controller"]
    #[inline(always)]
    pub fn flexcomm_iter(&self) -> impl Iterator<Item = &Flexcomm> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x6c0 - FRG clock selection register 14"]
    #[inline(always)]
    pub const fn frg14clksel(&self) -> &Frg14clksel {
        &self.frg14clksel
    }
    #[doc = "0x6c4 - FRG clock controller 14"]
    #[inline(always)]
    pub const fn frg14ctl(&self) -> &Frg14ctl {
        &self.frg14ctl
    }
    #[doc = "0x6c8 - flexcomm14 clock selection"]
    #[inline(always)]
    pub const fn fc14fclksel(&self) -> &Fc14fclksel {
        &self.fc14fclksel
    }
    #[doc = "0x6e0 - FRG clock selection register 15"]
    #[inline(always)]
    pub const fn frg15clksel(&self) -> &Frg15clksel {
        &self.frg15clksel
    }
    #[doc = "0x6e4 - FRG clock controller 15"]
    #[inline(always)]
    pub const fn frg15ctl(&self) -> &Frg15ctl {
        &self.frg15ctl
    }
    #[doc = "0x6e8 - flexcomm15 clock selection"]
    #[inline(always)]
    pub const fn fc15fclksel(&self) -> &Fc15fclksel {
        &self.fc15fclksel
    }
    #[doc = "0x6fc - FRG pll clock divider"]
    #[inline(always)]
    pub const fn frgpllclkdiv(&self) -> &Frgpllclkdiv {
        &self.frgpllclkdiv
    }
    #[doc = "0x700 - DMIC0 clk selection"]
    #[inline(always)]
    pub const fn dmic0fclksel(&self) -> &Dmic0fclksel {
        &self.dmic0fclksel
    }
    #[doc = "0x704 - DMIC clock clock divider"]
    #[inline(always)]
    pub const fn dmic0fclkdiv(&self) -> &Dmic0fclkdiv {
        &self.dmic0fclkdiv
    }
    #[doc = "0x720..0x734 - ct32bit timer N clock selection"]
    #[inline(always)]
    pub const fn ct32bitfclksel(&self, n: usize) -> &Ct32bitfclksel {
        &self.ct32bitfclksel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x720..0x734 - ct32bit timer N clock selection"]
    #[inline(always)]
    pub fn ct32bitfclksel_iter(&self) -> impl Iterator<Item = &Ct32bitfclksel> {
        self.ct32bitfclksel.iter()
    }
    #[doc = "0x740 - audio mclock selection"]
    #[inline(always)]
    pub const fn audiomclksel(&self) -> &Audiomclksel {
        &self.audiomclksel
    }
    #[doc = "0x744 - audio mclock divider"]
    #[inline(always)]
    pub const fn audiomclkdiv(&self) -> &Audiomclkdiv {
        &self.audiomclkdiv
    }
    #[doc = "0x760 - clock out selection 0"]
    #[inline(always)]
    pub const fn clkoutsel0(&self) -> &Clkoutsel0 {
        &self.clkoutsel0
    }
    #[doc = "0x764 - clock out selection 1"]
    #[inline(always)]
    pub const fn clkoutsel1(&self) -> &Clkoutsel1 {
        &self.clkoutsel1
    }
    #[doc = "0x768 - clock_out divider"]
    #[inline(always)]
    pub const fn clkoutdiv(&self) -> &Clkoutdiv {
        &self.clkoutdiv
    }
    #[doc = "0x780 - I3C0 fclk selection"]
    #[inline(always)]
    pub const fn i3c0fclksel(&self) -> &I3c0fclksel {
        &self.i3c0fclksel
    }
    #[doc = "0x784 - I3C0 fclk STC selection"]
    #[inline(always)]
    pub const fn i3c0fclkstcsel(&self) -> &I3c0fclkstcsel {
        &self.i3c0fclkstcsel
    }
    #[doc = "0x788 - I3C0 fclk STC divider"]
    #[inline(always)]
    pub const fn i3c0fclkstcdiv(&self) -> &I3c0fclkstcdiv {
        &self.i3c0fclkstcdiv
    }
    #[doc = "0x78c - I3C0 fclks divider"]
    #[inline(always)]
    pub const fn i3c0fclksdiv(&self) -> &I3c0fclksdiv {
        &self.i3c0fclksdiv
    }
    #[doc = "0x790 - I3C0 fclk divider"]
    #[inline(always)]
    pub const fn i3c0fclkdiv(&self) -> &I3c0fclkdiv {
        &self.i3c0fclkdiv
    }
    #[doc = "0x7a0 - WDT1 clock selection"]
    #[inline(always)]
    pub const fn wdt1fclksel(&self) -> &Wdt1fclksel {
        &self.wdt1fclksel
    }
    #[doc = "0x7c0 - acomparator 0 clock selection"]
    #[inline(always)]
    pub const fn acmp0fclksel(&self) -> &Acmp0fclksel {
        &self.acmp0fclksel
    }
    #[doc = "0x7c4 - acomparator 0 fclk divider"]
    #[inline(always)]
    pub const fn acmp0fclkdiv(&self) -> &Acmp0fclkdiv {
        &self.acmp0fclkdiv
    }
}
#[doc = "PSCCTL0 (rw) register accessor: clock control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl0`]
module"]
#[doc(alias = "PSCCTL0")]
pub type Pscctl0 = crate::Reg<pscctl0::Pscctl0Spec>;
#[doc = "clock control register 0"]
pub mod pscctl0;
#[doc = "PSCCTL1 (rw) register accessor: clock control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl1`]
module"]
#[doc(alias = "PSCCTL1")]
pub type Pscctl1 = crate::Reg<pscctl1::Pscctl1Spec>;
#[doc = "clock control register 1"]
pub mod pscctl1;
#[doc = "PSCCTL2 (rw) register accessor: clock control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl2`]
module"]
#[doc(alias = "PSCCTL2")]
pub type Pscctl2 = crate::Reg<pscctl2::Pscctl2Spec>;
#[doc = "clock control register 2"]
pub mod pscctl2;
#[doc = "PSCCTL0_SET (w) register accessor: clock set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl0_set`]
module"]
#[doc(alias = "PSCCTL0_SET")]
pub type Pscctl0Set = crate::Reg<pscctl0_set::Pscctl0SetSpec>;
#[doc = "clock set register 0"]
pub mod pscctl0_set;
#[doc = "PSCCTL1_SET (w) register accessor: clock set register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl1_set`]
module"]
#[doc(alias = "PSCCTL1_SET")]
pub type Pscctl1Set = crate::Reg<pscctl1_set::Pscctl1SetSpec>;
#[doc = "clock set register 1"]
pub mod pscctl1_set;
#[doc = "PSCCTL2_SET (w) register accessor: clock set register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl2_set`]
module"]
#[doc(alias = "PSCCTL2_SET")]
pub type Pscctl2Set = crate::Reg<pscctl2_set::Pscctl2SetSpec>;
#[doc = "clock set register 2"]
pub mod pscctl2_set;
#[doc = "PSCCTL0_CLR (w) register accessor: clock clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl0_clr`]
module"]
#[doc(alias = "PSCCTL0_CLR")]
pub type Pscctl0Clr = crate::Reg<pscctl0_clr::Pscctl0ClrSpec>;
#[doc = "clock clear register 0"]
pub mod pscctl0_clr;
#[doc = "PSCCTL1_CLR (w) register accessor: clock clear register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl1_clr`]
module"]
#[doc(alias = "PSCCTL1_CLR")]
pub type Pscctl1Clr = crate::Reg<pscctl1_clr::Pscctl1ClrSpec>;
#[doc = "clock clear register 1"]
pub mod pscctl1_clr;
#[doc = "PSCCTL2_CLR (w) register accessor: clock clear register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscctl2_clr`]
module"]
#[doc(alias = "PSCCTL2_CLR")]
pub type Pscctl2Clr = crate::Reg<pscctl2_clr::Pscctl2ClrSpec>;
#[doc = "clock clear register 2"]
pub mod pscctl2_clr;
#[doc = "AUDIOPLL0CLKSEL (rw) register accessor: audio pll0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0clksel`]
module"]
#[doc(alias = "AUDIOPLL0CLKSEL")]
pub type Audiopll0clksel = crate::Reg<audiopll0clksel::Audiopll0clkselSpec>;
#[doc = "audio pll0 clock selection"]
pub mod audiopll0clksel;
#[doc = "AUDIOPLL0CTL0 (rw) register accessor: audio pll0 control0\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0ctl0`]
module"]
#[doc(alias = "AUDIOPLL0CTL0")]
pub type Audiopll0ctl0 = crate::Reg<audiopll0ctl0::Audiopll0ctl0Spec>;
#[doc = "audio pll0 control0"]
pub mod audiopll0ctl0;
#[doc = "AUDIOPLL0LOCKTIMEDIV2 (rw) register accessor: audio pll0 lock time\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0locktimediv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0locktimediv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0locktimediv2`]
module"]
#[doc(alias = "AUDIOPLL0LOCKTIMEDIV2")]
pub type Audiopll0locktimediv2 = crate::Reg<audiopll0locktimediv2::Audiopll0locktimediv2Spec>;
#[doc = "audio pll0 lock time"]
pub mod audiopll0locktimediv2;
#[doc = "AUDIOPLL0NUM (rw) register accessor: audio pll0 number\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0num`]
module"]
#[doc(alias = "AUDIOPLL0NUM")]
pub type Audiopll0num = crate::Reg<audiopll0num::Audiopll0numSpec>;
#[doc = "audio pll0 number"]
pub mod audiopll0num;
#[doc = "AUDIOPLL0DENOM (rw) register accessor: Audio pll0 denom\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0denom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0denom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0denom`]
module"]
#[doc(alias = "AUDIOPLL0DENOM")]
pub type Audiopll0denom = crate::Reg<audiopll0denom::Audiopll0denomSpec>;
#[doc = "Audio pll0 denom"]
pub mod audiopll0denom;
#[doc = "AUDIOPLL0PFD (rw) register accessor: audio pll0 PFD\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0pfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0pfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopll0pfd`]
module"]
#[doc(alias = "AUDIOPLL0PFD")]
pub type Audiopll0pfd = crate::Reg<audiopll0pfd::Audiopll0pfdSpec>;
#[doc = "audio pll0 PFD"]
pub mod audiopll0pfd;
#[doc = "AUDIOPLLCLKDIV (rw) register accessor: audio pll0 clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiopllclkdiv`]
module"]
#[doc(alias = "AUDIOPLLCLKDIV")]
pub type Audiopllclkdiv = crate::Reg<audiopllclkdiv::AudiopllclkdivSpec>;
#[doc = "audio pll0 clock divider"]
pub mod audiopllclkdiv;
#[doc = "DSPCPUCLKDIV (rw) register accessor: DSP cpu clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`dspcpuclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspcpuclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspcpuclkdiv`]
module"]
#[doc(alias = "DSPCPUCLKDIV")]
pub type Dspcpuclkdiv = crate::Reg<dspcpuclkdiv::DspcpuclkdivSpec>;
#[doc = "DSP cpu clock divider"]
pub mod dspcpuclkdiv;
#[doc = "DSPMAINRAMCLKDIV (rw) register accessor: DSP main ram clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`dspmainramclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspmainramclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspmainramclkdiv`]
module"]
#[doc(alias = "DSPMAINRAMCLKDIV")]
pub type Dspmainramclkdiv = crate::Reg<dspmainramclkdiv::DspmainramclkdivSpec>;
#[doc = "DSP main ram clock divider"]
pub mod dspmainramclkdiv;
#[doc = "DSPCPUCLKSELA (rw) register accessor: DSP clock selection A\n\nYou can [`read`](crate::Reg::read) this register and get [`dspcpuclksela::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspcpuclksela::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspcpuclksela`]
module"]
#[doc(alias = "DSPCPUCLKSELA")]
pub type Dspcpuclksela = crate::Reg<dspcpuclksela::DspcpuclkselaSpec>;
#[doc = "DSP clock selection A"]
pub mod dspcpuclksela;
#[doc = "DSPCPUCLKSELB (rw) register accessor: DSP clock selection B\n\nYou can [`read`](crate::Reg::read) this register and get [`dspcpuclkselb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspcpuclkselb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspcpuclkselb`]
module"]
#[doc(alias = "DSPCPUCLKSELB")]
pub type Dspcpuclkselb = crate::Reg<dspcpuclkselb::DspcpuclkselbSpec>;
#[doc = "DSP clock selection B"]
pub mod dspcpuclkselb;
#[doc = "OSEVENTFCLKSEL (rw) register accessor: OS EVENT clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`oseventfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oseventfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oseventfclksel`]
module"]
#[doc(alias = "OSEVENTFCLKSEL")]
pub type Oseventfclksel = crate::Reg<oseventfclksel::OseventfclkselSpec>;
#[doc = "OS EVENT clock selection"]
pub mod oseventfclksel;
#[doc = "flexcomm clock controller"]
pub use self::flexcomm::Flexcomm;
#[doc = r"Cluster"]
#[doc = "flexcomm clock controller"]
pub mod flexcomm;
#[doc = "FRG14CLKSEL (rw) register accessor: FRG clock selection register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`frg14clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frg14clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frg14clksel`]
module"]
#[doc(alias = "FRG14CLKSEL")]
pub type Frg14clksel = crate::Reg<frg14clksel::Frg14clkselSpec>;
#[doc = "FRG clock selection register 14"]
pub mod frg14clksel;
#[doc = "FRG14CTL (rw) register accessor: FRG clock controller 14\n\nYou can [`read`](crate::Reg::read) this register and get [`frg14ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frg14ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frg14ctl`]
module"]
#[doc(alias = "FRG14CTL")]
pub type Frg14ctl = crate::Reg<frg14ctl::Frg14ctlSpec>;
#[doc = "FRG clock controller 14"]
pub mod frg14ctl;
#[doc = "FC14FCLKSEL (rw) register accessor: flexcomm14 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fc14fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc14fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc14fclksel`]
module"]
#[doc(alias = "FC14FCLKSEL")]
pub type Fc14fclksel = crate::Reg<fc14fclksel::Fc14fclkselSpec>;
#[doc = "flexcomm14 clock selection"]
pub mod fc14fclksel;
#[doc = "FRG15CLKSEL (rw) register accessor: FRG clock selection register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`frg15clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frg15clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frg15clksel`]
module"]
#[doc(alias = "FRG15CLKSEL")]
pub type Frg15clksel = crate::Reg<frg15clksel::Frg15clkselSpec>;
#[doc = "FRG clock selection register 15"]
pub mod frg15clksel;
#[doc = "FRG15CTL (rw) register accessor: FRG clock controller 15\n\nYou can [`read`](crate::Reg::read) this register and get [`frg15ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frg15ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frg15ctl`]
module"]
#[doc(alias = "FRG15CTL")]
pub type Frg15ctl = crate::Reg<frg15ctl::Frg15ctlSpec>;
#[doc = "FRG clock controller 15"]
pub mod frg15ctl;
#[doc = "FC15FCLKSEL (rw) register accessor: flexcomm15 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fc15fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc15fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc15fclksel`]
module"]
#[doc(alias = "FC15FCLKSEL")]
pub type Fc15fclksel = crate::Reg<fc15fclksel::Fc15fclkselSpec>;
#[doc = "flexcomm15 clock selection"]
pub mod fc15fclksel;
#[doc = "FRGPLLCLKDIV (rw) register accessor: FRG pll clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`frgpllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frgpllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frgpllclkdiv`]
module"]
#[doc(alias = "FRGPLLCLKDIV")]
pub type Frgpllclkdiv = crate::Reg<frgpllclkdiv::FrgpllclkdivSpec>;
#[doc = "FRG pll clock divider"]
pub mod frgpllclkdiv;
#[doc = "DMIC0FCLKSEL (rw) register accessor: DMIC0 clk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dmic0fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmic0fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic0fclksel`]
module"]
#[doc(alias = "DMIC0FCLKSEL")]
pub type Dmic0fclksel = crate::Reg<dmic0fclksel::Dmic0fclkselSpec>;
#[doc = "DMIC0 clk selection"]
pub mod dmic0fclksel;
#[doc = "DMIC0FCLKDIV (rw) register accessor: DMIC clock clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`dmic0fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmic0fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic0fclkdiv`]
module"]
#[doc(alias = "DMIC0FCLKDIV")]
pub type Dmic0fclkdiv = crate::Reg<dmic0fclkdiv::Dmic0fclkdivSpec>;
#[doc = "DMIC clock clock divider"]
pub mod dmic0fclkdiv;
#[doc = "CT32BITFCLKSEL (rw) register accessor: ct32bit timer N clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`ct32bitfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ct32bitfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct32bitfclksel`]
module"]
#[doc(alias = "CT32BITFCLKSEL")]
pub type Ct32bitfclksel = crate::Reg<ct32bitfclksel::Ct32bitfclkselSpec>;
#[doc = "ct32bit timer N clock selection"]
pub mod ct32bitfclksel;
#[doc = "AUDIOMCLKSEL (rw) register accessor: audio mclock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`audiomclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiomclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiomclksel`]
module"]
#[doc(alias = "AUDIOMCLKSEL")]
pub type Audiomclksel = crate::Reg<audiomclksel::AudiomclkselSpec>;
#[doc = "audio mclock selection"]
pub mod audiomclksel;
#[doc = "AUDIOMCLKDIV (rw) register accessor: audio mclock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`audiomclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiomclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audiomclkdiv`]
module"]
#[doc(alias = "AUDIOMCLKDIV")]
pub type Audiomclkdiv = crate::Reg<audiomclkdiv::AudiomclkdivSpec>;
#[doc = "audio mclock divider"]
pub mod audiomclkdiv;
#[doc = "CLKOUTSEL0 (rw) register accessor: clock out selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutsel0`]
module"]
#[doc(alias = "CLKOUTSEL0")]
pub type Clkoutsel0 = crate::Reg<clkoutsel0::Clkoutsel0Spec>;
#[doc = "clock out selection 0"]
pub mod clkoutsel0;
#[doc = "CLKOUTSEL1 (rw) register accessor: clock out selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutsel1`]
module"]
#[doc(alias = "CLKOUTSEL1")]
pub type Clkoutsel1 = crate::Reg<clkoutsel1::Clkoutsel1Spec>;
#[doc = "clock out selection 1"]
pub mod clkoutsel1;
#[doc = "CLKOUTDIV (rw) register accessor: clock_out divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutdiv`]
module"]
#[doc(alias = "CLKOUTDIV")]
pub type Clkoutdiv = crate::Reg<clkoutdiv::ClkoutdivSpec>;
#[doc = "clock_out divider"]
pub mod clkoutdiv;
#[doc = "I3C0FCLKSEL (rw) register accessor: I3C0 fclk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c0fclksel`]
module"]
#[doc(alias = "I3C0FCLKSEL")]
pub type I3c0fclksel = crate::Reg<i3c0fclksel::I3c0fclkselSpec>;
#[doc = "I3C0 fclk selection"]
pub mod i3c0fclksel;
#[doc = "I3C0FCLKSTCSEL (rw) register accessor: I3C0 fclk STC selection\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclkstcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclkstcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c0fclkstcsel`]
module"]
#[doc(alias = "I3C0FCLKSTCSEL")]
pub type I3c0fclkstcsel = crate::Reg<i3c0fclkstcsel::I3c0fclkstcselSpec>;
#[doc = "I3C0 fclk STC selection"]
pub mod i3c0fclkstcsel;
#[doc = "I3C0FCLKSTCDIV (rw) register accessor: I3C0 fclk STC divider\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclkstcdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclkstcdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c0fclkstcdiv`]
module"]
#[doc(alias = "I3C0FCLKSTCDIV")]
pub type I3c0fclkstcdiv = crate::Reg<i3c0fclkstcdiv::I3c0fclkstcdivSpec>;
#[doc = "I3C0 fclk STC divider"]
pub mod i3c0fclkstcdiv;
#[doc = "I3C0FCLKSDIV (rw) register accessor: I3C0 fclks divider\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclksdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclksdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c0fclksdiv`]
module"]
#[doc(alias = "I3C0FCLKSDIV")]
pub type I3c0fclksdiv = crate::Reg<i3c0fclksdiv::I3c0fclksdivSpec>;
#[doc = "I3C0 fclks divider"]
pub mod i3c0fclksdiv;
#[doc = "I3C0FCLKDIV (rw) register accessor: I3C0 fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c0fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c0fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c0fclkdiv`]
module"]
#[doc(alias = "I3C0FCLKDIV")]
pub type I3c0fclkdiv = crate::Reg<i3c0fclkdiv::I3c0fclkdivSpec>;
#[doc = "I3C0 fclk divider"]
pub mod i3c0fclkdiv;
#[doc = "WDT1FCLKSEL (rw) register accessor: WDT1 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt1fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt1fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt1fclksel`]
module"]
#[doc(alias = "WDT1FCLKSEL")]
pub type Wdt1fclksel = crate::Reg<wdt1fclksel::Wdt1fclkselSpec>;
#[doc = "WDT1 clock selection"]
pub mod wdt1fclksel;
#[doc = "ACMP0FCLKSEL (rw) register accessor: acomparator 0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0fclksel`]
module"]
#[doc(alias = "ACMP0FCLKSEL")]
pub type Acmp0fclksel = crate::Reg<acmp0fclksel::Acmp0fclkselSpec>;
#[doc = "acomparator 0 clock selection"]
pub mod acmp0fclksel;
#[doc = "ACMP0FCLKDIV (rw) register accessor: acomparator 0 fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0fclkdiv`]
module"]
#[doc(alias = "ACMP0FCLKDIV")]
pub type Acmp0fclkdiv = crate::Reg<acmp0fclkdiv::Acmp0fclkdivSpec>;
#[doc = "acomparator 0 fclk divider"]
pub mod acmp0fclkdiv;
