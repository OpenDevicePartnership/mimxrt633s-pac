#[doc = "Register `PDRUNCFG2_CLR` writer"]
pub type W = crate::W<Pdruncfg2ClrSpec>;
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf0Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf0Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf0Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0_APD` writer - Array Power"]
pub type SramIf0ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf0Apd>;
impl<'a, REG> SramIf0ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf1Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf1Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf1Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1_APD` writer - Array Power"]
pub type SramIf1ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf1Apd>;
impl<'a, REG> SramIf1ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf2Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf2Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf2Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2_APD` writer - Array Power"]
pub type SramIf2ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf2Apd>;
impl<'a, REG> SramIf2ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf3Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf3Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf3Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3_APD` writer - Array Power"]
pub type SramIf3ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf3Apd>;
impl<'a, REG> SramIf3ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf4Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf4Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf4Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4_APD` writer - Array Power"]
pub type SramIf4ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf4Apd>;
impl<'a, REG> SramIf4ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf5Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf5Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf5Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5_APD` writer - Array Power"]
pub type SramIf5ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf5Apd>;
impl<'a, REG> SramIf5ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf6Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf6Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf6Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6_APD` writer - Array Power"]
pub type SramIf6ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf6Apd>;
impl<'a, REG> SramIf6ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf7Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf7Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf7Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7_APD` writer - Array Power"]
pub type SramIf7ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf7Apd>;
impl<'a, REG> SramIf7ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf8Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf8Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf8Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8_APD` writer - Array Power"]
pub type SramIf8ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf8Apd>;
impl<'a, REG> SramIf8ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf9Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf9Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf9Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9_APD` writer - Array Power"]
pub type SramIf9ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf9Apd>;
impl<'a, REG> SramIf9ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf10Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf10Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf10Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10_APD` writer - Array Power"]
pub type SramIf10ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf10Apd>;
impl<'a, REG> SramIf10ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf11Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf11Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf11Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11_APD` writer - Array Power"]
pub type SramIf11ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf11Apd>;
impl<'a, REG> SramIf11ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf12Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf12Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf12Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12_APD` writer - Array Power"]
pub type SramIf12ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf12Apd>;
impl<'a, REG> SramIf12ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf13Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf13Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf13Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13_APD` writer - Array Power"]
pub type SramIf13ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf13Apd>;
impl<'a, REG> SramIf13ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf14Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf14Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf14Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14_APD` writer - Array Power"]
pub type SramIf14ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf14Apd>;
impl<'a, REG> SramIf14ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf15Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf15Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf15Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15_APD` writer - Array Power"]
pub type SramIf15ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf15Apd>;
impl<'a, REG> SramIf15ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf16Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf16Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf16Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16_APD` writer - Array Power"]
pub type SramIf16ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf16Apd>;
impl<'a, REG> SramIf16ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf17Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf17Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf17Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17_APD` writer - Array Power"]
pub type SramIf17ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf17Apd>;
impl<'a, REG> SramIf17ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf18Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf18Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf18Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF18_APD` writer - Array Power"]
pub type SramIf18ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf18Apd>;
impl<'a, REG> SramIf18ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf19Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf19Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf19Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19_APD` writer - Array Power"]
pub type SramIf19ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf19Apd>;
impl<'a, REG> SramIf19ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf20Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf20Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf20Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20_APD` writer - Array Power"]
pub type SramIf20ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf20Apd>;
impl<'a, REG> SramIf20ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf21Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf21Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf21Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21_APD` writer - Array Power"]
pub type SramIf21ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf21Apd>;
impl<'a, REG> SramIf21ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf22Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf22Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf22Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22_APD` writer - Array Power"]
pub type SramIf22ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf22Apd>;
impl<'a, REG> SramIf22ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf23Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf23Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf23Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23_APD` writer - Array Power"]
pub type SramIf23ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf23Apd>;
impl<'a, REG> SramIf23ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf24Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf24Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf24Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24_APD` writer - Array Power"]
pub type SramIf24ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf24Apd>;
impl<'a, REG> SramIf24ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf25Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf25Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf25Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25_APD` writer - Array Power"]
pub type SramIf25ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf25Apd>;
impl<'a, REG> SramIf25ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf26Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf26Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf26Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26_APD` writer - Array Power"]
pub type SramIf26ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf26Apd>;
impl<'a, REG> SramIf26ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf27Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf27Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf27Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27_APD` writer - Array Power"]
pub type SramIf27ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf27Apd>;
impl<'a, REG> SramIf27ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf28Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf28Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf28Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28_APD` writer - Array Power"]
pub type SramIf28ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf28Apd>;
impl<'a, REG> SramIf28ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Apd::ClrPdruncfg2)
    }
}
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf29Apd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG2 Bit"]
    ClrPdruncfg2 = 1,
}
impl From<SramIf29Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf29Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29_APD` writer - Array Power"]
pub type SramIf29ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf29Apd>;
impl<'a, REG> SramIf29ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Apd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG2 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg2(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Apd::ClrPdruncfg2)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pdruncfg2ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Array Power"]
    #[inline(always)]
    pub fn sram_if0_apd(&mut self) -> SramIf0ApdW<Pdruncfg2ClrSpec> {
        SramIf0ApdW::new(self, 0)
    }
    #[doc = "Bit 1 - Array Power"]
    #[inline(always)]
    pub fn sram_if1_apd(&mut self) -> SramIf1ApdW<Pdruncfg2ClrSpec> {
        SramIf1ApdW::new(self, 1)
    }
    #[doc = "Bit 2 - Array Power"]
    #[inline(always)]
    pub fn sram_if2_apd(&mut self) -> SramIf2ApdW<Pdruncfg2ClrSpec> {
        SramIf2ApdW::new(self, 2)
    }
    #[doc = "Bit 3 - Array Power"]
    #[inline(always)]
    pub fn sram_if3_apd(&mut self) -> SramIf3ApdW<Pdruncfg2ClrSpec> {
        SramIf3ApdW::new(self, 3)
    }
    #[doc = "Bit 4 - Array Power"]
    #[inline(always)]
    pub fn sram_if4_apd(&mut self) -> SramIf4ApdW<Pdruncfg2ClrSpec> {
        SramIf4ApdW::new(self, 4)
    }
    #[doc = "Bit 5 - Array Power"]
    #[inline(always)]
    pub fn sram_if5_apd(&mut self) -> SramIf5ApdW<Pdruncfg2ClrSpec> {
        SramIf5ApdW::new(self, 5)
    }
    #[doc = "Bit 6 - Array Power"]
    #[inline(always)]
    pub fn sram_if6_apd(&mut self) -> SramIf6ApdW<Pdruncfg2ClrSpec> {
        SramIf6ApdW::new(self, 6)
    }
    #[doc = "Bit 7 - Array Power"]
    #[inline(always)]
    pub fn sram_if7_apd(&mut self) -> SramIf7ApdW<Pdruncfg2ClrSpec> {
        SramIf7ApdW::new(self, 7)
    }
    #[doc = "Bit 8 - Array Power"]
    #[inline(always)]
    pub fn sram_if8_apd(&mut self) -> SramIf8ApdW<Pdruncfg2ClrSpec> {
        SramIf8ApdW::new(self, 8)
    }
    #[doc = "Bit 9 - Array Power"]
    #[inline(always)]
    pub fn sram_if9_apd(&mut self) -> SramIf9ApdW<Pdruncfg2ClrSpec> {
        SramIf9ApdW::new(self, 9)
    }
    #[doc = "Bit 10 - Array Power"]
    #[inline(always)]
    pub fn sram_if10_apd(&mut self) -> SramIf10ApdW<Pdruncfg2ClrSpec> {
        SramIf10ApdW::new(self, 10)
    }
    #[doc = "Bit 11 - Array Power"]
    #[inline(always)]
    pub fn sram_if11_apd(&mut self) -> SramIf11ApdW<Pdruncfg2ClrSpec> {
        SramIf11ApdW::new(self, 11)
    }
    #[doc = "Bit 12 - Array Power"]
    #[inline(always)]
    pub fn sram_if12_apd(&mut self) -> SramIf12ApdW<Pdruncfg2ClrSpec> {
        SramIf12ApdW::new(self, 12)
    }
    #[doc = "Bit 13 - Array Power"]
    #[inline(always)]
    pub fn sram_if13_apd(&mut self) -> SramIf13ApdW<Pdruncfg2ClrSpec> {
        SramIf13ApdW::new(self, 13)
    }
    #[doc = "Bit 14 - Array Power"]
    #[inline(always)]
    pub fn sram_if14_apd(&mut self) -> SramIf14ApdW<Pdruncfg2ClrSpec> {
        SramIf14ApdW::new(self, 14)
    }
    #[doc = "Bit 15 - Array Power"]
    #[inline(always)]
    pub fn sram_if15_apd(&mut self) -> SramIf15ApdW<Pdruncfg2ClrSpec> {
        SramIf15ApdW::new(self, 15)
    }
    #[doc = "Bit 16 - Array Power"]
    #[inline(always)]
    pub fn sram_if16_apd(&mut self) -> SramIf16ApdW<Pdruncfg2ClrSpec> {
        SramIf16ApdW::new(self, 16)
    }
    #[doc = "Bit 17 - Array Power"]
    #[inline(always)]
    pub fn sram_if17_apd(&mut self) -> SramIf17ApdW<Pdruncfg2ClrSpec> {
        SramIf17ApdW::new(self, 17)
    }
    #[doc = "Bit 18 - Array Power"]
    #[inline(always)]
    pub fn sram_if18_apd(&mut self) -> SramIf18ApdW<Pdruncfg2ClrSpec> {
        SramIf18ApdW::new(self, 18)
    }
    #[doc = "Bit 19 - Array Power"]
    #[inline(always)]
    pub fn sram_if19_apd(&mut self) -> SramIf19ApdW<Pdruncfg2ClrSpec> {
        SramIf19ApdW::new(self, 19)
    }
    #[doc = "Bit 20 - Array Power"]
    #[inline(always)]
    pub fn sram_if20_apd(&mut self) -> SramIf20ApdW<Pdruncfg2ClrSpec> {
        SramIf20ApdW::new(self, 20)
    }
    #[doc = "Bit 21 - Array Power"]
    #[inline(always)]
    pub fn sram_if21_apd(&mut self) -> SramIf21ApdW<Pdruncfg2ClrSpec> {
        SramIf21ApdW::new(self, 21)
    }
    #[doc = "Bit 22 - Array Power"]
    #[inline(always)]
    pub fn sram_if22_apd(&mut self) -> SramIf22ApdW<Pdruncfg2ClrSpec> {
        SramIf22ApdW::new(self, 22)
    }
    #[doc = "Bit 23 - Array Power"]
    #[inline(always)]
    pub fn sram_if23_apd(&mut self) -> SramIf23ApdW<Pdruncfg2ClrSpec> {
        SramIf23ApdW::new(self, 23)
    }
    #[doc = "Bit 24 - Array Power"]
    #[inline(always)]
    pub fn sram_if24_apd(&mut self) -> SramIf24ApdW<Pdruncfg2ClrSpec> {
        SramIf24ApdW::new(self, 24)
    }
    #[doc = "Bit 25 - Array Power"]
    #[inline(always)]
    pub fn sram_if25_apd(&mut self) -> SramIf25ApdW<Pdruncfg2ClrSpec> {
        SramIf25ApdW::new(self, 25)
    }
    #[doc = "Bit 26 - Array Power"]
    #[inline(always)]
    pub fn sram_if26_apd(&mut self) -> SramIf26ApdW<Pdruncfg2ClrSpec> {
        SramIf26ApdW::new(self, 26)
    }
    #[doc = "Bit 27 - Array Power"]
    #[inline(always)]
    pub fn sram_if27_apd(&mut self) -> SramIf27ApdW<Pdruncfg2ClrSpec> {
        SramIf27ApdW::new(self, 27)
    }
    #[doc = "Bit 28 - Array Power"]
    #[inline(always)]
    pub fn sram_if28_apd(&mut self) -> SramIf28ApdW<Pdruncfg2ClrSpec> {
        SramIf28ApdW::new(self, 28)
    }
    #[doc = "Bit 29 - Array Power"]
    #[inline(always)]
    pub fn sram_if29_apd(&mut self) -> SramIf29ApdW<Pdruncfg2ClrSpec> {
        SramIf29ApdW::new(self, 29)
    }
}
#[doc = "Run configuration 2 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg2_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg2ClrSpec;
impl crate::RegisterSpec for Pdruncfg2ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfg2_clr::W`](W) writer structure"]
impl crate::Writable for Pdruncfg2ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG2_CLR to value 0"]
impl crate::Resettable for Pdruncfg2ClrSpec {
    const RESET_VALUE: u32 = 0;
}
