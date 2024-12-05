#[doc = "Register `PDSLEEPCFG2` reader"]
pub type R = crate::R<Pdsleepcfg2Spec>;
#[doc = "Register `PDSLEEPCFG2` writer"]
pub type W = crate::W<Pdsleepcfg2Spec>;
#[doc = "Array Power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf0Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf0Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf0Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0_APD` reader - Array Power"]
pub type SramIf0ApdR = crate::BitReader<SramIf0Apd>;
impl SramIf0ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf0Apd {
        match self.bits {
            false => SramIf0Apd::Enabled,
            true => SramIf0Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf0Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf0Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF0_APD` writer - Array Power"]
pub type SramIf0ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf0Apd>;
impl<'a, REG> SramIf0ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf1Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf1Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf1Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1_APD` reader - Array Power"]
pub type SramIf1ApdR = crate::BitReader<SramIf1Apd>;
impl SramIf1ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf1Apd {
        match self.bits {
            false => SramIf1Apd::Enabled,
            true => SramIf1Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf1Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf1Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF1_APD` writer - Array Power"]
pub type SramIf1ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf1Apd>;
impl<'a, REG> SramIf1ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf2Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf2Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf2Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2_APD` reader - Array Power"]
pub type SramIf2ApdR = crate::BitReader<SramIf2Apd>;
impl SramIf2ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf2Apd {
        match self.bits {
            false => SramIf2Apd::Enabled,
            true => SramIf2Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf2Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf2Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF2_APD` writer - Array Power"]
pub type SramIf2ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf2Apd>;
impl<'a, REG> SramIf2ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf3Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf3Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf3Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3_APD` reader - Array Power"]
pub type SramIf3ApdR = crate::BitReader<SramIf3Apd>;
impl SramIf3ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf3Apd {
        match self.bits {
            false => SramIf3Apd::Enabled,
            true => SramIf3Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf3Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf3Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF3_APD` writer - Array Power"]
pub type SramIf3ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf3Apd>;
impl<'a, REG> SramIf3ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf4Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf4Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf4Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4_APD` reader - Array Power"]
pub type SramIf4ApdR = crate::BitReader<SramIf4Apd>;
impl SramIf4ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf4Apd {
        match self.bits {
            false => SramIf4Apd::Enabled,
            true => SramIf4Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf4Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf4Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF4_APD` writer - Array Power"]
pub type SramIf4ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf4Apd>;
impl<'a, REG> SramIf4ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf5Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf5Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf5Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5_APD` reader - Array Power"]
pub type SramIf5ApdR = crate::BitReader<SramIf5Apd>;
impl SramIf5ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf5Apd {
        match self.bits {
            false => SramIf5Apd::Enabled,
            true => SramIf5Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf5Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf5Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF5_APD` writer - Array Power"]
pub type SramIf5ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf5Apd>;
impl<'a, REG> SramIf5ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf6Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf6Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf6Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6_APD` reader - Array Power"]
pub type SramIf6ApdR = crate::BitReader<SramIf6Apd>;
impl SramIf6ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf6Apd {
        match self.bits {
            false => SramIf6Apd::Enabled,
            true => SramIf6Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf6Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf6Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF6_APD` writer - Array Power"]
pub type SramIf6ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf6Apd>;
impl<'a, REG> SramIf6ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf7Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf7Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf7Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7_APD` reader - Array Power"]
pub type SramIf7ApdR = crate::BitReader<SramIf7Apd>;
impl SramIf7ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf7Apd {
        match self.bits {
            false => SramIf7Apd::Enabled,
            true => SramIf7Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf7Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf7Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF7_APD` writer - Array Power"]
pub type SramIf7ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf7Apd>;
impl<'a, REG> SramIf7ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf8Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf8Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf8Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8_APD` reader - Array Power"]
pub type SramIf8ApdR = crate::BitReader<SramIf8Apd>;
impl SramIf8ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf8Apd {
        match self.bits {
            false => SramIf8Apd::Enabled,
            true => SramIf8Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf8Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf8Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF8_APD` writer - Array Power"]
pub type SramIf8ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf8Apd>;
impl<'a, REG> SramIf8ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf9Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf9Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf9Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9_APD` reader - Array Power"]
pub type SramIf9ApdR = crate::BitReader<SramIf9Apd>;
impl SramIf9ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf9Apd {
        match self.bits {
            false => SramIf9Apd::Enabled,
            true => SramIf9Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf9Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf9Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF9_APD` writer - Array Power"]
pub type SramIf9ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf9Apd>;
impl<'a, REG> SramIf9ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf10Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf10Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf10Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10_APD` reader - Array Power"]
pub type SramIf10ApdR = crate::BitReader<SramIf10Apd>;
impl SramIf10ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf10Apd {
        match self.bits {
            false => SramIf10Apd::Enabled,
            true => SramIf10Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf10Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf10Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF10_APD` writer - Array Power"]
pub type SramIf10ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf10Apd>;
impl<'a, REG> SramIf10ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf11Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf11Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf11Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11_APD` reader - Array Power"]
pub type SramIf11ApdR = crate::BitReader<SramIf11Apd>;
impl SramIf11ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf11Apd {
        match self.bits {
            false => SramIf11Apd::Enabled,
            true => SramIf11Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf11Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf11Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF11_APD` writer - Array Power"]
pub type SramIf11ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf11Apd>;
impl<'a, REG> SramIf11ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf12Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf12Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf12Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12_APD` reader - Array Power"]
pub type SramIf12ApdR = crate::BitReader<SramIf12Apd>;
impl SramIf12ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf12Apd {
        match self.bits {
            false => SramIf12Apd::Enabled,
            true => SramIf12Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf12Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf12Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF12_APD` writer - Array Power"]
pub type SramIf12ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf12Apd>;
impl<'a, REG> SramIf12ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf13Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf13Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf13Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13_APD` reader - Array Power"]
pub type SramIf13ApdR = crate::BitReader<SramIf13Apd>;
impl SramIf13ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf13Apd {
        match self.bits {
            false => SramIf13Apd::Enabled,
            true => SramIf13Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf13Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf13Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF13_APD` writer - Array Power"]
pub type SramIf13ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf13Apd>;
impl<'a, REG> SramIf13ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf14Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf14Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf14Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14_APD` reader - Array Power"]
pub type SramIf14ApdR = crate::BitReader<SramIf14Apd>;
impl SramIf14ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf14Apd {
        match self.bits {
            false => SramIf14Apd::Enabled,
            true => SramIf14Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf14Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf14Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF14_APD` writer - Array Power"]
pub type SramIf14ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf14Apd>;
impl<'a, REG> SramIf14ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf15Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf15Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf15Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15_APD` reader - Array Power"]
pub type SramIf15ApdR = crate::BitReader<SramIf15Apd>;
impl SramIf15ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf15Apd {
        match self.bits {
            false => SramIf15Apd::Enabled,
            true => SramIf15Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf15Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf15Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF15_APD` writer - Array Power"]
pub type SramIf15ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf15Apd>;
impl<'a, REG> SramIf15ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf16Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf16Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf16Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16_APD` reader - Array Power"]
pub type SramIf16ApdR = crate::BitReader<SramIf16Apd>;
impl SramIf16ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf16Apd {
        match self.bits {
            false => SramIf16Apd::Enabled,
            true => SramIf16Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf16Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf16Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF16_APD` writer - Array Power"]
pub type SramIf16ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf16Apd>;
impl<'a, REG> SramIf16ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf17Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf17Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf17Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17_APD` reader - Array Power"]
pub type SramIf17ApdR = crate::BitReader<SramIf17Apd>;
impl SramIf17ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf17Apd {
        match self.bits {
            false => SramIf17Apd::Enabled,
            true => SramIf17Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf17Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf17Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF17_APD` writer - Array Power"]
pub type SramIf17ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf17Apd>;
impl<'a, REG> SramIf17ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf18Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf18Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf18Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF18_APD` reader - Array Power"]
pub type SramIf18ApdR = crate::BitReader<SramIf18Apd>;
impl SramIf18ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf18Apd {
        match self.bits {
            false => SramIf18Apd::Enabled,
            true => SramIf18Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf18Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf18Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF18_APD` writer - Array Power"]
pub type SramIf18ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf18Apd>;
impl<'a, REG> SramIf18ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf18Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf19Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf19Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf19Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19_APD` reader - Array Power"]
pub type SramIf19ApdR = crate::BitReader<SramIf19Apd>;
impl SramIf19ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf19Apd {
        match self.bits {
            false => SramIf19Apd::Enabled,
            true => SramIf19Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf19Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf19Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF19_APD` writer - Array Power"]
pub type SramIf19ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf19Apd>;
impl<'a, REG> SramIf19ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf20Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf20Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf20Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20_APD` reader - Array Power"]
pub type SramIf20ApdR = crate::BitReader<SramIf20Apd>;
impl SramIf20ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf20Apd {
        match self.bits {
            false => SramIf20Apd::Enabled,
            true => SramIf20Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf20Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf20Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF20_APD` writer - Array Power"]
pub type SramIf20ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf20Apd>;
impl<'a, REG> SramIf20ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf21Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf21Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf21Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21_APD` reader - Array Power"]
pub type SramIf21ApdR = crate::BitReader<SramIf21Apd>;
impl SramIf21ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf21Apd {
        match self.bits {
            false => SramIf21Apd::Enabled,
            true => SramIf21Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf21Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf21Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF21_APD` writer - Array Power"]
pub type SramIf21ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf21Apd>;
impl<'a, REG> SramIf21ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf22Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf22Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf22Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22_APD` reader - Array Power"]
pub type SramIf22ApdR = crate::BitReader<SramIf22Apd>;
impl SramIf22ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf22Apd {
        match self.bits {
            false => SramIf22Apd::Enabled,
            true => SramIf22Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf22Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf22Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF22_APD` writer - Array Power"]
pub type SramIf22ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf22Apd>;
impl<'a, REG> SramIf22ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf23Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf23Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf23Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23_APD` reader - Array Power"]
pub type SramIf23ApdR = crate::BitReader<SramIf23Apd>;
impl SramIf23ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf23Apd {
        match self.bits {
            false => SramIf23Apd::Enabled,
            true => SramIf23Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf23Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf23Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF23_APD` writer - Array Power"]
pub type SramIf23ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf23Apd>;
impl<'a, REG> SramIf23ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf24Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf24Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf24Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24_APD` reader - Array Power"]
pub type SramIf24ApdR = crate::BitReader<SramIf24Apd>;
impl SramIf24ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf24Apd {
        match self.bits {
            false => SramIf24Apd::Enabled,
            true => SramIf24Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf24Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf24Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF24_APD` writer - Array Power"]
pub type SramIf24ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf24Apd>;
impl<'a, REG> SramIf24ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf25Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf25Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf25Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25_APD` reader - Array Power"]
pub type SramIf25ApdR = crate::BitReader<SramIf25Apd>;
impl SramIf25ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf25Apd {
        match self.bits {
            false => SramIf25Apd::Enabled,
            true => SramIf25Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf25Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf25Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF25_APD` writer - Array Power"]
pub type SramIf25ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf25Apd>;
impl<'a, REG> SramIf25ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf26Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf26Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf26Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26_APD` reader - Array Power"]
pub type SramIf26ApdR = crate::BitReader<SramIf26Apd>;
impl SramIf26ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf26Apd {
        match self.bits {
            false => SramIf26Apd::Enabled,
            true => SramIf26Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf26Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf26Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF26_APD` writer - Array Power"]
pub type SramIf26ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf26Apd>;
impl<'a, REG> SramIf26ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf27Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf27Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf27Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27_APD` reader - Array Power"]
pub type SramIf27ApdR = crate::BitReader<SramIf27Apd>;
impl SramIf27ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf27Apd {
        match self.bits {
            false => SramIf27Apd::Enabled,
            true => SramIf27Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf27Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf27Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF27_APD` writer - Array Power"]
pub type SramIf27ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf27Apd>;
impl<'a, REG> SramIf27ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf28Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf28Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf28Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28_APD` reader - Array Power"]
pub type SramIf28ApdR = crate::BitReader<SramIf28Apd>;
impl SramIf28ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf28Apd {
        match self.bits {
            false => SramIf28Apd::Enabled,
            true => SramIf28Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf28Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf28Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF28_APD` writer - Array Power"]
pub type SramIf28ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf28Apd>;
impl<'a, REG> SramIf28ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28Apd::PowerDown)
    }
}
#[doc = "Array Power\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf29Apd {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SramIf29Apd> for bool {
    #[inline(always)]
    fn from(variant: SramIf29Apd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29_APD` reader - Array Power"]
pub type SramIf29ApdR = crate::BitReader<SramIf29Apd>;
impl SramIf29ApdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf29Apd {
        match self.bits {
            false => SramIf29Apd::Enabled,
            true => SramIf29Apd::PowerDown,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf29Apd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SramIf29Apd::PowerDown
    }
}
#[doc = "Field `SRAM_IF29_APD` writer - Array Power"]
pub type SramIf29ApdW<'a, REG> = crate::BitWriter<'a, REG, SramIf29Apd>;
impl<'a, REG> SramIf29ApdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Apd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29Apd::PowerDown)
    }
}
impl R {
    #[doc = "Bit 0 - Array Power"]
    #[inline(always)]
    pub fn sram_if0_apd(&self) -> SramIf0ApdR {
        SramIf0ApdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Array Power"]
    #[inline(always)]
    pub fn sram_if1_apd(&self) -> SramIf1ApdR {
        SramIf1ApdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Array Power"]
    #[inline(always)]
    pub fn sram_if2_apd(&self) -> SramIf2ApdR {
        SramIf2ApdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Array Power"]
    #[inline(always)]
    pub fn sram_if3_apd(&self) -> SramIf3ApdR {
        SramIf3ApdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Array Power"]
    #[inline(always)]
    pub fn sram_if4_apd(&self) -> SramIf4ApdR {
        SramIf4ApdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Array Power"]
    #[inline(always)]
    pub fn sram_if5_apd(&self) -> SramIf5ApdR {
        SramIf5ApdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Array Power"]
    #[inline(always)]
    pub fn sram_if6_apd(&self) -> SramIf6ApdR {
        SramIf6ApdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Array Power"]
    #[inline(always)]
    pub fn sram_if7_apd(&self) -> SramIf7ApdR {
        SramIf7ApdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Array Power"]
    #[inline(always)]
    pub fn sram_if8_apd(&self) -> SramIf8ApdR {
        SramIf8ApdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Array Power"]
    #[inline(always)]
    pub fn sram_if9_apd(&self) -> SramIf9ApdR {
        SramIf9ApdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Array Power"]
    #[inline(always)]
    pub fn sram_if10_apd(&self) -> SramIf10ApdR {
        SramIf10ApdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Array Power"]
    #[inline(always)]
    pub fn sram_if11_apd(&self) -> SramIf11ApdR {
        SramIf11ApdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Array Power"]
    #[inline(always)]
    pub fn sram_if12_apd(&self) -> SramIf12ApdR {
        SramIf12ApdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Array Power"]
    #[inline(always)]
    pub fn sram_if13_apd(&self) -> SramIf13ApdR {
        SramIf13ApdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Array Power"]
    #[inline(always)]
    pub fn sram_if14_apd(&self) -> SramIf14ApdR {
        SramIf14ApdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Array Power"]
    #[inline(always)]
    pub fn sram_if15_apd(&self) -> SramIf15ApdR {
        SramIf15ApdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Array Power"]
    #[inline(always)]
    pub fn sram_if16_apd(&self) -> SramIf16ApdR {
        SramIf16ApdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Array Power"]
    #[inline(always)]
    pub fn sram_if17_apd(&self) -> SramIf17ApdR {
        SramIf17ApdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Array Power"]
    #[inline(always)]
    pub fn sram_if18_apd(&self) -> SramIf18ApdR {
        SramIf18ApdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Array Power"]
    #[inline(always)]
    pub fn sram_if19_apd(&self) -> SramIf19ApdR {
        SramIf19ApdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Array Power"]
    #[inline(always)]
    pub fn sram_if20_apd(&self) -> SramIf20ApdR {
        SramIf20ApdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Array Power"]
    #[inline(always)]
    pub fn sram_if21_apd(&self) -> SramIf21ApdR {
        SramIf21ApdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Array Power"]
    #[inline(always)]
    pub fn sram_if22_apd(&self) -> SramIf22ApdR {
        SramIf22ApdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Array Power"]
    #[inline(always)]
    pub fn sram_if23_apd(&self) -> SramIf23ApdR {
        SramIf23ApdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Array Power"]
    #[inline(always)]
    pub fn sram_if24_apd(&self) -> SramIf24ApdR {
        SramIf24ApdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Array Power"]
    #[inline(always)]
    pub fn sram_if25_apd(&self) -> SramIf25ApdR {
        SramIf25ApdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Array Power"]
    #[inline(always)]
    pub fn sram_if26_apd(&self) -> SramIf26ApdR {
        SramIf26ApdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Array Power"]
    #[inline(always)]
    pub fn sram_if27_apd(&self) -> SramIf27ApdR {
        SramIf27ApdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Array Power"]
    #[inline(always)]
    pub fn sram_if28_apd(&self) -> SramIf28ApdR {
        SramIf28ApdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Array Power"]
    #[inline(always)]
    pub fn sram_if29_apd(&self) -> SramIf29ApdR {
        SramIf29ApdR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSLEEPCFG2")
            .field("sram_if0_apd", &self.sram_if0_apd())
            .field("sram_if1_apd", &self.sram_if1_apd())
            .field("sram_if2_apd", &self.sram_if2_apd())
            .field("sram_if3_apd", &self.sram_if3_apd())
            .field("sram_if4_apd", &self.sram_if4_apd())
            .field("sram_if5_apd", &self.sram_if5_apd())
            .field("sram_if6_apd", &self.sram_if6_apd())
            .field("sram_if7_apd", &self.sram_if7_apd())
            .field("sram_if8_apd", &self.sram_if8_apd())
            .field("sram_if9_apd", &self.sram_if9_apd())
            .field("sram_if10_apd", &self.sram_if10_apd())
            .field("sram_if11_apd", &self.sram_if11_apd())
            .field("sram_if12_apd", &self.sram_if12_apd())
            .field("sram_if13_apd", &self.sram_if13_apd())
            .field("sram_if14_apd", &self.sram_if14_apd())
            .field("sram_if15_apd", &self.sram_if15_apd())
            .field("sram_if16_apd", &self.sram_if16_apd())
            .field("sram_if17_apd", &self.sram_if17_apd())
            .field("sram_if18_apd", &self.sram_if18_apd())
            .field("sram_if19_apd", &self.sram_if19_apd())
            .field("sram_if20_apd", &self.sram_if20_apd())
            .field("sram_if21_apd", &self.sram_if21_apd())
            .field("sram_if22_apd", &self.sram_if22_apd())
            .field("sram_if23_apd", &self.sram_if23_apd())
            .field("sram_if24_apd", &self.sram_if24_apd())
            .field("sram_if25_apd", &self.sram_if25_apd())
            .field("sram_if26_apd", &self.sram_if26_apd())
            .field("sram_if27_apd", &self.sram_if27_apd())
            .field("sram_if28_apd", &self.sram_if28_apd())
            .field("sram_if29_apd", &self.sram_if29_apd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Array Power"]
    #[inline(always)]
    pub fn sram_if0_apd(&mut self) -> SramIf0ApdW<Pdsleepcfg2Spec> {
        SramIf0ApdW::new(self, 0)
    }
    #[doc = "Bit 1 - Array Power"]
    #[inline(always)]
    pub fn sram_if1_apd(&mut self) -> SramIf1ApdW<Pdsleepcfg2Spec> {
        SramIf1ApdW::new(self, 1)
    }
    #[doc = "Bit 2 - Array Power"]
    #[inline(always)]
    pub fn sram_if2_apd(&mut self) -> SramIf2ApdW<Pdsleepcfg2Spec> {
        SramIf2ApdW::new(self, 2)
    }
    #[doc = "Bit 3 - Array Power"]
    #[inline(always)]
    pub fn sram_if3_apd(&mut self) -> SramIf3ApdW<Pdsleepcfg2Spec> {
        SramIf3ApdW::new(self, 3)
    }
    #[doc = "Bit 4 - Array Power"]
    #[inline(always)]
    pub fn sram_if4_apd(&mut self) -> SramIf4ApdW<Pdsleepcfg2Spec> {
        SramIf4ApdW::new(self, 4)
    }
    #[doc = "Bit 5 - Array Power"]
    #[inline(always)]
    pub fn sram_if5_apd(&mut self) -> SramIf5ApdW<Pdsleepcfg2Spec> {
        SramIf5ApdW::new(self, 5)
    }
    #[doc = "Bit 6 - Array Power"]
    #[inline(always)]
    pub fn sram_if6_apd(&mut self) -> SramIf6ApdW<Pdsleepcfg2Spec> {
        SramIf6ApdW::new(self, 6)
    }
    #[doc = "Bit 7 - Array Power"]
    #[inline(always)]
    pub fn sram_if7_apd(&mut self) -> SramIf7ApdW<Pdsleepcfg2Spec> {
        SramIf7ApdW::new(self, 7)
    }
    #[doc = "Bit 8 - Array Power"]
    #[inline(always)]
    pub fn sram_if8_apd(&mut self) -> SramIf8ApdW<Pdsleepcfg2Spec> {
        SramIf8ApdW::new(self, 8)
    }
    #[doc = "Bit 9 - Array Power"]
    #[inline(always)]
    pub fn sram_if9_apd(&mut self) -> SramIf9ApdW<Pdsleepcfg2Spec> {
        SramIf9ApdW::new(self, 9)
    }
    #[doc = "Bit 10 - Array Power"]
    #[inline(always)]
    pub fn sram_if10_apd(&mut self) -> SramIf10ApdW<Pdsleepcfg2Spec> {
        SramIf10ApdW::new(self, 10)
    }
    #[doc = "Bit 11 - Array Power"]
    #[inline(always)]
    pub fn sram_if11_apd(&mut self) -> SramIf11ApdW<Pdsleepcfg2Spec> {
        SramIf11ApdW::new(self, 11)
    }
    #[doc = "Bit 12 - Array Power"]
    #[inline(always)]
    pub fn sram_if12_apd(&mut self) -> SramIf12ApdW<Pdsleepcfg2Spec> {
        SramIf12ApdW::new(self, 12)
    }
    #[doc = "Bit 13 - Array Power"]
    #[inline(always)]
    pub fn sram_if13_apd(&mut self) -> SramIf13ApdW<Pdsleepcfg2Spec> {
        SramIf13ApdW::new(self, 13)
    }
    #[doc = "Bit 14 - Array Power"]
    #[inline(always)]
    pub fn sram_if14_apd(&mut self) -> SramIf14ApdW<Pdsleepcfg2Spec> {
        SramIf14ApdW::new(self, 14)
    }
    #[doc = "Bit 15 - Array Power"]
    #[inline(always)]
    pub fn sram_if15_apd(&mut self) -> SramIf15ApdW<Pdsleepcfg2Spec> {
        SramIf15ApdW::new(self, 15)
    }
    #[doc = "Bit 16 - Array Power"]
    #[inline(always)]
    pub fn sram_if16_apd(&mut self) -> SramIf16ApdW<Pdsleepcfg2Spec> {
        SramIf16ApdW::new(self, 16)
    }
    #[doc = "Bit 17 - Array Power"]
    #[inline(always)]
    pub fn sram_if17_apd(&mut self) -> SramIf17ApdW<Pdsleepcfg2Spec> {
        SramIf17ApdW::new(self, 17)
    }
    #[doc = "Bit 18 - Array Power"]
    #[inline(always)]
    pub fn sram_if18_apd(&mut self) -> SramIf18ApdW<Pdsleepcfg2Spec> {
        SramIf18ApdW::new(self, 18)
    }
    #[doc = "Bit 19 - Array Power"]
    #[inline(always)]
    pub fn sram_if19_apd(&mut self) -> SramIf19ApdW<Pdsleepcfg2Spec> {
        SramIf19ApdW::new(self, 19)
    }
    #[doc = "Bit 20 - Array Power"]
    #[inline(always)]
    pub fn sram_if20_apd(&mut self) -> SramIf20ApdW<Pdsleepcfg2Spec> {
        SramIf20ApdW::new(self, 20)
    }
    #[doc = "Bit 21 - Array Power"]
    #[inline(always)]
    pub fn sram_if21_apd(&mut self) -> SramIf21ApdW<Pdsleepcfg2Spec> {
        SramIf21ApdW::new(self, 21)
    }
    #[doc = "Bit 22 - Array Power"]
    #[inline(always)]
    pub fn sram_if22_apd(&mut self) -> SramIf22ApdW<Pdsleepcfg2Spec> {
        SramIf22ApdW::new(self, 22)
    }
    #[doc = "Bit 23 - Array Power"]
    #[inline(always)]
    pub fn sram_if23_apd(&mut self) -> SramIf23ApdW<Pdsleepcfg2Spec> {
        SramIf23ApdW::new(self, 23)
    }
    #[doc = "Bit 24 - Array Power"]
    #[inline(always)]
    pub fn sram_if24_apd(&mut self) -> SramIf24ApdW<Pdsleepcfg2Spec> {
        SramIf24ApdW::new(self, 24)
    }
    #[doc = "Bit 25 - Array Power"]
    #[inline(always)]
    pub fn sram_if25_apd(&mut self) -> SramIf25ApdW<Pdsleepcfg2Spec> {
        SramIf25ApdW::new(self, 25)
    }
    #[doc = "Bit 26 - Array Power"]
    #[inline(always)]
    pub fn sram_if26_apd(&mut self) -> SramIf26ApdW<Pdsleepcfg2Spec> {
        SramIf26ApdW::new(self, 26)
    }
    #[doc = "Bit 27 - Array Power"]
    #[inline(always)]
    pub fn sram_if27_apd(&mut self) -> SramIf27ApdW<Pdsleepcfg2Spec> {
        SramIf27ApdW::new(self, 27)
    }
    #[doc = "Bit 28 - Array Power"]
    #[inline(always)]
    pub fn sram_if28_apd(&mut self) -> SramIf28ApdW<Pdsleepcfg2Spec> {
        SramIf28ApdW::new(self, 28)
    }
    #[doc = "Bit 29 - Array Power"]
    #[inline(always)]
    pub fn sram_if29_apd(&mut self) -> SramIf29ApdW<Pdsleepcfg2Spec> {
        SramIf29ApdW::new(self, 29)
    }
}
#[doc = "Sleep configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsleepcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsleepcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdsleepcfg2Spec;
impl crate::RegisterSpec for Pdsleepcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsleepcfg2::R`](R) reader structure"]
impl crate::Readable for Pdsleepcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pdsleepcfg2::W`](W) writer structure"]
impl crate::Writable for Pdsleepcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG2 to value 0xffff_fffe"]
impl crate::Resettable for Pdsleepcfg2Spec {
    const RESET_VALUE: u32 = 0xffff_fffe;
}
