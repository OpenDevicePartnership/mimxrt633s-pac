#[doc = "Register `AHB_SRAM_ACCESS_DISABLE` reader"]
pub type R = crate::R<AhbSramAccessDisableSpec>;
#[doc = "Register `AHB_SRAM_ACCESS_DISABLE` writer"]
pub type W = crate::W<AhbSramAccessDisableSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram00If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram00If> for bool {
    #[inline(always)]
    fn from(variant: Sram00If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM00_IF` reader - no description available"]
pub type Sram00IfR = crate::BitReader<Sram00If>;
impl Sram00IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram00If {
        match self.bits {
            false => Sram00If::Enabled,
            true => Sram00If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram00If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram00If::Disabled
    }
}
#[doc = "Field `SRAM00_IF` writer - no description available"]
pub type Sram00IfW<'a, REG> = crate::BitWriter<'a, REG, Sram00If>;
impl<'a, REG> Sram00IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram00If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram00If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram01If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram01If> for bool {
    #[inline(always)]
    fn from(variant: Sram01If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM01_IF` reader - no description available"]
pub type Sram01IfR = crate::BitReader<Sram01If>;
impl Sram01IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram01If {
        match self.bits {
            false => Sram01If::Enabled,
            true => Sram01If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram01If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram01If::Disabled
    }
}
#[doc = "Field `SRAM01_IF` writer - no description available"]
pub type Sram01IfW<'a, REG> = crate::BitWriter<'a, REG, Sram01If>;
impl<'a, REG> Sram01IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram01If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram01If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram02If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram02If> for bool {
    #[inline(always)]
    fn from(variant: Sram02If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM02_IF` reader - no description available"]
pub type Sram02IfR = crate::BitReader<Sram02If>;
impl Sram02IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram02If {
        match self.bits {
            false => Sram02If::Enabled,
            true => Sram02If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram02If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram02If::Disabled
    }
}
#[doc = "Field `SRAM02_IF` writer - no description available"]
pub type Sram02IfW<'a, REG> = crate::BitWriter<'a, REG, Sram02If>;
impl<'a, REG> Sram02IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram02If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram02If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram03If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram03If> for bool {
    #[inline(always)]
    fn from(variant: Sram03If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM03_IF` reader - no description available"]
pub type Sram03IfR = crate::BitReader<Sram03If>;
impl Sram03IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram03If {
        match self.bits {
            false => Sram03If::Enabled,
            true => Sram03If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram03If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram03If::Disabled
    }
}
#[doc = "Field `SRAM03_IF` writer - no description available"]
pub type Sram03IfW<'a, REG> = crate::BitWriter<'a, REG, Sram03If>;
impl<'a, REG> Sram03IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram03If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram03If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram04If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram04If> for bool {
    #[inline(always)]
    fn from(variant: Sram04If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM04_IF` reader - no description available"]
pub type Sram04IfR = crate::BitReader<Sram04If>;
impl Sram04IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram04If {
        match self.bits {
            false => Sram04If::Enabled,
            true => Sram04If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram04If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram04If::Disabled
    }
}
#[doc = "Field `SRAM04_IF` writer - no description available"]
pub type Sram04IfW<'a, REG> = crate::BitWriter<'a, REG, Sram04If>;
impl<'a, REG> Sram04IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram04If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram04If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram05If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram05If> for bool {
    #[inline(always)]
    fn from(variant: Sram05If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM05_IF` reader - no description available"]
pub type Sram05IfR = crate::BitReader<Sram05If>;
impl Sram05IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram05If {
        match self.bits {
            false => Sram05If::Enabled,
            true => Sram05If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram05If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram05If::Disabled
    }
}
#[doc = "Field `SRAM05_IF` writer - no description available"]
pub type Sram05IfW<'a, REG> = crate::BitWriter<'a, REG, Sram05If>;
impl<'a, REG> Sram05IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram05If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram05If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram06If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram06If> for bool {
    #[inline(always)]
    fn from(variant: Sram06If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM06_IF` reader - no description available"]
pub type Sram06IfR = crate::BitReader<Sram06If>;
impl Sram06IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram06If {
        match self.bits {
            false => Sram06If::Enabled,
            true => Sram06If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram06If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram06If::Disabled
    }
}
#[doc = "Field `SRAM06_IF` writer - no description available"]
pub type Sram06IfW<'a, REG> = crate::BitWriter<'a, REG, Sram06If>;
impl<'a, REG> Sram06IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram06If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram06If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram07If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram07If> for bool {
    #[inline(always)]
    fn from(variant: Sram07If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM07_IF` reader - no description available"]
pub type Sram07IfR = crate::BitReader<Sram07If>;
impl Sram07IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram07If {
        match self.bits {
            false => Sram07If::Enabled,
            true => Sram07If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram07If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram07If::Disabled
    }
}
#[doc = "Field `SRAM07_IF` writer - no description available"]
pub type Sram07IfW<'a, REG> = crate::BitWriter<'a, REG, Sram07If>;
impl<'a, REG> Sram07IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram07If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram07If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram08If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram08If> for bool {
    #[inline(always)]
    fn from(variant: Sram08If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM08_IF` reader - no description available"]
pub type Sram08IfR = crate::BitReader<Sram08If>;
impl Sram08IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram08If {
        match self.bits {
            false => Sram08If::Enabled,
            true => Sram08If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram08If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram08If::Disabled
    }
}
#[doc = "Field `SRAM08_IF` writer - no description available"]
pub type Sram08IfW<'a, REG> = crate::BitWriter<'a, REG, Sram08If>;
impl<'a, REG> Sram08IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram08If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram08If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram09If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram09If> for bool {
    #[inline(always)]
    fn from(variant: Sram09If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM09_IF` reader - no description available"]
pub type Sram09IfR = crate::BitReader<Sram09If>;
impl Sram09IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram09If {
        match self.bits {
            false => Sram09If::Enabled,
            true => Sram09If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram09If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram09If::Disabled
    }
}
#[doc = "Field `SRAM09_IF` writer - no description available"]
pub type Sram09IfW<'a, REG> = crate::BitWriter<'a, REG, Sram09If>;
impl<'a, REG> Sram09IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram09If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram09If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram10If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram10If> for bool {
    #[inline(always)]
    fn from(variant: Sram10If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM10_IF` reader - no description available"]
pub type Sram10IfR = crate::BitReader<Sram10If>;
impl Sram10IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram10If {
        match self.bits {
            false => Sram10If::Enabled,
            true => Sram10If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram10If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram10If::Disabled
    }
}
#[doc = "Field `SRAM10_IF` writer - no description available"]
pub type Sram10IfW<'a, REG> = crate::BitWriter<'a, REG, Sram10If>;
impl<'a, REG> Sram10IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram10If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram10If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram11If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram11If> for bool {
    #[inline(always)]
    fn from(variant: Sram11If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM11_IF` reader - no description available"]
pub type Sram11IfR = crate::BitReader<Sram11If>;
impl Sram11IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram11If {
        match self.bits {
            false => Sram11If::Enabled,
            true => Sram11If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram11If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram11If::Disabled
    }
}
#[doc = "Field `SRAM11_IF` writer - no description available"]
pub type Sram11IfW<'a, REG> = crate::BitWriter<'a, REG, Sram11If>;
impl<'a, REG> Sram11IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram11If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram11If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram12If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram12If> for bool {
    #[inline(always)]
    fn from(variant: Sram12If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM12_IF` reader - no description available"]
pub type Sram12IfR = crate::BitReader<Sram12If>;
impl Sram12IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram12If {
        match self.bits {
            false => Sram12If::Enabled,
            true => Sram12If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram12If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram12If::Disabled
    }
}
#[doc = "Field `SRAM12_IF` writer - no description available"]
pub type Sram12IfW<'a, REG> = crate::BitWriter<'a, REG, Sram12If>;
impl<'a, REG> Sram12IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram12If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram12If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram13If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram13If> for bool {
    #[inline(always)]
    fn from(variant: Sram13If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM13_IF` reader - no description available"]
pub type Sram13IfR = crate::BitReader<Sram13If>;
impl Sram13IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram13If {
        match self.bits {
            false => Sram13If::Enabled,
            true => Sram13If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram13If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram13If::Disabled
    }
}
#[doc = "Field `SRAM13_IF` writer - no description available"]
pub type Sram13IfW<'a, REG> = crate::BitWriter<'a, REG, Sram13If>;
impl<'a, REG> Sram13IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram13If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram13If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram14If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram14If> for bool {
    #[inline(always)]
    fn from(variant: Sram14If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM14_IF` reader - no description available"]
pub type Sram14IfR = crate::BitReader<Sram14If>;
impl Sram14IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram14If {
        match self.bits {
            false => Sram14If::Enabled,
            true => Sram14If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram14If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram14If::Disabled
    }
}
#[doc = "Field `SRAM14_IF` writer - no description available"]
pub type Sram14IfW<'a, REG> = crate::BitWriter<'a, REG, Sram14If>;
impl<'a, REG> Sram14IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram14If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram14If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram15If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram15If> for bool {
    #[inline(always)]
    fn from(variant: Sram15If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM15_IF` reader - no description available"]
pub type Sram15IfR = crate::BitReader<Sram15If>;
impl Sram15IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram15If {
        match self.bits {
            false => Sram15If::Enabled,
            true => Sram15If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram15If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram15If::Disabled
    }
}
#[doc = "Field `SRAM15_IF` writer - no description available"]
pub type Sram15IfW<'a, REG> = crate::BitWriter<'a, REG, Sram15If>;
impl<'a, REG> Sram15IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram15If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram15If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram16If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram16If> for bool {
    #[inline(always)]
    fn from(variant: Sram16If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM16_IF` reader - no description available"]
pub type Sram16IfR = crate::BitReader<Sram16If>;
impl Sram16IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram16If {
        match self.bits {
            false => Sram16If::Enabled,
            true => Sram16If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram16If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram16If::Disabled
    }
}
#[doc = "Field `SRAM16_IF` writer - no description available"]
pub type Sram16IfW<'a, REG> = crate::BitWriter<'a, REG, Sram16If>;
impl<'a, REG> Sram16IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram16If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram16If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram17If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram17If> for bool {
    #[inline(always)]
    fn from(variant: Sram17If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM17_IF` reader - no description available"]
pub type Sram17IfR = crate::BitReader<Sram17If>;
impl Sram17IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram17If {
        match self.bits {
            false => Sram17If::Enabled,
            true => Sram17If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram17If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram17If::Disabled
    }
}
#[doc = "Field `SRAM17_IF` writer - no description available"]
pub type Sram17IfW<'a, REG> = crate::BitWriter<'a, REG, Sram17If>;
impl<'a, REG> Sram17IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram17If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram17If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram18If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram18If> for bool {
    #[inline(always)]
    fn from(variant: Sram18If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM18_IF` reader - no description available"]
pub type Sram18IfR = crate::BitReader<Sram18If>;
impl Sram18IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram18If {
        match self.bits {
            false => Sram18If::Enabled,
            true => Sram18If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram18If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram18If::Disabled
    }
}
#[doc = "Field `SRAM18_IF` writer - no description available"]
pub type Sram18IfW<'a, REG> = crate::BitWriter<'a, REG, Sram18If>;
impl<'a, REG> Sram18IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram18If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram18If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram19If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram19If> for bool {
    #[inline(always)]
    fn from(variant: Sram19If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM19_IF` reader - no description available"]
pub type Sram19IfR = crate::BitReader<Sram19If>;
impl Sram19IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram19If {
        match self.bits {
            false => Sram19If::Enabled,
            true => Sram19If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram19If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram19If::Disabled
    }
}
#[doc = "Field `SRAM19_IF` writer - no description available"]
pub type Sram19IfW<'a, REG> = crate::BitWriter<'a, REG, Sram19If>;
impl<'a, REG> Sram19IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram19If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram19If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram20If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram20If> for bool {
    #[inline(always)]
    fn from(variant: Sram20If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM20_IF` reader - no description available"]
pub type Sram20IfR = crate::BitReader<Sram20If>;
impl Sram20IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram20If {
        match self.bits {
            false => Sram20If::Enabled,
            true => Sram20If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram20If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram20If::Disabled
    }
}
#[doc = "Field `SRAM20_IF` writer - no description available"]
pub type Sram20IfW<'a, REG> = crate::BitWriter<'a, REG, Sram20If>;
impl<'a, REG> Sram20IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram20If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram20If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram21If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram21If> for bool {
    #[inline(always)]
    fn from(variant: Sram21If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM21_IF` reader - no description available"]
pub type Sram21IfR = crate::BitReader<Sram21If>;
impl Sram21IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram21If {
        match self.bits {
            false => Sram21If::Enabled,
            true => Sram21If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram21If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram21If::Disabled
    }
}
#[doc = "Field `SRAM21_IF` writer - no description available"]
pub type Sram21IfW<'a, REG> = crate::BitWriter<'a, REG, Sram21If>;
impl<'a, REG> Sram21IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram21If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram21If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram22If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram22If> for bool {
    #[inline(always)]
    fn from(variant: Sram22If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM22_IF` reader - no description available"]
pub type Sram22IfR = crate::BitReader<Sram22If>;
impl Sram22IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram22If {
        match self.bits {
            false => Sram22If::Enabled,
            true => Sram22If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram22If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram22If::Disabled
    }
}
#[doc = "Field `SRAM22_IF` writer - no description available"]
pub type Sram22IfW<'a, REG> = crate::BitWriter<'a, REG, Sram22If>;
impl<'a, REG> Sram22IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram22If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram22If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram23If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram23If> for bool {
    #[inline(always)]
    fn from(variant: Sram23If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM23_IF` reader - no description available"]
pub type Sram23IfR = crate::BitReader<Sram23If>;
impl Sram23IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram23If {
        match self.bits {
            false => Sram23If::Enabled,
            true => Sram23If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram23If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram23If::Disabled
    }
}
#[doc = "Field `SRAM23_IF` writer - no description available"]
pub type Sram23IfW<'a, REG> = crate::BitWriter<'a, REG, Sram23If>;
impl<'a, REG> Sram23IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram23If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram23If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram24If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram24If> for bool {
    #[inline(always)]
    fn from(variant: Sram24If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM24_IF` reader - no description available"]
pub type Sram24IfR = crate::BitReader<Sram24If>;
impl Sram24IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram24If {
        match self.bits {
            false => Sram24If::Enabled,
            true => Sram24If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram24If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram24If::Disabled
    }
}
#[doc = "Field `SRAM24_IF` writer - no description available"]
pub type Sram24IfW<'a, REG> = crate::BitWriter<'a, REG, Sram24If>;
impl<'a, REG> Sram24IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram24If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram24If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram25If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram25If> for bool {
    #[inline(always)]
    fn from(variant: Sram25If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM25_IF` reader - no description available"]
pub type Sram25IfR = crate::BitReader<Sram25If>;
impl Sram25IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram25If {
        match self.bits {
            false => Sram25If::Enabled,
            true => Sram25If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram25If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram25If::Disabled
    }
}
#[doc = "Field `SRAM25_IF` writer - no description available"]
pub type Sram25IfW<'a, REG> = crate::BitWriter<'a, REG, Sram25If>;
impl<'a, REG> Sram25IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram25If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram25If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram26If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram26If> for bool {
    #[inline(always)]
    fn from(variant: Sram26If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM26_IF` reader - no description available"]
pub type Sram26IfR = crate::BitReader<Sram26If>;
impl Sram26IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram26If {
        match self.bits {
            false => Sram26If::Enabled,
            true => Sram26If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram26If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram26If::Disabled
    }
}
#[doc = "Field `SRAM26_IF` writer - no description available"]
pub type Sram26IfW<'a, REG> = crate::BitWriter<'a, REG, Sram26If>;
impl<'a, REG> Sram26IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram26If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram26If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram27If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram27If> for bool {
    #[inline(always)]
    fn from(variant: Sram27If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM27_IF` reader - no description available"]
pub type Sram27IfR = crate::BitReader<Sram27If>;
impl Sram27IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram27If {
        match self.bits {
            false => Sram27If::Enabled,
            true => Sram27If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram27If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram27If::Disabled
    }
}
#[doc = "Field `SRAM27_IF` writer - no description available"]
pub type Sram27IfW<'a, REG> = crate::BitWriter<'a, REG, Sram27If>;
impl<'a, REG> Sram27IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram27If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram27If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram28If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram28If> for bool {
    #[inline(always)]
    fn from(variant: Sram28If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM28_IF` reader - no description available"]
pub type Sram28IfR = crate::BitReader<Sram28If>;
impl Sram28IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram28If {
        match self.bits {
            false => Sram28If::Enabled,
            true => Sram28If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram28If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram28If::Disabled
    }
}
#[doc = "Field `SRAM28_IF` writer - no description available"]
pub type Sram28IfW<'a, REG> = crate::BitWriter<'a, REG, Sram28If>;
impl<'a, REG> Sram28IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram28If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram28If::Disabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram29If {
    #[doc = "0: enable"]
    Enabled = 0,
    #[doc = "1: disable"]
    Disabled = 1,
}
impl From<Sram29If> for bool {
    #[inline(always)]
    fn from(variant: Sram29If) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM29_IF` reader - no description available"]
pub type Sram29IfR = crate::BitReader<Sram29If>;
impl Sram29IfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram29If {
        match self.bits {
            false => Sram29If::Enabled,
            true => Sram29If::Disabled,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sram29If::Enabled
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sram29If::Disabled
    }
}
#[doc = "Field `SRAM29_IF` writer - no description available"]
pub type Sram29IfW<'a, REG> = crate::BitWriter<'a, REG, Sram29If>;
impl<'a, REG> Sram29IfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram29If::Enabled)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sram29If::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sram00_if(&self) -> Sram00IfR {
        Sram00IfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sram01_if(&self) -> Sram01IfR {
        Sram01IfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sram02_if(&self) -> Sram02IfR {
        Sram02IfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn sram03_if(&self) -> Sram03IfR {
        Sram03IfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sram04_if(&self) -> Sram04IfR {
        Sram04IfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn sram05_if(&self) -> Sram05IfR {
        Sram05IfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn sram06_if(&self) -> Sram06IfR {
        Sram06IfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn sram07_if(&self) -> Sram07IfR {
        Sram07IfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn sram08_if(&self) -> Sram08IfR {
        Sram08IfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn sram09_if(&self) -> Sram09IfR {
        Sram09IfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn sram10_if(&self) -> Sram10IfR {
        Sram10IfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn sram11_if(&self) -> Sram11IfR {
        Sram11IfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn sram12_if(&self) -> Sram12IfR {
        Sram12IfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sram13_if(&self) -> Sram13IfR {
        Sram13IfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn sram14_if(&self) -> Sram14IfR {
        Sram14IfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn sram15_if(&self) -> Sram15IfR {
        Sram15IfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn sram16_if(&self) -> Sram16IfR {
        Sram16IfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn sram17_if(&self) -> Sram17IfR {
        Sram17IfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn sram18_if(&self) -> Sram18IfR {
        Sram18IfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn sram19_if(&self) -> Sram19IfR {
        Sram19IfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn sram20_if(&self) -> Sram20IfR {
        Sram20IfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn sram21_if(&self) -> Sram21IfR {
        Sram21IfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn sram22_if(&self) -> Sram22IfR {
        Sram22IfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn sram23_if(&self) -> Sram23IfR {
        Sram23IfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn sram24_if(&self) -> Sram24IfR {
        Sram24IfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn sram25_if(&self) -> Sram25IfR {
        Sram25IfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn sram26_if(&self) -> Sram26IfR {
        Sram26IfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn sram27_if(&self) -> Sram27IfR {
        Sram27IfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn sram28_if(&self) -> Sram28IfR {
        Sram28IfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    pub fn sram29_if(&self) -> Sram29IfR {
        Sram29IfR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_SRAM_ACCESS_DISABLE")
            .field("sram00_if", &self.sram00_if())
            .field("sram01_if", &self.sram01_if())
            .field("sram02_if", &self.sram02_if())
            .field("sram03_if", &self.sram03_if())
            .field("sram04_if", &self.sram04_if())
            .field("sram05_if", &self.sram05_if())
            .field("sram06_if", &self.sram06_if())
            .field("sram07_if", &self.sram07_if())
            .field("sram08_if", &self.sram08_if())
            .field("sram09_if", &self.sram09_if())
            .field("sram10_if", &self.sram10_if())
            .field("sram11_if", &self.sram11_if())
            .field("sram12_if", &self.sram12_if())
            .field("sram13_if", &self.sram13_if())
            .field("sram14_if", &self.sram14_if())
            .field("sram15_if", &self.sram15_if())
            .field("sram16_if", &self.sram16_if())
            .field("sram17_if", &self.sram17_if())
            .field("sram18_if", &self.sram18_if())
            .field("sram19_if", &self.sram19_if())
            .field("sram20_if", &self.sram20_if())
            .field("sram21_if", &self.sram21_if())
            .field("sram22_if", &self.sram22_if())
            .field("sram23_if", &self.sram23_if())
            .field("sram24_if", &self.sram24_if())
            .field("sram25_if", &self.sram25_if())
            .field("sram26_if", &self.sram26_if())
            .field("sram27_if", &self.sram27_if())
            .field("sram28_if", &self.sram28_if())
            .field("sram29_if", &self.sram29_if())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sram00_if(&mut self) -> Sram00IfW<AhbSramAccessDisableSpec> {
        Sram00IfW::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sram01_if(&mut self) -> Sram01IfW<AhbSramAccessDisableSpec> {
        Sram01IfW::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sram02_if(&mut self) -> Sram02IfW<AhbSramAccessDisableSpec> {
        Sram02IfW::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn sram03_if(&mut self) -> Sram03IfW<AhbSramAccessDisableSpec> {
        Sram03IfW::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sram04_if(&mut self) -> Sram04IfW<AhbSramAccessDisableSpec> {
        Sram04IfW::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn sram05_if(&mut self) -> Sram05IfW<AhbSramAccessDisableSpec> {
        Sram05IfW::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn sram06_if(&mut self) -> Sram06IfW<AhbSramAccessDisableSpec> {
        Sram06IfW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn sram07_if(&mut self) -> Sram07IfW<AhbSramAccessDisableSpec> {
        Sram07IfW::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn sram08_if(&mut self) -> Sram08IfW<AhbSramAccessDisableSpec> {
        Sram08IfW::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn sram09_if(&mut self) -> Sram09IfW<AhbSramAccessDisableSpec> {
        Sram09IfW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn sram10_if(&mut self) -> Sram10IfW<AhbSramAccessDisableSpec> {
        Sram10IfW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn sram11_if(&mut self) -> Sram11IfW<AhbSramAccessDisableSpec> {
        Sram11IfW::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn sram12_if(&mut self) -> Sram12IfW<AhbSramAccessDisableSpec> {
        Sram12IfW::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sram13_if(&mut self) -> Sram13IfW<AhbSramAccessDisableSpec> {
        Sram13IfW::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn sram14_if(&mut self) -> Sram14IfW<AhbSramAccessDisableSpec> {
        Sram14IfW::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn sram15_if(&mut self) -> Sram15IfW<AhbSramAccessDisableSpec> {
        Sram15IfW::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn sram16_if(&mut self) -> Sram16IfW<AhbSramAccessDisableSpec> {
        Sram16IfW::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn sram17_if(&mut self) -> Sram17IfW<AhbSramAccessDisableSpec> {
        Sram17IfW::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn sram18_if(&mut self) -> Sram18IfW<AhbSramAccessDisableSpec> {
        Sram18IfW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn sram19_if(&mut self) -> Sram19IfW<AhbSramAccessDisableSpec> {
        Sram19IfW::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn sram20_if(&mut self) -> Sram20IfW<AhbSramAccessDisableSpec> {
        Sram20IfW::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn sram21_if(&mut self) -> Sram21IfW<AhbSramAccessDisableSpec> {
        Sram21IfW::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn sram22_if(&mut self) -> Sram22IfW<AhbSramAccessDisableSpec> {
        Sram22IfW::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn sram23_if(&mut self) -> Sram23IfW<AhbSramAccessDisableSpec> {
        Sram23IfW::new(self, 23)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn sram24_if(&mut self) -> Sram24IfW<AhbSramAccessDisableSpec> {
        Sram24IfW::new(self, 24)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn sram25_if(&mut self) -> Sram25IfW<AhbSramAccessDisableSpec> {
        Sram25IfW::new(self, 25)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn sram26_if(&mut self) -> Sram26IfW<AhbSramAccessDisableSpec> {
        Sram26IfW::new(self, 26)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn sram27_if(&mut self) -> Sram27IfW<AhbSramAccessDisableSpec> {
        Sram27IfW::new(self, 27)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn sram28_if(&mut self) -> Sram28IfW<AhbSramAccessDisableSpec> {
        Sram28IfW::new(self, 28)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    pub fn sram29_if(&mut self) -> Sram29IfW<AhbSramAccessDisableSpec> {
        Sram29IfW::new(self, 29)
    }
}
#[doc = "AHB SRAM access disable\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_sram_access_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_sram_access_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbSramAccessDisableSpec;
impl crate::RegisterSpec for AhbSramAccessDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_sram_access_disable::R`](R) reader structure"]
impl crate::Readable for AhbSramAccessDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_sram_access_disable::W`](W) writer structure"]
impl crate::Writable for AhbSramAccessDisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_SRAM_ACCESS_DISABLE to value 0"]
impl crate::Resettable for AhbSramAccessDisableSpec {
    const RESET_VALUE: u32 = 0;
}
