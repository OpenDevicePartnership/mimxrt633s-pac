#[doc = "Register `PSCCTL0` reader"]
pub type R = crate::R<Pscctl0Spec>;
#[doc = "Register `PSCCTL0` writer"]
pub type W = crate::W<Pscctl0Spec>;
#[doc = "128KB ROM control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomCtl128kb {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<RomCtl128kb> for bool {
    #[inline(always)]
    fn from(variant: RomCtl128kb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_CTL_128KB` reader - 128KB ROM control"]
pub type RomCtl128kbR = crate::BitReader<RomCtl128kb>;
impl RomCtl128kbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RomCtl128kb {
        match self.bits {
            false => RomCtl128kb::DisableClock,
            true => RomCtl128kb::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == RomCtl128kb::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == RomCtl128kb::EnableClock
    }
}
#[doc = "Field `ROM_CTL_128KB` writer - 128KB ROM control"]
pub type RomCtl128kbW<'a, REG> = crate::BitWriter<'a, REG, RomCtl128kb>;
impl<'a, REG> RomCtl128kbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RomCtl128kb::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RomCtl128kb::EnableClock)
    }
}
#[doc = "powerquad clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerquadClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<PowerquadClk> for bool {
    #[inline(always)]
    fn from(variant: PowerquadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD_CLK` reader - powerquad clock control"]
pub type PowerquadClkR = crate::BitReader<PowerquadClk>;
impl PowerquadClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerquadClk {
        match self.bits {
            false => PowerquadClk::DisableClock,
            true => PowerquadClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == PowerquadClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == PowerquadClk::EnableClock
    }
}
#[doc = "Field `POWERQUAD_CLK` writer - powerquad clock control"]
pub type PowerquadClkW<'a, REG> = crate::BitWriter<'a, REG, PowerquadClk>;
impl<'a, REG> PowerquadClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadClk::EnableClock)
    }
}
#[doc = "CAPSER clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<CasperClk> for bool {
    #[inline(always)]
    fn from(variant: CasperClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_CLK` reader - CAPSER clock control"]
pub type CasperClkR = crate::BitReader<CasperClk>;
impl CasperClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperClk {
        match self.bits {
            false => CasperClk::DisableClock,
            true => CasperClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == CasperClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == CasperClk::EnableClock
    }
}
#[doc = "Field `CASPER_CLK` writer - CAPSER clock control"]
pub type CasperClkW<'a, REG> = crate::BitWriter<'a, REG, CasperClk>;
impl<'a, REG> CasperClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CasperClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CasperClk::EnableClock)
    }
}
#[doc = "HASHCRYPT clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashcryptClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<HashcryptClk> for bool {
    #[inline(always)]
    fn from(variant: HashcryptClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT_CLK` reader - HASHCRYPT clock control"]
pub type HashcryptClkR = crate::BitReader<HashcryptClk>;
impl HashcryptClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashcryptClk {
        match self.bits {
            false => HashcryptClk::DisableClock,
            true => HashcryptClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == HashcryptClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == HashcryptClk::EnableClock
    }
}
#[doc = "Field `HASHCRYPT_CLK` writer - HASHCRYPT clock control"]
pub type HashcryptClkW<'a, REG> = crate::BitWriter<'a, REG, HashcryptClk>;
impl<'a, REG> HashcryptClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(HashcryptClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(HashcryptClk::EnableClock)
    }
}
#[doc = "PUF clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PufClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<PufClk> for bool {
    #[inline(always)]
    fn from(variant: PufClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_CLK` reader - PUF clock control"]
pub type PufClkR = crate::BitReader<PufClk>;
impl PufClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PufClk {
        match self.bits {
            false => PufClk::DisableClock,
            true => PufClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == PufClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == PufClk::EnableClock
    }
}
#[doc = "Field `PUF_CLK` writer - PUF clock control"]
pub type PufClkW<'a, REG> = crate::BitWriter<'a, REG, PufClk>;
impl<'a, REG> PufClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PufClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PufClk::EnableClock)
    }
}
#[doc = "RNG clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<RngClk> for bool {
    #[inline(always)]
    fn from(variant: RngClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_CLK` reader - RNG clock control"]
pub type RngClkR = crate::BitReader<RngClk>;
impl RngClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RngClk {
        match self.bits {
            false => RngClk::DisableClock,
            true => RngClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == RngClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == RngClk::EnableClock
    }
}
#[doc = "Field `RNG_CLK` writer - RNG clock control"]
pub type RngClkW<'a, REG> = crate::BitWriter<'a, REG, RngClk>;
impl<'a, REG> RngClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RngClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RngClk::EnableClock)
    }
}
#[doc = "FLEXSPI clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfadClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<FlexspiOtfadClk> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD_CLK` reader - FLEXSPI clock control"]
pub type FlexspiOtfadClkR = crate::BitReader<FlexspiOtfadClk>;
impl FlexspiOtfadClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlexspiOtfadClk {
        match self.bits {
            false => FlexspiOtfadClk::DisableClock,
            true => FlexspiOtfadClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == FlexspiOtfadClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == FlexspiOtfadClk::EnableClock
    }
}
#[doc = "Field `FLEXSPI_OTFAD_CLK` writer - FLEXSPI clock control"]
pub type FlexspiOtfadClkW<'a, REG> = crate::BitWriter<'a, REG, FlexspiOtfadClk>;
impl<'a, REG> FlexspiOtfadClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfadClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfadClk::EnableClock)
    }
}
#[doc = "OTP clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<OtpClk> for bool {
    #[inline(always)]
    fn from(variant: OtpClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP_CLK` reader - OTP clock control"]
pub type OtpClkR = crate::BitReader<OtpClk>;
impl OtpClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpClk {
        match self.bits {
            false => OtpClk::DisableClock,
            true => OtpClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == OtpClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == OtpClk::EnableClock
    }
}
#[doc = "Field `OTP_CLK` writer - OTP clock control"]
pub type OtpClkW<'a, REG> = crate::BitWriter<'a, REG, OtpClk>;
impl<'a, REG> OtpClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OtpClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OtpClk::EnableClock)
    }
}
#[doc = "USB PHY clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhyClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<UsbhsPhyClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhyClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY_CLK` reader - USB PHY clock control"]
pub type UsbhsPhyClkR = crate::BitReader<UsbhsPhyClk>;
impl UsbhsPhyClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsPhyClk {
        match self.bits {
            false => UsbhsPhyClk::DisableClock,
            true => UsbhsPhyClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == UsbhsPhyClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == UsbhsPhyClk::EnableClock
    }
}
#[doc = "Field `USBHS_PHY_CLK` writer - USB PHY clock control"]
pub type UsbhsPhyClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsPhyClk>;
impl<'a, REG> UsbhsPhyClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhyClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhyClk::EnableClock)
    }
}
#[doc = "USB DEVICE clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDeviceClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<UsbhsDeviceClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDeviceClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` reader - USB DEVICE clock control"]
pub type UsbhsDeviceClkR = crate::BitReader<UsbhsDeviceClk>;
impl UsbhsDeviceClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsDeviceClk {
        match self.bits {
            false => UsbhsDeviceClk::DisableClock,
            true => UsbhsDeviceClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == UsbhsDeviceClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == UsbhsDeviceClk::EnableClock
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` writer - USB DEVICE clock control"]
pub type UsbhsDeviceClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsDeviceClk>;
impl<'a, REG> UsbhsDeviceClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDeviceClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDeviceClk::EnableClock)
    }
}
#[doc = "USB HOST clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHostClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<UsbhsHostClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHostClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST_CLK` reader - USB HOST clock control"]
pub type UsbhsHostClkR = crate::BitReader<UsbhsHostClk>;
impl UsbhsHostClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsHostClk {
        match self.bits {
            false => UsbhsHostClk::DisableClock,
            true => UsbhsHostClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == UsbhsHostClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == UsbhsHostClk::EnableClock
    }
}
#[doc = "Field `USBHS_HOST_CLK` writer - USB HOST clock control"]
pub type UsbhsHostClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsHostClk>;
impl<'a, REG> UsbhsHostClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHostClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHostClk::EnableClock)
    }
}
#[doc = "USBHS RAM clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<UsbhsSramClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_CLK` reader - USBHS RAM clock control"]
pub type UsbhsSramClkR = crate::BitReader<UsbhsSramClk>;
impl UsbhsSramClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsSramClk {
        match self.bits {
            false => UsbhsSramClk::DisableClock,
            true => UsbhsSramClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == UsbhsSramClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == UsbhsSramClk::EnableClock
    }
}
#[doc = "Field `USBHS_SRAM_CLK` writer - USBHS RAM clock control"]
pub type UsbhsSramClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramClk>;
impl<'a, REG> UsbhsSramClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramClk::EnableClock)
    }
}
#[doc = "SCT clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<SctClk> for bool {
    #[inline(always)]
    fn from(variant: SctClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_CLK` reader - SCT clock control"]
pub type SctClkR = crate::BitReader<SctClk>;
impl SctClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctClk {
        match self.bits {
            false => SctClk::DisableClock,
            true => SctClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == SctClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == SctClk::EnableClock
    }
}
#[doc = "Field `SCT_CLK` writer - SCT clock control"]
pub type SctClkW<'a, REG> = crate::BitWriter<'a, REG, SctClk>;
impl<'a, REG> SctClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SctClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SctClk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 2 - 128KB ROM control"]
    #[inline(always)]
    pub fn rom_ctl_128kb(&self) -> RomCtl128kbR {
        RomCtl128kbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - powerquad clock control"]
    #[inline(always)]
    pub fn powerquad_clk(&self) -> PowerquadClkR {
        PowerquadClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAPSER clock control"]
    #[inline(always)]
    pub fn casper_clk(&self) -> CasperClkR {
        CasperClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HASHCRYPT clock control"]
    #[inline(always)]
    pub fn hashcrypt_clk(&self) -> HashcryptClkR {
        HashcryptClkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUF clock control"]
    #[inline(always)]
    pub fn puf_clk(&self) -> PufClkR {
        PufClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RNG clock control"]
    #[inline(always)]
    pub fn rng_clk(&self) -> RngClkR {
        RngClkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FLEXSPI clock control"]
    #[inline(always)]
    pub fn flexspi_otfad_clk(&self) -> FlexspiOtfadClkR {
        FlexspiOtfadClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OTP clock control"]
    #[inline(always)]
    pub fn otp_clk(&self) -> OtpClkR {
        OtpClkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - USB PHY clock control"]
    #[inline(always)]
    pub fn usbhs_phy_clk(&self) -> UsbhsPhyClkR {
        UsbhsPhyClkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB DEVICE clock control"]
    #[inline(always)]
    pub fn usbhs_device_clk(&self) -> UsbhsDeviceClkR {
        UsbhsDeviceClkR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USB HOST clock control"]
    #[inline(always)]
    pub fn usbhs_host_clk(&self) -> UsbhsHostClkR {
        UsbhsHostClkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBHS RAM clock control"]
    #[inline(always)]
    pub fn usbhs_sram_clk(&self) -> UsbhsSramClkR {
        UsbhsSramClkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCT clock control"]
    #[inline(always)]
    pub fn sct_clk(&self) -> SctClkR {
        SctClkR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL0")
            .field("rom_ctl_128kb", &self.rom_ctl_128kb())
            .field("powerquad_clk", &self.powerquad_clk())
            .field("casper_clk", &self.casper_clk())
            .field("hashcrypt_clk", &self.hashcrypt_clk())
            .field("puf_clk", &self.puf_clk())
            .field("rng_clk", &self.rng_clk())
            .field("flexspi_otfad_clk", &self.flexspi_otfad_clk())
            .field("otp_clk", &self.otp_clk())
            .field("usbhs_phy_clk", &self.usbhs_phy_clk())
            .field("usbhs_device_clk", &self.usbhs_device_clk())
            .field("usbhs_host_clk", &self.usbhs_host_clk())
            .field("usbhs_sram_clk", &self.usbhs_sram_clk())
            .field("sct_clk", &self.sct_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - 128KB ROM control"]
    #[inline(always)]
    #[must_use]
    pub fn rom_ctl_128kb(&mut self) -> RomCtl128kbW<Pscctl0Spec> {
        RomCtl128kbW::new(self, 2)
    }
    #[doc = "Bit 8 - powerquad clock control"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad_clk(&mut self) -> PowerquadClkW<Pscctl0Spec> {
        PowerquadClkW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER clock control"]
    #[inline(always)]
    #[must_use]
    pub fn casper_clk(&mut self) -> CasperClkW<Pscctl0Spec> {
        CasperClkW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_clk(&mut self) -> HashcryptClkW<Pscctl0Spec> {
        HashcryptClkW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF clock control"]
    #[inline(always)]
    #[must_use]
    pub fn puf_clk(&mut self) -> PufClkW<Pscctl0Spec> {
        PufClkW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG clock control"]
    #[inline(always)]
    #[must_use]
    pub fn rng_clk(&mut self) -> RngClkW<Pscctl0Spec> {
        RngClkW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI clock control"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_otfad_clk(&mut self) -> FlexspiOtfadClkW<Pscctl0Spec> {
        FlexspiOtfadClkW::new(self, 16)
    }
    #[doc = "Bit 17 - OTP clock control"]
    #[inline(always)]
    #[must_use]
    pub fn otp_clk(&mut self) -> OtpClkW<Pscctl0Spec> {
        OtpClkW::new(self, 17)
    }
    #[doc = "Bit 20 - USB PHY clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy_clk(&mut self) -> UsbhsPhyClkW<Pscctl0Spec> {
        UsbhsPhyClkW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device_clk(&mut self) -> UsbhsDeviceClkW<Pscctl0Spec> {
        UsbhsDeviceClkW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host_clk(&mut self) -> UsbhsHostClkW<Pscctl0Spec> {
        UsbhsHostClkW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_clk(&mut self) -> UsbhsSramClkW<Pscctl0Spec> {
        UsbhsSramClkW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sct_clk(&mut self) -> SctClkW<Pscctl0Spec> {
        SctClkW::new(self, 24)
    }
}
#[doc = "clock control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0Spec;
impl crate::RegisterSpec for Pscctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscctl0::R`](R) reader structure"]
impl crate::Readable for Pscctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pscctl0::W`](W) writer structure"]
impl crate::Writable for Pscctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0 to value 0x05"]
impl crate::Resettable for Pscctl0Spec {
    const RESET_VALUE: u32 = 0x05;
}
