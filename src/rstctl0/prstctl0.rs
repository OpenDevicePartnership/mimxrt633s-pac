#[doc = "Register `PRSTCTL0` reader"]
pub type R = crate::R<Prstctl0Spec>;
#[doc = "Register `PRSTCTL0` writer"]
pub type W = crate::W<Prstctl0Spec>;
#[doc = "HIFI DSP reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HifiDsp {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<HifiDsp> for bool {
    #[inline(always)]
    fn from(variant: HifiDsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIFI_DSP` reader - HIFI DSP reset control"]
pub type HifiDspR = crate::BitReader<HifiDsp>;
impl HifiDspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HifiDsp {
        match self.bits {
            false => HifiDsp::ClearReset,
            true => HifiDsp::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == HifiDsp::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == HifiDsp::SetReset
    }
}
#[doc = "Field `HIFI_DSP` writer - HIFI DSP reset control"]
pub type HifiDspW<'a, REG> = crate::BitWriter<'a, REG, HifiDsp>;
impl<'a, REG> HifiDspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(HifiDsp::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(HifiDsp::SetReset)
    }
}
#[doc = "powerquad reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powerquad {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Powerquad> for bool {
    #[inline(always)]
    fn from(variant: Powerquad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` reader - powerquad reset control"]
pub type PowerquadR = crate::BitReader<Powerquad>;
impl PowerquadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Powerquad {
        match self.bits {
            false => Powerquad::ClearReset,
            true => Powerquad::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Powerquad::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Powerquad::SetReset
    }
}
#[doc = "Field `POWERQUAD` writer - powerquad reset control"]
pub type PowerquadW<'a, REG> = crate::BitWriter<'a, REG, Powerquad>;
impl<'a, REG> PowerquadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::SetReset)
    }
}
#[doc = "CAPSER reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` reader - CAPSER reset control"]
pub type CasperR = crate::BitReader<Casper>;
impl CasperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Casper {
        match self.bits {
            false => Casper::ClearReset,
            true => Casper::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Casper::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Casper::SetReset
    }
}
#[doc = "Field `CASPER` writer - CAPSER reset control"]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::SetReset)
    }
}
#[doc = "HASHCRYPT reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashcrypt {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hashcrypt> for bool {
    #[inline(always)]
    fn from(variant: Hashcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` reader - HASHCRYPT reset control"]
pub type HashcryptR = crate::BitReader<Hashcrypt>;
impl HashcryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashcrypt {
        match self.bits {
            false => Hashcrypt::ClearReset,
            true => Hashcrypt::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hashcrypt::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hashcrypt::SetReset
    }
}
#[doc = "Field `HASHCRYPT` writer - HASHCRYPT reset control"]
pub type HashcryptW<'a, REG> = crate::BitWriter<'a, REG, Hashcrypt>;
impl<'a, REG> HashcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::SetReset)
    }
}
#[doc = "PUF reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` reader - PUF reset control"]
pub type PufR = crate::BitReader<Puf>;
impl PufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Puf {
        match self.bits {
            false => Puf::ClearReset,
            true => Puf::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Puf::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Puf::SetReset
    }
}
#[doc = "Field `PUF` writer - PUF reset control"]
pub type PufW<'a, REG> = crate::BitWriter<'a, REG, Puf>;
impl<'a, REG> PufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::SetReset)
    }
}
#[doc = "RNG reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - RNG reset control"]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::ClearReset,
            true => Rng::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Rng::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Rng::SetReset
    }
}
#[doc = "Field `RNG` writer - RNG reset control"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::SetReset)
    }
}
#[doc = "FLEXSPI reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfad {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<FlexspiOtfad> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD` reader - FLEXSPI reset control"]
pub type FlexspiOtfadR = crate::BitReader<FlexspiOtfad>;
impl FlexspiOtfadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlexspiOtfad {
        match self.bits {
            false => FlexspiOtfad::ClearReset,
            true => FlexspiOtfad::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == FlexspiOtfad::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == FlexspiOtfad::SetReset
    }
}
#[doc = "Field `FLEXSPI_OTFAD` writer - FLEXSPI reset control"]
pub type FlexspiOtfadW<'a, REG> = crate::BitWriter<'a, REG, FlexspiOtfad>;
impl<'a, REG> FlexspiOtfadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfad::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfad::SetReset)
    }
}
#[doc = "USB PHY reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhy {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<UsbhsPhy> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY` reader - USB PHY reset control"]
pub type UsbhsPhyR = crate::BitReader<UsbhsPhy>;
impl UsbhsPhyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsPhy {
        match self.bits {
            false => UsbhsPhy::ClearReset,
            true => UsbhsPhy::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == UsbhsPhy::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == UsbhsPhy::SetReset
    }
}
#[doc = "Field `USBHS_PHY` writer - USB PHY reset control"]
pub type UsbhsPhyW<'a, REG> = crate::BitWriter<'a, REG, UsbhsPhy>;
impl<'a, REG> UsbhsPhyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhy::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhy::SetReset)
    }
}
#[doc = "USB DEVICE reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDevice {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<UsbhsDevice> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDevice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE` reader - USB DEVICE reset control"]
pub type UsbhsDeviceR = crate::BitReader<UsbhsDevice>;
impl UsbhsDeviceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsDevice {
        match self.bits {
            false => UsbhsDevice::ClearReset,
            true => UsbhsDevice::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == UsbhsDevice::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == UsbhsDevice::SetReset
    }
}
#[doc = "Field `USBHS_DEVICE` writer - USB DEVICE reset control"]
pub type UsbhsDeviceW<'a, REG> = crate::BitWriter<'a, REG, UsbhsDevice>;
impl<'a, REG> UsbhsDeviceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDevice::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDevice::SetReset)
    }
}
#[doc = "USB HOST reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHost {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<UsbhsHost> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST` reader - USB HOST reset control"]
pub type UsbhsHostR = crate::BitReader<UsbhsHost>;
impl UsbhsHostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsHost {
        match self.bits {
            false => UsbhsHost::ClearReset,
            true => UsbhsHost::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == UsbhsHost::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == UsbhsHost::SetReset
    }
}
#[doc = "Field `USBHS_HOST` writer - USB HOST reset control"]
pub type UsbhsHostW<'a, REG> = crate::BitWriter<'a, REG, UsbhsHost>;
impl<'a, REG> UsbhsHostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHost::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHost::SetReset)
    }
}
#[doc = "USBHS RAM reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSram {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<UsbhsSram> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM` reader - USBHS RAM reset control"]
pub type UsbhsSramR = crate::BitReader<UsbhsSram>;
impl UsbhsSramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsSram {
        match self.bits {
            false => UsbhsSram::ClearReset,
            true => UsbhsSram::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == UsbhsSram::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == UsbhsSram::SetReset
    }
}
#[doc = "Field `USBHS_SRAM` writer - USBHS RAM reset control"]
pub type UsbhsSramW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSram>;
impl<'a, REG> UsbhsSramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSram::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSram::SetReset)
    }
}
#[doc = "SCT reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Sct> for bool {
    #[inline(always)]
    fn from(variant: Sct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` reader - SCT reset control"]
pub type SctR = crate::BitReader<Sct>;
impl SctR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sct {
        match self.bits {
            false => Sct::ClearReset,
            true => Sct::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Sct::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Sct::SetReset
    }
}
#[doc = "Field `SCT` writer - SCT reset control"]
pub type SctW<'a, REG> = crate::BitWriter<'a, REG, Sct>;
impl<'a, REG> SctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::SetReset)
    }
}
impl R {
    #[doc = "Bit 1 - HIFI DSP reset control"]
    #[inline(always)]
    pub fn hifi_dsp(&self) -> HifiDspR {
        HifiDspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - powerquad reset control"]
    #[inline(always)]
    pub fn powerquad(&self) -> PowerquadR {
        PowerquadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAPSER reset control"]
    #[inline(always)]
    pub fn casper(&self) -> CasperR {
        CasperR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HASHCRYPT reset control"]
    #[inline(always)]
    pub fn hashcrypt(&self) -> HashcryptR {
        HashcryptR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUF reset control"]
    #[inline(always)]
    pub fn puf(&self) -> PufR {
        PufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RNG reset control"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FLEXSPI reset control"]
    #[inline(always)]
    pub fn flexspi_otfad(&self) -> FlexspiOtfadR {
        FlexspiOtfadR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - USB PHY reset control"]
    #[inline(always)]
    pub fn usbhs_phy(&self) -> UsbhsPhyR {
        UsbhsPhyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB DEVICE reset control"]
    #[inline(always)]
    pub fn usbhs_device(&self) -> UsbhsDeviceR {
        UsbhsDeviceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USB HOST reset control"]
    #[inline(always)]
    pub fn usbhs_host(&self) -> UsbhsHostR {
        UsbhsHostR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBHS RAM reset control"]
    #[inline(always)]
    pub fn usbhs_sram(&self) -> UsbhsSramR {
        UsbhsSramR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCT reset control"]
    #[inline(always)]
    pub fn sct(&self) -> SctR {
        SctR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL0")
            .field("hifi_dsp", &self.hifi_dsp())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("hashcrypt", &self.hashcrypt())
            .field("puf", &self.puf())
            .field("rng", &self.rng())
            .field("flexspi_otfad", &self.flexspi_otfad())
            .field("usbhs_phy", &self.usbhs_phy())
            .field("usbhs_device", &self.usbhs_device())
            .field("usbhs_host", &self.usbhs_host())
            .field("usbhs_sram", &self.usbhs_sram())
            .field("sct", &self.sct())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - HIFI DSP reset control"]
    #[inline(always)]
    pub fn hifi_dsp(&mut self) -> HifiDspW<Prstctl0Spec> {
        HifiDspW::new(self, 1)
    }
    #[doc = "Bit 8 - powerquad reset control"]
    #[inline(always)]
    pub fn powerquad(&mut self) -> PowerquadW<Prstctl0Spec> {
        PowerquadW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER reset control"]
    #[inline(always)]
    pub fn casper(&mut self) -> CasperW<Prstctl0Spec> {
        CasperW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT reset control"]
    #[inline(always)]
    pub fn hashcrypt(&mut self) -> HashcryptW<Prstctl0Spec> {
        HashcryptW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF reset control"]
    #[inline(always)]
    pub fn puf(&mut self) -> PufW<Prstctl0Spec> {
        PufW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG reset control"]
    #[inline(always)]
    pub fn rng(&mut self) -> RngW<Prstctl0Spec> {
        RngW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI reset control"]
    #[inline(always)]
    pub fn flexspi_otfad(&mut self) -> FlexspiOtfadW<Prstctl0Spec> {
        FlexspiOtfadW::new(self, 16)
    }
    #[doc = "Bit 20 - USB PHY reset control"]
    #[inline(always)]
    pub fn usbhs_phy(&mut self) -> UsbhsPhyW<Prstctl0Spec> {
        UsbhsPhyW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE reset control"]
    #[inline(always)]
    pub fn usbhs_device(&mut self) -> UsbhsDeviceW<Prstctl0Spec> {
        UsbhsDeviceW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST reset control"]
    #[inline(always)]
    pub fn usbhs_host(&mut self) -> UsbhsHostW<Prstctl0Spec> {
        UsbhsHostW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM reset control"]
    #[inline(always)]
    pub fn usbhs_sram(&mut self) -> UsbhsSramW<Prstctl0Spec> {
        UsbhsSramW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT reset control"]
    #[inline(always)]
    pub fn sct(&mut self) -> SctW<Prstctl0Spec> {
        SctW::new(self, 24)
    }
}
#[doc = "peripheral reset control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0Spec;
impl crate::RegisterSpec for Prstctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstctl0::R`](R) reader structure"]
impl crate::Readable for Prstctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`prstctl0::W`](W) writer structure"]
impl crate::Writable for Prstctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0 to value 0x01f1_1f02"]
impl crate::Resettable for Prstctl0Spec {
    const RESET_VALUE: u32 = 0x01f1_1f02;
}
