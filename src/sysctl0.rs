#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    dspstall: Dspstall,
    ahbmatrixprior: Ahbmatrixprior,
    packerenable: Packerenable,
    _reserved3: [u8; 0x18],
    m33nmisrcsel: M33nmisrcsel,
    system_stick_calib: SystemStickCalib,
    system_nstick_calib: SystemNstickCalib,
    _reserved6: [u8; 0x24],
    product_id: ProductId,
    siliconrev_id: SiliconrevId,
    jtag_id: JtagId,
    _reserved9: [u8; 0x14],
    autoclkgateoverride0: Autoclkgateoverride0,
    autoclkgateoverride1: Autoclkgateoverride1,
    _reserved11: [u8; 0x18],
    clkgateoverride0: Clkgateoverride0,
    _reserved12: [u8; 0x5c],
    ahb_sram_access_disable: AhbSramAccessDisable,
    dsp_sram_access_disable: DspSramAccessDisable,
    _reserved14: [u8; 0x30],
    ahb_flexspi_access_disable: AhbFlexspiAccessDisable,
    dsp_flexspi_access_disable: DspFlexspiAccessDisable,
    _reserved16: [u8; 0x0240],
    flexspi_bootrom_scratch0: FlexspiBootromScratch0,
    _reserved17: [u8; 0x88],
    usbclkctrl: Usbclkctrl,
    usbclkstat: Usbclkstat,
    usbphypll0locktimediv2: Usbphypll0locktimediv2,
    _reserved20: [u8; 0x01e8],
    pdsleepcfg0: Pdsleepcfg0,
    pdsleepcfg1: Pdsleepcfg1,
    pdsleepcfg2: Pdsleepcfg2,
    pdsleepcfg3: Pdsleepcfg3,
    pdruncfg0: Pdruncfg0,
    pdruncfg1: Pdruncfg1,
    pdruncfg2: Pdruncfg2,
    pdruncfg3: Pdruncfg3,
    pdruncfg0_set: Pdruncfg0Set,
    pdruncfg1_set: Pdruncfg1Set,
    pdruncfg2_set: Pdruncfg2Set,
    pdruncfg3_set: Pdruncfg3Set,
    pdruncfg0_clr: Pdruncfg0Clr,
    pdruncfg1_clr: Pdruncfg1Clr,
    pdruncfg2_clr: Pdruncfg2Clr,
    pdruncfg3_clr: Pdruncfg3Clr,
    _reserved36: [u8; 0x20],
    pdwakecfg: Pdwakecfg,
    _reserved37: [u8; 0x1c],
    starten0: Starten0,
    starten1: Starten1,
    _reserved39: [u8; 0x18],
    starten0_set: Starten0Set,
    starten1_set: Starten1Set,
    _reserved41: [u8; 0x18],
    starten0_clr: Starten0Clr,
    starten1_clr: Starten1Clr,
    _reserved43: [u8; 0x48],
    mainclksafety: Mainclksafety,
    _reserved44: [u8; 0x6c],
    hwwake: Hwwake,
    _reserved45: [u8; 0x0688],
    tempsensorctl: Tempsensorctl,
    _reserved46: [u8; 0x40],
    bootstateseed: [Bootstateseed; 8],
    bootstatehmac: [Bootstatehmac; 8],
    _reserved48: [u8; 0x68],
    flexspipadctrl: Flexspipadctrl,
    sdiopadctl: Sdiopadctl,
    dicehwreg: [Dicehwreg; 8],
    _reserved51: [u8; 0x30],
    uuid: [Uuid; 4],
    _reserved52: [u8; 0x20],
    aeskey_srcsel: AeskeySrcsel,
    _reserved53: [u8; 0x04],
    hashhwkeydisable: Hashhwkeydisable,
    _reserved54: [u8; 0x14],
    dbg_locken: DbgLocken,
    dbg_features: DbgFeatures,
    dbg_features_dp: DbgFeaturesDp,
    hwunlock_disable: HwunlockDisable,
    _reserved58: [u8; 0x04],
    cs_protcpu0: CsProtcpu0,
    cs_protcpu1: CsProtcpu1,
    _reserved60: [u8; 0x04],
    dbg_auth_scratch: DbgAuthScratch,
    _reserved61: [u8; 0x0c],
    key_block: KeyBlock,
}
impl RegisterBlock {
    #[doc = "0x0c - DSP stall register"]
    #[inline(always)]
    pub const fn dspstall(&self) -> &Dspstall {
        &self.dspstall
    }
    #[doc = "0x10 - AHB matrix priority"]
    #[inline(always)]
    pub const fn ahbmatrixprior(&self) -> &Ahbmatrixprior {
        &self.ahbmatrixprior
    }
    #[doc = "0x14 - Packer enable for DSP RAM packer"]
    #[inline(always)]
    pub const fn packerenable(&self) -> &Packerenable {
        &self.packerenable
    }
    #[doc = "0x30 - M33 nmi source selection"]
    #[inline(always)]
    pub const fn m33nmisrcsel(&self) -> &M33nmisrcsel {
        &self.m33nmisrcsel
    }
    #[doc = "0x34 - system stick calibration"]
    #[inline(always)]
    pub const fn system_stick_calib(&self) -> &SystemStickCalib {
        &self.system_stick_calib
    }
    #[doc = "0x38 - system nstick calibration"]
    #[inline(always)]
    pub const fn system_nstick_calib(&self) -> &SystemNstickCalib {
        &self.system_nstick_calib
    }
    #[doc = "0x60 - product ID"]
    #[inline(always)]
    pub const fn product_id(&self) -> &ProductId {
        &self.product_id
    }
    #[doc = "0x64 - SILICONREV ID"]
    #[inline(always)]
    pub const fn siliconrev_id(&self) -> &SiliconrevId {
        &self.siliconrev_id
    }
    #[doc = "0x68 - jtag ID"]
    #[inline(always)]
    pub const fn jtag_id(&self) -> &JtagId {
        &self.jtag_id
    }
    #[doc = "0x80 - auto clock gating override 0"]
    #[inline(always)]
    pub const fn autoclkgateoverride0(&self) -> &Autoclkgateoverride0 {
        &self.autoclkgateoverride0
    }
    #[doc = "0x84 - auto clock gating override 1"]
    #[inline(always)]
    pub const fn autoclkgateoverride1(&self) -> &Autoclkgateoverride1 {
        &self.autoclkgateoverride1
    }
    #[doc = "0xa0 - Clock gate override 0"]
    #[inline(always)]
    pub const fn clkgateoverride0(&self) -> &Clkgateoverride0 {
        &self.clkgateoverride0
    }
    #[doc = "0x100 - AHB SRAM access disable"]
    #[inline(always)]
    pub const fn ahb_sram_access_disable(&self) -> &AhbSramAccessDisable {
        &self.ahb_sram_access_disable
    }
    #[doc = "0x104 - DSP SRAM access disable"]
    #[inline(always)]
    pub const fn dsp_sram_access_disable(&self) -> &DspSramAccessDisable {
        &self.dsp_sram_access_disable
    }
    #[doc = "0x138 - AHB Flexspi access control"]
    #[inline(always)]
    pub const fn ahb_flexspi_access_disable(&self) -> &AhbFlexspiAccessDisable {
        &self.ahb_flexspi_access_disable
    }
    #[doc = "0x13c - DSP Flexspi access control"]
    #[inline(always)]
    pub const fn dsp_flexspi_access_disable(&self) -> &DspFlexspiAccessDisable {
        &self.dsp_flexspi_access_disable
    }
    #[doc = "0x380 - FLEXSPI NOR flash configure context register"]
    #[inline(always)]
    pub const fn flexspi_bootrom_scratch0(&self) -> &FlexspiBootromScratch0 {
        &self.flexspi_bootrom_scratch0
    }
    #[doc = "0x40c - USB clock control"]
    #[inline(always)]
    pub const fn usbclkctrl(&self) -> &Usbclkctrl {
        &self.usbclkctrl
    }
    #[doc = "0x410 - USB clock status"]
    #[inline(always)]
    pub const fn usbclkstat(&self) -> &Usbclkstat {
        &self.usbclkstat
    }
    #[doc = "0x414 - USB PHY PLL0 lock time division 2"]
    #[inline(always)]
    pub const fn usbphypll0locktimediv2(&self) -> &Usbphypll0locktimediv2 {
        &self.usbphypll0locktimediv2
    }
    #[doc = "0x600 - Sleep configuration 0"]
    #[inline(always)]
    pub const fn pdsleepcfg0(&self) -> &Pdsleepcfg0 {
        &self.pdsleepcfg0
    }
    #[doc = "0x604 - Sleep configuration 1"]
    #[inline(always)]
    pub const fn pdsleepcfg1(&self) -> &Pdsleepcfg1 {
        &self.pdsleepcfg1
    }
    #[doc = "0x608 - Sleep configuration 2"]
    #[inline(always)]
    pub const fn pdsleepcfg2(&self) -> &Pdsleepcfg2 {
        &self.pdsleepcfg2
    }
    #[doc = "0x60c - Sleep configuration 3"]
    #[inline(always)]
    pub const fn pdsleepcfg3(&self) -> &Pdsleepcfg3 {
        &self.pdsleepcfg3
    }
    #[doc = "0x610 - Run configuration 0"]
    #[inline(always)]
    pub const fn pdruncfg0(&self) -> &Pdruncfg0 {
        &self.pdruncfg0
    }
    #[doc = "0x614 - Run configuration 1"]
    #[inline(always)]
    pub const fn pdruncfg1(&self) -> &Pdruncfg1 {
        &self.pdruncfg1
    }
    #[doc = "0x618 - Run configuration 2"]
    #[inline(always)]
    pub const fn pdruncfg2(&self) -> &Pdruncfg2 {
        &self.pdruncfg2
    }
    #[doc = "0x61c - Run configuration 3"]
    #[inline(always)]
    pub const fn pdruncfg3(&self) -> &Pdruncfg3 {
        &self.pdruncfg3
    }
    #[doc = "0x620 - Run configuration 0 set"]
    #[inline(always)]
    pub const fn pdruncfg0_set(&self) -> &Pdruncfg0Set {
        &self.pdruncfg0_set
    }
    #[doc = "0x624 - Run configuration 1 set"]
    #[inline(always)]
    pub const fn pdruncfg1_set(&self) -> &Pdruncfg1Set {
        &self.pdruncfg1_set
    }
    #[doc = "0x628 - Run configuration 2 set"]
    #[inline(always)]
    pub const fn pdruncfg2_set(&self) -> &Pdruncfg2Set {
        &self.pdruncfg2_set
    }
    #[doc = "0x62c - Run configuration 3 set"]
    #[inline(always)]
    pub const fn pdruncfg3_set(&self) -> &Pdruncfg3Set {
        &self.pdruncfg3_set
    }
    #[doc = "0x630 - Run configuration 0 clear"]
    #[inline(always)]
    pub const fn pdruncfg0_clr(&self) -> &Pdruncfg0Clr {
        &self.pdruncfg0_clr
    }
    #[doc = "0x634 - Run configuration 1 clear"]
    #[inline(always)]
    pub const fn pdruncfg1_clr(&self) -> &Pdruncfg1Clr {
        &self.pdruncfg1_clr
    }
    #[doc = "0x638 - Run configuration 2 clear"]
    #[inline(always)]
    pub const fn pdruncfg2_clr(&self) -> &Pdruncfg2Clr {
        &self.pdruncfg2_clr
    }
    #[doc = "0x63c - Run configuration 3 clear"]
    #[inline(always)]
    pub const fn pdruncfg3_clr(&self) -> &Pdruncfg3Clr {
        &self.pdruncfg3_clr
    }
    #[doc = "0x660 - PD Wake Configuration"]
    #[inline(always)]
    pub const fn pdwakecfg(&self) -> &Pdwakecfg {
        &self.pdwakecfg
    }
    #[doc = "0x680 - Start enable 0"]
    #[inline(always)]
    pub const fn starten0(&self) -> &Starten0 {
        &self.starten0
    }
    #[doc = "0x684 - Start enable 1"]
    #[inline(always)]
    pub const fn starten1(&self) -> &Starten1 {
        &self.starten1
    }
    #[doc = "0x6a0 - Start enable 0 set"]
    #[inline(always)]
    pub const fn starten0_set(&self) -> &Starten0Set {
        &self.starten0_set
    }
    #[doc = "0x6a4 - Start enable 1 set"]
    #[inline(always)]
    pub const fn starten1_set(&self) -> &Starten1Set {
        &self.starten1_set
    }
    #[doc = "0x6c0 - Start enable 0 clear"]
    #[inline(always)]
    pub const fn starten0_clr(&self) -> &Starten0Clr {
        &self.starten0_clr
    }
    #[doc = "0x6c4 - Start enable 1 clear"]
    #[inline(always)]
    pub const fn starten1_clr(&self) -> &Starten1Clr {
        &self.starten1_clr
    }
    #[doc = "0x710 - Main Clock Safety"]
    #[inline(always)]
    pub const fn mainclksafety(&self) -> &Mainclksafety {
        &self.mainclksafety
    }
    #[doc = "0x780 - Hardware Wake-up control"]
    #[inline(always)]
    pub const fn hwwake(&self) -> &Hwwake {
        &self.hwwake
    }
    #[doc = "0xe0c - tempsensor ctrl"]
    #[inline(always)]
    pub const fn tempsensorctl(&self) -> &Tempsensorctl {
        &self.tempsensorctl
    }
    #[doc = "0xe50..0xe70 - boot state seed register"]
    #[inline(always)]
    pub const fn bootstateseed(&self, n: usize) -> &Bootstateseed {
        &self.bootstateseed[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe50..0xe70 - boot state seed register"]
    #[inline(always)]
    pub fn bootstateseed_iter(&self) -> impl Iterator<Item = &Bootstateseed> {
        self.bootstateseed.iter()
    }
    #[doc = "0xe70..0xe90 - boot state hmac register"]
    #[inline(always)]
    pub const fn bootstatehmac(&self, n: usize) -> &Bootstatehmac {
        &self.bootstatehmac[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe70..0xe90 - boot state hmac register"]
    #[inline(always)]
    pub fn bootstatehmac_iter(&self) -> impl Iterator<Item = &Bootstatehmac> {
        self.bootstatehmac.iter()
    }
    #[doc = "0xef8 - FLEXSPI IO pads ctrl register"]
    #[inline(always)]
    pub const fn flexspipadctrl(&self) -> &Flexspipadctrl {
        &self.flexspipadctrl
    }
    #[doc = "0xefc - sdio pad ctrl"]
    #[inline(always)]
    pub const fn sdiopadctl(&self) -> &Sdiopadctl {
        &self.sdiopadctl
    }
    #[doc = "0xf00..0xf20 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub const fn dicehwreg(&self, n: usize) -> &Dicehwreg {
        &self.dicehwreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf00..0xf20 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub fn dicehwreg_iter(&self) -> impl Iterator<Item = &Dicehwreg> {
        self.dicehwreg.iter()
    }
    #[doc = "0xf50..0xf60 - UUIDn 32-Bit Data Register"]
    #[inline(always)]
    pub const fn uuid(&self, n: usize) -> &Uuid {
        &self.uuid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf50..0xf60 - UUIDn 32-Bit Data Register"]
    #[inline(always)]
    pub fn uuid_iter(&self) -> impl Iterator<Item = &Uuid> {
        self.uuid.iter()
    }
    #[doc = "0xf80 - AES key source selection"]
    #[inline(always)]
    pub const fn aeskey_srcsel(&self) -> &AeskeySrcsel {
        &self.aeskey_srcsel
    }
    #[doc = "0xf88 - Hash hardware key disable"]
    #[inline(always)]
    pub const fn hashhwkeydisable(&self) -> &Hashhwkeydisable {
        &self.hashhwkeydisable
    }
    #[doc = "0xfa0 - Debug Write Lock registers"]
    #[inline(always)]
    pub const fn dbg_locken(&self) -> &DbgLocken {
        &self.dbg_locken
    }
    #[doc = "0xfa4 - Debug features control for the CM33"]
    #[inline(always)]
    pub const fn dbg_features(&self) -> &DbgFeatures {
        &self.dbg_features
    }
    #[doc = "0xfa8 - Debug features duplicate"]
    #[inline(always)]
    pub const fn dbg_features_dp(&self) -> &DbgFeaturesDp {
        &self.dbg_features_dp
    }
    #[doc = "0xfac - HW unlock disable"]
    #[inline(always)]
    pub const fn hwunlock_disable(&self) -> &HwunlockDisable {
        &self.hwunlock_disable
    }
    #[doc = "0xfb4 - Code Security for CPU0"]
    #[inline(always)]
    pub const fn cs_protcpu0(&self) -> &CsProtcpu0 {
        &self.cs_protcpu0
    }
    #[doc = "0xfb8 - Code Security for CPU1"]
    #[inline(always)]
    pub const fn cs_protcpu1(&self) -> &CsProtcpu1 {
        &self.cs_protcpu1
    }
    #[doc = "0xfc0 - Debug authorization scratch"]
    #[inline(always)]
    pub const fn dbg_auth_scratch(&self) -> &DbgAuthScratch {
        &self.dbg_auth_scratch
    }
    #[doc = "0xfd0 - Key block"]
    #[inline(always)]
    pub const fn key_block(&self) -> &KeyBlock {
        &self.key_block
    }
}
#[doc = "DSPSTALL (rw) register accessor: DSP stall register\n\nYou can [`read`](crate::Reg::read) this register and get [`dspstall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspstall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dspstall`]
module"]
#[doc(alias = "DSPSTALL")]
pub type Dspstall = crate::Reg<dspstall::DspstallSpec>;
#[doc = "DSP stall register"]
pub mod dspstall;
#[doc = "AHBMATRIXPRIOR (rw) register accessor: AHB matrix priority\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatrixprior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatrixprior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmatrixprior`]
module"]
#[doc(alias = "AHBMATRIXPRIOR")]
pub type Ahbmatrixprior = crate::Reg<ahbmatrixprior::AhbmatrixpriorSpec>;
#[doc = "AHB matrix priority"]
pub mod ahbmatrixprior;
#[doc = "PACKERENABLE (rw) register accessor: Packer enable for DSP RAM packer\n\nYou can [`read`](crate::Reg::read) this register and get [`packerenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packerenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packerenable`]
module"]
#[doc(alias = "PACKERENABLE")]
pub type Packerenable = crate::Reg<packerenable::PackerenableSpec>;
#[doc = "Packer enable for DSP RAM packer"]
pub mod packerenable;
#[doc = "M33NMISRCSEL (rw) register accessor: M33 nmi source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`m33nmisrcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33nmisrcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m33nmisrcsel`]
module"]
#[doc(alias = "M33NMISRCSEL")]
pub type M33nmisrcsel = crate::Reg<m33nmisrcsel::M33nmisrcselSpec>;
#[doc = "M33 nmi source selection"]
pub mod m33nmisrcsel;
#[doc = "SYSTEM_STICK_CALIB (rw) register accessor: system stick calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`system_stick_calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_stick_calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_stick_calib`]
module"]
#[doc(alias = "SYSTEM_STICK_CALIB")]
pub type SystemStickCalib = crate::Reg<system_stick_calib::SystemStickCalibSpec>;
#[doc = "system stick calibration"]
pub mod system_stick_calib;
#[doc = "SYSTEM_NSTICK_CALIB (rw) register accessor: system nstick calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`system_nstick_calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_nstick_calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_nstick_calib`]
module"]
#[doc(alias = "SYSTEM_NSTICK_CALIB")]
pub type SystemNstickCalib = crate::Reg<system_nstick_calib::SystemNstickCalibSpec>;
#[doc = "system nstick calibration"]
pub mod system_nstick_calib;
#[doc = "PRODUCT_ID (r) register accessor: product ID\n\nYou can [`read`](crate::Reg::read) this register and get [`product_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id`]
module"]
#[doc(alias = "PRODUCT_ID")]
pub type ProductId = crate::Reg<product_id::ProductIdSpec>;
#[doc = "product ID"]
pub mod product_id;
#[doc = "SILICONREV_ID (r) register accessor: SILICONREV ID\n\nYou can [`read`](crate::Reg::read) this register and get [`siliconrev_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@siliconrev_id`]
module"]
#[doc(alias = "SILICONREV_ID")]
pub type SiliconrevId = crate::Reg<siliconrev_id::SiliconrevIdSpec>;
#[doc = "SILICONREV ID"]
pub mod siliconrev_id;
#[doc = "JTAG_ID (r) register accessor: jtag ID\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_id`]
module"]
#[doc(alias = "JTAG_ID")]
pub type JtagId = crate::Reg<jtag_id::JtagIdSpec>;
#[doc = "jtag ID"]
pub mod jtag_id;
#[doc = "AUTOCLKGATEOVERRIDE0 (rw) register accessor: auto clock gating override 0\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoclkgateoverride0`]
module"]
#[doc(alias = "AUTOCLKGATEOVERRIDE0")]
pub type Autoclkgateoverride0 = crate::Reg<autoclkgateoverride0::Autoclkgateoverride0Spec>;
#[doc = "auto clock gating override 0"]
pub mod autoclkgateoverride0;
#[doc = "AUTOCLKGATEOVERRIDE1 (rw) register accessor: auto clock gating override 1\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoclkgateoverride1`]
module"]
#[doc(alias = "AUTOCLKGATEOVERRIDE1")]
pub type Autoclkgateoverride1 = crate::Reg<autoclkgateoverride1::Autoclkgateoverride1Spec>;
#[doc = "auto clock gating override 1"]
pub mod autoclkgateoverride1;
#[doc = "CLKGATEOVERRIDE0 (rw) register accessor: Clock gate override 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clkgateoverride0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkgateoverride0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgateoverride0`]
module"]
#[doc(alias = "CLKGATEOVERRIDE0")]
pub type Clkgateoverride0 = crate::Reg<clkgateoverride0::Clkgateoverride0Spec>;
#[doc = "Clock gate override 0"]
pub mod clkgateoverride0;
#[doc = "AHB_SRAM_ACCESS_DISABLE (rw) register accessor: AHB SRAM access disable\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_sram_access_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_sram_access_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_sram_access_disable`]
module"]
#[doc(alias = "AHB_SRAM_ACCESS_DISABLE")]
pub type AhbSramAccessDisable = crate::Reg<ahb_sram_access_disable::AhbSramAccessDisableSpec>;
#[doc = "AHB SRAM access disable"]
pub mod ahb_sram_access_disable;
#[doc = "DSP_SRAM_ACCESS_DISABLE (rw) register accessor: DSP SRAM access disable\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_sram_access_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_sram_access_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_sram_access_disable`]
module"]
#[doc(alias = "DSP_SRAM_ACCESS_DISABLE")]
pub type DspSramAccessDisable = crate::Reg<dsp_sram_access_disable::DspSramAccessDisableSpec>;
#[doc = "DSP SRAM access disable"]
pub mod dsp_sram_access_disable;
#[doc = "AHB_FLEXSPI_ACCESS_DISABLE (rw) register accessor: AHB Flexspi access control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_flexspi_access_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_flexspi_access_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_flexspi_access_disable`]
module"]
#[doc(alias = "AHB_FLEXSPI_ACCESS_DISABLE")]
pub type AhbFlexspiAccessDisable =
    crate::Reg<ahb_flexspi_access_disable::AhbFlexspiAccessDisableSpec>;
#[doc = "AHB Flexspi access control"]
pub mod ahb_flexspi_access_disable;
#[doc = "DSP_FLEXSPI_ACCESS_DISABLE (rw) register accessor: DSP Flexspi access control\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_flexspi_access_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_flexspi_access_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_flexspi_access_disable`]
module"]
#[doc(alias = "DSP_FLEXSPI_ACCESS_DISABLE")]
pub type DspFlexspiAccessDisable =
    crate::Reg<dsp_flexspi_access_disable::DspFlexspiAccessDisableSpec>;
#[doc = "DSP Flexspi access control"]
pub mod dsp_flexspi_access_disable;
#[doc = "FLEXSPI_BOOTROM_SCRATCH0 (rw) register accessor: FLEXSPI NOR flash configure context register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi_bootrom_scratch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi_bootrom_scratch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspi_bootrom_scratch0`]
module"]
#[doc(alias = "FLEXSPI_BOOTROM_SCRATCH0")]
pub type FlexspiBootromScratch0 = crate::Reg<flexspi_bootrom_scratch0::FlexspiBootromScratch0Spec>;
#[doc = "FLEXSPI NOR flash configure context register"]
pub mod flexspi_bootrom_scratch0;
#[doc = "USBCLKCTRL (rw) register accessor: USB clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkctrl`]
module"]
#[doc(alias = "USBCLKCTRL")]
pub type Usbclkctrl = crate::Reg<usbclkctrl::UsbclkctrlSpec>;
#[doc = "USB clock control"]
pub mod usbclkctrl;
#[doc = "USBCLKSTAT (rw) register accessor: USB clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkstat`]
module"]
#[doc(alias = "USBCLKSTAT")]
pub type Usbclkstat = crate::Reg<usbclkstat::UsbclkstatSpec>;
#[doc = "USB clock status"]
pub mod usbclkstat;
#[doc = "USBPHYPLL0LOCKTIMEDIV2 (rw) register accessor: USB PHY PLL0 lock time division 2\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphypll0locktimediv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphypll0locktimediv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphypll0locktimediv2`]
module"]
#[doc(alias = "USBPHYPLL0LOCKTIMEDIV2")]
pub type Usbphypll0locktimediv2 = crate::Reg<usbphypll0locktimediv2::Usbphypll0locktimediv2Spec>;
#[doc = "USB PHY PLL0 lock time division 2"]
pub mod usbphypll0locktimediv2;
#[doc = "PDSLEEPCFG0 (rw) register accessor: Sleep configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsleepcfg0`]
module"]
#[doc(alias = "PDSLEEPCFG0")]
pub type Pdsleepcfg0 = crate::Reg<pdsleepcfg0::Pdsleepcfg0Spec>;
#[doc = "Sleep configuration 0"]
pub mod pdsleepcfg0;
#[doc = "PDSLEEPCFG1 (rw) register accessor: Sleep configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsleepcfg1`]
module"]
#[doc(alias = "PDSLEEPCFG1")]
pub type Pdsleepcfg1 = crate::Reg<pdsleepcfg1::Pdsleepcfg1Spec>;
#[doc = "Sleep configuration 1"]
pub mod pdsleepcfg1;
#[doc = "PDSLEEPCFG2 (rw) register accessor: Sleep configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsleepcfg2`]
module"]
#[doc(alias = "PDSLEEPCFG2")]
pub type Pdsleepcfg2 = crate::Reg<pdsleepcfg2::Pdsleepcfg2Spec>;
#[doc = "Sleep configuration 2"]
pub mod pdsleepcfg2;
#[doc = "PDSLEEPCFG3 (rw) register accessor: Sleep configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsleepcfg3`]
module"]
#[doc(alias = "PDSLEEPCFG3")]
pub type Pdsleepcfg3 = crate::Reg<pdsleepcfg3::Pdsleepcfg3Spec>;
#[doc = "Sleep configuration 3"]
pub mod pdsleepcfg3;
#[doc = "PDRUNCFG0 (rw) register accessor: Run configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg0`]
module"]
#[doc(alias = "PDRUNCFG0")]
pub type Pdruncfg0 = crate::Reg<pdruncfg0::Pdruncfg0Spec>;
#[doc = "Run configuration 0"]
pub mod pdruncfg0;
#[doc = "PDRUNCFG1 (rw) register accessor: Run configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg1`]
module"]
#[doc(alias = "PDRUNCFG1")]
pub type Pdruncfg1 = crate::Reg<pdruncfg1::Pdruncfg1Spec>;
#[doc = "Run configuration 1"]
pub mod pdruncfg1;
#[doc = "PDRUNCFG2 (rw) register accessor: Run configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg2`]
module"]
#[doc(alias = "PDRUNCFG2")]
pub type Pdruncfg2 = crate::Reg<pdruncfg2::Pdruncfg2Spec>;
#[doc = "Run configuration 2"]
pub mod pdruncfg2;
#[doc = "PDRUNCFG3 (rw) register accessor: Run configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg3`]
module"]
#[doc(alias = "PDRUNCFG3")]
pub type Pdruncfg3 = crate::Reg<pdruncfg3::Pdruncfg3Spec>;
#[doc = "Run configuration 3"]
pub mod pdruncfg3;
#[doc = "PDRUNCFG0_SET (w) register accessor: Run configuration 0 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg0_set`]
module"]
#[doc(alias = "PDRUNCFG0_SET")]
pub type Pdruncfg0Set = crate::Reg<pdruncfg0_set::Pdruncfg0SetSpec>;
#[doc = "Run configuration 0 set"]
pub mod pdruncfg0_set;
#[doc = "PDRUNCFG1_SET (w) register accessor: Run configuration 1 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg1_set`]
module"]
#[doc(alias = "PDRUNCFG1_SET")]
pub type Pdruncfg1Set = crate::Reg<pdruncfg1_set::Pdruncfg1SetSpec>;
#[doc = "Run configuration 1 set"]
pub mod pdruncfg1_set;
#[doc = "PDRUNCFG2_SET (w) register accessor: Run configuration 2 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg2_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg2_set`]
module"]
#[doc(alias = "PDRUNCFG2_SET")]
pub type Pdruncfg2Set = crate::Reg<pdruncfg2_set::Pdruncfg2SetSpec>;
#[doc = "Run configuration 2 set"]
pub mod pdruncfg2_set;
#[doc = "PDRUNCFG3_SET (w) register accessor: Run configuration 3 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg3_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg3_set`]
module"]
#[doc(alias = "PDRUNCFG3_SET")]
pub type Pdruncfg3Set = crate::Reg<pdruncfg3_set::Pdruncfg3SetSpec>;
#[doc = "Run configuration 3 set"]
pub mod pdruncfg3_set;
#[doc = "PDRUNCFG0_CLR (w) register accessor: Run configuration 0 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg0_clr`]
module"]
#[doc(alias = "PDRUNCFG0_CLR")]
pub type Pdruncfg0Clr = crate::Reg<pdruncfg0_clr::Pdruncfg0ClrSpec>;
#[doc = "Run configuration 0 clear"]
pub mod pdruncfg0_clr;
#[doc = "PDRUNCFG1_CLR (w) register accessor: Run configuration 1 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg1_clr`]
module"]
#[doc(alias = "PDRUNCFG1_CLR")]
pub type Pdruncfg1Clr = crate::Reg<pdruncfg1_clr::Pdruncfg1ClrSpec>;
#[doc = "Run configuration 1 clear"]
pub mod pdruncfg1_clr;
#[doc = "PDRUNCFG2_CLR (w) register accessor: Run configuration 2 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg2_clr`]
module"]
#[doc(alias = "PDRUNCFG2_CLR")]
pub type Pdruncfg2Clr = crate::Reg<pdruncfg2_clr::Pdruncfg2ClrSpec>;
#[doc = "Run configuration 2 clear"]
pub mod pdruncfg2_clr;
#[doc = "PDRUNCFG3_CLR (w) register accessor: Run configuration 3 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdruncfg3_clr`]
module"]
#[doc(alias = "PDRUNCFG3_CLR")]
pub type Pdruncfg3Clr = crate::Reg<pdruncfg3_clr::Pdruncfg3ClrSpec>;
#[doc = "Run configuration 3 clear"]
pub mod pdruncfg3_clr;
#[doc = "PDWAKECFG (rw) register accessor: PD Wake Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwakecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwakecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwakecfg`]
module"]
#[doc(alias = "PDWAKECFG")]
pub type Pdwakecfg = crate::Reg<pdwakecfg::PdwakecfgSpec>;
#[doc = "PD Wake Configuration"]
pub mod pdwakecfg;
#[doc = "STARTEN0 (rw) register accessor: Start enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`starten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten0`]
module"]
#[doc(alias = "STARTEN0")]
pub type Starten0 = crate::Reg<starten0::Starten0Spec>;
#[doc = "Start enable 0"]
pub mod starten0;
#[doc = "STARTEN1 (rw) register accessor: Start enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`starten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten1`]
module"]
#[doc(alias = "STARTEN1")]
pub type Starten1 = crate::Reg<starten1::Starten1Spec>;
#[doc = "Start enable 1"]
pub mod starten1;
#[doc = "STARTEN0_SET (w) register accessor: Start enable 0 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten0_set`]
module"]
#[doc(alias = "STARTEN0_SET")]
pub type Starten0Set = crate::Reg<starten0_set::Starten0SetSpec>;
#[doc = "Start enable 0 set"]
pub mod starten0_set;
#[doc = "STARTEN1_SET (w) register accessor: Start enable 1 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten1_set`]
module"]
#[doc(alias = "STARTEN1_SET")]
pub type Starten1Set = crate::Reg<starten1_set::Starten1SetSpec>;
#[doc = "Start enable 1 set"]
pub mod starten1_set;
#[doc = "STARTEN0_CLR (w) register accessor: Start enable 0 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten0_clr`]
module"]
#[doc(alias = "STARTEN0_CLR")]
pub type Starten0Clr = crate::Reg<starten0_clr::Starten0ClrSpec>;
#[doc = "Start enable 0 clear"]
pub mod starten0_clr;
#[doc = "STARTEN1_CLR (w) register accessor: Start enable 1 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@starten1_clr`]
module"]
#[doc(alias = "STARTEN1_CLR")]
pub type Starten1Clr = crate::Reg<starten1_clr::Starten1ClrSpec>;
#[doc = "Start enable 1 clear"]
pub mod starten1_clr;
#[doc = "MAINCLKSAFETY (rw) register accessor: Main Clock Safety\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksafety::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksafety::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclksafety`]
module"]
#[doc(alias = "MAINCLKSAFETY")]
pub type Mainclksafety = crate::Reg<mainclksafety::MainclksafetySpec>;
#[doc = "Main Clock Safety"]
pub mod mainclksafety;
#[doc = "HWWAKE (rw) register accessor: Hardware Wake-up control\n\nYou can [`read`](crate::Reg::read) this register and get [`hwwake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwwake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwwake`]
module"]
#[doc(alias = "HWWAKE")]
pub type Hwwake = crate::Reg<hwwake::HwwakeSpec>;
#[doc = "Hardware Wake-up control"]
pub mod hwwake;
#[doc = "TEMPSENSORCTL (rw) register accessor: tempsensor ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`tempsensorctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempsensorctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempsensorctl`]
module"]
#[doc(alias = "TEMPSENSORCTL")]
pub type Tempsensorctl = crate::Reg<tempsensorctl::TempsensorctlSpec>;
#[doc = "tempsensor ctrl"]
pub mod tempsensorctl;
#[doc = "BOOTSTATESEED (rw) register accessor: boot state seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootstateseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootstateseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootstateseed`]
module"]
#[doc(alias = "BOOTSTATESEED")]
pub type Bootstateseed = crate::Reg<bootstateseed::BootstateseedSpec>;
#[doc = "boot state seed register"]
pub mod bootstateseed;
#[doc = "BOOTSTATEHMAC (rw) register accessor: boot state hmac register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootstatehmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootstatehmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootstatehmac`]
module"]
#[doc(alias = "BOOTSTATEHMAC")]
pub type Bootstatehmac = crate::Reg<bootstatehmac::BootstatehmacSpec>;
#[doc = "boot state hmac register"]
pub mod bootstatehmac;
#[doc = "FLEXSPIPADCTRL (rw) register accessor: FLEXSPI IO pads ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspipadctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspipadctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexspipadctrl`]
module"]
#[doc(alias = "FLEXSPIPADCTRL")]
pub type Flexspipadctrl = crate::Reg<flexspipadctrl::FlexspipadctrlSpec>;
#[doc = "FLEXSPI IO pads ctrl register"]
pub mod flexspipadctrl;
#[doc = "SDIOPADCTL (rw) register accessor: sdio pad ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`sdiopadctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdiopadctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdiopadctl`]
module"]
#[doc(alias = "SDIOPADCTL")]
pub type Sdiopadctl = crate::Reg<sdiopadctl::SdiopadctlSpec>;
#[doc = "sdio pad ctrl"]
pub mod sdiopadctl;
#[doc = "DICEHWREG (rw) register accessor: DICE General Purpose 32-Bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dicehwreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dicehwreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dicehwreg`]
module"]
#[doc(alias = "DICEHWREG")]
pub type Dicehwreg = crate::Reg<dicehwreg::DicehwregSpec>;
#[doc = "DICE General Purpose 32-Bit Data Register"]
pub mod dicehwreg;
#[doc = "UUID (r) register accessor: UUIDn 32-Bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid`]
module"]
#[doc(alias = "UUID")]
pub type Uuid = crate::Reg<uuid::UuidSpec>;
#[doc = "UUIDn 32-Bit Data Register"]
pub mod uuid;
#[doc = "AESKEY_SRCSEL (rw) register accessor: AES key source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey_srcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey_srcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey_srcsel`]
module"]
#[doc(alias = "AESKEY_SRCSEL")]
pub type AeskeySrcsel = crate::Reg<aeskey_srcsel::AeskeySrcselSpec>;
#[doc = "AES key source selection"]
pub mod aeskey_srcsel;
#[doc = "HASHHWKEYDISABLE (rw) register accessor: Hash hardware key disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hashhwkeydisable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashhwkeydisable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashhwkeydisable`]
module"]
#[doc(alias = "HASHHWKEYDISABLE")]
pub type Hashhwkeydisable = crate::Reg<hashhwkeydisable::HashhwkeydisableSpec>;
#[doc = "Hash hardware key disable"]
pub mod hashhwkeydisable;
#[doc = "DBG_LOCKEN (rw) register accessor: Debug Write Lock registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_locken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_locken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_locken`]
module"]
#[doc(alias = "DBG_LOCKEN")]
pub type DbgLocken = crate::Reg<dbg_locken::DbgLockenSpec>;
#[doc = "Debug Write Lock registers"]
pub mod dbg_locken;
#[doc = "DBG_FEATURES (rw) register accessor: Debug features control for the CM33\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_features::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_features::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_features`]
module"]
#[doc(alias = "DBG_FEATURES")]
pub type DbgFeatures = crate::Reg<dbg_features::DbgFeaturesSpec>;
#[doc = "Debug features control for the CM33"]
pub mod dbg_features;
#[doc = "DBG_FEATURES_DP (rw) register accessor: Debug features duplicate\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_features_dp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_features_dp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_features_dp`]
module"]
#[doc(alias = "DBG_FEATURES_DP")]
pub type DbgFeaturesDp = crate::Reg<dbg_features_dp::DbgFeaturesDpSpec>;
#[doc = "Debug features duplicate"]
pub mod dbg_features_dp;
#[doc = "HWUNLOCK_DISABLE (rw) register accessor: HW unlock disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hwunlock_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwunlock_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwunlock_disable`]
module"]
#[doc(alias = "HWUNLOCK_DISABLE")]
pub type HwunlockDisable = crate::Reg<hwunlock_disable::HwunlockDisableSpec>;
#[doc = "HW unlock disable"]
pub mod hwunlock_disable;
#[doc = "CS_PROTCPU0 (rw) register accessor: Code Security for CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_protcpu0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs_protcpu0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs_protcpu0`]
module"]
#[doc(alias = "CS_PROTCPU0")]
pub type CsProtcpu0 = crate::Reg<cs_protcpu0::CsProtcpu0Spec>;
#[doc = "Code Security for CPU0"]
pub mod cs_protcpu0;
#[doc = "CS_PROTCPU1 (rw) register accessor: Code Security for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_protcpu1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs_protcpu1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs_protcpu1`]
module"]
#[doc(alias = "CS_PROTCPU1")]
pub type CsProtcpu1 = crate::Reg<cs_protcpu1::CsProtcpu1Spec>;
#[doc = "Code Security for CPU1"]
pub mod cs_protcpu1;
#[doc = "DBG_AUTH_SCRATCH (rw) register accessor: Debug authorization scratch\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_scratch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_scratch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_auth_scratch`]
module"]
#[doc(alias = "DBG_AUTH_SCRATCH")]
pub type DbgAuthScratch = crate::Reg<dbg_auth_scratch::DbgAuthScratchSpec>;
#[doc = "Debug authorization scratch"]
pub mod dbg_auth_scratch;
#[doc = "KEY_BLOCK (rw) register accessor: Key block\n\nYou can [`read`](crate::Reg::read) this register and get [`key_block::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_block::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_block`]
module"]
#[doc(alias = "KEY_BLOCK")]
pub type KeyBlock = crate::Reg<key_block::KeyBlockSpec>;
#[doc = "Key block"]
pub mod key_block;
