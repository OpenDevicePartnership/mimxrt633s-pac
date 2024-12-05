#[doc = "Register `PDRUNCFG3_CLR` writer"]
pub type W = crate::W<Pdruncfg3ClrSpec>;
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf0Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf0Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf0Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0_PPD` writer - Periph Power"]
pub type SramIf0PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf0Ppd>;
impl<'a, REG> SramIf0PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf1Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf1Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf1Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1_PPD` writer - Periph Power"]
pub type SramIf1PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf1Ppd>;
impl<'a, REG> SramIf1PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf2Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf2Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf2Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2_PPD` writer - Periph Power"]
pub type SramIf2PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf2Ppd>;
impl<'a, REG> SramIf2PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf3Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf3Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf3Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3_PPD` writer - Periph Power"]
pub type SramIf3PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf3Ppd>;
impl<'a, REG> SramIf3PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf4Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf4Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf4Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4_PPD` writer - Periph Power"]
pub type SramIf4PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf4Ppd>;
impl<'a, REG> SramIf4PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf5Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf5Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf5Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5_PPD` writer - Periph Power"]
pub type SramIf5PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf5Ppd>;
impl<'a, REG> SramIf5PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf6Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf6Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf6Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6_PPD` writer - Periph Power"]
pub type SramIf6PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf6Ppd>;
impl<'a, REG> SramIf6PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf7Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf7Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf7Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7_PPD` writer - Periph Power"]
pub type SramIf7PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf7Ppd>;
impl<'a, REG> SramIf7PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf8Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf8Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf8Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8_PPD` writer - Periph Power"]
pub type SramIf8PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf8Ppd>;
impl<'a, REG> SramIf8PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf9Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf9Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf9Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9_PPD` writer - Periph Power"]
pub type SramIf9PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf9Ppd>;
impl<'a, REG> SramIf9PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf10Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf10Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf10Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10_PPD` writer - Periph Power"]
pub type SramIf10PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf10Ppd>;
impl<'a, REG> SramIf10PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf11Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf11Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf11Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11_PPD` writer - Periph Power"]
pub type SramIf11PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf11Ppd>;
impl<'a, REG> SramIf11PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf12Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf12Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf12Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12_PPD` writer - Periph Power"]
pub type SramIf12PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf12Ppd>;
impl<'a, REG> SramIf12PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf13Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf13Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf13Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13_PPD` writer - Periph Power"]
pub type SramIf13PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf13Ppd>;
impl<'a, REG> SramIf13PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf14Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf14Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf14Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14_PPD` writer - Periph Power"]
pub type SramIf14PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf14Ppd>;
impl<'a, REG> SramIf14PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf15Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf15Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf15Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15_PPD` writer - Periph Power"]
pub type SramIf15PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf15Ppd>;
impl<'a, REG> SramIf15PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf16Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf16Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf16Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16_PPD` writer - Periph Power"]
pub type SramIf16PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf16Ppd>;
impl<'a, REG> SramIf16PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf17Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf17Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf17Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17_PPD` writer - Periph Power"]
pub type SramIf17PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf17Ppd>;
impl<'a, REG> SramIf17PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf18Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf18Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf18Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF18_PPD` writer - Periph Power"]
pub type SramIf18PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf18Ppd>;
impl<'a, REG> SramIf18PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf19Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf19Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf19Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19_PPD` writer - Periph Power"]
pub type SramIf19PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf19Ppd>;
impl<'a, REG> SramIf19PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf20Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf20Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf20Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20_PPD` writer - Periph Power"]
pub type SramIf20PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf20Ppd>;
impl<'a, REG> SramIf20PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf21Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf21Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf21Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21_PPD` writer - Periph Power"]
pub type SramIf21PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf21Ppd>;
impl<'a, REG> SramIf21PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf22Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf22Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf22Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22_PPD` writer - Periph Power"]
pub type SramIf22PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf22Ppd>;
impl<'a, REG> SramIf22PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf23Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf23Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf23Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23_PPD` writer - Periph Power"]
pub type SramIf23PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf23Ppd>;
impl<'a, REG> SramIf23PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf24Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf24Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf24Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24_PPD` writer - Periph Power"]
pub type SramIf24PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf24Ppd>;
impl<'a, REG> SramIf24PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf25Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf25Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf25Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25_PPD` writer - Periph Power"]
pub type SramIf25PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf25Ppd>;
impl<'a, REG> SramIf25PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf26Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf26Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf26Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26_PPD` writer - Periph Power"]
pub type SramIf26PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf26Ppd>;
impl<'a, REG> SramIf26PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf27Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf27Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf27Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27_PPD` writer - Periph Power"]
pub type SramIf27PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf27Ppd>;
impl<'a, REG> SramIf27PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf28Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf28Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf28Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28_PPD` writer - Periph Power"]
pub type SramIf28PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf28Ppd>;
impl<'a, REG> SramIf28PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Ppd::ClrPdruncfg3)
    }
}
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf29Ppd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG3 Bit"]
    ClrPdruncfg3 = 1,
}
impl From<SramIf29Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf29Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29_PPD` writer - Periph Power"]
pub type SramIf29PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf29Ppd>;
impl<'a, REG> SramIf29PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Ppd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg3(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Ppd::ClrPdruncfg3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pdruncfg3ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Periph Power"]
    #[inline(always)]
    pub fn sram_if0_ppd(&mut self) -> SramIf0PpdW<Pdruncfg3ClrSpec> {
        SramIf0PpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Periph Power"]
    #[inline(always)]
    pub fn sram_if1_ppd(&mut self) -> SramIf1PpdW<Pdruncfg3ClrSpec> {
        SramIf1PpdW::new(self, 1)
    }
    #[doc = "Bit 2 - Periph Power"]
    #[inline(always)]
    pub fn sram_if2_ppd(&mut self) -> SramIf2PpdW<Pdruncfg3ClrSpec> {
        SramIf2PpdW::new(self, 2)
    }
    #[doc = "Bit 3 - Periph Power"]
    #[inline(always)]
    pub fn sram_if3_ppd(&mut self) -> SramIf3PpdW<Pdruncfg3ClrSpec> {
        SramIf3PpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Periph Power"]
    #[inline(always)]
    pub fn sram_if4_ppd(&mut self) -> SramIf4PpdW<Pdruncfg3ClrSpec> {
        SramIf4PpdW::new(self, 4)
    }
    #[doc = "Bit 5 - Periph Power"]
    #[inline(always)]
    pub fn sram_if5_ppd(&mut self) -> SramIf5PpdW<Pdruncfg3ClrSpec> {
        SramIf5PpdW::new(self, 5)
    }
    #[doc = "Bit 6 - Periph Power"]
    #[inline(always)]
    pub fn sram_if6_ppd(&mut self) -> SramIf6PpdW<Pdruncfg3ClrSpec> {
        SramIf6PpdW::new(self, 6)
    }
    #[doc = "Bit 7 - Periph Power"]
    #[inline(always)]
    pub fn sram_if7_ppd(&mut self) -> SramIf7PpdW<Pdruncfg3ClrSpec> {
        SramIf7PpdW::new(self, 7)
    }
    #[doc = "Bit 8 - Periph Power"]
    #[inline(always)]
    pub fn sram_if8_ppd(&mut self) -> SramIf8PpdW<Pdruncfg3ClrSpec> {
        SramIf8PpdW::new(self, 8)
    }
    #[doc = "Bit 9 - Periph Power"]
    #[inline(always)]
    pub fn sram_if9_ppd(&mut self) -> SramIf9PpdW<Pdruncfg3ClrSpec> {
        SramIf9PpdW::new(self, 9)
    }
    #[doc = "Bit 10 - Periph Power"]
    #[inline(always)]
    pub fn sram_if10_ppd(&mut self) -> SramIf10PpdW<Pdruncfg3ClrSpec> {
        SramIf10PpdW::new(self, 10)
    }
    #[doc = "Bit 11 - Periph Power"]
    #[inline(always)]
    pub fn sram_if11_ppd(&mut self) -> SramIf11PpdW<Pdruncfg3ClrSpec> {
        SramIf11PpdW::new(self, 11)
    }
    #[doc = "Bit 12 - Periph Power"]
    #[inline(always)]
    pub fn sram_if12_ppd(&mut self) -> SramIf12PpdW<Pdruncfg3ClrSpec> {
        SramIf12PpdW::new(self, 12)
    }
    #[doc = "Bit 13 - Periph Power"]
    #[inline(always)]
    pub fn sram_if13_ppd(&mut self) -> SramIf13PpdW<Pdruncfg3ClrSpec> {
        SramIf13PpdW::new(self, 13)
    }
    #[doc = "Bit 14 - Periph Power"]
    #[inline(always)]
    pub fn sram_if14_ppd(&mut self) -> SramIf14PpdW<Pdruncfg3ClrSpec> {
        SramIf14PpdW::new(self, 14)
    }
    #[doc = "Bit 15 - Periph Power"]
    #[inline(always)]
    pub fn sram_if15_ppd(&mut self) -> SramIf15PpdW<Pdruncfg3ClrSpec> {
        SramIf15PpdW::new(self, 15)
    }
    #[doc = "Bit 16 - Periph Power"]
    #[inline(always)]
    pub fn sram_if16_ppd(&mut self) -> SramIf16PpdW<Pdruncfg3ClrSpec> {
        SramIf16PpdW::new(self, 16)
    }
    #[doc = "Bit 17 - Periph Power"]
    #[inline(always)]
    pub fn sram_if17_ppd(&mut self) -> SramIf17PpdW<Pdruncfg3ClrSpec> {
        SramIf17PpdW::new(self, 17)
    }
    #[doc = "Bit 18 - Periph Power"]
    #[inline(always)]
    pub fn sram_if18_ppd(&mut self) -> SramIf18PpdW<Pdruncfg3ClrSpec> {
        SramIf18PpdW::new(self, 18)
    }
    #[doc = "Bit 19 - Periph Power"]
    #[inline(always)]
    pub fn sram_if19_ppd(&mut self) -> SramIf19PpdW<Pdruncfg3ClrSpec> {
        SramIf19PpdW::new(self, 19)
    }
    #[doc = "Bit 20 - Periph Power"]
    #[inline(always)]
    pub fn sram_if20_ppd(&mut self) -> SramIf20PpdW<Pdruncfg3ClrSpec> {
        SramIf20PpdW::new(self, 20)
    }
    #[doc = "Bit 21 - Periph Power"]
    #[inline(always)]
    pub fn sram_if21_ppd(&mut self) -> SramIf21PpdW<Pdruncfg3ClrSpec> {
        SramIf21PpdW::new(self, 21)
    }
    #[doc = "Bit 22 - Periph Power"]
    #[inline(always)]
    pub fn sram_if22_ppd(&mut self) -> SramIf22PpdW<Pdruncfg3ClrSpec> {
        SramIf22PpdW::new(self, 22)
    }
    #[doc = "Bit 23 - Periph Power"]
    #[inline(always)]
    pub fn sram_if23_ppd(&mut self) -> SramIf23PpdW<Pdruncfg3ClrSpec> {
        SramIf23PpdW::new(self, 23)
    }
    #[doc = "Bit 24 - Periph Power"]
    #[inline(always)]
    pub fn sram_if24_ppd(&mut self) -> SramIf24PpdW<Pdruncfg3ClrSpec> {
        SramIf24PpdW::new(self, 24)
    }
    #[doc = "Bit 25 - Periph Power"]
    #[inline(always)]
    pub fn sram_if25_ppd(&mut self) -> SramIf25PpdW<Pdruncfg3ClrSpec> {
        SramIf25PpdW::new(self, 25)
    }
    #[doc = "Bit 26 - Periph Power"]
    #[inline(always)]
    pub fn sram_if26_ppd(&mut self) -> SramIf26PpdW<Pdruncfg3ClrSpec> {
        SramIf26PpdW::new(self, 26)
    }
    #[doc = "Bit 27 - Periph Power"]
    #[inline(always)]
    pub fn sram_if27_ppd(&mut self) -> SramIf27PpdW<Pdruncfg3ClrSpec> {
        SramIf27PpdW::new(self, 27)
    }
    #[doc = "Bit 28 - Periph Power"]
    #[inline(always)]
    pub fn sram_if28_ppd(&mut self) -> SramIf28PpdW<Pdruncfg3ClrSpec> {
        SramIf28PpdW::new(self, 28)
    }
    #[doc = "Bit 29 - Periph Power"]
    #[inline(always)]
    pub fn sram_if29_ppd(&mut self) -> SramIf29PpdW<Pdruncfg3ClrSpec> {
        SramIf29PpdW::new(self, 29)
    }
}
#[doc = "Run configuration 3 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg3_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg3ClrSpec;
impl crate::RegisterSpec for Pdruncfg3ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfg3_clr::W`](W) writer structure"]
impl crate::Writable for Pdruncfg3ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG3_CLR to value 0"]
impl crate::Resettable for Pdruncfg3ClrSpec {
    const RESET_VALUE: u32 = 0;
}
