#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: CFG, CTLSTAT, XFERCFG"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    cfg: Cfg,
    ctlstat: Ctlstat,
    xfercfg: Xfercfg,
}
impl Channel {
    #[doc = "0x00 - Configuration register for DMA channel ."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - Control and status register for DMA channel ."]
    #[inline(always)]
    pub const fn ctlstat(&self) -> &Ctlstat {
        &self.ctlstat
    }
    #[doc = "0x08 - Transfer configuration register for DMA channel ."]
    #[inline(always)]
    pub const fn xfercfg(&self) -> &Xfercfg {
        &self.xfercfg
    }
}
#[doc = "CFG (rw) register accessor: Configuration register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration register for DMA channel ."]
pub mod cfg;
#[doc = "CTLSTAT (r) register accessor: Control and status register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlstat`]
module"]
#[doc(alias = "CTLSTAT")]
pub type Ctlstat = crate::Reg<ctlstat::CtlstatSpec>;
#[doc = "Control and status register for DMA channel ."]
pub mod ctlstat;
#[doc = "XFERCFG (rw) register accessor: Transfer configuration register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`xfercfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xfercfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfercfg`]
module"]
#[doc(alias = "XFERCFG")]
pub type Xfercfg = crate::Reg<xfercfg::XfercfgSpec>;
#[doc = "Transfer configuration register for DMA channel ."]
pub mod xfercfg;
