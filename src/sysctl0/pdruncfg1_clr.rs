#[doc = "Register `PDRUNCFG1_CLR` writer"]
pub type W = crate::W<Pdruncfg1ClrSpec>;
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqSramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<PqSramApd> for bool {
    #[inline(always)]
    fn from(variant: PqSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_APD` writer - Array power"]
pub type PqSramApdW<'a, REG> = crate::BitWriter<'a, REG, PqSramApd>;
impl<'a, REG> PqSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqSramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<PqSramPpd> for bool {
    #[inline(always)]
    fn from(variant: PqSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_PPD` writer - Peiph power"]
pub type PqSramPpdW<'a, REG> = crate::BitWriter<'a, REG, PqSramPpd>;
impl<'a, REG> PqSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(PqSramPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiSramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<FlexspiSramApd> for bool {
    #[inline(always)]
    fn from(variant: FlexspiSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_SRAM_APD` writer - Array power"]
pub type FlexspiSramApdW<'a, REG> = crate::BitWriter<'a, REG, FlexspiSramApd>;
impl<'a, REG> FlexspiSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexspiSramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<FlexspiSramPpd> for bool {
    #[inline(always)]
    fn from(variant: FlexspiSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI_SRAM_PPD` writer - Peiph power"]
pub type FlexspiSramPpdW<'a, REG> = crate::BitWriter<'a, REG, FlexspiSramPpd>;
impl<'a, REG> FlexspiSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(FlexspiSramPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<UsbhsSramApd> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_APD` writer - Array power"]
pub type UsbhsSramApdW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramApd>;
impl<'a, REG> UsbhsSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbhsSramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<UsbhsSramPpd> for bool {
    #[inline(always)]
    fn from(variant: UsbhsSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_PPD` writer - Peiph power"]
pub type UsbhsSramPpdW<'a, REG> = crate::BitWriter<'a, REG, UsbhsSramPpd>;
impl<'a, REG> UsbhsSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbhsSramPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc0SramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<Usdhc0SramApd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc0SramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_APD` writer - Array power"]
pub type Usdhc0SramApdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc0SramApd>;
impl<'a, REG> Usdhc0SramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramApd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc0SramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<Usdhc0SramPpd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc0SramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` writer - Array power"]
pub type Usdhc0SramPpdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc0SramPpd>;
impl<'a, REG> Usdhc0SramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc0SramPpd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc1SramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<Usdhc1SramApd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc1SramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_APD` writer - Peiph power"]
pub type Usdhc1SramApdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc1SramApd>;
impl<'a, REG> Usdhc1SramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usdhc1SramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<Usdhc1SramPpd> for bool {
    #[inline(always)]
    fn from(variant: Usdhc1SramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` writer - Peiph power"]
pub type Usdhc1SramPpdW<'a, REG> = crate::BitWriter<'a, REG, Usdhc1SramPpd>;
impl<'a, REG> Usdhc1SramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(Usdhc1SramPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperSramApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<CasperSramApd> for bool {
    #[inline(always)]
    fn from(variant: CasperSramApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_SRAM_APD` writer - Array power"]
pub type CasperSramApdW<'a, REG> = crate::BitWriter<'a, REG, CasperSramApd>;
impl<'a, REG> CasperSramApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperSramPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<CasperSramPpd> for bool {
    #[inline(always)]
    fn from(variant: CasperSramPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_SRAM_PPD` writer - Peiph power"]
pub type CasperSramPpdW<'a, REG> = crate::BitWriter<'a, REG, CasperSramPpd>;
impl<'a, REG> CasperSramPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(CasperSramPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspcacheRegfApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<DspcacheRegfApd> for bool {
    #[inline(always)]
    fn from(variant: DspcacheRegfApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPCACHE_REGF_APD` writer - Array power"]
pub type DspcacheRegfApdW<'a, REG> = crate::BitWriter<'a, REG, DspcacheRegfApd>;
impl<'a, REG> DspcacheRegfApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DspcacheRegfPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<DspcacheRegfPpd> for bool {
    #[inline(always)]
    fn from(variant: DspcacheRegfPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPCACHE_REGF_PPD` writer - Peiph power"]
pub type DspcacheRegfPpdW<'a, REG> = crate::BitWriter<'a, REG, DspcacheRegfPpd>;
impl<'a, REG> DspcacheRegfPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(DspcacheRegfPpd::ClrPdruncfg1)
    }
}
#[doc = "Array power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsptcmRegfApd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<DsptcmRegfApd> for bool {
    #[inline(always)]
    fn from(variant: DsptcmRegfApd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPTCM_REGF_APD` writer - Array power"]
pub type DsptcmRegfApdW<'a, REG> = crate::BitWriter<'a, REG, DsptcmRegfApd>;
impl<'a, REG> DsptcmRegfApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfApd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfApd::ClrPdruncfg1)
    }
}
#[doc = "Peiph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsptcmRegfPpd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<DsptcmRegfPpd> for bool {
    #[inline(always)]
    fn from(variant: DsptcmRegfPpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPTCM_REGF_PPD` writer - Peiph power"]
pub type DsptcmRegfPpdW<'a, REG> = crate::BitWriter<'a, REG, DsptcmRegfPpd>;
impl<'a, REG> DsptcmRegfPpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfPpd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(DsptcmRegfPpd::ClrPdruncfg1)
    }
}
#[doc = "array power and periph power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<RomPd> for bool {
    #[inline(always)]
    fn from(variant: RomPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_PD` writer - array power and periph power"]
pub type RomPdW<'a, REG> = crate::BitWriter<'a, REG, RomPd>;
impl<'a, REG> RomPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RomPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(RomPd::ClrPdruncfg1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramSleep {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG1 Bit"]
    ClrPdruncfg1 = 1,
}
impl From<SramSleep> for bool {
    #[inline(always)]
    fn from(variant: SramSleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_SLEEP` writer - no description available"]
pub type SramSleepW<'a, REG> = crate::BitWriter<'a, REG, SramSleep>;
impl<'a, REG> SramSleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramSleep::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg1(self) -> &'a mut crate::W<REG> {
        self.variant(SramSleep::ClrPdruncfg1)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pdruncfg1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Array power"]
    #[inline(always)]
    pub fn pq_sram_apd(&mut self) -> PqSramApdW<Pdruncfg1ClrSpec> {
        PqSramApdW::new(self, 0)
    }
    #[doc = "Bit 1 - Peiph power"]
    #[inline(always)]
    pub fn pq_sram_ppd(&mut self) -> PqSramPpdW<Pdruncfg1ClrSpec> {
        PqSramPpdW::new(self, 1)
    }
    #[doc = "Bit 2 - Array power"]
    #[inline(always)]
    pub fn flexspi_sram_apd(&mut self) -> FlexspiSramApdW<Pdruncfg1ClrSpec> {
        FlexspiSramApdW::new(self, 2)
    }
    #[doc = "Bit 3 - Peiph power"]
    #[inline(always)]
    pub fn flexspi_sram_ppd(&mut self) -> FlexspiSramPpdW<Pdruncfg1ClrSpec> {
        FlexspiSramPpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Array power"]
    #[inline(always)]
    pub fn usbhs_sram_apd(&mut self) -> UsbhsSramApdW<Pdruncfg1ClrSpec> {
        UsbhsSramApdW::new(self, 4)
    }
    #[doc = "Bit 5 - Peiph power"]
    #[inline(always)]
    pub fn usbhs_sram_ppd(&mut self) -> UsbhsSramPpdW<Pdruncfg1ClrSpec> {
        UsbhsSramPpdW::new(self, 5)
    }
    #[doc = "Bit 6 - Array power"]
    #[inline(always)]
    pub fn usdhc0_sram_apd(&mut self) -> Usdhc0SramApdW<Pdruncfg1ClrSpec> {
        Usdhc0SramApdW::new(self, 6)
    }
    #[doc = "Bit 7 - Array power"]
    #[inline(always)]
    pub fn usdhc0_sram_ppd(&mut self) -> Usdhc0SramPpdW<Pdruncfg1ClrSpec> {
        Usdhc0SramPpdW::new(self, 7)
    }
    #[doc = "Bit 8 - Peiph power"]
    #[inline(always)]
    pub fn usdhc1_sram_apd(&mut self) -> Usdhc1SramApdW<Pdruncfg1ClrSpec> {
        Usdhc1SramApdW::new(self, 8)
    }
    #[doc = "Bit 9 - Peiph power"]
    #[inline(always)]
    pub fn usdhc1_sram_ppd(&mut self) -> Usdhc1SramPpdW<Pdruncfg1ClrSpec> {
        Usdhc1SramPpdW::new(self, 9)
    }
    #[doc = "Bit 10 - Array power"]
    #[inline(always)]
    pub fn casper_sram_apd(&mut self) -> CasperSramApdW<Pdruncfg1ClrSpec> {
        CasperSramApdW::new(self, 10)
    }
    #[doc = "Bit 11 - Peiph power"]
    #[inline(always)]
    pub fn casper_sram_ppd(&mut self) -> CasperSramPpdW<Pdruncfg1ClrSpec> {
        CasperSramPpdW::new(self, 11)
    }
    #[doc = "Bit 24 - Array power"]
    #[inline(always)]
    pub fn dspcache_regf_apd(&mut self) -> DspcacheRegfApdW<Pdruncfg1ClrSpec> {
        DspcacheRegfApdW::new(self, 24)
    }
    #[doc = "Bit 25 - Peiph power"]
    #[inline(always)]
    pub fn dspcache_regf_ppd(&mut self) -> DspcacheRegfPpdW<Pdruncfg1ClrSpec> {
        DspcacheRegfPpdW::new(self, 25)
    }
    #[doc = "Bit 26 - Array power"]
    #[inline(always)]
    pub fn dsptcm_regf_apd(&mut self) -> DsptcmRegfApdW<Pdruncfg1ClrSpec> {
        DsptcmRegfApdW::new(self, 26)
    }
    #[doc = "Bit 27 - Peiph power"]
    #[inline(always)]
    pub fn dsptcm_regf_ppd(&mut self) -> DsptcmRegfPpdW<Pdruncfg1ClrSpec> {
        DsptcmRegfPpdW::new(self, 27)
    }
    #[doc = "Bit 28 - array power and periph power"]
    #[inline(always)]
    pub fn rom_pd(&mut self) -> RomPdW<Pdruncfg1ClrSpec> {
        RomPdW::new(self, 28)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn sram_sleep(&mut self) -> SramSleepW<Pdruncfg1ClrSpec> {
        SramSleepW::new(self, 31)
    }
}
#[doc = "Run configuration 1 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg1ClrSpec;
impl crate::RegisterSpec for Pdruncfg1ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfg1_clr::W`](W) writer structure"]
impl crate::Writable for Pdruncfg1ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG1_CLR to value 0"]
impl crate::Resettable for Pdruncfg1ClrSpec {
    const RESET_VALUE: u32 = 0;
}
