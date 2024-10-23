#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ds_addr: DsAddr,
    blk_att: BlkAtt,
    cmd_arg: CmdArg,
    cmd_xfr_typ: CmdXfrTyp,
    cmd_rsp0: CmdRsp0,
    cmd_rsp1: CmdRsp1,
    cmd_rsp2: CmdRsp2,
    cmd_rsp3: CmdRsp3,
    data_buff_acc_port: DataBuffAccPort,
    pres_state: PresState,
    prot_ctrl: ProtCtrl,
    sys_ctrl: SysCtrl,
    int_status: IntStatus,
    int_status_en: IntStatusEn,
    int_signal_en: IntSignalEn,
    autocmd12_err_status: Autocmd12ErrStatus,
    host_ctrl_cap: HostCtrlCap,
    wtmk_lvl: WtmkLvl,
    mix_ctrl: MixCtrl,
    _reserved19: [u8; 0x04],
    force_event: ForceEvent,
    adma_err_status: AdmaErrStatus,
    adma_sys_addr: AdmaSysAddr,
    _reserved22: [u8; 0x04],
    dll_ctrl: DllCtrl,
    dll_status: DllStatus,
    clk_tune_ctrl_status: ClkTuneCtrlStatus,
    _reserved25: [u8; 0x04],
    strobe_dll_ctrl: StrobeDllCtrl,
    strobe_dll_status: StrobeDllStatus,
    _reserved27: [u8; 0x48],
    vend_spec: VendSpec,
    mmc_boot: MmcBoot,
    vend_spec2: VendSpec2,
    tuning_ctrl: TuningCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA System Address"]
    #[inline(always)]
    pub const fn ds_addr(&self) -> &DsAddr {
        &self.ds_addr
    }
    #[doc = "0x04 - Block Attributes"]
    #[inline(always)]
    pub const fn blk_att(&self) -> &BlkAtt {
        &self.blk_att
    }
    #[doc = "0x08 - Command Argument"]
    #[inline(always)]
    pub const fn cmd_arg(&self) -> &CmdArg {
        &self.cmd_arg
    }
    #[doc = "0x0c - Command Transfer Type"]
    #[inline(always)]
    pub const fn cmd_xfr_typ(&self) -> &CmdXfrTyp {
        &self.cmd_xfr_typ
    }
    #[doc = "0x10 - Command Response0"]
    #[inline(always)]
    pub const fn cmd_rsp0(&self) -> &CmdRsp0 {
        &self.cmd_rsp0
    }
    #[doc = "0x14 - Command Response1"]
    #[inline(always)]
    pub const fn cmd_rsp1(&self) -> &CmdRsp1 {
        &self.cmd_rsp1
    }
    #[doc = "0x18 - Command Response2"]
    #[inline(always)]
    pub const fn cmd_rsp2(&self) -> &CmdRsp2 {
        &self.cmd_rsp2
    }
    #[doc = "0x1c - Command Response3"]
    #[inline(always)]
    pub const fn cmd_rsp3(&self) -> &CmdRsp3 {
        &self.cmd_rsp3
    }
    #[doc = "0x20 - Data Buffer Access Port"]
    #[inline(always)]
    pub const fn data_buff_acc_port(&self) -> &DataBuffAccPort {
        &self.data_buff_acc_port
    }
    #[doc = "0x24 - Present State"]
    #[inline(always)]
    pub const fn pres_state(&self) -> &PresState {
        &self.pres_state
    }
    #[doc = "0x28 - Protocol Control"]
    #[inline(always)]
    pub const fn prot_ctrl(&self) -> &ProtCtrl {
        &self.prot_ctrl
    }
    #[doc = "0x2c - System Control"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SysCtrl {
        &self.sys_ctrl
    }
    #[doc = "0x30 - Interrupt Status"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x34 - Interrupt Status Enable"]
    #[inline(always)]
    pub const fn int_status_en(&self) -> &IntStatusEn {
        &self.int_status_en
    }
    #[doc = "0x38 - Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn int_signal_en(&self) -> &IntSignalEn {
        &self.int_signal_en
    }
    #[doc = "0x3c - Auto CMD12 Error Status"]
    #[inline(always)]
    pub const fn autocmd12_err_status(&self) -> &Autocmd12ErrStatus {
        &self.autocmd12_err_status
    }
    #[doc = "0x40 - Host Controller Capabilities"]
    #[inline(always)]
    pub const fn host_ctrl_cap(&self) -> &HostCtrlCap {
        &self.host_ctrl_cap
    }
    #[doc = "0x44 - Watermark Level"]
    #[inline(always)]
    pub const fn wtmk_lvl(&self) -> &WtmkLvl {
        &self.wtmk_lvl
    }
    #[doc = "0x48 - Mixer Control"]
    #[inline(always)]
    pub const fn mix_ctrl(&self) -> &MixCtrl {
        &self.mix_ctrl
    }
    #[doc = "0x50 - Force Event"]
    #[inline(always)]
    pub const fn force_event(&self) -> &ForceEvent {
        &self.force_event
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn adma_err_status(&self) -> &AdmaErrStatus {
        &self.adma_err_status
    }
    #[doc = "0x58 - ADMA System Address"]
    #[inline(always)]
    pub const fn adma_sys_addr(&self) -> &AdmaSysAddr {
        &self.adma_sys_addr
    }
    #[doc = "0x60 - DLL (Delay Line) Control"]
    #[inline(always)]
    pub const fn dll_ctrl(&self) -> &DllCtrl {
        &self.dll_ctrl
    }
    #[doc = "0x64 - DLL Status"]
    #[inline(always)]
    pub const fn dll_status(&self) -> &DllStatus {
        &self.dll_status
    }
    #[doc = "0x68 - CLK Tuning Control and Status"]
    #[inline(always)]
    pub const fn clk_tune_ctrl_status(&self) -> &ClkTuneCtrlStatus {
        &self.clk_tune_ctrl_status
    }
    #[doc = "0x70 - Strobe DLL Control"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl(&self) -> &StrobeDllCtrl {
        &self.strobe_dll_ctrl
    }
    #[doc = "0x74 - Strobe DLL Status"]
    #[inline(always)]
    pub const fn strobe_dll_status(&self) -> &StrobeDllStatus {
        &self.strobe_dll_status
    }
    #[doc = "0xc0 - Vendor Specific Register"]
    #[inline(always)]
    pub const fn vend_spec(&self) -> &VendSpec {
        &self.vend_spec
    }
    #[doc = "0xc4 - MMC Boot Register"]
    #[inline(always)]
    pub const fn mmc_boot(&self) -> &MmcBoot {
        &self.mmc_boot
    }
    #[doc = "0xc8 - Vendor Specific 2 Register"]
    #[inline(always)]
    pub const fn vend_spec2(&self) -> &VendSpec2 {
        &self.vend_spec2
    }
    #[doc = "0xcc - Tuning Control Register"]
    #[inline(always)]
    pub const fn tuning_ctrl(&self) -> &TuningCtrl {
        &self.tuning_ctrl
    }
}
#[doc = "DS_ADDR (rw) register accessor: DMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_addr`]
module"]
#[doc(alias = "DS_ADDR")]
pub type DsAddr = crate::Reg<ds_addr::DsAddrSpec>;
#[doc = "DMA System Address"]
pub mod ds_addr;
#[doc = "BLK_ATT (rw) register accessor: Block Attributes\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_att::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_att::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_att`]
module"]
#[doc(alias = "BLK_ATT")]
pub type BlkAtt = crate::Reg<blk_att::BlkAttSpec>;
#[doc = "Block Attributes"]
pub mod blk_att;
#[doc = "CMD_ARG (rw) register accessor: Command Argument\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_arg`]
module"]
#[doc(alias = "CMD_ARG")]
pub type CmdArg = crate::Reg<cmd_arg::CmdArgSpec>;
#[doc = "Command Argument"]
pub mod cmd_arg;
#[doc = "CMD_XFR_TYP (rw) register accessor: Command Transfer Type\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_xfr_typ::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_xfr_typ::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_xfr_typ`]
module"]
#[doc(alias = "CMD_XFR_TYP")]
pub type CmdXfrTyp = crate::Reg<cmd_xfr_typ::CmdXfrTypSpec>;
#[doc = "Command Transfer Type"]
pub mod cmd_xfr_typ;
#[doc = "CMD_RSP0 (r) register accessor: Command Response0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_rsp0`]
module"]
#[doc(alias = "CMD_RSP0")]
pub type CmdRsp0 = crate::Reg<cmd_rsp0::CmdRsp0Spec>;
#[doc = "Command Response0"]
pub mod cmd_rsp0;
#[doc = "CMD_RSP1 (r) register accessor: Command Response1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_rsp1`]
module"]
#[doc(alias = "CMD_RSP1")]
pub type CmdRsp1 = crate::Reg<cmd_rsp1::CmdRsp1Spec>;
#[doc = "Command Response1"]
pub mod cmd_rsp1;
#[doc = "CMD_RSP2 (r) register accessor: Command Response2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_rsp2`]
module"]
#[doc(alias = "CMD_RSP2")]
pub type CmdRsp2 = crate::Reg<cmd_rsp2::CmdRsp2Spec>;
#[doc = "Command Response2"]
pub mod cmd_rsp2;
#[doc = "CMD_RSP3 (r) register accessor: Command Response3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_rsp3`]
module"]
#[doc(alias = "CMD_RSP3")]
pub type CmdRsp3 = crate::Reg<cmd_rsp3::CmdRsp3Spec>;
#[doc = "Command Response3"]
pub mod cmd_rsp3;
#[doc = "DATA_BUFF_ACC_PORT (rw) register accessor: Data Buffer Access Port\n\nYou can [`read`](crate::Reg::read) this register and get [`data_buff_acc_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_buff_acc_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_buff_acc_port`]
module"]
#[doc(alias = "DATA_BUFF_ACC_PORT")]
pub type DataBuffAccPort = crate::Reg<data_buff_acc_port::DataBuffAccPortSpec>;
#[doc = "Data Buffer Access Port"]
pub mod data_buff_acc_port;
#[doc = "PRES_STATE (r) register accessor: Present State\n\nYou can [`read`](crate::Reg::read) this register and get [`pres_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pres_state`]
module"]
#[doc(alias = "PRES_STATE")]
pub type PresState = crate::Reg<pres_state::PresStateSpec>;
#[doc = "Present State"]
pub mod pres_state;
#[doc = "PROT_CTRL (rw) register accessor: Protocol Control\n\nYou can [`read`](crate::Reg::read) this register and get [`prot_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prot_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prot_ctrl`]
module"]
#[doc(alias = "PROT_CTRL")]
pub type ProtCtrl = crate::Reg<prot_ctrl::ProtCtrlSpec>;
#[doc = "Protocol Control"]
pub mod prot_ctrl;
#[doc = "SYS_CTRL (rw) register accessor: System Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`]
module"]
#[doc(alias = "SYS_CTRL")]
pub type SysCtrl = crate::Reg<sys_ctrl::SysCtrlSpec>;
#[doc = "System Control"]
pub mod sys_ctrl;
#[doc = "INT_STATUS (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "INT_STATUS_EN (rw) register accessor: Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_en`]
module"]
#[doc(alias = "INT_STATUS_EN")]
pub type IntStatusEn = crate::Reg<int_status_en::IntStatusEnSpec>;
#[doc = "Interrupt Status Enable"]
pub mod int_status_en;
#[doc = "INT_SIGNAL_EN (rw) register accessor: Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_signal_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_signal_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_signal_en`]
module"]
#[doc(alias = "INT_SIGNAL_EN")]
pub type IntSignalEn = crate::Reg<int_signal_en::IntSignalEnSpec>;
#[doc = "Interrupt Signal Enable"]
pub mod int_signal_en;
#[doc = "AUTOCMD12_ERR_STATUS (rw) register accessor: Auto CMD12 Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`autocmd12_err_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocmd12_err_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocmd12_err_status`]
module"]
#[doc(alias = "AUTOCMD12_ERR_STATUS")]
pub type Autocmd12ErrStatus = crate::Reg<autocmd12_err_status::Autocmd12ErrStatusSpec>;
#[doc = "Auto CMD12 Error Status"]
pub mod autocmd12_err_status;
#[doc = "HOST_CTRL_CAP (rw) register accessor: Host Controller Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl_cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl_cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl_cap`]
module"]
#[doc(alias = "HOST_CTRL_CAP")]
pub type HostCtrlCap = crate::Reg<host_ctrl_cap::HostCtrlCapSpec>;
#[doc = "Host Controller Capabilities"]
pub mod host_ctrl_cap;
#[doc = "WTMK_LVL (rw) register accessor: Watermark Level\n\nYou can [`read`](crate::Reg::read) this register and get [`wtmk_lvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtmk_lvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtmk_lvl`]
module"]
#[doc(alias = "WTMK_LVL")]
pub type WtmkLvl = crate::Reg<wtmk_lvl::WtmkLvlSpec>;
#[doc = "Watermark Level"]
pub mod wtmk_lvl;
#[doc = "MIX_CTRL (rw) register accessor: Mixer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mix_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mix_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mix_ctrl`]
module"]
#[doc(alias = "MIX_CTRL")]
pub type MixCtrl = crate::Reg<mix_ctrl::MixCtrlSpec>;
#[doc = "Mixer Control"]
pub mod mix_ctrl;
#[doc = "FORCE_EVENT (rw) register accessor: Force Event\n\nYou can [`read`](crate::Reg::read) this register and get [`force_event::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_event::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event`]
module"]
#[doc(alias = "FORCE_EVENT")]
pub type ForceEvent = crate::Reg<force_event::ForceEventSpec>;
#[doc = "Force Event"]
pub mod force_event;
#[doc = "ADMA_ERR_STATUS (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_err_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_err_status`]
module"]
#[doc(alias = "ADMA_ERR_STATUS")]
pub type AdmaErrStatus = crate::Reg<adma_err_status::AdmaErrStatusSpec>;
#[doc = "ADMA Error Status Register"]
pub mod adma_err_status;
#[doc = "ADMA_SYS_ADDR (rw) register accessor: ADMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_sys_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_sys_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_sys_addr`]
module"]
#[doc(alias = "ADMA_SYS_ADDR")]
pub type AdmaSysAddr = crate::Reg<adma_sys_addr::AdmaSysAddrSpec>;
#[doc = "ADMA System Address"]
pub mod adma_sys_addr;
#[doc = "DLL_CTRL (rw) register accessor: DLL (Delay Line) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_ctrl`]
module"]
#[doc(alias = "DLL_CTRL")]
pub type DllCtrl = crate::Reg<dll_ctrl::DllCtrlSpec>;
#[doc = "DLL (Delay Line) Control"]
pub mod dll_ctrl;
#[doc = "DLL_STATUS (r) register accessor: DLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status`]
module"]
#[doc(alias = "DLL_STATUS")]
pub type DllStatus = crate::Reg<dll_status::DllStatusSpec>;
#[doc = "DLL Status"]
pub mod dll_status;
#[doc = "CLK_TUNE_CTRL_STATUS (rw) register accessor: CLK Tuning Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_tune_ctrl_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_tune_ctrl_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tune_ctrl_status`]
module"]
#[doc(alias = "CLK_TUNE_CTRL_STATUS")]
pub type ClkTuneCtrlStatus = crate::Reg<clk_tune_ctrl_status::ClkTuneCtrlStatusSpec>;
#[doc = "CLK Tuning Control and Status"]
pub mod clk_tune_ctrl_status;
#[doc = "STROBE_DLL_CTRL (rw) register accessor: Strobe DLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`strobe_dll_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strobe_dll_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strobe_dll_ctrl`]
module"]
#[doc(alias = "STROBE_DLL_CTRL")]
pub type StrobeDllCtrl = crate::Reg<strobe_dll_ctrl::StrobeDllCtrlSpec>;
#[doc = "Strobe DLL Control"]
pub mod strobe_dll_ctrl;
#[doc = "STROBE_DLL_STATUS (r) register accessor: Strobe DLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`strobe_dll_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strobe_dll_status`]
module"]
#[doc(alias = "STROBE_DLL_STATUS")]
pub type StrobeDllStatus = crate::Reg<strobe_dll_status::StrobeDllStatusSpec>;
#[doc = "Strobe DLL Status"]
pub mod strobe_dll_status;
#[doc = "VEND_SPEC (rw) register accessor: Vendor Specific Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vend_spec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vend_spec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vend_spec`]
module"]
#[doc(alias = "VEND_SPEC")]
pub type VendSpec = crate::Reg<vend_spec::VendSpecSpec>;
#[doc = "Vendor Specific Register"]
pub mod vend_spec;
#[doc = "MMC_BOOT (rw) register accessor: MMC Boot Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_boot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_boot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_boot`]
module"]
#[doc(alias = "MMC_BOOT")]
pub type MmcBoot = crate::Reg<mmc_boot::MmcBootSpec>;
#[doc = "MMC Boot Register"]
pub mod mmc_boot;
#[doc = "VEND_SPEC2 (rw) register accessor: Vendor Specific 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vend_spec2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vend_spec2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vend_spec2`]
module"]
#[doc(alias = "VEND_SPEC2")]
pub type VendSpec2 = crate::Reg<vend_spec2::VendSpec2Spec>;
#[doc = "Vendor Specific 2 Register"]
pub mod vend_spec2;
#[doc = "TUNING_CTRL (rw) register accessor: Tuning Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tuning_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tuning_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tuning_ctrl`]
module"]
#[doc(alias = "TUNING_CTRL")]
pub type TuningCtrl = crate::Reg<tuning_ctrl::TuningCtrlSpec>;
#[doc = "Tuning Control Register"]
pub mod tuning_ctrl;
