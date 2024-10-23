#[doc = "Register `PDSLEEPCFG1` reader"]
pub type R = crate::R<Pdsleepcfg1Spec>;
#[doc = "Register `PDSLEEPCFG1` writer"]
pub type W = crate::W<Pdsleepcfg1Spec>;
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqSramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<PqSramApd> for bool {
    #[inline(always)]
    fn from(variant: PqSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_APD` reader - Array power"]
pub type PqSramApdR = crate::BitReader<PqSramApd>;
impl PqSramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PqSramApd {
        match self.bits {
            false => PqSramApd::Enabled,
            true => PqSramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PqSramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PqSramApd::PowerDown
    }
}
#[doc = "Field `PQ_SRAM_APD` writer - Array power"]
pub type PqSramApdW<'a, REG> = crate::BitWriter<'a, REG, PqSramApd>;
impl<'a, REG> PqSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqSramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<PqSramPpd> for bool {
    #[inline(always)]
    fn from(variant: PqSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_PPD` reader - Peiph power"]
pub type PqSramPpdR = crate::BitReader<PqSramPpd>;
impl PqSramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PqSramPpd {
        match self.bits {
            false => PqSramPpd::Enabled,
            true => PqSramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PqSramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PqSramPpd::PowerDown
    }
}
#[doc = "Field `PQ_SRAM_PPD` writer - Peiph power"]
pub type PqSramPpdW<'a, REG> = crate::BitWriter<'a, REG, PqSramPpd>;
impl<'a, REG> PqSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiSramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<FlexspiSramApd> for bool {
    #[inline(always)]
    fn from(variant: FlexspiSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_SRAM_APD` reader - Array power"]
pub type FlexspiSramApdR = crate::BitReader<FlexspiSramApd>;
impl FlexspiSramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlexspiSramApd {
        match self.bits {
            false => FlexspiSramApd::Enabled,
            true => FlexspiSramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FlexspiSramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FlexspiSramApd::PowerDown
    }
}
#[doc = "Field `FLEXSPI_SRAM_APD` writer - Array power"]
pub type FlexspiSramApdW<'a, REG> = crate::BitWriter<'a, REG, FlexspiSramApd>;
impl<'a, REG> FlexspiSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiSramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<FlexspiSramPpd> for bool {
    #[inline(always)]
    fn from(variant: FlexspiSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_SRAM_PPD` reader - Peiph power"]
pub type FlexspiSramPpdR = crate::BitReader<FlexspiSramPpd>;
impl FlexspiSramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlexspiSramPpd {
        match self.bits {
            false => FlexspiSramPpd::Enabled,
            true => FlexspiSramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FlexspiSramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FlexspiSramPpd::PowerDown
    }
}
#[doc = "Field `FLEXSPI_SRAM_PPD` writer - Peiph power"]
pub type FlexspiSramPpdW<'a, REG> = crate::BitWriter<'a, REG, FlexspiSramPpd>;
impl<'a, REG> FlexspiSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<UsbhsSramApd> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_APD` reader - Array power"]
pub type UsbhsSramApdR = crate::BitReader<UsbhsSramApd>;
impl UsbhsSramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsSramApd {
        match self.bits {
            false => UsbhsSramApd::Enabled,
            true => UsbhsSramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UsbhsSramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == UsbhsSramApd::PowerDown
    }
}
#[doc = "Field `USBHS_SRAM_APD` writer - Array power"]
pub type UsbhsSramApdW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramApd>;
impl<'a, REG> UsbhsSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<UsbhsSramPpd> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_PPD` reader - Peiph power"]
pub type UsbhsSramPpdR = crate::BitReader<UsbhsSramPpd>;
impl UsbhsSramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbhsSramPpd {
        match self.bits {
            false => UsbhsSramPpd::Enabled,
            true => UsbhsSramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UsbhsSramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == UsbhsSramPpd::PowerDown
    }
}
#[doc = "Field `USBHS_SRAM_PPD` writer - Peiph power"]
pub type UsbhsSramPpdW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramPpd>;
impl<'a, REG> UsbhsSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc0SramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<Usdhc0SramApd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc0SramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_APD` reader - Array power"]
pub type Usdhc0SramApdR = crate::BitReader<Usdhc0SramApd>;
impl Usdhc0SramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usdhc0SramApd {
        match self.bits {
            false => Usdhc0SramApd::Enabled,
            true => Usdhc0SramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usdhc0SramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Usdhc0SramApd::PowerDown
    }
}
#[doc = "Field `USDHC0_SRAM_APD` writer - Array power"]
pub type Usdhc0SramApdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc0SramApd>;
impl<'a, REG> Usdhc0SramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc0SramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<Usdhc0SramPpd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc0SramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` reader - Peiph power"]
pub type Usdhc0SramPpdR = crate::BitReader<Usdhc0SramPpd>;
impl Usdhc0SramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usdhc0SramPpd {
        match self.bits {
            false => Usdhc0SramPpd::Enabled,
            true => Usdhc0SramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usdhc0SramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Usdhc0SramPpd::PowerDown
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` writer - Peiph power"]
pub type Usdhc0SramPpdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc0SramPpd>;
impl<'a, REG> Usdhc0SramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc1SramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<Usdhc1SramApd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc1SramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_APD` reader - Array power"]
pub type Usdhc1SramApdR = crate::BitReader<Usdhc1SramApd>;
impl Usdhc1SramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usdhc1SramApd {
        match self.bits {
            false => Usdhc1SramApd::Enabled,
            true => Usdhc1SramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usdhc1SramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Usdhc1SramApd::PowerDown
    }
}
#[doc = "Field `USDHC1_SRAM_APD` writer - Array power"]
pub type Usdhc1SramApdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc1SramApd>;
impl<'a, REG> Usdhc1SramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc1SramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<Usdhc1SramPpd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc1SramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` reader - Peiph power"]
pub type Usdhc1SramPpdR = crate::BitReader<Usdhc1SramPpd>;
impl Usdhc1SramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usdhc1SramPpd {
        match self.bits {
            false => Usdhc1SramPpd::Enabled,
            true => Usdhc1SramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usdhc1SramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Usdhc1SramPpd::PowerDown
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` writer - Peiph power"]
pub type Usdhc1SramPpdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc1SramPpd>;
impl<'a, REG> Usdhc1SramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperSramApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<CasperSramApd> for bool {
    #[inline(always)]
    fn from(variant: CasperSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_SRAM_APD` reader - Array power"]
pub type CasperSramApdR = crate::BitReader<CasperSramApd>;
impl CasperSramApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperSramApd {
        match self.bits {
            false => CasperSramApd::Enabled,
            true => CasperSramApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CasperSramApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == CasperSramApd::PowerDown
    }
}
#[doc = "Field `CASPER_SRAM_APD` writer - Array power"]
pub type CasperSramApdW<'a, REG> = crate::BitWriter<'a, REG, CasperSramApd>;
impl<'a, REG> CasperSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperSramPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<CasperSramPpd> for bool {
    #[inline(always)]
    fn from(variant: CasperSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_SRAM_PPD` reader - Peiph power"]
pub type CasperSramPpdR = crate::BitReader<CasperSramPpd>;
impl CasperSramPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperSramPpd {
        match self.bits {
            false => CasperSramPpd::Enabled,
            true => CasperSramPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CasperSramPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == CasperSramPpd::PowerDown
    }
}
#[doc = "Field `CASPER_SRAM_PPD` writer - Peiph power"]
pub type CasperSramPpdW<'a, REG> = crate::BitWriter<'a, REG, CasperSramPpd>;
impl<'a, REG> CasperSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspcacheRegfApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<DspcacheRegfApd> for bool {
    #[inline(always)]
    fn from(variant: DspcacheRegfApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPCACHE_REGF_APD` reader - Array power"]
pub type DspcacheRegfApdR = crate::BitReader<DspcacheRegfApd>;
impl DspcacheRegfApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspcacheRegfApd {
        match self.bits {
            false => DspcacheRegfApd::Enabled,
            true => DspcacheRegfApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DspcacheRegfApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DspcacheRegfApd::PowerDown
    }
}
#[doc = "Field `DSPCACHE_REGF_APD` writer - Array power"]
pub type DspcacheRegfApdW<'a, REG> = crate::BitWriter<'a, REG, DspcacheRegfApd>;
impl<'a, REG> DspcacheRegfApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspcacheRegfPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<DspcacheRegfPpd> for bool {
    #[inline(always)]
    fn from(variant: DspcacheRegfPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPCACHE_REGF_PPD` reader - Peiph power"]
pub type DspcacheRegfPpdR = crate::BitReader<DspcacheRegfPpd>;
impl DspcacheRegfPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspcacheRegfPpd {
        match self.bits {
            false => DspcacheRegfPpd::Enabled,
            true => DspcacheRegfPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DspcacheRegfPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DspcacheRegfPpd::PowerDown
    }
}
#[doc = "Field `DSPCACHE_REGF_PPD` writer - Peiph power"]
pub type DspcacheRegfPpdW<'a, REG> = crate::BitWriter<'a, REG, DspcacheRegfPpd>;
impl<'a, REG> DspcacheRegfPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfPpd::PowerDown)
    }
}
#[doc = "Array power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsptcmRegfApd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<DsptcmRegfApd> for bool {
    #[inline(always)]
    fn from(variant: DsptcmRegfApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPTCM_REGF_APD` reader - Array power"]
pub type DsptcmRegfApdR = crate::BitReader<DsptcmRegfApd>;
impl DsptcmRegfApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DsptcmRegfApd {
        match self.bits {
            false => DsptcmRegfApd::Enabled,
            true => DsptcmRegfApd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DsptcmRegfApd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DsptcmRegfApd::PowerDown
    }
}
#[doc = "Field `DSPTCM_REGF_APD` writer - Array power"]
pub type DsptcmRegfApdW<'a, REG> = crate::BitWriter<'a, REG, DsptcmRegfApd>;
impl<'a, REG> DsptcmRegfApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfApd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfApd::PowerDown)
    }
}
#[doc = "Peiph power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsptcmRegfPpd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<DsptcmRegfPpd> for bool {
    #[inline(always)]
    fn from(variant: DsptcmRegfPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPTCM_REGF_PPD` reader - Peiph power"]
pub type DsptcmRegfPpdR = crate::BitReader<DsptcmRegfPpd>;
impl DsptcmRegfPpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DsptcmRegfPpd {
        match self.bits {
            false => DsptcmRegfPpd::Enabled,
            true => DsptcmRegfPpd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DsptcmRegfPpd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == DsptcmRegfPpd::PowerDown
    }
}
#[doc = "Field `DSPTCM_REGF_PPD` writer - Peiph power"]
pub type DsptcmRegfPpdW<'a, REG> = crate::BitWriter<'a, REG, DsptcmRegfPpd>;
impl<'a, REG> DsptcmRegfPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfPpd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfPpd::PowerDown)
    }
}
#[doc = "array power and periph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomPd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<RomPd> for bool {
    #[inline(always)]
    fn from(variant: RomPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_PD` reader - array power and periph power"]
pub type RomPdR = crate::BitReader<RomPd>;
impl RomPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RomPd {
        match self.bits {
            false => RomPd::Enabled,
            true => RomPd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RomPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == RomPd::PowerDown
    }
}
#[doc = "Field `ROM_PD` writer - array power and periph power"]
pub type RomPdW<'a, REG> = crate::BitWriter<'a, REG, RomPd>;
impl<'a, REG> RomPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RomPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(RomPd::PowerDown)
    }
}
#[doc = "Needed when vddcore can be smaller than 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramSleep {
    #[doc = "0: RAM Normal Mode"]
    NormalMode = 0,
    #[doc = "1: RAM Sleep Mode."]
    SleepMode = 1,
}
impl From<SramSleep> for bool {
    #[inline(always)]
    fn from(variant: SramSleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_SLEEP` reader - Needed when vddcore can be smaller than 0"]
pub type SramSleepR = crate::BitReader<SramSleep>;
impl SramSleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramSleep {
        match self.bits {
            false => SramSleep::NormalMode,
            true => SramSleep::SleepMode,
        }
    }
    #[doc = "RAM Normal Mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == SramSleep::NormalMode
    }
    #[doc = "RAM Sleep Mode."]
    #[inline(always)]
    pub fn is_sleep_mode(&self) -> bool {
        *self == SramSleep::SleepMode
    }
}
#[doc = "Field `SRAM_SLEEP` writer - Needed when vddcore can be smaller than 0"]
pub type SramSleepW<'a, REG> = crate::BitWriter<'a, REG, SramSleep>;
impl<'a, REG> SramSleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM Normal Mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SramSleep::NormalMode)
    }
    #[doc = "RAM Sleep Mode."]
    #[inline(always)]
    pub fn sleep_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SramSleep::SleepMode)
    }
}
impl R {
    #[doc = "Bit 0 - Array power"]
    #[inline(always)]
    pub fn pq_sram_apd(&self) -> PqSramApdR {
        PqSramApdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peiph power"]
    #[inline(always)]
    pub fn pq_sram_ppd(&self) -> PqSramPpdR {
        PqSramPpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Array power"]
    #[inline(always)]
    pub fn flexspi_sram_apd(&self) -> FlexspiSramApdR {
        FlexspiSramApdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peiph power"]
    #[inline(always)]
    pub fn flexspi_sram_ppd(&self) -> FlexspiSramPpdR {
        FlexspiSramPpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Array power"]
    #[inline(always)]
    pub fn usbhs_sram_apd(&self) -> UsbhsSramApdR {
        UsbhsSramApdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peiph power"]
    #[inline(always)]
    pub fn usbhs_sram_ppd(&self) -> UsbhsSramPpdR {
        UsbhsSramPpdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Array power"]
    #[inline(always)]
    pub fn usdhc0_sram_apd(&self) -> Usdhc0SramApdR {
        Usdhc0SramApdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peiph power"]
    #[inline(always)]
    pub fn usdhc0_sram_ppd(&self) -> Usdhc0SramPpdR {
        Usdhc0SramPpdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Array power"]
    #[inline(always)]
    pub fn usdhc1_sram_apd(&self) -> Usdhc1SramApdR {
        Usdhc1SramApdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peiph power"]
    #[inline(always)]
    pub fn usdhc1_sram_ppd(&self) -> Usdhc1SramPpdR {
        Usdhc1SramPpdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Array power"]
    #[inline(always)]
    pub fn casper_sram_apd(&self) -> CasperSramApdR {
        CasperSramApdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peiph power"]
    #[inline(always)]
    pub fn casper_sram_ppd(&self) -> CasperSramPpdR {
        CasperSramPpdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Array power"]
    #[inline(always)]
    pub fn dspcache_regf_apd(&self) -> DspcacheRegfApdR {
        DspcacheRegfApdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peiph power"]
    #[inline(always)]
    pub fn dspcache_regf_ppd(&self) -> DspcacheRegfPpdR {
        DspcacheRegfPpdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Array power"]
    #[inline(always)]
    pub fn dsptcm_regf_apd(&self) -> DsptcmRegfApdR {
        DsptcmRegfApdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peiph power"]
    #[inline(always)]
    pub fn dsptcm_regf_ppd(&self) -> DsptcmRegfPpdR {
        DsptcmRegfPpdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - array power and periph power"]
    #[inline(always)]
    pub fn rom_pd(&self) -> RomPdR {
        RomPdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Needed when vddcore can be smaller than 0"]
    #[inline(always)]
    pub fn sram_sleep(&self) -> SramSleepR {
        SramSleepR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSLEEPCFG1")
            .field("pq_sram_apd", &self.pq_sram_apd())
            .field("pq_sram_ppd", &self.pq_sram_ppd())
            .field("flexspi_sram_apd", &self.flexspi_sram_apd())
            .field("flexspi_sram_ppd", &self.flexspi_sram_ppd())
            .field("usbhs_sram_apd", &self.usbhs_sram_apd())
            .field("usbhs_sram_ppd", &self.usbhs_sram_ppd())
            .field("usdhc0_sram_apd", &self.usdhc0_sram_apd())
            .field("usdhc0_sram_ppd", &self.usdhc0_sram_ppd())
            .field("usdhc1_sram_apd", &self.usdhc1_sram_apd())
            .field("usdhc1_sram_ppd", &self.usdhc1_sram_ppd())
            .field("casper_sram_apd", &self.casper_sram_apd())
            .field("casper_sram_ppd", &self.casper_sram_ppd())
            .field("dspcache_regf_apd", &self.dspcache_regf_apd())
            .field("dspcache_regf_ppd", &self.dspcache_regf_ppd())
            .field("dsptcm_regf_apd", &self.dsptcm_regf_apd())
            .field("dsptcm_regf_ppd", &self.dsptcm_regf_ppd())
            .field("rom_pd", &self.rom_pd())
            .field("sram_sleep", &self.sram_sleep())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn pq_sram_apd(&mut self) -> PqSramApdW<Pdsleepcfg1Spec> {
        PqSramApdW::new(self, 0)
    }
    #[doc = "Bit 1 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn pq_sram_ppd(&mut self) -> PqSramPpdW<Pdsleepcfg1Spec> {
        PqSramPpdW::new(self, 1)
    }
    #[doc = "Bit 2 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_sram_apd(&mut self) -> FlexspiSramApdW<Pdsleepcfg1Spec> {
        FlexspiSramApdW::new(self, 2)
    }
    #[doc = "Bit 3 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_sram_ppd(&mut self) -> FlexspiSramPpdW<Pdsleepcfg1Spec> {
        FlexspiSramPpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_apd(&mut self) -> UsbhsSramApdW<Pdsleepcfg1Spec> {
        UsbhsSramApdW::new(self, 4)
    }
    #[doc = "Bit 5 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_ppd(&mut self) -> UsbhsSramPpdW<Pdsleepcfg1Spec> {
        UsbhsSramPpdW::new(self, 5)
    }
    #[doc = "Bit 6 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc0_sram_apd(&mut self) -> Usdhc0SramApdW<Pdsleepcfg1Spec> {
        Usdhc0SramApdW::new(self, 6)
    }
    #[doc = "Bit 7 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc0_sram_ppd(&mut self) -> Usdhc0SramPpdW<Pdsleepcfg1Spec> {
        Usdhc0SramPpdW::new(self, 7)
    }
    #[doc = "Bit 8 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc1_sram_apd(&mut self) -> Usdhc1SramApdW<Pdsleepcfg1Spec> {
        Usdhc1SramApdW::new(self, 8)
    }
    #[doc = "Bit 9 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc1_sram_ppd(&mut self) -> Usdhc1SramPpdW<Pdsleepcfg1Spec> {
        Usdhc1SramPpdW::new(self, 9)
    }
    #[doc = "Bit 10 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn casper_sram_apd(&mut self) -> CasperSramApdW<Pdsleepcfg1Spec> {
        CasperSramApdW::new(self, 10)
    }
    #[doc = "Bit 11 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn casper_sram_ppd(&mut self) -> CasperSramPpdW<Pdsleepcfg1Spec> {
        CasperSramPpdW::new(self, 11)
    }
    #[doc = "Bit 24 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn dspcache_regf_apd(&mut self) -> DspcacheRegfApdW<Pdsleepcfg1Spec> {
        DspcacheRegfApdW::new(self, 24)
    }
    #[doc = "Bit 25 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn dspcache_regf_ppd(&mut self) -> DspcacheRegfPpdW<Pdsleepcfg1Spec> {
        DspcacheRegfPpdW::new(self, 25)
    }
    #[doc = "Bit 26 - Array power"]
    #[inline(always)]
    #[must_use]
    pub fn dsptcm_regf_apd(&mut self) -> DsptcmRegfApdW<Pdsleepcfg1Spec> {
        DsptcmRegfApdW::new(self, 26)
    }
    #[doc = "Bit 27 - Peiph power"]
    #[inline(always)]
    #[must_use]
    pub fn dsptcm_regf_ppd(&mut self) -> DsptcmRegfPpdW<Pdsleepcfg1Spec> {
        DsptcmRegfPpdW::new(self, 27)
    }
    #[doc = "Bit 28 - array power and periph power"]
    #[inline(always)]
    #[must_use]
    pub fn rom_pd(&mut self) -> RomPdW<Pdsleepcfg1Spec> {
        RomPdW::new(self, 28)
    }
    #[doc = "Bit 31 - Needed when vddcore can be smaller than 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram_sleep(&mut self) -> SramSleepW<Pdsleepcfg1Spec> {
        SramSleepW::new(self, 31)
    }
}
#[doc = "Sleep configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdsleepcfg1Spec;
impl crate::RegisterSpec for Pdsleepcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsleepcfg1::R`](R) reader structure"]
impl crate::Readable for Pdsleepcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdsleepcfg1::W`](W) writer structure"]
impl crate::Writable for Pdsleepcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG1 to value 0x0fff_ffff"]
impl crate::Resettable for Pdsleepcfg1Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
