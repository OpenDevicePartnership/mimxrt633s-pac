#[doc = "Register `PRSTCTL0_CLR` writer"]
pub type W = crate::W<Prstctl0ClrSpec>;
#[doc = "HIFI DSP reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HifiDsp {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<HifiDsp> for bool {
    #[inline(always)]
    fn from(variant: HifiDsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIFI_DSP` writer - HIFI DSP reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(HifiDsp::ClrReset)
    }
}
#[doc = "powerquad reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powerquad {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Powerquad> for bool {
    #[inline(always)]
    fn from(variant: Powerquad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` writer - powerquad reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::ClrReset)
    }
}
#[doc = "CAPSER reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` writer - CAPSER reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::ClrReset)
    }
}
#[doc = "HASHCRYPT reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashcrypt {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Hashcrypt> for bool {
    #[inline(always)]
    fn from(variant: Hashcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` writer - HASHCRYPT reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::ClrReset)
    }
}
#[doc = "PUF reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` writer - PUF reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::ClrReset)
    }
}
#[doc = "RNG reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` writer - RNG reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::ClrReset)
    }
}
#[doc = "FLEXSPI reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfad {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<FlexspiOtfad> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD` writer - FLEXSPI reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfad::ClrReset)
    }
}
#[doc = "USB PHY reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhy {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<UsbhsPhy> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY` writer - USB PHY reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhy::ClrReset)
    }
}
#[doc = "USB DEVICE reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDevice {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<UsbhsDevice> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDevice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE` writer - USB DEVICE reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDevice::ClrReset)
    }
}
#[doc = "USB HOST reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHost {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<UsbhsHost> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST` writer - USB HOST reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHost::ClrReset)
    }
}
#[doc = "USBHS RAM reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSram {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<UsbhsSram> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM` writer - USBHS RAM reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSram::ClrReset)
    }
}
#[doc = "SCT reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Sct> for bool {
    #[inline(always)]
    fn from(variant: Sct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` writer - SCT reset clear"]
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
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sct::ClrReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - HIFI DSP reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hifi_dsp(&mut self) -> HifiDspW<Prstctl0ClrSpec> {
        HifiDspW::new(self, 1)
    }
    #[doc = "Bit 8 - powerquad reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> PowerquadW<Prstctl0ClrSpec> {
        PowerquadW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CasperW<Prstctl0ClrSpec> {
        CasperW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt(&mut self) -> HashcryptW<Prstctl0ClrSpec> {
        HashcryptW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PufW<Prstctl0ClrSpec> {
        PufW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<Prstctl0ClrSpec> {
        RngW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_otfad(&mut self) -> FlexspiOtfadW<Prstctl0ClrSpec> {
        FlexspiOtfadW::new(self, 16)
    }
    #[doc = "Bit 20 - USB PHY reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy(&mut self) -> UsbhsPhyW<Prstctl0ClrSpec> {
        UsbhsPhyW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device(&mut self) -> UsbhsDeviceW<Prstctl0ClrSpec> {
        UsbhsDeviceW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host(&mut self) -> UsbhsHostW<Prstctl0ClrSpec> {
        UsbhsHostW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram(&mut self) -> UsbhsSramW<Prstctl0ClrSpec> {
        UsbhsSramW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SctW<Prstctl0ClrSpec> {
        SctW::new(self, 24)
    }
}
#[doc = "peripheral reset clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0ClrSpec;
impl crate::RegisterSpec for Prstctl0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl0_clr::W`](W) writer structure"]
impl crate::Writable for Prstctl0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_CLR to value 0"]
impl crate::Resettable for Prstctl0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
