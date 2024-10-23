#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    otp_shadow: [OtpShadow; 496],
    _reserved1: [u8; 0x40],
    otp_ctrl: OtpCtrl,
    otp_pdn: OtpPdn,
    otp_write_data: OtpWriteData,
    otp_read_ctrl: OtpReadCtrl,
    otp_read_data: OtpReadData,
    otp_clk_div: OtpClkDiv,
    _reserved7: [u8; 0x04],
    otp_crc_addr: OtpCrcAddr,
    otp_crc_value: OtpCrcValue,
    otp_status: OtpStatus,
    _reserved10: [u8; 0x04],
    otp_version: OtpVersion,
}
impl RegisterBlock {
    #[doc = "0x00..0x7c0 - OTP shadow register N"]
    #[inline(always)]
    pub const fn otp_shadow(&self, n: usize) -> &OtpShadow {
        &self.otp_shadow[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x7c0 - OTP shadow register N"]
    #[inline(always)]
    pub fn otp_shadow_iter(&self) -> impl Iterator<Item = &OtpShadow> {
        self.otp_shadow.iter()
    }
    #[doc = "0x800 - Control/address register"]
    #[inline(always)]
    pub const fn otp_ctrl(&self) -> &OtpCtrl {
        &self.otp_ctrl
    }
    #[doc = "0x804 - Power-down register"]
    #[inline(always)]
    pub const fn otp_pdn(&self) -> &OtpPdn {
        &self.otp_pdn
    }
    #[doc = "0x808 - OTP programming data register"]
    #[inline(always)]
    pub const fn otp_write_data(&self) -> &OtpWriteData {
        &self.otp_write_data
    }
    #[doc = "0x80c - OTP read start register"]
    #[inline(always)]
    pub const fn otp_read_ctrl(&self) -> &OtpReadCtrl {
        &self.otp_read_ctrl
    }
    #[doc = "0x810 - OTP read data register"]
    #[inline(always)]
    pub const fn otp_read_data(&self) -> &OtpReadData {
        &self.otp_read_data
    }
    #[doc = "0x814 - OTP clock divider register"]
    #[inline(always)]
    pub const fn otp_clk_div(&self) -> &OtpClkDiv {
        &self.otp_clk_div
    }
    #[doc = "0x81c - CRC address range register"]
    #[inline(always)]
    pub const fn otp_crc_addr(&self) -> &OtpCrcAddr {
        &self.otp_crc_addr
    }
    #[doc = "0x820 - CRC result register"]
    #[inline(always)]
    pub const fn otp_crc_value(&self) -> &OtpCrcValue {
        &self.otp_crc_value
    }
    #[doc = "0x824 - Status register"]
    #[inline(always)]
    pub const fn otp_status(&self) -> &OtpStatus {
        &self.otp_status
    }
    #[doc = "0x82c - VERSION ID register"]
    #[inline(always)]
    pub const fn otp_version(&self) -> &OtpVersion {
        &self.otp_version
    }
}
#[doc = "OTP_SHADOW (rw) register accessor: OTP shadow register N\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_shadow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_shadow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_shadow`]
module"]
#[doc(alias = "OTP_SHADOW")]
pub type OtpShadow = crate::Reg<otp_shadow::OtpShadowSpec>;
#[doc = "OTP shadow register N"]
pub mod otp_shadow;
#[doc = "OTP_CTRL (rw) register accessor: Control/address register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_ctrl`]
module"]
#[doc(alias = "OTP_CTRL")]
pub type OtpCtrl = crate::Reg<otp_ctrl::OtpCtrlSpec>;
#[doc = "Control/address register"]
pub mod otp_ctrl;
#[doc = "OTP_PDN (rw) register accessor: Power-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_pdn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_pdn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_pdn`]
module"]
#[doc(alias = "OTP_PDN")]
pub type OtpPdn = crate::Reg<otp_pdn::OtpPdnSpec>;
#[doc = "Power-down register"]
pub mod otp_pdn;
#[doc = "OTP_WRITE_DATA (r) register accessor: OTP programming data register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_write_data`]
module"]
#[doc(alias = "OTP_WRITE_DATA")]
pub type OtpWriteData = crate::Reg<otp_write_data::OtpWriteDataSpec>;
#[doc = "OTP programming data register"]
pub mod otp_write_data;
#[doc = "OTP_READ_CTRL (rw) register accessor: OTP read start register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_read_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_read_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_read_ctrl`]
module"]
#[doc(alias = "OTP_READ_CTRL")]
pub type OtpReadCtrl = crate::Reg<otp_read_ctrl::OtpReadCtrlSpec>;
#[doc = "OTP read start register"]
pub mod otp_read_ctrl;
#[doc = "OTP_READ_DATA (r) register accessor: OTP read data register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_read_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_read_data`]
module"]
#[doc(alias = "OTP_READ_DATA")]
pub type OtpReadData = crate::Reg<otp_read_data::OtpReadDataSpec>;
#[doc = "OTP read data register"]
pub mod otp_read_data;
#[doc = "OTP_CLK_DIV (rw) register accessor: OTP clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_clk_div`]
module"]
#[doc(alias = "OTP_CLK_DIV")]
pub type OtpClkDiv = crate::Reg<otp_clk_div::OtpClkDivSpec>;
#[doc = "OTP clock divider register"]
pub mod otp_clk_div;
#[doc = "OTP_CRC_ADDR (rw) register accessor: CRC address range register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_crc_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_crc_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_crc_addr`]
module"]
#[doc(alias = "OTP_CRC_ADDR")]
pub type OtpCrcAddr = crate::Reg<otp_crc_addr::OtpCrcAddrSpec>;
#[doc = "CRC address range register"]
pub mod otp_crc_addr;
#[doc = "OTP_CRC_VALUE (rw) register accessor: CRC result register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_crc_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_crc_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_crc_value`]
module"]
#[doc(alias = "OTP_CRC_VALUE")]
pub type OtpCrcValue = crate::Reg<otp_crc_value::OtpCrcValueSpec>;
#[doc = "CRC result register"]
pub mod otp_crc_value;
#[doc = "OTP_STATUS (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_status`]
module"]
#[doc(alias = "OTP_STATUS")]
pub type OtpStatus = crate::Reg<otp_status::OtpStatusSpec>;
#[doc = "Status register"]
pub mod otp_status;
#[doc = "OTP_VERSION (r) register accessor: VERSION ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_version`]
module"]
#[doc(alias = "OTP_VERSION")]
pub type OtpVersion = crate::Reg<otp_version::OtpVersionSpec>;
#[doc = "VERSION ID register"]
pub mod otp_version;
