#[doc = "Register `PRSTCTL0_SET` writer"]
pub type W = crate::W<Prstctl0SetSpec>;
#[doc = "HIFI DSP reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HifiDsp {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<HifiDsp> for bool {
    #[inline(always)]
    fn from(variant: HifiDsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIFI_DSP` writer - HIFI DSP reset set"]
pub type HifiDspW<'a, REG> = crate::BitWriter<'a, REG, HifiDsp>;
impl<'a, REG> HifiDspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HifiDsp::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(HifiDsp::SetReset)
    }
}
#[doc = "powerquad reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powerquad {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Powerquad> for bool {
    #[inline(always)]
    fn from(variant: Powerquad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` writer - powerquad reset set"]
pub type PowerquadW<'a, REG> = crate::BitWriter<'a, REG, Powerquad>;
impl<'a, REG> PowerquadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::SetReset)
    }
}
#[doc = "CAPSER reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` writer - CAPSER reset set"]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::SetReset)
    }
}
#[doc = "HASHCRYPT reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashcrypt {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Hashcrypt> for bool {
    #[inline(always)]
    fn from(variant: Hashcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` writer - HASHCRYPT reset set"]
pub type HashcryptW<'a, REG> = crate::BitWriter<'a, REG, Hashcrypt>;
impl<'a, REG> HashcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::SetReset)
    }
}
#[doc = "PUF reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` writer - PUF reset set"]
pub type PufW<'a, REG> = crate::BitWriter<'a, REG, Puf>;
impl<'a, REG> PufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::SetReset)
    }
}
#[doc = "RNG reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` writer - RNG reset set"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::SetReset)
    }
}
#[doc = "FLEXSPI reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfad {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<FlexspiOtfad> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD` writer - FLEXSPI reset set"]
pub type FlexspiOtfadW<'a, REG> = crate::BitWriter<'a, REG, FlexspiOtfad>;
impl<'a, REG> FlexspiOtfadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfad::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfad::SetReset)
    }
}
#[doc = "USB PHY reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhy {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<UsbhsPhy> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY` writer - USB PHY reset set"]
pub type UsbhsPhyW<'a, REG> = crate::BitWriter<'a, REG, UsbhsPhy>;
impl<'a, REG> UsbhsPhyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhy::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhy::SetReset)
    }
}
#[doc = "USB DEVICE reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDevice {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<UsbhsDevice> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDevice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE` writer - USB DEVICE reset set"]
pub type UsbhsDeviceW<'a, REG> = crate::BitWriter<'a, REG, UsbhsDevice>;
impl<'a, REG> UsbhsDeviceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDevice::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDevice::SetReset)
    }
}
#[doc = "USB HOST reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHost {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<UsbhsHost> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST` writer - USB HOST reset set"]
pub type UsbhsHostW<'a, REG> = crate::BitWriter<'a, REG, UsbhsHost>;
impl<'a, REG> UsbhsHostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHost::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHost::SetReset)
    }
}
#[doc = "USBHS RAM reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSram {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<UsbhsSram> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM` writer - USBHS RAM reset set"]
pub type UsbhsSramW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSram>;
impl<'a, REG> UsbhsSramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSram::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSram::SetReset)
    }
}
#[doc = "SCT reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Sct> for bool {
    #[inline(always)]
    fn from(variant: Sct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` writer - SCT reset set"]
pub type SctW<'a, REG> = crate::BitWriter<'a, REG, Sct>;
impl<'a, REG> SctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::SetReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl0SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - HIFI DSP reset set"]
    #[inline(always)]
    pub fn hifi_dsp(&mut self) -> HifiDspW<Prstctl0SetSpec> {
        HifiDspW::new(self, 1)
    }
    #[doc = "Bit 8 - powerquad reset set"]
    #[inline(always)]
    pub fn powerquad(&mut self) -> PowerquadW<Prstctl0SetSpec> {
        PowerquadW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER reset set"]
    #[inline(always)]
    pub fn casper(&mut self) -> CasperW<Prstctl0SetSpec> {
        CasperW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT reset set"]
    #[inline(always)]
    pub fn hashcrypt(&mut self) -> HashcryptW<Prstctl0SetSpec> {
        HashcryptW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF reset set"]
    #[inline(always)]
    pub fn puf(&mut self) -> PufW<Prstctl0SetSpec> {
        PufW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG reset set"]
    #[inline(always)]
    pub fn rng(&mut self) -> RngW<Prstctl0SetSpec> {
        RngW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI reset set"]
    #[inline(always)]
    pub fn flexspi_otfad(&mut self) -> FlexspiOtfadW<Prstctl0SetSpec> {
        FlexspiOtfadW::new(self, 16)
    }
    #[doc = "Bit 20 - USB PHY reset set"]
    #[inline(always)]
    pub fn usbhs_phy(&mut self) -> UsbhsPhyW<Prstctl0SetSpec> {
        UsbhsPhyW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE reset set"]
    #[inline(always)]
    pub fn usbhs_device(&mut self) -> UsbhsDeviceW<Prstctl0SetSpec> {
        UsbhsDeviceW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST reset set"]
    #[inline(always)]
    pub fn usbhs_host(&mut self) -> UsbhsHostW<Prstctl0SetSpec> {
        UsbhsHostW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM reset set"]
    #[inline(always)]
    pub fn usbhs_sram(&mut self) -> UsbhsSramW<Prstctl0SetSpec> {
        UsbhsSramW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT reset set"]
    #[inline(always)]
    pub fn sct(&mut self) -> SctW<Prstctl0SetSpec> {
        SctW::new(self, 24)
    }
}
#[doc = "peripheral reset set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0SetSpec;
impl crate::RegisterSpec for Prstctl0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl0_set::W`](W) writer structure"]
impl crate::Writable for Prstctl0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_SET to value 0"]
impl crate::Resettable for Prstctl0SetSpec {
    const RESET_VALUE: u32 = 0;
}
