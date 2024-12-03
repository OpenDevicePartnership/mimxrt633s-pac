#[doc = "Register `PSCCTL0_CLR` writer"]
pub type W = crate::W<Pscctl0ClrSpec>;
#[doc = "ROM controller clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomCtl128kbClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<RomCtl128kbClk> for bool {
    #[inline(always)]
    fn from(variant: RomCtl128kbClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_CTL_128KB_CLK` writer - ROM controller clock clear"]
pub type RomCtl128kbClkW<'a, REG> = crate::BitWriter<'a, REG, RomCtl128kbClk>;
impl<'a, REG> RomCtl128kbClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RomCtl128kbClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RomCtl128kbClk::ClrClock)
    }
}
#[doc = "powerquad clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerquadClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<PowerquadClk> for bool {
    #[inline(always)]
    fn from(variant: PowerquadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD_CLK` writer - powerquad clock clear"]
pub type PowerquadClkW<'a, REG> = crate::BitWriter<'a, REG, PowerquadClk>;
impl<'a, REG> PowerquadClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadClk::ClrClock)
    }
}
#[doc = "CAPSER clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<CasperClk> for bool {
    #[inline(always)]
    fn from(variant: CasperClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_CLK` writer - CAPSER clock clear"]
pub type CasperClkW<'a, REG> = crate::BitWriter<'a, REG, CasperClk>;
impl<'a, REG> CasperClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CasperClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CasperClk::ClrClock)
    }
}
#[doc = "HASHCRYPT clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashcryptClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<HashcryptClk> for bool {
    #[inline(always)]
    fn from(variant: HashcryptClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT_CLK` writer - HASHCRYPT clock clear"]
pub type HashcryptClkW<'a, REG> = crate::BitWriter<'a, REG, HashcryptClk>;
impl<'a, REG> HashcryptClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HashcryptClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(HashcryptClk::ClrClock)
    }
}
#[doc = "PUF clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PufClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<PufClk> for bool {
    #[inline(always)]
    fn from(variant: PufClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_CLK` writer - PUF clock clear"]
pub type PufClkW<'a, REG> = crate::BitWriter<'a, REG, PufClk>;
impl<'a, REG> PufClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PufClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PufClk::ClrClock)
    }
}
#[doc = "RNG clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<RngClk> for bool {
    #[inline(always)]
    fn from(variant: RngClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_CLK` writer - RNG clock clear"]
pub type RngClkW<'a, REG> = crate::BitWriter<'a, REG, RngClk>;
impl<'a, REG> RngClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RngClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RngClk::ClrClock)
    }
}
#[doc = "FLEXSPI clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfadClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<FlexspiOtfadClk> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD_CLK` writer - FLEXSPI clock clear"]
pub type FlexspiOtfadClkW<'a, REG> = crate::BitWriter<'a, REG, FlexspiOtfadClk>;
impl<'a, REG> FlexspiOtfadClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfadClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfadClk::ClrClock)
    }
}
#[doc = "OTP clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<OtpClk> for bool {
    #[inline(always)]
    fn from(variant: OtpClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP_CLK` writer - OTP clock clear"]
pub type OtpClkW<'a, REG> = crate::BitWriter<'a, REG, OtpClk>;
impl<'a, REG> OtpClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OtpClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OtpClk::ClrClock)
    }
}
#[doc = "USB PHY clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhyClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<UsbhsPhyClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhyClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY_CLK` writer - USB PHY clock clear"]
pub type UsbhsPhyClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsPhyClk>;
impl<'a, REG> UsbhsPhyClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhyClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhyClk::ClrClock)
    }
}
#[doc = "USB DEVICE clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDeviceClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<UsbhsDeviceClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDeviceClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` writer - USB DEVICE clock clear"]
pub type UsbhsDeviceClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsDeviceClk>;
impl<'a, REG> UsbhsDeviceClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDeviceClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDeviceClk::ClrClock)
    }
}
#[doc = "USB HOST clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHostClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<UsbhsHostClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHostClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST_CLK` writer - USB HOST clock clear"]
pub type UsbhsHostClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsHostClk>;
impl<'a, REG> UsbhsHostClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHostClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHostClk::ClrClock)
    }
}
#[doc = "USBHS RAM clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<UsbhsSramClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_CLK` writer - USBHS RAM clock clear"]
pub type UsbhsSramClkW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramClk>;
impl<'a, REG> UsbhsSramClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramClk::ClrClock)
    }
}
#[doc = "SCT clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<SctClk> for bool {
    #[inline(always)]
    fn from(variant: SctClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_CLK` writer - SCT clock clear"]
pub type SctClkW<'a, REG> = crate::BitWriter<'a, REG, SctClk>;
impl<'a, REG> SctClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SctClk::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SctClk::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - ROM controller clock clear"]
    #[inline(always)]
    pub fn rom_ctl_128kb_clk(&mut self) -> RomCtl128kbClkW<Pscctl0ClrSpec> {
        RomCtl128kbClkW::new(self, 2)
    }
    #[doc = "Bit 8 - powerquad clock clear"]
    #[inline(always)]
    pub fn powerquad_clk(&mut self) -> PowerquadClkW<Pscctl0ClrSpec> {
        PowerquadClkW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER clock clear"]
    #[inline(always)]
    pub fn casper_clk(&mut self) -> CasperClkW<Pscctl0ClrSpec> {
        CasperClkW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT clock clear"]
    #[inline(always)]
    pub fn hashcrypt_clk(&mut self) -> HashcryptClkW<Pscctl0ClrSpec> {
        HashcryptClkW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF clock clear"]
    #[inline(always)]
    pub fn puf_clk(&mut self) -> PufClkW<Pscctl0ClrSpec> {
        PufClkW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG clock clear"]
    #[inline(always)]
    pub fn rng_clk(&mut self) -> RngClkW<Pscctl0ClrSpec> {
        RngClkW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI clock clear"]
    #[inline(always)]
    pub fn flexspi_otfad_clk(&mut self) -> FlexspiOtfadClkW<Pscctl0ClrSpec> {
        FlexspiOtfadClkW::new(self, 16)
    }
    #[doc = "Bit 17 - OTP clock clear"]
    #[inline(always)]
    pub fn otp_clk(&mut self) -> OtpClkW<Pscctl0ClrSpec> {
        OtpClkW::new(self, 17)
    }
    #[doc = "Bit 20 - USB PHY clock clear"]
    #[inline(always)]
    pub fn usbhs_phy_clk(&mut self) -> UsbhsPhyClkW<Pscctl0ClrSpec> {
        UsbhsPhyClkW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE clock clear"]
    #[inline(always)]
    pub fn usbhs_device_clk(&mut self) -> UsbhsDeviceClkW<Pscctl0ClrSpec> {
        UsbhsDeviceClkW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST clock clear"]
    #[inline(always)]
    pub fn usbhs_host_clk(&mut self) -> UsbhsHostClkW<Pscctl0ClrSpec> {
        UsbhsHostClkW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM clock clear"]
    #[inline(always)]
    pub fn usbhs_sram_clk(&mut self) -> UsbhsSramClkW<Pscctl0ClrSpec> {
        UsbhsSramClkW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT clock clear"]
    #[inline(always)]
    pub fn sct_clk(&mut self) -> SctClkW<Pscctl0ClrSpec> {
        SctClkW::new(self, 24)
    }
}
#[doc = "clock clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0ClrSpec;
impl crate::RegisterSpec for Pscctl0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl0_clr::W`](W) writer structure"]
impl crate::Writable for Pscctl0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0_CLR to value 0"]
impl crate::Resettable for Pscctl0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
