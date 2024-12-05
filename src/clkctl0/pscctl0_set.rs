#[doc = "Register `PSCCTL0_SET` writer"]
pub type W = crate::W<Pscctl0SetSpec>;
#[doc = "128KB ROM controller clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomCtl128kbClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<RomCtl128kbClk> for bool {
    #[inline(always)]
    fn from(variant: RomCtl128kbClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_CTL_128KB_CLK` writer - 128KB ROM controller clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RomCtl128kbClk::SetClock)
    }
}
#[doc = "powerquad clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerquadClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<PowerquadClk> for bool {
    #[inline(always)]
    fn from(variant: PowerquadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD_CLK` writer - powerquad clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadClk::SetClock)
    }
}
#[doc = "CAPSER clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<CasperClk> for bool {
    #[inline(always)]
    fn from(variant: CasperClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_CLK` writer - CAPSER clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CasperClk::SetClock)
    }
}
#[doc = "HASHCRYPT clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashcryptClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<HashcryptClk> for bool {
    #[inline(always)]
    fn from(variant: HashcryptClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT_CLK` writer - HASHCRYPT clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(HashcryptClk::SetClock)
    }
}
#[doc = "PUF clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PufClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<PufClk> for bool {
    #[inline(always)]
    fn from(variant: PufClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_CLK` writer - PUF clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PufClk::SetClock)
    }
}
#[doc = "RNG clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<RngClk> for bool {
    #[inline(always)]
    fn from(variant: RngClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_CLK` writer - RNG clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RngClk::SetClock)
    }
}
#[doc = "FLEXSPI clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiOtfadClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<FlexspiOtfadClk> for bool {
    #[inline(always)]
    fn from(variant: FlexspiOtfadClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_OTFAD_CLK` writer - FLEXSPI clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiOtfadClk::SetClock)
    }
}
#[doc = "OTP clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<OtpClk> for bool {
    #[inline(always)]
    fn from(variant: OtpClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP_CLK` writer - OTP clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OtpClk::SetClock)
    }
}
#[doc = "USB PHY clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsPhyClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<UsbhsPhyClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsPhyClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY_CLK` writer - USB PHY clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsPhyClk::SetClock)
    }
}
#[doc = "USB DEVICE clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsDeviceClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<UsbhsDeviceClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsDeviceClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` writer - USB DEVICE clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsDeviceClk::SetClock)
    }
}
#[doc = "USB HOST clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsHostClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<UsbhsHostClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsHostClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST_CLK` writer - USB HOST clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsHostClk::SetClock)
    }
}
#[doc = "USBHS RAM clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<UsbhsSramClk> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_CLK` writer - USBHS RAM clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramClk::SetClock)
    }
}
#[doc = "SCT clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctClk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<SctClk> for bool {
    #[inline(always)]
    fn from(variant: SctClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_CLK` writer - SCT clock set"]
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
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SctClk::SetClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl0SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - 128KB ROM controller clock set"]
    #[inline(always)]
    pub fn rom_ctl_128kb_clk(&mut self) -> RomCtl128kbClkW<Pscctl0SetSpec> {
        RomCtl128kbClkW::new(self, 2)
    }
    #[doc = "Bit 8 - powerquad clock set"]
    #[inline(always)]
    pub fn powerquad_clk(&mut self) -> PowerquadClkW<Pscctl0SetSpec> {
        PowerquadClkW::new(self, 8)
    }
    #[doc = "Bit 9 - CAPSER clock set"]
    #[inline(always)]
    pub fn casper_clk(&mut self) -> CasperClkW<Pscctl0SetSpec> {
        CasperClkW::new(self, 9)
    }
    #[doc = "Bit 10 - HASHCRYPT clock set"]
    #[inline(always)]
    pub fn hashcrypt_clk(&mut self) -> HashcryptClkW<Pscctl0SetSpec> {
        HashcryptClkW::new(self, 10)
    }
    #[doc = "Bit 11 - PUF clock set"]
    #[inline(always)]
    pub fn puf_clk(&mut self) -> PufClkW<Pscctl0SetSpec> {
        PufClkW::new(self, 11)
    }
    #[doc = "Bit 12 - RNG clock set"]
    #[inline(always)]
    pub fn rng_clk(&mut self) -> RngClkW<Pscctl0SetSpec> {
        RngClkW::new(self, 12)
    }
    #[doc = "Bit 16 - FLEXSPI clock set"]
    #[inline(always)]
    pub fn flexspi_otfad_clk(&mut self) -> FlexspiOtfadClkW<Pscctl0SetSpec> {
        FlexspiOtfadClkW::new(self, 16)
    }
    #[doc = "Bit 17 - OTP clock set"]
    #[inline(always)]
    pub fn otp_clk(&mut self) -> OtpClkW<Pscctl0SetSpec> {
        OtpClkW::new(self, 17)
    }
    #[doc = "Bit 20 - USB PHY clock set"]
    #[inline(always)]
    pub fn usbhs_phy_clk(&mut self) -> UsbhsPhyClkW<Pscctl0SetSpec> {
        UsbhsPhyClkW::new(self, 20)
    }
    #[doc = "Bit 21 - USB DEVICE clock set"]
    #[inline(always)]
    pub fn usbhs_device_clk(&mut self) -> UsbhsDeviceClkW<Pscctl0SetSpec> {
        UsbhsDeviceClkW::new(self, 21)
    }
    #[doc = "Bit 22 - USB HOST clock set"]
    #[inline(always)]
    pub fn usbhs_host_clk(&mut self) -> UsbhsHostClkW<Pscctl0SetSpec> {
        UsbhsHostClkW::new(self, 22)
    }
    #[doc = "Bit 23 - USBHS RAM clock set"]
    #[inline(always)]
    pub fn usbhs_sram_clk(&mut self) -> UsbhsSramClkW<Pscctl0SetSpec> {
        UsbhsSramClkW::new(self, 23)
    }
    #[doc = "Bit 24 - SCT clock set"]
    #[inline(always)]
    pub fn sct_clk(&mut self) -> SctClkW<Pscctl0SetSpec> {
        SctClkW::new(self, 24)
    }
}
#[doc = "clock set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0SetSpec;
impl crate::RegisterSpec for Pscctl0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl0_set::W`](W) writer structure"]
impl crate::Writable for Pscctl0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0_SET to value 0"]
impl crate::Resettable for Pscctl0SetSpec {
    const RESET_VALUE: u32 = 0;
}
