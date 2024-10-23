#[doc = "Register `PDSLEEPCFG3` reader"]
pub type R = crate::R<Pdsleepcfg3Spec>;
#[doc = "Register `PDSLEEPCFG3` writer"]
pub type W = crate::W<Pdsleepcfg3Spec>;
#[doc = "Periph Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf0Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf0Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf0Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0_PPD` reader - Periph Power"]
pub type SramIf0PpdR = crate::BitReader<SramIf0Ppd>;
impl SramIf0PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf0Ppd {
        match self.bits {
            false => SramIf0Ppd::Enabled,
            true => SramIf0Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf0Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf0Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF0_PPD` writer - Periph Power"]
pub type SramIf0PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf0Ppd>;
impl<'a, REG> SramIf0PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf1Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf1Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf1Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1_PPD` reader - Periph Power"]
pub type SramIf1PpdR = crate::BitReader<SramIf1Ppd>;
impl SramIf1PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf1Ppd {
        match self.bits {
            false => SramIf1Ppd::Enabled,
            true => SramIf1Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf1Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf1Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF1_PPD` writer - Periph Power"]
pub type SramIf1PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf1Ppd>;
impl<'a, REG> SramIf1PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf2Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf2Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf2Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2_PPD` reader - Periph Power"]
pub type SramIf2PpdR = crate::BitReader<SramIf2Ppd>;
impl SramIf2PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf2Ppd {
        match self.bits {
            false => SramIf2Ppd::Enabled,
            true => SramIf2Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf2Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf2Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF2_PPD` writer - Periph Power"]
pub type SramIf2PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf2Ppd>;
impl<'a, REG> SramIf2PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf3Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf3Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf3Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3_PPD` reader - Periph Power"]
pub type SramIf3PpdR = crate::BitReader<SramIf3Ppd>;
impl SramIf3PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf3Ppd {
        match self.bits {
            false => SramIf3Ppd::Enabled,
            true => SramIf3Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf3Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf3Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF3_PPD` writer - Periph Power"]
pub type SramIf3PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf3Ppd>;
impl<'a, REG> SramIf3PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf4Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf4Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf4Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4_PPD` reader - Periph Power"]
pub type SramIf4PpdR = crate::BitReader<SramIf4Ppd>;
impl SramIf4PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf4Ppd {
        match self.bits {
            false => SramIf4Ppd::Enabled,
            true => SramIf4Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf4Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf4Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF4_PPD` writer - Periph Power"]
pub type SramIf4PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf4Ppd>;
impl<'a, REG> SramIf4PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf5Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf5Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf5Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5_PPD` reader - Periph Power"]
pub type SramIf5PpdR = crate::BitReader<SramIf5Ppd>;
impl SramIf5PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf5Ppd {
        match self.bits {
            false => SramIf5Ppd::Enabled,
            true => SramIf5Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf5Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf5Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF5_PPD` writer - Periph Power"]
pub type SramIf5PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf5Ppd>;
impl<'a, REG> SramIf5PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf6Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf6Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf6Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6_PPD` reader - Periph Power"]
pub type SramIf6PpdR = crate::BitReader<SramIf6Ppd>;
impl SramIf6PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf6Ppd {
        match self.bits {
            false => SramIf6Ppd::Enabled,
            true => SramIf6Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf6Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf6Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF6_PPD` writer - Periph Power"]
pub type SramIf6PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf6Ppd>;
impl<'a, REG> SramIf6PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf7Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf7Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf7Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7_PPD` reader - Periph Power"]
pub type SramIf7PpdR = crate::BitReader<SramIf7Ppd>;
impl SramIf7PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf7Ppd {
        match self.bits {
            false => SramIf7Ppd::Enabled,
            true => SramIf7Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf7Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf7Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF7_PPD` writer - Periph Power"]
pub type SramIf7PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf7Ppd>;
impl<'a, REG> SramIf7PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf8Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf8Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf8Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8_PPD` reader - Periph Power"]
pub type SramIf8PpdR = crate::BitReader<SramIf8Ppd>;
impl SramIf8PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf8Ppd {
        match self.bits {
            false => SramIf8Ppd::Enabled,
            true => SramIf8Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf8Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf8Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF8_PPD` writer - Periph Power"]
pub type SramIf8PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf8Ppd>;
impl<'a, REG> SramIf8PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf9Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf9Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf9Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9_PPD` reader - Periph Power"]
pub type SramIf9PpdR = crate::BitReader<SramIf9Ppd>;
impl SramIf9PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf9Ppd {
        match self.bits {
            false => SramIf9Ppd::Enabled,
            true => SramIf9Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf9Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf9Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF9_PPD` writer - Periph Power"]
pub type SramIf9PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf9Ppd>;
impl<'a, REG> SramIf9PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf10Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf10Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf10Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10_PPD` reader - Periph Power"]
pub type SramIf10PpdR = crate::BitReader<SramIf10Ppd>;
impl SramIf10PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf10Ppd {
        match self.bits {
            false => SramIf10Ppd::Enabled,
            true => SramIf10Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf10Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf10Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF10_PPD` writer - Periph Power"]
pub type SramIf10PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf10Ppd>;
impl<'a, REG> SramIf10PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf11Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf11Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf11Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11_PPD` reader - Periph Power"]
pub type SramIf11PpdR = crate::BitReader<SramIf11Ppd>;
impl SramIf11PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf11Ppd {
        match self.bits {
            false => SramIf11Ppd::Enabled,
            true => SramIf11Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf11Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf11Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF11_PPD` writer - Periph Power"]
pub type SramIf11PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf11Ppd>;
impl<'a, REG> SramIf11PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf12Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf12Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf12Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12_PPD` reader - Periph Power"]
pub type SramIf12PpdR = crate::BitReader<SramIf12Ppd>;
impl SramIf12PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf12Ppd {
        match self.bits {
            false => SramIf12Ppd::Enabled,
            true => SramIf12Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf12Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf12Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF12_PPD` writer - Periph Power"]
pub type SramIf12PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf12Ppd>;
impl<'a, REG> SramIf12PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf13Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf13Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf13Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13_PPD` reader - Periph Power"]
pub type SramIf13PpdR = crate::BitReader<SramIf13Ppd>;
impl SramIf13PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf13Ppd {
        match self.bits {
            false => SramIf13Ppd::Enabled,
            true => SramIf13Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf13Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf13Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF13_PPD` writer - Periph Power"]
pub type SramIf13PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf13Ppd>;
impl<'a, REG> SramIf13PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf14Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf14Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf14Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14_PPD` reader - Periph Power"]
pub type SramIf14PpdR = crate::BitReader<SramIf14Ppd>;
impl SramIf14PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf14Ppd {
        match self.bits {
            false => SramIf14Ppd::Enabled,
            true => SramIf14Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf14Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf14Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF14_PPD` writer - Periph Power"]
pub type SramIf14PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf14Ppd>;
impl<'a, REG> SramIf14PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf15Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf15Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf15Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15_PPD` reader - Periph Power"]
pub type SramIf15PpdR = crate::BitReader<SramIf15Ppd>;
impl SramIf15PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf15Ppd {
        match self.bits {
            false => SramIf15Ppd::Enabled,
            true => SramIf15Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf15Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf15Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF15_PPD` writer - Periph Power"]
pub type SramIf15PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf15Ppd>;
impl<'a, REG> SramIf15PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf16Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf16Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf16Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16_PPD` reader - Periph Power"]
pub type SramIf16PpdR = crate::BitReader<SramIf16Ppd>;
impl SramIf16PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf16Ppd {
        match self.bits {
            false => SramIf16Ppd::Enabled,
            true => SramIf16Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf16Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf16Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF16_PPD` writer - Periph Power"]
pub type SramIf16PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf16Ppd>;
impl<'a, REG> SramIf16PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf17Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf17Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf17Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17_PPD` reader - Periph Power"]
pub type SramIf17PpdR = crate::BitReader<SramIf17Ppd>;
impl SramIf17PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf17Ppd {
        match self.bits {
            false => SramIf17Ppd::Enabled,
            true => SramIf17Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf17Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf17Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF17_PPD` writer - Periph Power"]
pub type SramIf17PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf17Ppd>;
impl<'a, REG> SramIf17PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf18Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf18Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf18Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF18_PPD` reader - Periph Power"]
pub type SramIf18PpdR = crate::BitReader<SramIf18Ppd>;
impl SramIf18PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf18Ppd {
        match self.bits {
            false => SramIf18Ppd::Enabled,
            true => SramIf18Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf18Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf18Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF18_PPD` writer - Periph Power"]
pub type SramIf18PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf18Ppd>;
impl<'a, REG> SramIf18PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf19Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf19Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf19Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19_PPD` reader - Periph Power"]
pub type SramIf19PpdR = crate::BitReader<SramIf19Ppd>;
impl SramIf19PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf19Ppd {
        match self.bits {
            false => SramIf19Ppd::Enabled,
            true => SramIf19Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf19Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf19Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF19_PPD` writer - Periph Power"]
pub type SramIf19PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf19Ppd>;
impl<'a, REG> SramIf19PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf20Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf20Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf20Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20_PPD` reader - Periph Power"]
pub type SramIf20PpdR = crate::BitReader<SramIf20Ppd>;
impl SramIf20PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf20Ppd {
        match self.bits {
            false => SramIf20Ppd::Enabled,
            true => SramIf20Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf20Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf20Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF20_PPD` writer - Periph Power"]
pub type SramIf20PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf20Ppd>;
impl<'a, REG> SramIf20PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf21Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf21Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf21Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21_PPD` reader - Periph Power"]
pub type SramIf21PpdR = crate::BitReader<SramIf21Ppd>;
impl SramIf21PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf21Ppd {
        match self.bits {
            false => SramIf21Ppd::Enabled,
            true => SramIf21Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf21Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf21Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF21_PPD` writer - Periph Power"]
pub type SramIf21PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf21Ppd>;
impl<'a, REG> SramIf21PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf22Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf22Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf22Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22_PPD` reader - Periph Power"]
pub type SramIf22PpdR = crate::BitReader<SramIf22Ppd>;
impl SramIf22PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf22Ppd {
        match self.bits {
            false => SramIf22Ppd::Enabled,
            true => SramIf22Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf22Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf22Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF22_PPD` writer - Periph Power"]
pub type SramIf22PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf22Ppd>;
impl<'a, REG> SramIf22PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf23Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf23Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf23Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23_PPD` reader - Periph Power"]
pub type SramIf23PpdR = crate::BitReader<SramIf23Ppd>;
impl SramIf23PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf23Ppd {
        match self.bits {
            false => SramIf23Ppd::Enabled,
            true => SramIf23Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf23Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf23Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF23_PPD` writer - Periph Power"]
pub type SramIf23PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf23Ppd>;
impl<'a, REG> SramIf23PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf24Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf24Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf24Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24_PPD` reader - Periph Power"]
pub type SramIf24PpdR = crate::BitReader<SramIf24Ppd>;
impl SramIf24PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf24Ppd {
        match self.bits {
            false => SramIf24Ppd::Enabled,
            true => SramIf24Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf24Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf24Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF24_PPD` writer - Periph Power"]
pub type SramIf24PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf24Ppd>;
impl<'a, REG> SramIf24PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf25Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf25Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf25Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25_PPD` reader - Periph Power"]
pub type SramIf25PpdR = crate::BitReader<SramIf25Ppd>;
impl SramIf25PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf25Ppd {
        match self.bits {
            false => SramIf25Ppd::Enabled,
            true => SramIf25Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf25Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf25Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF25_PPD` writer - Periph Power"]
pub type SramIf25PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf25Ppd>;
impl<'a, REG> SramIf25PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf26Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf26Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf26Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26_PPD` reader - Periph Power"]
pub type SramIf26PpdR = crate::BitReader<SramIf26Ppd>;
impl SramIf26PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf26Ppd {
        match self.bits {
            false => SramIf26Ppd::Enabled,
            true => SramIf26Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf26Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf26Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF26_PPD` writer - Periph Power"]
pub type SramIf26PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf26Ppd>;
impl<'a, REG> SramIf26PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf27Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf27Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf27Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27_PPD` reader - Periph Power"]
pub type SramIf27PpdR = crate::BitReader<SramIf27Ppd>;
impl SramIf27PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf27Ppd {
        match self.bits {
            false => SramIf27Ppd::Enabled,
            true => SramIf27Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf27Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf27Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF27_PPD` writer - Periph Power"]
pub type SramIf27PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf27Ppd>;
impl<'a, REG> SramIf27PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf28Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf28Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf28Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28_PPD` reader - Periph Power"]
pub type SramIf28PpdR = crate::BitReader<SramIf28Ppd>;
impl SramIf28PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf28Ppd {
        match self.bits {
            false => SramIf28Ppd::Enabled,
            true => SramIf28Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf28Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf28Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF28_PPD` writer - Periph Power"]
pub type SramIf28PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf28Ppd>;
impl<'a, REG> SramIf28PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Ppd::PowerDown)
    }
}
#[doc = "Periph Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf29Ppd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf29Ppd> for bool {
    #[inline(always)]
    fn from(variant: SramIf29Ppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29_PPD` reader - Periph Power"]
pub type SramIf29PpdR = crate::BitReader<SramIf29Ppd>;
impl SramIf29PpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf29Ppd {
        match self.bits {
            false => SramIf29Ppd::Enabled,
            true => SramIf29Ppd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf29Ppd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf29Ppd::PowerDown
    }
}
#[doc = "Field `SRAM_IF29_PPD` writer - Periph Power"]
pub type SramIf29PpdW<'a, REG> = crate::BitWriter<'a, REG, SramIf29Ppd>;
impl<'a, REG> SramIf29PpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Ppd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Ppd::PowerDown)
    }
}
impl R {
    #[doc = "Bit 0 - Periph Power"]
    #[inline(always)]
    pub fn sram_if0_ppd(&self) -> SramIf0PpdR {
        SramIf0PpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periph Power"]
    #[inline(always)]
    pub fn sram_if1_ppd(&self) -> SramIf1PpdR {
        SramIf1PpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periph Power"]
    #[inline(always)]
    pub fn sram_if2_ppd(&self) -> SramIf2PpdR {
        SramIf2PpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periph Power"]
    #[inline(always)]
    pub fn sram_if3_ppd(&self) -> SramIf3PpdR {
        SramIf3PpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periph Power"]
    #[inline(always)]
    pub fn sram_if4_ppd(&self) -> SramIf4PpdR {
        SramIf4PpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periph Power"]
    #[inline(always)]
    pub fn sram_if5_ppd(&self) -> SramIf5PpdR {
        SramIf5PpdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periph Power"]
    #[inline(always)]
    pub fn sram_if6_ppd(&self) -> SramIf6PpdR {
        SramIf6PpdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periph Power"]
    #[inline(always)]
    pub fn sram_if7_ppd(&self) -> SramIf7PpdR {
        SramIf7PpdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periph Power"]
    #[inline(always)]
    pub fn sram_if8_ppd(&self) -> SramIf8PpdR {
        SramIf8PpdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Periph Power"]
    #[inline(always)]
    pub fn sram_if9_ppd(&self) -> SramIf9PpdR {
        SramIf9PpdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Periph Power"]
    #[inline(always)]
    pub fn sram_if10_ppd(&self) -> SramIf10PpdR {
        SramIf10PpdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Periph Power"]
    #[inline(always)]
    pub fn sram_if11_ppd(&self) -> SramIf11PpdR {
        SramIf11PpdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Periph Power"]
    #[inline(always)]
    pub fn sram_if12_ppd(&self) -> SramIf12PpdR {
        SramIf12PpdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Periph Power"]
    #[inline(always)]
    pub fn sram_if13_ppd(&self) -> SramIf13PpdR {
        SramIf13PpdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periph Power"]
    #[inline(always)]
    pub fn sram_if14_ppd(&self) -> SramIf14PpdR {
        SramIf14PpdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Periph Power"]
    #[inline(always)]
    pub fn sram_if15_ppd(&self) -> SramIf15PpdR {
        SramIf15PpdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Periph Power"]
    #[inline(always)]
    pub fn sram_if16_ppd(&self) -> SramIf16PpdR {
        SramIf16PpdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periph Power"]
    #[inline(always)]
    pub fn sram_if17_ppd(&self) -> SramIf17PpdR {
        SramIf17PpdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Periph Power"]
    #[inline(always)]
    pub fn sram_if18_ppd(&self) -> SramIf18PpdR {
        SramIf18PpdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Periph Power"]
    #[inline(always)]
    pub fn sram_if19_ppd(&self) -> SramIf19PpdR {
        SramIf19PpdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Periph Power"]
    #[inline(always)]
    pub fn sram_if20_ppd(&self) -> SramIf20PpdR {
        SramIf20PpdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Periph Power"]
    #[inline(always)]
    pub fn sram_if21_ppd(&self) -> SramIf21PpdR {
        SramIf21PpdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Periph Power"]
    #[inline(always)]
    pub fn sram_if22_ppd(&self) -> SramIf22PpdR {
        SramIf22PpdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Periph Power"]
    #[inline(always)]
    pub fn sram_if23_ppd(&self) -> SramIf23PpdR {
        SramIf23PpdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Periph Power"]
    #[inline(always)]
    pub fn sram_if24_ppd(&self) -> SramIf24PpdR {
        SramIf24PpdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Periph Power"]
    #[inline(always)]
    pub fn sram_if25_ppd(&self) -> SramIf25PpdR {
        SramIf25PpdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periph Power"]
    #[inline(always)]
    pub fn sram_if26_ppd(&self) -> SramIf26PpdR {
        SramIf26PpdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Periph Power"]
    #[inline(always)]
    pub fn sram_if27_ppd(&self) -> SramIf27PpdR {
        SramIf27PpdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Periph Power"]
    #[inline(always)]
    pub fn sram_if28_ppd(&self) -> SramIf28PpdR {
        SramIf28PpdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Periph Power"]
    #[inline(always)]
    pub fn sram_if29_ppd(&self) -> SramIf29PpdR {
        SramIf29PpdR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSLEEPCFG3")
            .field("sram_if0_ppd", &self.sram_if0_ppd())
            .field("sram_if1_ppd", &self.sram_if1_ppd())
            .field("sram_if2_ppd", &self.sram_if2_ppd())
            .field("sram_if3_ppd", &self.sram_if3_ppd())
            .field("sram_if4_ppd", &self.sram_if4_ppd())
            .field("sram_if5_ppd", &self.sram_if5_ppd())
            .field("sram_if6_ppd", &self.sram_if6_ppd())
            .field("sram_if7_ppd", &self.sram_if7_ppd())
            .field("sram_if8_ppd", &self.sram_if8_ppd())
            .field("sram_if9_ppd", &self.sram_if9_ppd())
            .field("sram_if10_ppd", &self.sram_if10_ppd())
            .field("sram_if11_ppd", &self.sram_if11_ppd())
            .field("sram_if12_ppd", &self.sram_if12_ppd())
            .field("sram_if13_ppd", &self.sram_if13_ppd())
            .field("sram_if14_ppd", &self.sram_if14_ppd())
            .field("sram_if15_ppd", &self.sram_if15_ppd())
            .field("sram_if16_ppd", &self.sram_if16_ppd())
            .field("sram_if17_ppd", &self.sram_if17_ppd())
            .field("sram_if18_ppd", &self.sram_if18_ppd())
            .field("sram_if19_ppd", &self.sram_if19_ppd())
            .field("sram_if20_ppd", &self.sram_if20_ppd())
            .field("sram_if21_ppd", &self.sram_if21_ppd())
            .field("sram_if22_ppd", &self.sram_if22_ppd())
            .field("sram_if23_ppd", &self.sram_if23_ppd())
            .field("sram_if24_ppd", &self.sram_if24_ppd())
            .field("sram_if25_ppd", &self.sram_if25_ppd())
            .field("sram_if26_ppd", &self.sram_if26_ppd())
            .field("sram_if27_ppd", &self.sram_if27_ppd())
            .field("sram_if28_ppd", &self.sram_if28_ppd())
            .field("sram_if29_ppd", &self.sram_if29_ppd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if0_ppd(&mut self) -> SramIf0PpdW<Pdsleepcfg3Spec> {
        SramIf0PpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if1_ppd(&mut self) -> SramIf1PpdW<Pdsleepcfg3Spec> {
        SramIf1PpdW::new(self, 1)
    }
    #[doc = "Bit 2 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if2_ppd(&mut self) -> SramIf2PpdW<Pdsleepcfg3Spec> {
        SramIf2PpdW::new(self, 2)
    }
    #[doc = "Bit 3 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if3_ppd(&mut self) -> SramIf3PpdW<Pdsleepcfg3Spec> {
        SramIf3PpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if4_ppd(&mut self) -> SramIf4PpdW<Pdsleepcfg3Spec> {
        SramIf4PpdW::new(self, 4)
    }
    #[doc = "Bit 5 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if5_ppd(&mut self) -> SramIf5PpdW<Pdsleepcfg3Spec> {
        SramIf5PpdW::new(self, 5)
    }
    #[doc = "Bit 6 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if6_ppd(&mut self) -> SramIf6PpdW<Pdsleepcfg3Spec> {
        SramIf6PpdW::new(self, 6)
    }
    #[doc = "Bit 7 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if7_ppd(&mut self) -> SramIf7PpdW<Pdsleepcfg3Spec> {
        SramIf7PpdW::new(self, 7)
    }
    #[doc = "Bit 8 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if8_ppd(&mut self) -> SramIf8PpdW<Pdsleepcfg3Spec> {
        SramIf8PpdW::new(self, 8)
    }
    #[doc = "Bit 9 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if9_ppd(&mut self) -> SramIf9PpdW<Pdsleepcfg3Spec> {
        SramIf9PpdW::new(self, 9)
    }
    #[doc = "Bit 10 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if10_ppd(&mut self) -> SramIf10PpdW<Pdsleepcfg3Spec> {
        SramIf10PpdW::new(self, 10)
    }
    #[doc = "Bit 11 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if11_ppd(&mut self) -> SramIf11PpdW<Pdsleepcfg3Spec> {
        SramIf11PpdW::new(self, 11)
    }
    #[doc = "Bit 12 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if12_ppd(&mut self) -> SramIf12PpdW<Pdsleepcfg3Spec> {
        SramIf12PpdW::new(self, 12)
    }
    #[doc = "Bit 13 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if13_ppd(&mut self) -> SramIf13PpdW<Pdsleepcfg3Spec> {
        SramIf13PpdW::new(self, 13)
    }
    #[doc = "Bit 14 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if14_ppd(&mut self) -> SramIf14PpdW<Pdsleepcfg3Spec> {
        SramIf14PpdW::new(self, 14)
    }
    #[doc = "Bit 15 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if15_ppd(&mut self) -> SramIf15PpdW<Pdsleepcfg3Spec> {
        SramIf15PpdW::new(self, 15)
    }
    #[doc = "Bit 16 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if16_ppd(&mut self) -> SramIf16PpdW<Pdsleepcfg3Spec> {
        SramIf16PpdW::new(self, 16)
    }
    #[doc = "Bit 17 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if17_ppd(&mut self) -> SramIf17PpdW<Pdsleepcfg3Spec> {
        SramIf17PpdW::new(self, 17)
    }
    #[doc = "Bit 18 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if18_ppd(&mut self) -> SramIf18PpdW<Pdsleepcfg3Spec> {
        SramIf18PpdW::new(self, 18)
    }
    #[doc = "Bit 19 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if19_ppd(&mut self) -> SramIf19PpdW<Pdsleepcfg3Spec> {
        SramIf19PpdW::new(self, 19)
    }
    #[doc = "Bit 20 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if20_ppd(&mut self) -> SramIf20PpdW<Pdsleepcfg3Spec> {
        SramIf20PpdW::new(self, 20)
    }
    #[doc = "Bit 21 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if21_ppd(&mut self) -> SramIf21PpdW<Pdsleepcfg3Spec> {
        SramIf21PpdW::new(self, 21)
    }
    #[doc = "Bit 22 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if22_ppd(&mut self) -> SramIf22PpdW<Pdsleepcfg3Spec> {
        SramIf22PpdW::new(self, 22)
    }
    #[doc = "Bit 23 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if23_ppd(&mut self) -> SramIf23PpdW<Pdsleepcfg3Spec> {
        SramIf23PpdW::new(self, 23)
    }
    #[doc = "Bit 24 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if24_ppd(&mut self) -> SramIf24PpdW<Pdsleepcfg3Spec> {
        SramIf24PpdW::new(self, 24)
    }
    #[doc = "Bit 25 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if25_ppd(&mut self) -> SramIf25PpdW<Pdsleepcfg3Spec> {
        SramIf25PpdW::new(self, 25)
    }
    #[doc = "Bit 26 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if26_ppd(&mut self) -> SramIf26PpdW<Pdsleepcfg3Spec> {
        SramIf26PpdW::new(self, 26)
    }
    #[doc = "Bit 27 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if27_ppd(&mut self) -> SramIf27PpdW<Pdsleepcfg3Spec> {
        SramIf27PpdW::new(self, 27)
    }
    #[doc = "Bit 28 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if28_ppd(&mut self) -> SramIf28PpdW<Pdsleepcfg3Spec> {
        SramIf28PpdW::new(self, 28)
    }
    #[doc = "Bit 29 - Periph Power"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if29_ppd(&mut self) -> SramIf29PpdW<Pdsleepcfg3Spec> {
        SramIf29PpdW::new(self, 29)
    }
}
#[doc = "Sleep configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdsleepcfg3Spec;
impl crate::RegisterSpec for Pdsleepcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsleepcfg3::R`](R) reader structure"]
impl crate::Readable for Pdsleepcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pdsleepcfg3::W`](W) writer structure"]
impl crate::Writable for Pdsleepcfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG3 to value 0xffff_fffe"]
impl crate::Resettable for Pdsleepcfg3Spec {
    const RESET_VALUE: u32 = 0xffff_fffe;
}
