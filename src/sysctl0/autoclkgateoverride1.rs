#[doc = "Register `AUTOCLKGATEOVERRIDE1` reader"]
pub type R = crate::R<Autoclkgateoverride1Spec>;
#[doc = "Register `AUTOCLKGATEOVERRIDE1` writer"]
pub type W = crate::W<Autoclkgateoverride1Spec>;
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf0 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf0> for bool {
    #[inline(always)]
    fn from(variant: SramIf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0` reader - auto clock gating enable"]
pub type SramIf0R = crate::BitReader<SramIf0>;
impl SramIf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf0 {
        match self.bits {
            false => SramIf0::Enabled,
            true => SramIf0::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf0::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf0::Disabled
    }
}
#[doc = "Field `SRAM_IF0` writer - auto clock gating enable"]
pub type SramIf0W<'a, REG> = crate::BitWriter<'a, REG, SramIf0>;
impl<'a, REG> SramIf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf0::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf1 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf1> for bool {
    #[inline(always)]
    fn from(variant: SramIf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1` reader - auto clock gating enable"]
pub type SramIf1R = crate::BitReader<SramIf1>;
impl SramIf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf1 {
        match self.bits {
            false => SramIf1::Enabled,
            true => SramIf1::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf1::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf1::Disabled
    }
}
#[doc = "Field `SRAM_IF1` writer - auto clock gating enable"]
pub type SramIf1W<'a, REG> = crate::BitWriter<'a, REG, SramIf1>;
impl<'a, REG> SramIf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf1::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf2 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf2> for bool {
    #[inline(always)]
    fn from(variant: SramIf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2` reader - auto clock gating enable"]
pub type SramIf2R = crate::BitReader<SramIf2>;
impl SramIf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf2 {
        match self.bits {
            false => SramIf2::Enabled,
            true => SramIf2::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf2::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf2::Disabled
    }
}
#[doc = "Field `SRAM_IF2` writer - auto clock gating enable"]
pub type SramIf2W<'a, REG> = crate::BitWriter<'a, REG, SramIf2>;
impl<'a, REG> SramIf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf2::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf3 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf3> for bool {
    #[inline(always)]
    fn from(variant: SramIf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3` reader - auto clock gating enable"]
pub type SramIf3R = crate::BitReader<SramIf3>;
impl SramIf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf3 {
        match self.bits {
            false => SramIf3::Enabled,
            true => SramIf3::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf3::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf3::Disabled
    }
}
#[doc = "Field `SRAM_IF3` writer - auto clock gating enable"]
pub type SramIf3W<'a, REG> = crate::BitWriter<'a, REG, SramIf3>;
impl<'a, REG> SramIf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf3::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf4 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf4> for bool {
    #[inline(always)]
    fn from(variant: SramIf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4` reader - auto clock gating enable"]
pub type SramIf4R = crate::BitReader<SramIf4>;
impl SramIf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf4 {
        match self.bits {
            false => SramIf4::Enabled,
            true => SramIf4::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf4::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf4::Disabled
    }
}
#[doc = "Field `SRAM_IF4` writer - auto clock gating enable"]
pub type SramIf4W<'a, REG> = crate::BitWriter<'a, REG, SramIf4>;
impl<'a, REG> SramIf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf4::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf5 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf5> for bool {
    #[inline(always)]
    fn from(variant: SramIf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5` reader - auto clock gating enable"]
pub type SramIf5R = crate::BitReader<SramIf5>;
impl SramIf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf5 {
        match self.bits {
            false => SramIf5::Enabled,
            true => SramIf5::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf5::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf5::Disabled
    }
}
#[doc = "Field `SRAM_IF5` writer - auto clock gating enable"]
pub type SramIf5W<'a, REG> = crate::BitWriter<'a, REG, SramIf5>;
impl<'a, REG> SramIf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf5::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf6 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf6> for bool {
    #[inline(always)]
    fn from(variant: SramIf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6` reader - auto clock gating enable"]
pub type SramIf6R = crate::BitReader<SramIf6>;
impl SramIf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf6 {
        match self.bits {
            false => SramIf6::Enabled,
            true => SramIf6::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf6::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf6::Disabled
    }
}
#[doc = "Field `SRAM_IF6` writer - auto clock gating enable"]
pub type SramIf6W<'a, REG> = crate::BitWriter<'a, REG, SramIf6>;
impl<'a, REG> SramIf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf6::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf7 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf7> for bool {
    #[inline(always)]
    fn from(variant: SramIf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7` reader - auto clock gating enable"]
pub type SramIf7R = crate::BitReader<SramIf7>;
impl SramIf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf7 {
        match self.bits {
            false => SramIf7::Enabled,
            true => SramIf7::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf7::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf7::Disabled
    }
}
#[doc = "Field `SRAM_IF7` writer - auto clock gating enable"]
pub type SramIf7W<'a, REG> = crate::BitWriter<'a, REG, SramIf7>;
impl<'a, REG> SramIf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf7::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf8 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf8> for bool {
    #[inline(always)]
    fn from(variant: SramIf8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8` reader - auto clock gating enable"]
pub type SramIf8R = crate::BitReader<SramIf8>;
impl SramIf8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf8 {
        match self.bits {
            false => SramIf8::Enabled,
            true => SramIf8::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf8::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf8::Disabled
    }
}
#[doc = "Field `SRAM_IF8` writer - auto clock gating enable"]
pub type SramIf8W<'a, REG> = crate::BitWriter<'a, REG, SramIf8>;
impl<'a, REG> SramIf8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf8::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf9 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf9> for bool {
    #[inline(always)]
    fn from(variant: SramIf9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9` reader - auto clock gating enable"]
pub type SramIf9R = crate::BitReader<SramIf9>;
impl SramIf9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf9 {
        match self.bits {
            false => SramIf9::Enabled,
            true => SramIf9::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf9::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf9::Disabled
    }
}
#[doc = "Field `SRAM_IF9` writer - auto clock gating enable"]
pub type SramIf9W<'a, REG> = crate::BitWriter<'a, REG, SramIf9>;
impl<'a, REG> SramIf9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf9::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf10 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf10> for bool {
    #[inline(always)]
    fn from(variant: SramIf10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10` reader - auto clock gating enable"]
pub type SramIf10R = crate::BitReader<SramIf10>;
impl SramIf10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf10 {
        match self.bits {
            false => SramIf10::Enabled,
            true => SramIf10::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf10::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf10::Disabled
    }
}
#[doc = "Field `SRAM_IF10` writer - auto clock gating enable"]
pub type SramIf10W<'a, REG> = crate::BitWriter<'a, REG, SramIf10>;
impl<'a, REG> SramIf10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf10::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf11 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf11> for bool {
    #[inline(always)]
    fn from(variant: SramIf11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11` reader - auto clock gating enable"]
pub type SramIf11R = crate::BitReader<SramIf11>;
impl SramIf11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf11 {
        match self.bits {
            false => SramIf11::Enabled,
            true => SramIf11::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf11::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf11::Disabled
    }
}
#[doc = "Field `SRAM_IF11` writer - auto clock gating enable"]
pub type SramIf11W<'a, REG> = crate::BitWriter<'a, REG, SramIf11>;
impl<'a, REG> SramIf11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf11::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf12 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf12> for bool {
    #[inline(always)]
    fn from(variant: SramIf12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12` reader - auto clock gating enable"]
pub type SramIf12R = crate::BitReader<SramIf12>;
impl SramIf12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf12 {
        match self.bits {
            false => SramIf12::Enabled,
            true => SramIf12::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf12::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf12::Disabled
    }
}
#[doc = "Field `SRAM_IF12` writer - auto clock gating enable"]
pub type SramIf12W<'a, REG> = crate::BitWriter<'a, REG, SramIf12>;
impl<'a, REG> SramIf12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf12::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf13 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf13> for bool {
    #[inline(always)]
    fn from(variant: SramIf13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13` reader - auto clock gating enable"]
pub type SramIf13R = crate::BitReader<SramIf13>;
impl SramIf13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf13 {
        match self.bits {
            false => SramIf13::Enabled,
            true => SramIf13::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf13::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf13::Disabled
    }
}
#[doc = "Field `SRAM_IF13` writer - auto clock gating enable"]
pub type SramIf13W<'a, REG> = crate::BitWriter<'a, REG, SramIf13>;
impl<'a, REG> SramIf13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf13::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf14 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf14> for bool {
    #[inline(always)]
    fn from(variant: SramIf14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14` reader - auto clock gating enable"]
pub type SramIf14R = crate::BitReader<SramIf14>;
impl SramIf14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf14 {
        match self.bits {
            false => SramIf14::Enabled,
            true => SramIf14::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf14::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf14::Disabled
    }
}
#[doc = "Field `SRAM_IF14` writer - auto clock gating enable"]
pub type SramIf14W<'a, REG> = crate::BitWriter<'a, REG, SramIf14>;
impl<'a, REG> SramIf14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf14::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf15 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf15> for bool {
    #[inline(always)]
    fn from(variant: SramIf15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15` reader - auto clock gating enable"]
pub type SramIf15R = crate::BitReader<SramIf15>;
impl SramIf15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf15 {
        match self.bits {
            false => SramIf15::Enabled,
            true => SramIf15::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf15::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf15::Disabled
    }
}
#[doc = "Field `SRAM_IF15` writer - auto clock gating enable"]
pub type SramIf15W<'a, REG> = crate::BitWriter<'a, REG, SramIf15>;
impl<'a, REG> SramIf15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf15::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf16 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf16> for bool {
    #[inline(always)]
    fn from(variant: SramIf16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16` reader - auto clock gating enable"]
pub type SramIf16R = crate::BitReader<SramIf16>;
impl SramIf16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf16 {
        match self.bits {
            false => SramIf16::Enabled,
            true => SramIf16::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf16::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf16::Disabled
    }
}
#[doc = "Field `SRAM_IF16` writer - auto clock gating enable"]
pub type SramIf16W<'a, REG> = crate::BitWriter<'a, REG, SramIf16>;
impl<'a, REG> SramIf16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf16::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf17 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf17> for bool {
    #[inline(always)]
    fn from(variant: SramIf17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17` reader - auto clock gating enable"]
pub type SramIf17R = crate::BitReader<SramIf17>;
impl SramIf17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf17 {
        match self.bits {
            false => SramIf17::Enabled,
            true => SramIf17::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf17::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf17::Disabled
    }
}
#[doc = "Field `SRAM_IF17` writer - auto clock gating enable"]
pub type SramIf17W<'a, REG> = crate::BitWriter<'a, REG, SramIf17>;
impl<'a, REG> SramIf17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf17::Disabled)
    }
}
#[doc = "Field `SRAM_IF18` reader - auto clock gating enable"]
pub type SramIf18R = crate::BitReader;
#[doc = "Field `SRAM_IF18` writer - auto clock gating enable"]
pub type SramIf18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf19 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf19> for bool {
    #[inline(always)]
    fn from(variant: SramIf19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19` reader - auto clock gating enable"]
pub type SramIf19R = crate::BitReader<SramIf19>;
impl SramIf19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf19 {
        match self.bits {
            false => SramIf19::Enabled,
            true => SramIf19::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf19::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf19::Disabled
    }
}
#[doc = "Field `SRAM_IF19` writer - auto clock gating enable"]
pub type SramIf19W<'a, REG> = crate::BitWriter<'a, REG, SramIf19>;
impl<'a, REG> SramIf19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf19::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf20 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf20> for bool {
    #[inline(always)]
    fn from(variant: SramIf20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20` reader - auto clock gating enable"]
pub type SramIf20R = crate::BitReader<SramIf20>;
impl SramIf20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf20 {
        match self.bits {
            false => SramIf20::Enabled,
            true => SramIf20::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf20::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf20::Disabled
    }
}
#[doc = "Field `SRAM_IF20` writer - auto clock gating enable"]
pub type SramIf20W<'a, REG> = crate::BitWriter<'a, REG, SramIf20>;
impl<'a, REG> SramIf20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf20::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf21 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf21> for bool {
    #[inline(always)]
    fn from(variant: SramIf21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21` reader - auto clock gating enable"]
pub type SramIf21R = crate::BitReader<SramIf21>;
impl SramIf21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf21 {
        match self.bits {
            false => SramIf21::Enabled,
            true => SramIf21::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf21::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf21::Disabled
    }
}
#[doc = "Field `SRAM_IF21` writer - auto clock gating enable"]
pub type SramIf21W<'a, REG> = crate::BitWriter<'a, REG, SramIf21>;
impl<'a, REG> SramIf21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf21::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf22 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf22> for bool {
    #[inline(always)]
    fn from(variant: SramIf22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22` reader - auto clock gating enable"]
pub type SramIf22R = crate::BitReader<SramIf22>;
impl SramIf22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf22 {
        match self.bits {
            false => SramIf22::Enabled,
            true => SramIf22::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf22::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf22::Disabled
    }
}
#[doc = "Field `SRAM_IF22` writer - auto clock gating enable"]
pub type SramIf22W<'a, REG> = crate::BitWriter<'a, REG, SramIf22>;
impl<'a, REG> SramIf22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf22::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf23 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf23> for bool {
    #[inline(always)]
    fn from(variant: SramIf23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23` reader - auto clock gating enable"]
pub type SramIf23R = crate::BitReader<SramIf23>;
impl SramIf23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf23 {
        match self.bits {
            false => SramIf23::Enabled,
            true => SramIf23::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf23::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf23::Disabled
    }
}
#[doc = "Field `SRAM_IF23` writer - auto clock gating enable"]
pub type SramIf23W<'a, REG> = crate::BitWriter<'a, REG, SramIf23>;
impl<'a, REG> SramIf23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf23::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf24 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf24> for bool {
    #[inline(always)]
    fn from(variant: SramIf24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24` reader - auto clock gating enable"]
pub type SramIf24R = crate::BitReader<SramIf24>;
impl SramIf24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf24 {
        match self.bits {
            false => SramIf24::Enabled,
            true => SramIf24::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf24::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf24::Disabled
    }
}
#[doc = "Field `SRAM_IF24` writer - auto clock gating enable"]
pub type SramIf24W<'a, REG> = crate::BitWriter<'a, REG, SramIf24>;
impl<'a, REG> SramIf24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf24::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf25 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf25> for bool {
    #[inline(always)]
    fn from(variant: SramIf25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25` reader - auto clock gating enable"]
pub type SramIf25R = crate::BitReader<SramIf25>;
impl SramIf25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf25 {
        match self.bits {
            false => SramIf25::Enabled,
            true => SramIf25::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf25::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf25::Disabled
    }
}
#[doc = "Field `SRAM_IF25` writer - auto clock gating enable"]
pub type SramIf25W<'a, REG> = crate::BitWriter<'a, REG, SramIf25>;
impl<'a, REG> SramIf25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf25::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf26 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf26> for bool {
    #[inline(always)]
    fn from(variant: SramIf26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26` reader - auto clock gating enable"]
pub type SramIf26R = crate::BitReader<SramIf26>;
impl SramIf26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf26 {
        match self.bits {
            false => SramIf26::Enabled,
            true => SramIf26::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf26::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf26::Disabled
    }
}
#[doc = "Field `SRAM_IF26` writer - auto clock gating enable"]
pub type SramIf26W<'a, REG> = crate::BitWriter<'a, REG, SramIf26>;
impl<'a, REG> SramIf26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf26::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf27 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf27> for bool {
    #[inline(always)]
    fn from(variant: SramIf27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27` reader - auto clock gating enable"]
pub type SramIf27R = crate::BitReader<SramIf27>;
impl SramIf27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf27 {
        match self.bits {
            false => SramIf27::Enabled,
            true => SramIf27::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf27::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf27::Disabled
    }
}
#[doc = "Field `SRAM_IF27` writer - auto clock gating enable"]
pub type SramIf27W<'a, REG> = crate::BitWriter<'a, REG, SramIf27>;
impl<'a, REG> SramIf27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf27::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf28 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf28> for bool {
    #[inline(always)]
    fn from(variant: SramIf28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28` reader - auto clock gating enable"]
pub type SramIf28R = crate::BitReader<SramIf28>;
impl SramIf28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf28 {
        match self.bits {
            false => SramIf28::Enabled,
            true => SramIf28::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf28::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf28::Disabled
    }
}
#[doc = "Field `SRAM_IF28` writer - auto clock gating enable"]
pub type SramIf28W<'a, REG> = crate::BitWriter<'a, REG, SramIf28>;
impl<'a, REG> SramIf28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf28::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramIf29 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<SramIf29> for bool {
    #[inline(always)]
    fn from(variant: SramIf29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29` reader - auto clock gating enable"]
pub type SramIf29R = crate::BitReader<SramIf29>;
impl SramIf29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramIf29 {
        match self.bits {
            false => SramIf29::Enabled,
            true => SramIf29::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SramIf29::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SramIf29::Disabled
    }
}
#[doc = "Field `SRAM_IF29` writer - auto clock gating enable"]
pub type SramIf29W<'a, REG> = crate::BitWriter<'a, REG, SramIf29>;
impl<'a, REG> SramIf29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SramIf29::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if0(&self) -> SramIf0R {
        SramIf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if1(&self) -> SramIf1R {
        SramIf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if2(&self) -> SramIf2R {
        SramIf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if3(&self) -> SramIf3R {
        SramIf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if4(&self) -> SramIf4R {
        SramIf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if5(&self) -> SramIf5R {
        SramIf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if6(&self) -> SramIf6R {
        SramIf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if7(&self) -> SramIf7R {
        SramIf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if8(&self) -> SramIf8R {
        SramIf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if9(&self) -> SramIf9R {
        SramIf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if10(&self) -> SramIf10R {
        SramIf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if11(&self) -> SramIf11R {
        SramIf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if12(&self) -> SramIf12R {
        SramIf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if13(&self) -> SramIf13R {
        SramIf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if14(&self) -> SramIf14R {
        SramIf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if15(&self) -> SramIf15R {
        SramIf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if16(&self) -> SramIf16R {
        SramIf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if17(&self) -> SramIf17R {
        SramIf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if18(&self) -> SramIf18R {
        SramIf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if19(&self) -> SramIf19R {
        SramIf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if20(&self) -> SramIf20R {
        SramIf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if21(&self) -> SramIf21R {
        SramIf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if22(&self) -> SramIf22R {
        SramIf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if23(&self) -> SramIf23R {
        SramIf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if24(&self) -> SramIf24R {
        SramIf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if25(&self) -> SramIf25R {
        SramIf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if26(&self) -> SramIf26R {
        SramIf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if27(&self) -> SramIf27R {
        SramIf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if28(&self) -> SramIf28R {
        SramIf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if29(&self) -> SramIf29R {
        SramIf29R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCLKGATEOVERRIDE1")
            .field("sram_if0", &self.sram_if0())
            .field("sram_if1", &self.sram_if1())
            .field("sram_if2", &self.sram_if2())
            .field("sram_if3", &self.sram_if3())
            .field("sram_if4", &self.sram_if4())
            .field("sram_if5", &self.sram_if5())
            .field("sram_if6", &self.sram_if6())
            .field("sram_if7", &self.sram_if7())
            .field("sram_if8", &self.sram_if8())
            .field("sram_if9", &self.sram_if9())
            .field("sram_if10", &self.sram_if10())
            .field("sram_if11", &self.sram_if11())
            .field("sram_if12", &self.sram_if12())
            .field("sram_if13", &self.sram_if13())
            .field("sram_if14", &self.sram_if14())
            .field("sram_if15", &self.sram_if15())
            .field("sram_if16", &self.sram_if16())
            .field("sram_if17", &self.sram_if17())
            .field("sram_if18", &self.sram_if18())
            .field("sram_if19", &self.sram_if19())
            .field("sram_if20", &self.sram_if20())
            .field("sram_if21", &self.sram_if21())
            .field("sram_if22", &self.sram_if22())
            .field("sram_if23", &self.sram_if23())
            .field("sram_if24", &self.sram_if24())
            .field("sram_if25", &self.sram_if25())
            .field("sram_if26", &self.sram_if26())
            .field("sram_if27", &self.sram_if27())
            .field("sram_if28", &self.sram_if28())
            .field("sram_if29", &self.sram_if29())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if0(&mut self) -> SramIf0W<Autoclkgateoverride1Spec> {
        SramIf0W::new(self, 0)
    }
    #[doc = "Bit 1 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if1(&mut self) -> SramIf1W<Autoclkgateoverride1Spec> {
        SramIf1W::new(self, 1)
    }
    #[doc = "Bit 2 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if2(&mut self) -> SramIf2W<Autoclkgateoverride1Spec> {
        SramIf2W::new(self, 2)
    }
    #[doc = "Bit 3 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if3(&mut self) -> SramIf3W<Autoclkgateoverride1Spec> {
        SramIf3W::new(self, 3)
    }
    #[doc = "Bit 4 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if4(&mut self) -> SramIf4W<Autoclkgateoverride1Spec> {
        SramIf4W::new(self, 4)
    }
    #[doc = "Bit 5 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if5(&mut self) -> SramIf5W<Autoclkgateoverride1Spec> {
        SramIf5W::new(self, 5)
    }
    #[doc = "Bit 6 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if6(&mut self) -> SramIf6W<Autoclkgateoverride1Spec> {
        SramIf6W::new(self, 6)
    }
    #[doc = "Bit 7 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if7(&mut self) -> SramIf7W<Autoclkgateoverride1Spec> {
        SramIf7W::new(self, 7)
    }
    #[doc = "Bit 8 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if8(&mut self) -> SramIf8W<Autoclkgateoverride1Spec> {
        SramIf8W::new(self, 8)
    }
    #[doc = "Bit 9 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if9(&mut self) -> SramIf9W<Autoclkgateoverride1Spec> {
        SramIf9W::new(self, 9)
    }
    #[doc = "Bit 10 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if10(&mut self) -> SramIf10W<Autoclkgateoverride1Spec> {
        SramIf10W::new(self, 10)
    }
    #[doc = "Bit 11 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if11(&mut self) -> SramIf11W<Autoclkgateoverride1Spec> {
        SramIf11W::new(self, 11)
    }
    #[doc = "Bit 12 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if12(&mut self) -> SramIf12W<Autoclkgateoverride1Spec> {
        SramIf12W::new(self, 12)
    }
    #[doc = "Bit 13 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if13(&mut self) -> SramIf13W<Autoclkgateoverride1Spec> {
        SramIf13W::new(self, 13)
    }
    #[doc = "Bit 14 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if14(&mut self) -> SramIf14W<Autoclkgateoverride1Spec> {
        SramIf14W::new(self, 14)
    }
    #[doc = "Bit 15 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if15(&mut self) -> SramIf15W<Autoclkgateoverride1Spec> {
        SramIf15W::new(self, 15)
    }
    #[doc = "Bit 16 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if16(&mut self) -> SramIf16W<Autoclkgateoverride1Spec> {
        SramIf16W::new(self, 16)
    }
    #[doc = "Bit 17 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if17(&mut self) -> SramIf17W<Autoclkgateoverride1Spec> {
        SramIf17W::new(self, 17)
    }
    #[doc = "Bit 18 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if18(&mut self) -> SramIf18W<Autoclkgateoverride1Spec> {
        SramIf18W::new(self, 18)
    }
    #[doc = "Bit 19 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if19(&mut self) -> SramIf19W<Autoclkgateoverride1Spec> {
        SramIf19W::new(self, 19)
    }
    #[doc = "Bit 20 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if20(&mut self) -> SramIf20W<Autoclkgateoverride1Spec> {
        SramIf20W::new(self, 20)
    }
    #[doc = "Bit 21 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if21(&mut self) -> SramIf21W<Autoclkgateoverride1Spec> {
        SramIf21W::new(self, 21)
    }
    #[doc = "Bit 22 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if22(&mut self) -> SramIf22W<Autoclkgateoverride1Spec> {
        SramIf22W::new(self, 22)
    }
    #[doc = "Bit 23 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if23(&mut self) -> SramIf23W<Autoclkgateoverride1Spec> {
        SramIf23W::new(self, 23)
    }
    #[doc = "Bit 24 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if24(&mut self) -> SramIf24W<Autoclkgateoverride1Spec> {
        SramIf24W::new(self, 24)
    }
    #[doc = "Bit 25 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if25(&mut self) -> SramIf25W<Autoclkgateoverride1Spec> {
        SramIf25W::new(self, 25)
    }
    #[doc = "Bit 26 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if26(&mut self) -> SramIf26W<Autoclkgateoverride1Spec> {
        SramIf26W::new(self, 26)
    }
    #[doc = "Bit 27 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if27(&mut self) -> SramIf27W<Autoclkgateoverride1Spec> {
        SramIf27W::new(self, 27)
    }
    #[doc = "Bit 28 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if28(&mut self) -> SramIf28W<Autoclkgateoverride1Spec> {
        SramIf28W::new(self, 28)
    }
    #[doc = "Bit 29 - auto clock gating enable"]
    #[inline(always)]
    pub fn sram_if29(&mut self) -> SramIf29W<Autoclkgateoverride1Spec> {
        SramIf29W::new(self, 29)
    }
}
#[doc = "auto clock gating override 1\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autoclkgateoverride1Spec;
impl crate::RegisterSpec for Autoclkgateoverride1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autoclkgateoverride1::R`](R) reader structure"]
impl crate::Readable for Autoclkgateoverride1Spec {}
#[doc = "`write(|w| ..)` method takes [`autoclkgateoverride1::W`](W) writer structure"]
impl crate::Writable for Autoclkgateoverride1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE1 to value 0xffff_ffff"]
impl crate::Resettable for Autoclkgateoverride1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
