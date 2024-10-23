#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwd: Pwd,
    pwd_set: PwdSet,
    pwd_clr: PwdClr,
    pwd_tog: PwdTog,
    tx: Tx,
    tx_set: TxSet,
    tx_clr: TxClr,
    tx_tog: TxTog,
    rx: Rx,
    rx_set: RxSet,
    rx_clr: RxClr,
    rx_tog: RxTog,
    ctrl: Ctrl,
    ctrl_set: CtrlSet,
    ctrl_clr: CtrlClr,
    ctrl_tog: CtrlTog,
    status: Status,
    _reserved17: [u8; 0x0c],
    debug0: Debug0,
    debug0_set: Debug0Set,
    debug0_clr: Debug0Clr,
    debug0_tog: Debug0Tog,
    _reserved21: [u8; 0x10],
    debug1: Debug1,
    debug1_set: Debug1Set,
    debug1_clr: Debug1Clr,
    debug1_tog: Debug1Tog,
    version: Version,
    _reserved26: [u8; 0x1c],
    pll_sic: PllSic,
    pll_sic_set: PllSicSet,
    pll_sic_clr: PllSicClr,
    pll_sic_tog: PllSicTog,
    _reserved30: [u8; 0x10],
    usb1_vbus_detect: Usb1VbusDetect,
    usb1_vbus_detect_set: Usb1VbusDetectSet,
    usb1_vbus_detect_clr: Usb1VbusDetectClr,
    usb1_vbus_detect_tog: Usb1VbusDetectTog,
    usb1_vbus_det_stat: Usb1VbusDetStat,
    _reserved35: [u8; 0x0c],
    usb1_chrg_detect: Usb1ChrgDetect,
    usb1_chrg_detect_set: Usb1ChrgDetectSet,
    usb1_chrg_detect_clr: Usb1ChrgDetectClr,
    usb1_chrg_detect_tog: Usb1ChrgDetectTog,
    usb1_chrg_det_stat: Usb1ChrgDetStat,
    _reserved40: [u8; 0x0c],
    anactrl: Anactrl,
    anactrl_set: AnactrlSet,
    anactrl_clr: AnactrlClr,
    anactrl_tog: AnactrlTog,
    usb1_loopback: Usb1Loopback,
    usb1_loopback_set: Usb1LoopbackSet,
    usb1_loopback_clr: Usb1LoopbackClr,
    usb1_loopback_tog: Usb1LoopbackTog,
    usb1_loopback_hsfscnt: Usb1LoopbackHsfscnt,
    usb1_loopback_hsfscnt_set: Usb1LoopbackHsfscntSet,
    usb1_loopback_hsfscnt_clr: Usb1LoopbackHsfscntClr,
    usb1_loopback_hsfscnt_tog: Usb1LoopbackHsfscntTog,
    trim_override_en: TrimOverrideEn,
    trim_override_en_set: TrimOverrideEnSet,
    trim_override_en_clr: TrimOverrideEnClr,
    trim_override_en_tog: TrimOverrideEnTog,
}
impl RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd(&self) -> &Pwd {
        &self.pwd
    }
    #[doc = "0x04 - USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_set(&self) -> &PwdSet {
        &self.pwd_set
    }
    #[doc = "0x08 - USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_clr(&self) -> &PwdClr {
        &self.pwd_clr
    }
    #[doc = "0x0c - USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_tog(&self) -> &PwdTog {
        &self.pwd_tog
    }
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_set(&self) -> &TxSet {
        &self.tx_set
    }
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_clr(&self) -> &TxClr {
        &self.tx_clr
    }
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_tog(&self) -> &TxTog {
        &self.tx_tog
    }
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_set(&self) -> &RxSet {
        &self.rx_set
    }
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_clr(&self) -> &RxClr {
        &self.rx_clr
    }
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_tog(&self) -> &RxTog {
        &self.rx_tog
    }
    #[doc = "0x30 - USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x34 - USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_set(&self) -> &CtrlSet {
        &self.ctrl_set
    }
    #[doc = "0x38 - USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_clr(&self) -> &CtrlClr {
        &self.ctrl_clr
    }
    #[doc = "0x3c - USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_tog(&self) -> &CtrlTog {
        &self.ctrl_tog
    }
    #[doc = "0x40 - USB PHY Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x50 - USB PHY Debug Register 0"]
    #[inline(always)]
    pub const fn debug0(&self) -> &Debug0 {
        &self.debug0
    }
    #[doc = "0x54 - USB PHY Debug Register 0"]
    #[inline(always)]
    pub const fn debug0_set(&self) -> &Debug0Set {
        &self.debug0_set
    }
    #[doc = "0x58 - USB PHY Debug Register 0"]
    #[inline(always)]
    pub const fn debug0_clr(&self) -> &Debug0Clr {
        &self.debug0_clr
    }
    #[doc = "0x5c - USB PHY Debug Register 0"]
    #[inline(always)]
    pub const fn debug0_tog(&self) -> &Debug0Tog {
        &self.debug0_tog
    }
    #[doc = "0x70 - UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1(&self) -> &Debug1 {
        &self.debug1
    }
    #[doc = "0x74 - UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_set(&self) -> &Debug1Set {
        &self.debug1_set
    }
    #[doc = "0x78 - UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_clr(&self) -> &Debug1Clr {
        &self.debug1_clr
    }
    #[doc = "0x7c - UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_tog(&self) -> &Debug1Tog {
        &self.debug1_tog
    }
    #[doc = "0x80 - UTMI RTL Version"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0xa0 - USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic(&self) -> &PllSic {
        &self.pll_sic
    }
    #[doc = "0xa4 - USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_set(&self) -> &PllSicSet {
        &self.pll_sic_set
    }
    #[doc = "0xa8 - USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_clr(&self) -> &PllSicClr {
        &self.pll_sic_clr
    }
    #[doc = "0xac - USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_tog(&self) -> &PllSicTog {
        &self.pll_sic_tog
    }
    #[doc = "0xc0 - USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect(&self) -> &Usb1VbusDetect {
        &self.usb1_vbus_detect
    }
    #[doc = "0xc4 - USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(&self) -> &Usb1VbusDetectSet {
        &self.usb1_vbus_detect_set
    }
    #[doc = "0xc8 - USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(&self) -> &Usb1VbusDetectClr {
        &self.usb1_vbus_detect_clr
    }
    #[doc = "0xcc - USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(&self) -> &Usb1VbusDetectTog {
        &self.usb1_vbus_detect_tog
    }
    #[doc = "0xd0 - USB PHY VBUS Detector Status Register"]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat(&self) -> &Usb1VbusDetStat {
        &self.usb1_vbus_det_stat
    }
    #[doc = "0xe0 - USB PHY Charger Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect(&self) -> &Usb1ChrgDetect {
        &self.usb1_chrg_detect
    }
    #[doc = "0xe4 - USB PHY Charger Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_set(&self) -> &Usb1ChrgDetectSet {
        &self.usb1_chrg_detect_set
    }
    #[doc = "0xe8 - USB PHY Charger Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_clr(&self) -> &Usb1ChrgDetectClr {
        &self.usb1_chrg_detect_clr
    }
    #[doc = "0xec - USB PHY Charger Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_tog(&self) -> &Usb1ChrgDetectTog {
        &self.usb1_chrg_detect_tog
    }
    #[doc = "0xf0 - USB PHY Charger Detect Status Register"]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat(&self) -> &Usb1ChrgDetStat {
        &self.usb1_chrg_det_stat
    }
    #[doc = "0x100 - USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl(&self) -> &Anactrl {
        &self.anactrl
    }
    #[doc = "0x104 - USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_set(&self) -> &AnactrlSet {
        &self.anactrl_set
    }
    #[doc = "0x108 - USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_clr(&self) -> &AnactrlClr {
        &self.anactrl_clr
    }
    #[doc = "0x10c - USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_tog(&self) -> &AnactrlTog {
        &self.anactrl_tog
    }
    #[doc = "0x110 - USB PHY Loopback Control/Status Register"]
    #[inline(always)]
    pub const fn usb1_loopback(&self) -> &Usb1Loopback {
        &self.usb1_loopback
    }
    #[doc = "0x114 - USB PHY Loopback Control/Status Register"]
    #[inline(always)]
    pub const fn usb1_loopback_set(&self) -> &Usb1LoopbackSet {
        &self.usb1_loopback_set
    }
    #[doc = "0x118 - USB PHY Loopback Control/Status Register"]
    #[inline(always)]
    pub const fn usb1_loopback_clr(&self) -> &Usb1LoopbackClr {
        &self.usb1_loopback_clr
    }
    #[doc = "0x11c - USB PHY Loopback Control/Status Register"]
    #[inline(always)]
    pub const fn usb1_loopback_tog(&self) -> &Usb1LoopbackTog {
        &self.usb1_loopback_tog
    }
    #[doc = "0x120 - USB PHY Loopback Packet Number Select Register"]
    #[inline(always)]
    pub const fn usb1_loopback_hsfscnt(&self) -> &Usb1LoopbackHsfscnt {
        &self.usb1_loopback_hsfscnt
    }
    #[doc = "0x124 - USB PHY Loopback Packet Number Select Register"]
    #[inline(always)]
    pub const fn usb1_loopback_hsfscnt_set(&self) -> &Usb1LoopbackHsfscntSet {
        &self.usb1_loopback_hsfscnt_set
    }
    #[doc = "0x128 - USB PHY Loopback Packet Number Select Register"]
    #[inline(always)]
    pub const fn usb1_loopback_hsfscnt_clr(&self) -> &Usb1LoopbackHsfscntClr {
        &self.usb1_loopback_hsfscnt_clr
    }
    #[doc = "0x12c - USB PHY Loopback Packet Number Select Register"]
    #[inline(always)]
    pub const fn usb1_loopback_hsfscnt_tog(&self) -> &Usb1LoopbackHsfscntTog {
        &self.usb1_loopback_hsfscnt_tog
    }
    #[doc = "0x130 - USB PHY Trim Override Enable Register"]
    #[inline(always)]
    pub const fn trim_override_en(&self) -> &TrimOverrideEn {
        &self.trim_override_en
    }
    #[doc = "0x134 - USB PHY Trim Override Enable Register"]
    #[inline(always)]
    pub const fn trim_override_en_set(&self) -> &TrimOverrideEnSet {
        &self.trim_override_en_set
    }
    #[doc = "0x138 - USB PHY Trim Override Enable Register"]
    #[inline(always)]
    pub const fn trim_override_en_clr(&self) -> &TrimOverrideEnClr {
        &self.trim_override_en_clr
    }
    #[doc = "0x13c - USB PHY Trim Override Enable Register"]
    #[inline(always)]
    pub const fn trim_override_en_tog(&self) -> &TrimOverrideEnTog {
        &self.trim_override_en_tog
    }
}
#[doc = "PWD (rw) register accessor: USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwd`]
module"]
#[doc(alias = "PWD")]
pub type Pwd = crate::Reg<pwd::PwdSpec>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "PWD_SET (rw) register accessor: USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwd_set`]
module"]
#[doc(alias = "PWD_SET")]
pub type PwdSet = crate::Reg<pwd_set::PwdSetSpec>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "PWD_CLR (rw) register accessor: USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwd_clr`]
module"]
#[doc(alias = "PWD_CLR")]
pub type PwdClr = crate::Reg<pwd_clr::PwdClrSpec>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "PWD_TOG (rw) register accessor: USB PHY Power-Down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwd_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwd_tog`]
module"]
#[doc(alias = "PWD_TOG")]
pub type PwdTog = crate::Reg<pwd_tog::PwdTogSpec>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "TX (rw) register accessor: USB PHY Transmitter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx`]
module"]
#[doc(alias = "TX")]
pub type Tx = crate::Reg<tx::TxSpec>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "TX_SET (rw) register accessor: USB PHY Transmitter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_set`]
module"]
#[doc(alias = "TX_SET")]
pub type TxSet = crate::Reg<tx_set::TxSetSpec>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "TX_CLR (rw) register accessor: USB PHY Transmitter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_clr`]
module"]
#[doc(alias = "TX_CLR")]
pub type TxClr = crate::Reg<tx_clr::TxClrSpec>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "TX_TOG (rw) register accessor: USB PHY Transmitter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_tog`]
module"]
#[doc(alias = "TX_TOG")]
pub type TxTog = crate::Reg<tx_tog::TxTogSpec>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "RX (rw) register accessor: USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`]
module"]
#[doc(alias = "RX")]
pub type Rx = crate::Reg<rx::RxSpec>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "RX_SET (rw) register accessor: USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_set`]
module"]
#[doc(alias = "RX_SET")]
pub type RxSet = crate::Reg<rx_set::RxSetSpec>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "RX_CLR (rw) register accessor: USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_clr`]
module"]
#[doc(alias = "RX_CLR")]
pub type RxClr = crate::Reg<rx_clr::RxClrSpec>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "RX_TOG (rw) register accessor: USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_tog`]
module"]
#[doc(alias = "RX_TOG")]
pub type RxTog = crate::Reg<rx_tog::RxTogSpec>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "CTRL (rw) register accessor: USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "CTRL_SET (rw) register accessor: USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_set`]
module"]
#[doc(alias = "CTRL_SET")]
pub type CtrlSet = crate::Reg<ctrl_set::CtrlSetSpec>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "CTRL_CLR (rw) register accessor: USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_clr`]
module"]
#[doc(alias = "CTRL_CLR")]
pub type CtrlClr = crate::Reg<ctrl_clr::CtrlClrSpec>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG (rw) register accessor: USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_tog`]
module"]
#[doc(alias = "CTRL_TOG")]
pub type CtrlTog = crate::Reg<ctrl_tog::CtrlTogSpec>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "STATUS (rw) register accessor: USB PHY Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "DEBUG0 (rw) register accessor: USB PHY Debug Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug0`]
module"]
#[doc(alias = "DEBUG0")]
pub type Debug0 = crate::Reg<debug0::Debug0Spec>;
#[doc = "USB PHY Debug Register 0"]
pub mod debug0;
#[doc = "DEBUG0_SET (rw) register accessor: USB PHY Debug Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug0_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug0_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug0_set`]
module"]
#[doc(alias = "DEBUG0_SET")]
pub type Debug0Set = crate::Reg<debug0_set::Debug0SetSpec>;
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_set;
#[doc = "DEBUG0_CLR (rw) register accessor: USB PHY Debug Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug0_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug0_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug0_clr`]
module"]
#[doc(alias = "DEBUG0_CLR")]
pub type Debug0Clr = crate::Reg<debug0_clr::Debug0ClrSpec>;
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_clr;
#[doc = "DEBUG0_TOG (rw) register accessor: USB PHY Debug Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug0_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug0_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug0_tog`]
module"]
#[doc(alias = "DEBUG0_TOG")]
pub type Debug0Tog = crate::Reg<debug0_tog::Debug0TogSpec>;
#[doc = "USB PHY Debug Register 0"]
pub mod debug0_tog;
#[doc = "DEBUG1 (rw) register accessor: UTMI Debug Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug1`]
module"]
#[doc(alias = "DEBUG1")]
pub type Debug1 = crate::Reg<debug1::Debug1Spec>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1;
#[doc = "DEBUG1_SET (rw) register accessor: UTMI Debug Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug1_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug1_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug1_set`]
module"]
#[doc(alias = "DEBUG1_SET")]
pub type Debug1Set = crate::Reg<debug1_set::Debug1SetSpec>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_set;
#[doc = "DEBUG1_CLR (rw) register accessor: UTMI Debug Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug1_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug1_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug1_clr`]
module"]
#[doc(alias = "DEBUG1_CLR")]
pub type Debug1Clr = crate::Reg<debug1_clr::Debug1ClrSpec>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_clr;
#[doc = "DEBUG1_TOG (rw) register accessor: UTMI Debug Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug1_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug1_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug1_tog`]
module"]
#[doc(alias = "DEBUG1_TOG")]
pub type Debug1Tog = crate::Reg<debug1_tog::Debug1TogSpec>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_tog;
#[doc = "VERSION (r) register accessor: UTMI RTL Version\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "UTMI RTL Version"]
pub mod version;
#[doc = "PLL_SIC (rw) register accessor: USB PHY PLL Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_sic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_sic`]
module"]
#[doc(alias = "PLL_SIC")]
pub type PllSic = crate::Reg<pll_sic::PllSicSpec>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic;
#[doc = "PLL_SIC_SET (rw) register accessor: USB PHY PLL Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_sic_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sic_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_sic_set`]
module"]
#[doc(alias = "PLL_SIC_SET")]
pub type PllSicSet = crate::Reg<pll_sic_set::PllSicSetSpec>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_set;
#[doc = "PLL_SIC_CLR (rw) register accessor: USB PHY PLL Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_sic_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sic_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_sic_clr`]
module"]
#[doc(alias = "PLL_SIC_CLR")]
pub type PllSicClr = crate::Reg<pll_sic_clr::PllSicClrSpec>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_clr;
#[doc = "PLL_SIC_TOG (rw) register accessor: USB PHY PLL Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_sic_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sic_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_sic_tog`]
module"]
#[doc(alias = "PLL_SIC_TOG")]
pub type PllSicTog = crate::Reg<pll_sic_tog::PllSicTogSpec>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_tog;
#[doc = "USB1_VBUS_DETECT (rw) register accessor: USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_vbus_detect`]
module"]
#[doc(alias = "USB1_VBUS_DETECT")]
pub type Usb1VbusDetect = crate::Reg<usb1_vbus_detect::Usb1VbusDetectSpec>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect;
#[doc = "USB1_VBUS_DETECT_SET (rw) register accessor: USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_vbus_detect_set`]
module"]
#[doc(alias = "USB1_VBUS_DETECT_SET")]
pub type Usb1VbusDetectSet = crate::Reg<usb1_vbus_detect_set::Usb1VbusDetectSetSpec>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB1_VBUS_DETECT_CLR (rw) register accessor: USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_vbus_detect_clr`]
module"]
#[doc(alias = "USB1_VBUS_DETECT_CLR")]
pub type Usb1VbusDetectClr = crate::Reg<usb1_vbus_detect_clr::Usb1VbusDetectClrSpec>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB1_VBUS_DETECT_TOG (rw) register accessor: USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_vbus_detect_tog`]
module"]
#[doc(alias = "USB1_VBUS_DETECT_TOG")]
pub type Usb1VbusDetectTog = crate::Reg<usb1_vbus_detect_tog::Usb1VbusDetectTogSpec>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB1_VBUS_DET_STAT (r) register accessor: USB PHY VBUS Detector Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_det_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_vbus_det_stat`]
module"]
#[doc(alias = "USB1_VBUS_DET_STAT")]
pub type Usb1VbusDetStat = crate::Reg<usb1_vbus_det_stat::Usb1VbusDetStatSpec>;
#[doc = "USB PHY VBUS Detector Status Register"]
pub mod usb1_vbus_det_stat;
#[doc = "USB1_CHRG_DETECT (rw) register accessor: USB PHY Charger Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_detect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_chrg_detect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_chrg_detect`]
module"]
#[doc(alias = "USB1_CHRG_DETECT")]
pub type Usb1ChrgDetect = crate::Reg<usb1_chrg_detect::Usb1ChrgDetectSpec>;
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect;
#[doc = "USB1_CHRG_DETECT_SET (rw) register accessor: USB PHY Charger Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_detect_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_chrg_detect_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_chrg_detect_set`]
module"]
#[doc(alias = "USB1_CHRG_DETECT_SET")]
pub type Usb1ChrgDetectSet = crate::Reg<usb1_chrg_detect_set::Usb1ChrgDetectSetSpec>;
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_set;
#[doc = "USB1_CHRG_DETECT_CLR (rw) register accessor: USB PHY Charger Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_detect_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_chrg_detect_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_chrg_detect_clr`]
module"]
#[doc(alias = "USB1_CHRG_DETECT_CLR")]
pub type Usb1ChrgDetectClr = crate::Reg<usb1_chrg_detect_clr::Usb1ChrgDetectClrSpec>;
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB1_CHRG_DETECT_TOG (rw) register accessor: USB PHY Charger Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_detect_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_chrg_detect_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_chrg_detect_tog`]
module"]
#[doc(alias = "USB1_CHRG_DETECT_TOG")]
pub type Usb1ChrgDetectTog = crate::Reg<usb1_chrg_detect_tog::Usb1ChrgDetectTogSpec>;
#[doc = "USB PHY Charger Detect Control Register"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB1_CHRG_DET_STAT (r) register accessor: USB PHY Charger Detect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_det_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_chrg_det_stat`]
module"]
#[doc(alias = "USB1_CHRG_DET_STAT")]
pub type Usb1ChrgDetStat = crate::Reg<usb1_chrg_det_stat::Usb1ChrgDetStatSpec>;
#[doc = "USB PHY Charger Detect Status Register"]
pub mod usb1_chrg_det_stat;
#[doc = "ANACTRL (rw) register accessor: USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl`]
module"]
#[doc(alias = "ANACTRL")]
pub type Anactrl = crate::Reg<anactrl::AnactrlSpec>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl;
#[doc = "ANACTRL_SET (rw) register accessor: USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl_set`]
module"]
#[doc(alias = "ANACTRL_SET")]
pub type AnactrlSet = crate::Reg<anactrl_set::AnactrlSetSpec>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_set;
#[doc = "ANACTRL_CLR (rw) register accessor: USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl_clr`]
module"]
#[doc(alias = "ANACTRL_CLR")]
pub type AnactrlClr = crate::Reg<anactrl_clr::AnactrlClrSpec>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_clr;
#[doc = "ANACTRL_TOG (rw) register accessor: USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl_tog`]
module"]
#[doc(alias = "ANACTRL_TOG")]
pub type AnactrlTog = crate::Reg<anactrl_tog::AnactrlTogSpec>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_tog;
#[doc = "USB1_LOOPBACK (rw) register accessor: USB PHY Loopback Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback`]
module"]
#[doc(alias = "USB1_LOOPBACK")]
pub type Usb1Loopback = crate::Reg<usb1_loopback::Usb1LoopbackSpec>;
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback;
#[doc = "USB1_LOOPBACK_SET (rw) register accessor: USB PHY Loopback Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_set`]
module"]
#[doc(alias = "USB1_LOOPBACK_SET")]
pub type Usb1LoopbackSet = crate::Reg<usb1_loopback_set::Usb1LoopbackSetSpec>;
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_set;
#[doc = "USB1_LOOPBACK_CLR (rw) register accessor: USB PHY Loopback Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_clr`]
module"]
#[doc(alias = "USB1_LOOPBACK_CLR")]
pub type Usb1LoopbackClr = crate::Reg<usb1_loopback_clr::Usb1LoopbackClrSpec>;
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_clr;
#[doc = "USB1_LOOPBACK_TOG (rw) register accessor: USB PHY Loopback Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_tog`]
module"]
#[doc(alias = "USB1_LOOPBACK_TOG")]
pub type Usb1LoopbackTog = crate::Reg<usb1_loopback_tog::Usb1LoopbackTogSpec>;
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_tog;
#[doc = "USB1_LOOPBACK_HSFSCNT (rw) register accessor: USB PHY Loopback Packet Number Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_hsfscnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_hsfscnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_hsfscnt`]
module"]
#[doc(alias = "USB1_LOOPBACK_HSFSCNT")]
pub type Usb1LoopbackHsfscnt = crate::Reg<usb1_loopback_hsfscnt::Usb1LoopbackHsfscntSpec>;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt;
#[doc = "USB1_LOOPBACK_HSFSCNT_SET (rw) register accessor: USB PHY Loopback Packet Number Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_hsfscnt_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_hsfscnt_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_hsfscnt_set`]
module"]
#[doc(alias = "USB1_LOOPBACK_HSFSCNT_SET")]
pub type Usb1LoopbackHsfscntSet = crate::Reg<usb1_loopback_hsfscnt_set::Usb1LoopbackHsfscntSetSpec>;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_set;
#[doc = "USB1_LOOPBACK_HSFSCNT_CLR (rw) register accessor: USB PHY Loopback Packet Number Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_hsfscnt_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_hsfscnt_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_hsfscnt_clr`]
module"]
#[doc(alias = "USB1_LOOPBACK_HSFSCNT_CLR")]
pub type Usb1LoopbackHsfscntClr = crate::Reg<usb1_loopback_hsfscnt_clr::Usb1LoopbackHsfscntClrSpec>;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_clr;
#[doc = "USB1_LOOPBACK_HSFSCNT_TOG (rw) register accessor: USB PHY Loopback Packet Number Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_hsfscnt_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_hsfscnt_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_loopback_hsfscnt_tog`]
module"]
#[doc(alias = "USB1_LOOPBACK_HSFSCNT_TOG")]
pub type Usb1LoopbackHsfscntTog = crate::Reg<usb1_loopback_hsfscnt_tog::Usb1LoopbackHsfscntTogSpec>;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_tog;
#[doc = "TRIM_OVERRIDE_EN (rw) register accessor: USB PHY Trim Override Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_override_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_override_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_override_en`]
module"]
#[doc(alias = "TRIM_OVERRIDE_EN")]
pub type TrimOverrideEn = crate::Reg<trim_override_en::TrimOverrideEnSpec>;
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en;
#[doc = "TRIM_OVERRIDE_EN_SET (rw) register accessor: USB PHY Trim Override Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_override_en_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_override_en_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_override_en_set`]
module"]
#[doc(alias = "TRIM_OVERRIDE_EN_SET")]
pub type TrimOverrideEnSet = crate::Reg<trim_override_en_set::TrimOverrideEnSetSpec>;
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_set;
#[doc = "TRIM_OVERRIDE_EN_CLR (rw) register accessor: USB PHY Trim Override Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_override_en_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_override_en_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_override_en_clr`]
module"]
#[doc(alias = "TRIM_OVERRIDE_EN_CLR")]
pub type TrimOverrideEnClr = crate::Reg<trim_override_en_clr::TrimOverrideEnClrSpec>;
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_clr;
#[doc = "TRIM_OVERRIDE_EN_TOG (rw) register accessor: USB PHY Trim Override Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_override_en_tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_override_en_tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_override_en_tog`]
module"]
#[doc(alias = "TRIM_OVERRIDE_EN_TOG")]
pub type TrimOverrideEnTog = crate::Reg<trim_override_en_tog::TrimOverrideEnTogSpec>;
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_tog;
