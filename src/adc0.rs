#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    param: Param,
    _reserved2: [u8; 0x08],
    ctrl: Ctrl,
    stat: Stat,
    ie: Ie,
    de: De,
    cfg: Cfg,
    pause: Pause,
    _reserved8: [u8; 0x08],
    fctrl: Fctrl,
    swtrig: Swtrig,
    _reserved10: [u8; 0x88],
    tctrl: [Tctrl; 16],
    cmdl: (),
    _reserved12: [u8; 0x04],
    cmdh: (),
    _reserved13: [u8; 0xfc],
    cv: [Cv; 4],
    _reserved14: [u8; 0xf0],
    resfifo: Resfifo,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x04 - Parameter Register"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x10 - ADC Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - ADC Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x18 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x1c - DMA Enable Register"]
    #[inline(always)]
    pub const fn de(&self) -> &De {
        &self.de
    }
    #[doc = "0x20 - ADC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x24 - ADC Pause Register"]
    #[inline(always)]
    pub const fn pause(&self) -> &Pause {
        &self.pause
    }
    #[doc = "0x30 - ADC FIFO Control Register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &Fctrl {
        &self.fctrl
    }
    #[doc = "0x34 - Software Trigger Register"]
    #[inline(always)]
    pub const fn swtrig(&self) -> &Swtrig {
        &self.swtrig
    }
    #[doc = "0xc0..0x100 - Trigger Control Register"]
    #[inline(always)]
    pub const fn tctrl(&self, n: usize) -> &Tctrl {
        &self.tctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0x100 - Trigger Control Register"]
    #[inline(always)]
    pub fn tctrl_iter(&self) -> impl Iterator<Item = &Tctrl> {
        self.tctrl.iter()
    }
    #[doc = "0x100..0x13c - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl(&self, n: usize) -> &Cmdl {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x13c - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub fn cmdl_iter(&self) -> impl Iterator<Item = &Cmdl> {
        (0..15).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x100 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl1(&self) -> &Cmdl {
        self.cmdl(0)
    }
    #[doc = "0x108 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl2(&self) -> &Cmdl {
        self.cmdl(1)
    }
    #[doc = "0x110 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl3(&self) -> &Cmdl {
        self.cmdl(2)
    }
    #[doc = "0x118 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl4(&self) -> &Cmdl {
        self.cmdl(3)
    }
    #[doc = "0x120 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl5(&self) -> &Cmdl {
        self.cmdl(4)
    }
    #[doc = "0x128 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl6(&self) -> &Cmdl {
        self.cmdl(5)
    }
    #[doc = "0x130 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl7(&self) -> &Cmdl {
        self.cmdl(6)
    }
    #[doc = "0x138 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl8(&self) -> &Cmdl {
        self.cmdl(7)
    }
    #[doc = "0x140 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl9(&self) -> &Cmdl {
        self.cmdl(8)
    }
    #[doc = "0x148 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl10(&self) -> &Cmdl {
        self.cmdl(9)
    }
    #[doc = "0x150 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl11(&self) -> &Cmdl {
        self.cmdl(10)
    }
    #[doc = "0x158 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl12(&self) -> &Cmdl {
        self.cmdl(11)
    }
    #[doc = "0x160 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl13(&self) -> &Cmdl {
        self.cmdl(12)
    }
    #[doc = "0x168 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl14(&self) -> &Cmdl {
        self.cmdl(13)
    }
    #[doc = "0x170 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl15(&self) -> &Cmdl {
        self.cmdl(14)
    }
    #[doc = "0x104..0x140 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh(&self, n: usize) -> &Cmdh {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x140 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub fn cmdh_iter(&self) -> impl Iterator<Item = &Cmdh> {
        (0..15).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x104 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh1(&self) -> &Cmdh {
        self.cmdh(0)
    }
    #[doc = "0x10c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh2(&self) -> &Cmdh {
        self.cmdh(1)
    }
    #[doc = "0x114 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh3(&self) -> &Cmdh {
        self.cmdh(2)
    }
    #[doc = "0x11c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh4(&self) -> &Cmdh {
        self.cmdh(3)
    }
    #[doc = "0x124 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh5(&self) -> &Cmdh {
        self.cmdh(4)
    }
    #[doc = "0x12c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh6(&self) -> &Cmdh {
        self.cmdh(5)
    }
    #[doc = "0x134 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh7(&self) -> &Cmdh {
        self.cmdh(6)
    }
    #[doc = "0x13c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh8(&self) -> &Cmdh {
        self.cmdh(7)
    }
    #[doc = "0x144 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh9(&self) -> &Cmdh {
        self.cmdh(8)
    }
    #[doc = "0x14c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh10(&self) -> &Cmdh {
        self.cmdh(9)
    }
    #[doc = "0x154 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh11(&self) -> &Cmdh {
        self.cmdh(10)
    }
    #[doc = "0x15c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh12(&self) -> &Cmdh {
        self.cmdh(11)
    }
    #[doc = "0x164 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh13(&self) -> &Cmdh {
        self.cmdh(12)
    }
    #[doc = "0x16c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh14(&self) -> &Cmdh {
        self.cmdh(13)
    }
    #[doc = "0x174 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh15(&self) -> &Cmdh {
        self.cmdh(14)
    }
    #[doc = "0x200..0x210 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv(&self, n: usize) -> &Cv {
        &self.cv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x210 - Compare Value Register"]
    #[inline(always)]
    pub fn cv_iter(&self) -> impl Iterator<Item = &Cv> {
        self.cv.iter()
    }
    #[doc = "0x200 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv1(&self) -> &Cv {
        self.cv(0)
    }
    #[doc = "0x204 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv2(&self) -> &Cv {
        self.cv(1)
    }
    #[doc = "0x208 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv3(&self) -> &Cv {
        self.cv(2)
    }
    #[doc = "0x20c - Compare Value Register"]
    #[inline(always)]
    pub const fn cv4(&self) -> &Cv {
        self.cv(3)
    }
    #[doc = "0x300 - ADC Data Result FIFO Register"]
    #[inline(always)]
    pub const fn resfifo(&self) -> &Resfifo {
        &self.resfifo
    }
}
#[doc = "VERID (r) register accessor: Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`]
module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: ADC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "ADC Control Register"]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: ADC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "ADC Status Register"]
pub mod stat;
#[doc = "IE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "DE (rw) register accessor: DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`de::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`de::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@de`]
module"]
#[doc(alias = "DE")]
pub type De = crate::Reg<de::DeSpec>;
#[doc = "DMA Enable Register"]
pub mod de;
#[doc = "CFG (rw) register accessor: ADC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "ADC Configuration Register"]
pub mod cfg;
#[doc = "PAUSE (rw) register accessor: ADC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pause`]
module"]
#[doc(alias = "PAUSE")]
pub type Pause = crate::Reg<pause::PauseSpec>;
#[doc = "ADC Pause Register"]
pub mod pause;
#[doc = "FCTRL (rw) register accessor: ADC FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`]
module"]
#[doc(alias = "FCTRL")]
pub type Fctrl = crate::Reg<fctrl::FctrlSpec>;
#[doc = "ADC FIFO Control Register"]
pub mod fctrl;
#[doc = "SWTRIG (rw) register accessor: Software Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`]
module"]
#[doc(alias = "SWTRIG")]
pub type Swtrig = crate::Reg<swtrig::SwtrigSpec>;
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "TCTRL (rw) register accessor: Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tctrl`]
module"]
#[doc(alias = "TCTRL")]
pub type Tctrl = crate::Reg<tctrl::TctrlSpec>;
#[doc = "Trigger Control Register"]
pub mod tctrl;
#[doc = "CMDL (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl`]
module"]
#[doc(alias = "CMDL")]
pub type Cmdl = crate::Reg<cmdl::CmdlSpec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl;
#[doc = "CMDH (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh`]
module"]
#[doc(alias = "CMDH")]
pub type Cmdh = crate::Reg<cmdh::CmdhSpec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh;
#[doc = "CV (rw) register accessor: Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`]
module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "RESFIFO (r) register accessor: ADC Data Result FIFO Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resfifo`]
module"]
#[doc(alias = "RESFIFO")]
pub type Resfifo = crate::Reg<resfifo::ResfifoSpec>;
#[doc = "ADC Data Result FIFO Register"]
pub mod resfifo;
