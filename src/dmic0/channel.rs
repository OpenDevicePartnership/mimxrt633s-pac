#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: OSR, DIVHFCLK, PREAC2FSCOEF, PREAC4FSCOEF, GAINSHIFT, FIFO_CTRL, FIFO_STATUS, FIFO_DATA, PHY_CTRL, DC_CTRL"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    osr: Osr,
    divhfclk: Divhfclk,
    preac2fscoef: Preac2fscoef,
    preac4fscoef: Preac4fscoef,
    gainshift: Gainshift,
    _reserved5: [u8; 0x6c],
    fifo_ctrl: FifoCtrl,
    fifo_status: FifoStatus,
    fifo_data: FifoData,
    phy_ctrl: PhyCtrl,
    dc_ctrl: DcCtrl,
}
impl Channel {
    #[doc = "0x00 - CIC Filter decimation rate"]
    #[inline(always)]
    pub const fn osr(&self) -> &Osr {
        &self.osr
    }
    #[doc = "0x04 - Divider for generating PDM clock from DMIC clock input"]
    #[inline(always)]
    pub const fn divhfclk(&self) -> &Divhfclk {
        &self.divhfclk
    }
    #[doc = "0x08 - Compensation filter for 2FS"]
    #[inline(always)]
    pub const fn preac2fscoef(&self) -> &Preac2fscoef {
        &self.preac2fscoef
    }
    #[doc = "0x0c - Compensation filter for 4FS"]
    #[inline(always)]
    pub const fn preac4fscoef(&self) -> &Preac4fscoef {
        &self.preac4fscoef
    }
    #[doc = "0x10 - Decimator output gain adjustment"]
    #[inline(always)]
    pub const fn gainshift(&self) -> &Gainshift {
        &self.gainshift
    }
    #[doc = "0x80 - FIFO Control"]
    #[inline(always)]
    pub const fn fifo_ctrl(&self) -> &FifoCtrl {
        &self.fifo_ctrl
    }
    #[doc = "0x84 - FIFO Status"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FifoStatus {
        &self.fifo_status
    }
    #[doc = "0x88 - FIFO Data"]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FifoData {
        &self.fifo_data
    }
    #[doc = "0x8c - Phy Ctrl"]
    #[inline(always)]
    pub const fn phy_ctrl(&self) -> &PhyCtrl {
        &self.phy_ctrl
    }
    #[doc = "0x90 - DC Filter Control"]
    #[inline(always)]
    pub const fn dc_ctrl(&self) -> &DcCtrl {
        &self.dc_ctrl
    }
}
#[doc = "OSR (rw) register accessor: CIC Filter decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`]
module"]
#[doc(alias = "OSR")]
pub type Osr = crate::Reg<osr::OsrSpec>;
#[doc = "CIC Filter decimation rate"]
pub mod osr;
#[doc = "DIVHFCLK (rw) register accessor: Divider for generating PDM clock from DMIC clock input\n\nYou can [`read`](crate::Reg::read) this register and get [`divhfclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divhfclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divhfclk`]
module"]
#[doc(alias = "DIVHFCLK")]
pub type Divhfclk = crate::Reg<divhfclk::DivhfclkSpec>;
#[doc = "Divider for generating PDM clock from DMIC clock input"]
pub mod divhfclk;
#[doc = "PREAC2FSCOEF (rw) register accessor: Compensation filter for 2FS\n\nYou can [`read`](crate::Reg::read) this register and get [`preac2fscoef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preac2fscoef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preac2fscoef`]
module"]
#[doc(alias = "PREAC2FSCOEF")]
pub type Preac2fscoef = crate::Reg<preac2fscoef::Preac2fscoefSpec>;
#[doc = "Compensation filter for 2FS"]
pub mod preac2fscoef;
#[doc = "PREAC4FSCOEF (rw) register accessor: Compensation filter for 4FS\n\nYou can [`read`](crate::Reg::read) this register and get [`preac4fscoef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preac4fscoef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preac4fscoef`]
module"]
#[doc(alias = "PREAC4FSCOEF")]
pub type Preac4fscoef = crate::Reg<preac4fscoef::Preac4fscoefSpec>;
#[doc = "Compensation filter for 4FS"]
pub mod preac4fscoef;
#[doc = "GAINSHIFT (rw) register accessor: Decimator output gain adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`gainshift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gainshift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gainshift`]
module"]
#[doc(alias = "GAINSHIFT")]
pub type Gainshift = crate::Reg<gainshift::GainshiftSpec>;
#[doc = "Decimator output gain adjustment"]
pub mod gainshift;
#[doc = "FIFO_CTRL (rw) register accessor: FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_ctrl`]
module"]
#[doc(alias = "FIFO_CTRL")]
pub type FifoCtrl = crate::Reg<fifo_ctrl::FifoCtrlSpec>;
#[doc = "FIFO Control"]
pub mod fifo_ctrl;
#[doc = "FIFO_STATUS (rw) register accessor: FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`]
module"]
#[doc(alias = "FIFO_STATUS")]
pub type FifoStatus = crate::Reg<fifo_status::FifoStatusSpec>;
#[doc = "FIFO Status"]
pub mod fifo_status;
#[doc = "FIFO_DATA (r) register accessor: FIFO Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
#[doc(alias = "FIFO_DATA")]
pub type FifoData = crate::Reg<fifo_data::FifoDataSpec>;
#[doc = "FIFO Data"]
pub mod fifo_data;
#[doc = "PHY_CTRL (rw) register accessor: Phy Ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_ctrl`]
module"]
#[doc(alias = "PHY_CTRL")]
pub type PhyCtrl = crate::Reg<phy_ctrl::PhyCtrlSpec>;
#[doc = "Phy Ctrl"]
pub mod phy_ctrl;
#[doc = "DC_CTRL (rw) register accessor: DC Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_ctrl`]
module"]
#[doc(alias = "DC_CTRL")]
pub type DcCtrl = crate::Reg<dc_ctrl::DcCtrlSpec>;
#[doc = "DC Filter Control"]
pub mod dc_ctrl;
