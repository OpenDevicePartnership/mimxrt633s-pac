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
    _reserved9: [u8; 0x84],
    ffroctl0: Ffroctl0,
    ffroctl1: Ffroctl1,
    _reserved11: [u8; 0x58],
    sysoscctl0: Sysoscctl0,
    _reserved12: [u8; 0x04],
    sysoscbypass: Sysoscbypass,
    _reserved13: [u8; 0x24],
    lposcctl0: Lposcctl0,
    _reserved14: [u8; 0x2c],
    osc32khzctl0: Osc32khzctl0,
    _reserved15: [u8; 0x3c],
    syspll0clksel: Syspll0clksel,
    syspll0ctl0: Syspll0ctl0,
    _reserved17: [u8; 0x04],
    syspll0locktimediv2: Syspll0locktimediv2,
    syspll0num: Syspll0num,
    syspll0denom: Syspll0denom,
    syspll0pfd: Syspll0pfd,
    _reserved21: [u8; 0x24],
    mainpllclkdiv: Mainpllclkdiv,
    dsppllclkdiv: Dsppllclkdiv,
    aux0pllclkdiv: Aux0pllclkdiv,
    aux1pllclkdiv: Aux1pllclkdiv,
    _reserved25: [u8; 0x01b0],
    syscpuahbclkdiv: Syscpuahbclkdiv,
    _reserved26: [u8; 0x2c],
    mainclksela: Mainclksela,
    mainclkselb: Mainclkselb,
    _reserved28: [u8; 0xc8],
    pfcdiv: [Pfcdiv; 2],
    _reserved29: [u8; 0x0118],
    flexspifclksel: Flexspifclksel,
    flexspifclkdiv: Flexspifclkdiv,
    _reserved31: [u8; 0x18],
    sctfclksel: Sctfclksel,
    sctfclkdiv: Sctfclkdiv,
    _reserved33: [u8; 0x18],
    usbhsfclksel: Usbhsfclksel,
    usbhsfclkdiv: Usbhsfclkdiv,
    _reserved35: [u8; 0x18],
    sdio0fclksel: Sdio0fclksel,
    sdio0fclkdiv: Sdio0fclkdiv,
    _reserved37: [u8; 0x08],
    sdio1fclksel: Sdio1fclksel,
    sdio1fclkdiv: Sdio1fclkdiv,
    _reserved39: [u8; 0x08],
    espiclksel: Espiclksel,
    _reserved40: [u8; 0x2c],
    adc0fclksel0: Adc0fclksel0,
    adc0fclksel1: Adc0fclksel1,
    adc0fclkdiv: Adc0fclkdiv,
    _reserved43: [u8; 0x24],
    utickfclksel: Utickfclksel,
    _reserved44: [u8; 0x1c],
    wdt0fclksel: Wdt0fclksel,
    _reserved45: [u8; 0x0c],
    wakeclk32khzsel: Wakeclk32khzsel,
    wakeclk32khzdiv: Wakeclk32khzdiv,
    _reserved47: [u8; 0x28],
    systickfclksel: Systickfclksel,
    systickfclkdiv: Systickfclkdiv,
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
    #[doc = "0x100 - FFRO control 0"]
    #[inline(always)]
    pub const fn ffroctl0(&self) -> &Ffroctl0 {
        &self.ffroctl0
    }
    #[doc = "0x104 - FFRO control 1"]
    #[inline(always)]
    pub const fn ffroctl1(&self) -> &Ffroctl1 {
        &self.ffroctl1
    }
    #[doc = "0x160 - system oscillator control 0"]
    #[inline(always)]
    pub const fn sysoscctl0(&self) -> &Sysoscctl0 {
        &self.sysoscctl0
    }
    #[doc = "0x168 - system oscillator bypass"]
    #[inline(always)]
    pub const fn sysoscbypass(&self) -> &Sysoscbypass {
        &self.sysoscbypass
    }
    #[doc = "0x190 - low power oscillator control 0"]
    #[inline(always)]
    pub const fn lposcctl0(&self) -> &Lposcctl0 {
        &self.lposcctl0
    }
    #[doc = "0x1c0 - 32k oscillator control0"]
    #[inline(always)]
    pub const fn osc32khzctl0(&self) -> &Osc32khzctl0 {
        &self.osc32khzctl0
    }
    #[doc = "0x200 - system pll0 clock selection"]
    #[inline(always)]
    pub const fn syspll0clksel(&self) -> &Syspll0clksel {
        &self.syspll0clksel
    }
    #[doc = "0x204 - system pll0 control0"]
    #[inline(always)]
    pub const fn syspll0ctl0(&self) -> &Syspll0ctl0 {
        &self.syspll0ctl0
    }
    #[doc = "0x20c - system pll0 lock time"]
    #[inline(always)]
    pub const fn syspll0locktimediv2(&self) -> &Syspll0locktimediv2 {
        &self.syspll0locktimediv2
    }
    #[doc = "0x210 - system pll0 number"]
    #[inline(always)]
    pub const fn syspll0num(&self) -> &Syspll0num {
        &self.syspll0num
    }
    #[doc = "0x214 - system pll0 denom"]
    #[inline(always)]
    pub const fn syspll0denom(&self) -> &Syspll0denom {
        &self.syspll0denom
    }
    #[doc = "0x218 - sys pll0 PFD"]
    #[inline(always)]
    pub const fn syspll0pfd(&self) -> &Syspll0pfd {
        &self.syspll0pfd
    }
    #[doc = "0x240 - main pll clk divider"]
    #[inline(always)]
    pub const fn mainpllclkdiv(&self) -> &Mainpllclkdiv {
        &self.mainpllclkdiv
    }
    #[doc = "0x244 - dsp pll clk divider"]
    #[inline(always)]
    pub const fn dsppllclkdiv(&self) -> &Dsppllclkdiv {
        &self.dsppllclkdiv
    }
    #[doc = "0x248 - aux0 pll clk divider"]
    #[inline(always)]
    pub const fn aux0pllclkdiv(&self) -> &Aux0pllclkdiv {
        &self.aux0pllclkdiv
    }
    #[doc = "0x24c - aux1 pll clk divider"]
    #[inline(always)]
    pub const fn aux1pllclkdiv(&self) -> &Aux1pllclkdiv {
        &self.aux1pllclkdiv
    }
    #[doc = "0x400 - system cpu AHB clock divider"]
    #[inline(always)]
    pub const fn syscpuahbclkdiv(&self) -> &Syscpuahbclkdiv {
        &self.syscpuahbclkdiv
    }
    #[doc = "0x430 - main clock selection A"]
    #[inline(always)]
    pub const fn mainclksela(&self) -> &Mainclksela {
        &self.mainclksela
    }
    #[doc = "0x434 - main clock selection B"]
    #[inline(always)]
    pub const fn mainclkselb(&self) -> &Mainclkselb {
        &self.mainclkselb
    }
    #[doc = "0x500..0x508 - PFC divider register N"]
    #[inline(always)]
    pub const fn pfcdiv(&self, n: usize) -> &Pfcdiv {
        &self.pfcdiv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x508 - PFC divider register N"]
    #[inline(always)]
    pub fn pfcdiv_iter(&self) -> impl Iterator<Item = &Pfcdiv> {
        self.pfcdiv.iter()
    }
    #[doc = "0x620 - FlexSPI FCLK selection"]
    #[inline(always)]
    pub const fn flexspifclksel(&self) -> &Flexspifclksel {
        &self.flexspifclksel
    }
    #[doc = "0x624 - FlexSPI FCLK divider"]
    #[inline(always)]
    pub const fn flexspifclkdiv(&self) -> &Flexspifclkdiv {
        &self.flexspifclkdiv
    }
    #[doc = "0x640 - SCT FCLK selection"]
    #[inline(always)]
    pub const fn sctfclksel(&self) -> &Sctfclksel {
        &self.sctfclksel
    }
    #[doc = "0x644 - SCT fclk divider"]
    #[inline(always)]
    pub const fn sctfclkdiv(&self) -> &Sctfclkdiv {
        &self.sctfclkdiv
    }
    #[doc = "0x660 - USBHS Fclk selection"]
    #[inline(always)]
    pub const fn usbhsfclksel(&self) -> &Usbhsfclksel {
        &self.usbhsfclksel
    }
    #[doc = "0x664 - USBHS Fclk divider"]
    #[inline(always)]
    pub const fn usbhsfclkdiv(&self) -> &Usbhsfclkdiv {
        &self.usbhsfclkdiv
    }
    #[doc = "0x680 - SDIO0 FCLK selection"]
    #[inline(always)]
    pub const fn sdio0fclksel(&self) -> &Sdio0fclksel {
        &self.sdio0fclksel
    }
    #[doc = "0x684 - SDIO0 FCLK divider"]
    #[inline(always)]
    pub const fn sdio0fclkdiv(&self) -> &Sdio0fclkdiv {
        &self.sdio0fclkdiv
    }
    #[doc = "0x690 - SDIO1 FCLK selection"]
    #[inline(always)]
    pub const fn sdio1fclksel(&self) -> &Sdio1fclksel {
        &self.sdio1fclksel
    }
    #[doc = "0x694 - SDIO1 FCLK divider"]
    #[inline(always)]
    pub const fn sdio1fclkdiv(&self) -> &Sdio1fclkdiv {
        &self.sdio1fclkdiv
    }
    #[doc = "0x6a0 - ESPI clock selection"]
    #[inline(always)]
    pub const fn espiclksel(&self) -> &Espiclksel {
        &self.espiclksel
    }
    #[doc = "0x6d0 - ADC0 fclk selection 0"]
    #[inline(always)]
    pub const fn adc0fclksel0(&self) -> &Adc0fclksel0 {
        &self.adc0fclksel0
    }
    #[doc = "0x6d4 - ADC0 fclk selection 1"]
    #[inline(always)]
    pub const fn adc0fclksel1(&self) -> &Adc0fclksel1 {
        &self.adc0fclksel1
    }
    #[doc = "0x6d8 - ADC0 fclk divider"]
    #[inline(always)]
    pub const fn adc0fclkdiv(&self) -> &Adc0fclkdiv {
        &self.adc0fclkdiv
    }
    #[doc = "0x700 - UTICK fclk selection"]
    #[inline(always)]
    pub const fn utickfclksel(&self) -> &Utickfclksel {
        &self.utickfclksel
    }
    #[doc = "0x720 - wdt clock selection"]
    #[inline(always)]
    pub const fn wdt0fclksel(&self) -> &Wdt0fclksel {
        &self.wdt0fclksel
    }
    #[doc = "0x730 - 32k wake clock selection"]
    #[inline(always)]
    pub const fn wakeclk32khzsel(&self) -> &Wakeclk32khzsel {
        &self.wakeclk32khzsel
    }
    #[doc = "0x734 - 32k wake clock divider"]
    #[inline(always)]
    pub const fn wakeclk32khzdiv(&self) -> &Wakeclk32khzdiv {
        &self.wakeclk32khzdiv
    }
    #[doc = "0x760 - system tick fclk selection"]
    #[inline(always)]
    pub const fn systickfclksel(&self) -> &Systickfclksel {
        &self.systickfclksel
    }
    #[doc = "0x764 - system tick fclk divider"]
    #[inline(always)]
    pub const fn systickfclkdiv(&self) -> &Systickfclkdiv {
        &self.systickfclkdiv
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
#[doc = "FFROCTL0 (rw) register accessor: FFRO control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ffroctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffroctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffroctl0`]
module"]
#[doc(alias = "FFROCTL0")]
pub type Ffroctl0 = crate::Reg<ffroctl0::Ffroctl0Spec>;
#[doc = "FFRO control 0"]
pub mod ffroctl0;
#[doc = "FFROCTL1 (rw) register accessor: FFRO control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ffroctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffroctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffroctl1`]
module"]
#[doc(alias = "FFROCTL1")]
pub type Ffroctl1 = crate::Reg<ffroctl1::Ffroctl1Spec>;
#[doc = "FFRO control 1"]
pub mod ffroctl1;
#[doc = "SYSOSCCTL0 (rw) register accessor: system oscillator control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysoscctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysoscctl0`]
module"]
#[doc(alias = "SYSOSCCTL0")]
pub type Sysoscctl0 = crate::Reg<sysoscctl0::Sysoscctl0Spec>;
#[doc = "system oscillator control 0"]
pub mod sysoscctl0;
#[doc = "SYSOSCBYPASS (rw) register accessor: system oscillator bypass\n\nYou can [`read`](crate::Reg::read) this register and get [`sysoscbypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscbypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysoscbypass`]
module"]
#[doc(alias = "SYSOSCBYPASS")]
pub type Sysoscbypass = crate::Reg<sysoscbypass::SysoscbypassSpec>;
#[doc = "system oscillator bypass"]
pub mod sysoscbypass;
#[doc = "LPOSCCTL0 (rw) register accessor: low power oscillator control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lposcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lposcctl0`]
module"]
#[doc(alias = "LPOSCCTL0")]
pub type Lposcctl0 = crate::Reg<lposcctl0::Lposcctl0Spec>;
#[doc = "low power oscillator control 0"]
pub mod lposcctl0;
#[doc = "OSC32KHZCTL0 (rw) register accessor: 32k oscillator control0\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32khzctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32khzctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc32khzctl0`]
module"]
#[doc(alias = "OSC32KHZCTL0")]
pub type Osc32khzctl0 = crate::Reg<osc32khzctl0::Osc32khzctl0Spec>;
#[doc = "32k oscillator control0"]
pub mod osc32khzctl0;
#[doc = "SYSPLL0CLKSEL (rw) register accessor: system pll0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0clksel`]
module"]
#[doc(alias = "SYSPLL0CLKSEL")]
pub type Syspll0clksel = crate::Reg<syspll0clksel::Syspll0clkselSpec>;
#[doc = "system pll0 clock selection"]
pub mod syspll0clksel;
#[doc = "SYSPLL0CTL0 (rw) register accessor: system pll0 control0\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0ctl0`]
module"]
#[doc(alias = "SYSPLL0CTL0")]
pub type Syspll0ctl0 = crate::Reg<syspll0ctl0::Syspll0ctl0Spec>;
#[doc = "system pll0 control0"]
pub mod syspll0ctl0;
#[doc = "SYSPLL0LOCKTIMEDIV2 (rw) register accessor: system pll0 lock time\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0locktimediv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0locktimediv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0locktimediv2`]
module"]
#[doc(alias = "SYSPLL0LOCKTIMEDIV2")]
pub type Syspll0locktimediv2 = crate::Reg<syspll0locktimediv2::Syspll0locktimediv2Spec>;
#[doc = "system pll0 lock time"]
pub mod syspll0locktimediv2;
#[doc = "SYSPLL0NUM (rw) register accessor: system pll0 number\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0num`]
module"]
#[doc(alias = "SYSPLL0NUM")]
pub type Syspll0num = crate::Reg<syspll0num::Syspll0numSpec>;
#[doc = "system pll0 number"]
pub mod syspll0num;
#[doc = "SYSPLL0DENOM (rw) register accessor: system pll0 denom\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0denom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0denom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0denom`]
module"]
#[doc(alias = "SYSPLL0DENOM")]
pub type Syspll0denom = crate::Reg<syspll0denom::Syspll0denomSpec>;
#[doc = "system pll0 denom"]
pub mod syspll0denom;
#[doc = "SYSPLL0PFD (rw) register accessor: sys pll0 PFD\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0pfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0pfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspll0pfd`]
module"]
#[doc(alias = "SYSPLL0PFD")]
pub type Syspll0pfd = crate::Reg<syspll0pfd::Syspll0pfdSpec>;
#[doc = "sys pll0 PFD"]
pub mod syspll0pfd;
#[doc = "MAINPLLCLKDIV (rw) register accessor: main pll clk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`mainpllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainpllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainpllclkdiv`]
module"]
#[doc(alias = "MAINPLLCLKDIV")]
pub type Mainpllclkdiv = crate::Reg<mainpllclkdiv::MainpllclkdivSpec>;
#[doc = "main pll clk divider"]
pub mod mainpllclkdiv;
#[doc = "DSPPLLCLKDIV (rw) register accessor: dsp pll clk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`dsppllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsppllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsppllclkdiv`]
module"]
#[doc(alias = "DSPPLLCLKDIV")]
pub type Dsppllclkdiv = crate::Reg<dsppllclkdiv::DsppllclkdivSpec>;
#[doc = "dsp pll clk divider"]
pub mod dsppllclkdiv;
#[doc = "AUX0PLLCLKDIV (rw) register accessor: aux0 pll clk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`aux0pllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux0pllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux0pllclkdiv`]
module"]
#[doc(alias = "AUX0PLLCLKDIV")]
pub type Aux0pllclkdiv = crate::Reg<aux0pllclkdiv::Aux0pllclkdivSpec>;
#[doc = "aux0 pll clk divider"]
pub mod aux0pllclkdiv;
#[doc = "AUX1PLLCLKDIV (rw) register accessor: aux1 pll clk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`aux1pllclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aux1pllclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aux1pllclkdiv`]
module"]
#[doc(alias = "AUX1PLLCLKDIV")]
pub type Aux1pllclkdiv = crate::Reg<aux1pllclkdiv::Aux1pllclkdivSpec>;
#[doc = "aux1 pll clk divider"]
pub mod aux1pllclkdiv;
#[doc = "SYSCPUAHBCLKDIV (rw) register accessor: system cpu AHB clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syscpuahbclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscpuahbclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscpuahbclkdiv`]
module"]
#[doc(alias = "SYSCPUAHBCLKDIV")]
pub type Syscpuahbclkdiv = crate::Reg<syscpuahbclkdiv::SyscpuahbclkdivSpec>;
#[doc = "system cpu AHB clock divider"]
pub mod syscpuahbclkdiv;
#[doc = "MAINCLKSELA (rw) register accessor: main clock selection A\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksela::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksela::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclksela`]
module"]
#[doc(alias = "MAINCLKSELA")]
pub type Mainclksela = crate::Reg<mainclksela::MainclkselaSpec>;
#[doc = "main clock selection A"]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: main clock selection B\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclkselb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclkselb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclkselb`]
module"]
#[doc(alias = "MAINCLKSELB")]
pub type Mainclkselb = crate::Reg<mainclkselb::MainclkselbSpec>;
#[doc = "main clock selection B"]
pub mod mainclkselb;
#[doc = "PFCDIV (rw) register accessor: PFC divider register N\n\nYou can [`read`](crate::Reg::read) this register and get [`pfcdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfcdiv`]
module"]
#[doc(alias = "PFCDIV")]
pub type Pfcdiv = crate::Reg<pfcdiv::PfcdivSpec>;
#[doc = "PFC divider register N"]
pub mod pfcdiv;
#[doc = "FLEXSPIFCLKSEL (rw) register accessor: FlexSPI FCLK selection\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspifclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspifclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspifclksel`]
module"]
#[doc(alias = "FLEXSPIFCLKSEL")]
pub type Flexspifclksel = crate::Reg<flexspifclksel::FlexspifclkselSpec>;
#[doc = "FlexSPI FCLK selection"]
pub mod flexspifclksel;
#[doc = "FLEXSPIFCLKDIV (rw) register accessor: FlexSPI FCLK divider\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspifclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspifclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspifclkdiv`]
module"]
#[doc(alias = "FLEXSPIFCLKDIV")]
pub type Flexspifclkdiv = crate::Reg<flexspifclkdiv::FlexspifclkdivSpec>;
#[doc = "FlexSPI FCLK divider"]
pub mod flexspifclkdiv;
#[doc = "SCTFCLKSEL (rw) register accessor: SCT FCLK selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sctfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctfclksel`]
module"]
#[doc(alias = "SCTFCLKSEL")]
pub type Sctfclksel = crate::Reg<sctfclksel::SctfclkselSpec>;
#[doc = "SCT FCLK selection"]
pub mod sctfclksel;
#[doc = "SCTFCLKDIV (rw) register accessor: SCT fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sctfclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctfclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctfclkdiv`]
module"]
#[doc(alias = "SCTFCLKDIV")]
pub type Sctfclkdiv = crate::Reg<sctfclkdiv::SctfclkdivSpec>;
#[doc = "SCT fclk divider"]
pub mod sctfclkdiv;
#[doc = "USBHSFCLKSEL (rw) register accessor: USBHS Fclk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhsfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhsfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhsfclksel`]
module"]
#[doc(alias = "USBHSFCLKSEL")]
pub type Usbhsfclksel = crate::Reg<usbhsfclksel::UsbhsfclkselSpec>;
#[doc = "USBHS Fclk selection"]
pub mod usbhsfclksel;
#[doc = "USBHSFCLKDIV (rw) register accessor: USBHS Fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhsfclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhsfclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhsfclkdiv`]
module"]
#[doc(alias = "USBHSFCLKDIV")]
pub type Usbhsfclkdiv = crate::Reg<usbhsfclkdiv::UsbhsfclkdivSpec>;
#[doc = "USBHS Fclk divider"]
pub mod usbhsfclkdiv;
#[doc = "SDIO0FCLKSEL (rw) register accessor: SDIO0 FCLK selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio0fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio0fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio0fclksel`]
module"]
#[doc(alias = "SDIO0FCLKSEL")]
pub type Sdio0fclksel = crate::Reg<sdio0fclksel::Sdio0fclkselSpec>;
#[doc = "SDIO0 FCLK selection"]
pub mod sdio0fclksel;
#[doc = "SDIO0FCLKDIV (rw) register accessor: SDIO0 FCLK divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio0fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio0fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio0fclkdiv`]
module"]
#[doc(alias = "SDIO0FCLKDIV")]
pub type Sdio0fclkdiv = crate::Reg<sdio0fclkdiv::Sdio0fclkdivSpec>;
#[doc = "SDIO0 FCLK divider"]
pub mod sdio0fclkdiv;
#[doc = "SDIO1FCLKSEL (rw) register accessor: SDIO1 FCLK selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio1fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio1fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio1fclksel`]
module"]
#[doc(alias = "SDIO1FCLKSEL")]
pub type Sdio1fclksel = crate::Reg<sdio1fclksel::Sdio1fclkselSpec>;
#[doc = "SDIO1 FCLK selection"]
pub mod sdio1fclksel;
#[doc = "SDIO1FCLKDIV (rw) register accessor: SDIO1 FCLK divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio1fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio1fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio1fclkdiv`]
module"]
#[doc(alias = "SDIO1FCLKDIV")]
pub type Sdio1fclkdiv = crate::Reg<sdio1fclkdiv::Sdio1fclkdivSpec>;
#[doc = "SDIO1 FCLK divider"]
pub mod sdio1fclkdiv;
#[doc = "ESPICLKSEL (rw) register accessor: ESPI clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`espiclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espiclksel`]
module"]
#[doc(alias = "ESPICLKSEL")]
pub type Espiclksel = crate::Reg<espiclksel::EspiclkselSpec>;
#[doc = "ESPI clock selection"]
pub mod espiclksel;
#[doc = "ADC0FCLKSEL0 (rw) register accessor: ADC0 fclk selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0fclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0fclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0fclksel0`]
module"]
#[doc(alias = "ADC0FCLKSEL0")]
pub type Adc0fclksel0 = crate::Reg<adc0fclksel0::Adc0fclksel0Spec>;
#[doc = "ADC0 fclk selection 0"]
pub mod adc0fclksel0;
#[doc = "ADC0FCLKSEL1 (rw) register accessor: ADC0 fclk selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0fclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0fclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0fclksel1`]
module"]
#[doc(alias = "ADC0FCLKSEL1")]
pub type Adc0fclksel1 = crate::Reg<adc0fclksel1::Adc0fclksel1Spec>;
#[doc = "ADC0 fclk selection 1"]
pub mod adc0fclksel1;
#[doc = "ADC0FCLKDIV (rw) register accessor: ADC0 fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0fclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0fclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0fclkdiv`]
module"]
#[doc(alias = "ADC0FCLKDIV")]
pub type Adc0fclkdiv = crate::Reg<adc0fclkdiv::Adc0fclkdivSpec>;
#[doc = "ADC0 fclk divider"]
pub mod adc0fclkdiv;
#[doc = "UTICKFCLKSEL (rw) register accessor: UTICK fclk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`utickfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utickfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utickfclksel`]
module"]
#[doc(alias = "UTICKFCLKSEL")]
pub type Utickfclksel = crate::Reg<utickfclksel::UtickfclkselSpec>;
#[doc = "UTICK fclk selection"]
pub mod utickfclksel;
#[doc = "WDT0FCLKSEL (rw) register accessor: wdt clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt0fclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt0fclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt0fclksel`]
module"]
#[doc(alias = "WDT0FCLKSEL")]
pub type Wdt0fclksel = crate::Reg<wdt0fclksel::Wdt0fclkselSpec>;
#[doc = "wdt clock selection"]
pub mod wdt0fclksel;
#[doc = "WAKECLK32KHZSEL (rw) register accessor: 32k wake clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeclk32khzsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeclk32khzsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeclk32khzsel`]
module"]
#[doc(alias = "WAKECLK32KHZSEL")]
pub type Wakeclk32khzsel = crate::Reg<wakeclk32khzsel::Wakeclk32khzselSpec>;
#[doc = "32k wake clock selection"]
pub mod wakeclk32khzsel;
#[doc = "WAKECLK32KHZDIV (rw) register accessor: 32k wake clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeclk32khzdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeclk32khzdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeclk32khzdiv`]
module"]
#[doc(alias = "WAKECLK32KHZDIV")]
pub type Wakeclk32khzdiv = crate::Reg<wakeclk32khzdiv::Wakeclk32khzdivSpec>;
#[doc = "32k wake clock divider"]
pub mod wakeclk32khzdiv;
#[doc = "SYSTICKFCLKSEL (rw) register accessor: system tick fclk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`systickfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickfclksel`]
module"]
#[doc(alias = "SYSTICKFCLKSEL")]
pub type Systickfclksel = crate::Reg<systickfclksel::SystickfclkselSpec>;
#[doc = "system tick fclk selection"]
pub mod systickfclksel;
#[doc = "SYSTICKFCLKDIV (rw) register accessor: system tick fclk divider\n\nYou can [`read`](crate::Reg::read) this register and get [`systickfclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickfclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickfclkdiv`]
module"]
#[doc(alias = "SYSTICKFCLKDIV")]
pub type Systickfclkdiv = crate::Reg<systickfclkdiv::SystickfclkdivSpec>;
#[doc = "system tick fclk divider"]
pub mod systickfclkdiv;
