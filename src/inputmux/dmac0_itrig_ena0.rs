#[doc = "Register `DMAC0_ITRIG_ENA0` reader"]
pub type R = crate::R<Dmac0ItrigEna0Spec>;
#[doc = "Register `DMAC0_ITRIG_ENA0` writer"]
pub type W = crate::W<Dmac0ItrigEna0Spec>;
#[doc = "DMAC0 input trigger inmux 0 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux0 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux0> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX0` reader - DMAC0 input trigger inmux 0 enable"]
pub type Dmac0ItrigInmux0R = crate::BitReader<Dmac0ItrigInmux0>;
impl Dmac0ItrigInmux0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux0 {
        match self.bits {
            false => Dmac0ItrigInmux0::Disabled,
            true => Dmac0ItrigInmux0::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux0::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX0` writer - DMAC0 input trigger inmux 0 enable"]
pub type Dmac0ItrigInmux0W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux0>;
impl<'a, REG> Dmac0ItrigInmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux0::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 1 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux1 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux1> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX1` reader - DMAC0 input trigger inmux 1 enable"]
pub type Dmac0ItrigInmux1R = crate::BitReader<Dmac0ItrigInmux1>;
impl Dmac0ItrigInmux1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux1 {
        match self.bits {
            false => Dmac0ItrigInmux1::Disabled,
            true => Dmac0ItrigInmux1::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux1::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX1` writer - DMAC0 input trigger inmux 1 enable"]
pub type Dmac0ItrigInmux1W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux1>;
impl<'a, REG> Dmac0ItrigInmux1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux1::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 2 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux2 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux2> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX2` reader - DMAC0 input trigger inmux 2 enable"]
pub type Dmac0ItrigInmux2R = crate::BitReader<Dmac0ItrigInmux2>;
impl Dmac0ItrigInmux2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux2 {
        match self.bits {
            false => Dmac0ItrigInmux2::Disabled,
            true => Dmac0ItrigInmux2::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux2::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX2` writer - DMAC0 input trigger inmux 2 enable"]
pub type Dmac0ItrigInmux2W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux2>;
impl<'a, REG> Dmac0ItrigInmux2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux2::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 3 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux3 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux3> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX3` reader - DMAC0 input trigger inmux 3 enable"]
pub type Dmac0ItrigInmux3R = crate::BitReader<Dmac0ItrigInmux3>;
impl Dmac0ItrigInmux3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux3 {
        match self.bits {
            false => Dmac0ItrigInmux3::Disabled,
            true => Dmac0ItrigInmux3::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux3::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX3` writer - DMAC0 input trigger inmux 3 enable"]
pub type Dmac0ItrigInmux3W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux3>;
impl<'a, REG> Dmac0ItrigInmux3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux3::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 4 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux4 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux4> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX4` reader - DMAC0 input trigger inmux 4 enable"]
pub type Dmac0ItrigInmux4R = crate::BitReader<Dmac0ItrigInmux4>;
impl Dmac0ItrigInmux4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux4 {
        match self.bits {
            false => Dmac0ItrigInmux4::Disabled,
            true => Dmac0ItrigInmux4::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux4::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX4` writer - DMAC0 input trigger inmux 4 enable"]
pub type Dmac0ItrigInmux4W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux4>;
impl<'a, REG> Dmac0ItrigInmux4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux4::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 5 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux5 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux5> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX5` reader - DMAC0 input trigger inmux 5 enable"]
pub type Dmac0ItrigInmux5R = crate::BitReader<Dmac0ItrigInmux5>;
impl Dmac0ItrigInmux5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux5 {
        match self.bits {
            false => Dmac0ItrigInmux5::Disabled,
            true => Dmac0ItrigInmux5::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux5::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux5::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX5` writer - DMAC0 input trigger inmux 5 enable"]
pub type Dmac0ItrigInmux5W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux5>;
impl<'a, REG> Dmac0ItrigInmux5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux5::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux5::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 6 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux6 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux6> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX6` reader - DMAC0 input trigger inmux 6 enable"]
pub type Dmac0ItrigInmux6R = crate::BitReader<Dmac0ItrigInmux6>;
impl Dmac0ItrigInmux6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux6 {
        match self.bits {
            false => Dmac0ItrigInmux6::Disabled,
            true => Dmac0ItrigInmux6::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux6::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux6::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX6` writer - DMAC0 input trigger inmux 6 enable"]
pub type Dmac0ItrigInmux6W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux6>;
impl<'a, REG> Dmac0ItrigInmux6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux6::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux6::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 7 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux7 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux7> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX7` reader - DMAC0 input trigger inmux 7 enable"]
pub type Dmac0ItrigInmux7R = crate::BitReader<Dmac0ItrigInmux7>;
impl Dmac0ItrigInmux7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux7 {
        match self.bits {
            false => Dmac0ItrigInmux7::Disabled,
            true => Dmac0ItrigInmux7::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux7::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux7::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX7` writer - DMAC0 input trigger inmux 7 enable"]
pub type Dmac0ItrigInmux7W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux7>;
impl<'a, REG> Dmac0ItrigInmux7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux7::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux7::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 8 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux8 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux8> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX8` reader - DMAC0 input trigger inmux 8 enable"]
pub type Dmac0ItrigInmux8R = crate::BitReader<Dmac0ItrigInmux8>;
impl Dmac0ItrigInmux8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux8 {
        match self.bits {
            false => Dmac0ItrigInmux8::Disabled,
            true => Dmac0ItrigInmux8::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux8::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux8::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX8` writer - DMAC0 input trigger inmux 8 enable"]
pub type Dmac0ItrigInmux8W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux8>;
impl<'a, REG> Dmac0ItrigInmux8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux8::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux8::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 9 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux9 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux9> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX9` reader - DMAC0 input trigger inmux 9 enable"]
pub type Dmac0ItrigInmux9R = crate::BitReader<Dmac0ItrigInmux9>;
impl Dmac0ItrigInmux9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux9 {
        match self.bits {
            false => Dmac0ItrigInmux9::Disabled,
            true => Dmac0ItrigInmux9::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux9::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux9::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX9` writer - DMAC0 input trigger inmux 9 enable"]
pub type Dmac0ItrigInmux9W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux9>;
impl<'a, REG> Dmac0ItrigInmux9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux9::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux9::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 10 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux10 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux10> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX10` reader - DMAC0 input trigger inmux 10 enable"]
pub type Dmac0ItrigInmux10R = crate::BitReader<Dmac0ItrigInmux10>;
impl Dmac0ItrigInmux10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux10 {
        match self.bits {
            false => Dmac0ItrigInmux10::Disabled,
            true => Dmac0ItrigInmux10::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux10::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux10::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX10` writer - DMAC0 input trigger inmux 10 enable"]
pub type Dmac0ItrigInmux10W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux10>;
impl<'a, REG> Dmac0ItrigInmux10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux10::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux10::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 11 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux11 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux11> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX11` reader - DMAC0 input trigger inmux 11 enable"]
pub type Dmac0ItrigInmux11R = crate::BitReader<Dmac0ItrigInmux11>;
impl Dmac0ItrigInmux11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux11 {
        match self.bits {
            false => Dmac0ItrigInmux11::Disabled,
            true => Dmac0ItrigInmux11::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux11::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux11::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX11` writer - DMAC0 input trigger inmux 11 enable"]
pub type Dmac0ItrigInmux11W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux11>;
impl<'a, REG> Dmac0ItrigInmux11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux11::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux11::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 12 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux12 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux12> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX12` reader - DMAC0 input trigger inmux 12 enable"]
pub type Dmac0ItrigInmux12R = crate::BitReader<Dmac0ItrigInmux12>;
impl Dmac0ItrigInmux12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux12 {
        match self.bits {
            false => Dmac0ItrigInmux12::Disabled,
            true => Dmac0ItrigInmux12::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux12::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux12::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX12` writer - DMAC0 input trigger inmux 12 enable"]
pub type Dmac0ItrigInmux12W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux12>;
impl<'a, REG> Dmac0ItrigInmux12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux12::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux12::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 13 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux13 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux13> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX13` reader - DMAC0 input trigger inmux 13 enable"]
pub type Dmac0ItrigInmux13R = crate::BitReader<Dmac0ItrigInmux13>;
impl Dmac0ItrigInmux13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux13 {
        match self.bits {
            false => Dmac0ItrigInmux13::Disabled,
            true => Dmac0ItrigInmux13::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux13::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux13::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX13` writer - DMAC0 input trigger inmux 13 enable"]
pub type Dmac0ItrigInmux13W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux13>;
impl<'a, REG> Dmac0ItrigInmux13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux13::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux13::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 14 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux14 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux14> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX14` reader - DMAC0 input trigger inmux 14 enable"]
pub type Dmac0ItrigInmux14R = crate::BitReader<Dmac0ItrigInmux14>;
impl Dmac0ItrigInmux14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux14 {
        match self.bits {
            false => Dmac0ItrigInmux14::Disabled,
            true => Dmac0ItrigInmux14::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux14::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux14::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX14` writer - DMAC0 input trigger inmux 14 enable"]
pub type Dmac0ItrigInmux14W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux14>;
impl<'a, REG> Dmac0ItrigInmux14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux14::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux14::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 15 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux15 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux15> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX15` reader - DMAC0 input trigger inmux 15 enable"]
pub type Dmac0ItrigInmux15R = crate::BitReader<Dmac0ItrigInmux15>;
impl Dmac0ItrigInmux15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux15 {
        match self.bits {
            false => Dmac0ItrigInmux15::Disabled,
            true => Dmac0ItrigInmux15::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux15::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux15::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX15` writer - DMAC0 input trigger inmux 15 enable"]
pub type Dmac0ItrigInmux15W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux15>;
impl<'a, REG> Dmac0ItrigInmux15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux15::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux15::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 16 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux16 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux16> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX16` reader - DMAC0 input trigger inmux 16 enable"]
pub type Dmac0ItrigInmux16R = crate::BitReader<Dmac0ItrigInmux16>;
impl Dmac0ItrigInmux16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux16 {
        match self.bits {
            false => Dmac0ItrigInmux16::Disabled,
            true => Dmac0ItrigInmux16::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux16::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux16::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX16` writer - DMAC0 input trigger inmux 16 enable"]
pub type Dmac0ItrigInmux16W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux16>;
impl<'a, REG> Dmac0ItrigInmux16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux16::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux16::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 17 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux17 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux17> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX17` reader - DMAC0 input trigger inmux 17 enable"]
pub type Dmac0ItrigInmux17R = crate::BitReader<Dmac0ItrigInmux17>;
impl Dmac0ItrigInmux17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux17 {
        match self.bits {
            false => Dmac0ItrigInmux17::Disabled,
            true => Dmac0ItrigInmux17::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux17::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux17::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX17` writer - DMAC0 input trigger inmux 17 enable"]
pub type Dmac0ItrigInmux17W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux17>;
impl<'a, REG> Dmac0ItrigInmux17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux17::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux17::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 18 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux18 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux18> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX18` reader - DMAC0 input trigger inmux 18 enable"]
pub type Dmac0ItrigInmux18R = crate::BitReader<Dmac0ItrigInmux18>;
impl Dmac0ItrigInmux18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux18 {
        match self.bits {
            false => Dmac0ItrigInmux18::Disabled,
            true => Dmac0ItrigInmux18::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux18::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux18::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX18` writer - DMAC0 input trigger inmux 18 enable"]
pub type Dmac0ItrigInmux18W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux18>;
impl<'a, REG> Dmac0ItrigInmux18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux18::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux18::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 19 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux19 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux19> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX19` reader - DMAC0 input trigger inmux 19 enable"]
pub type Dmac0ItrigInmux19R = crate::BitReader<Dmac0ItrigInmux19>;
impl Dmac0ItrigInmux19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux19 {
        match self.bits {
            false => Dmac0ItrigInmux19::Disabled,
            true => Dmac0ItrigInmux19::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux19::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux19::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX19` writer - DMAC0 input trigger inmux 19 enable"]
pub type Dmac0ItrigInmux19W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux19>;
impl<'a, REG> Dmac0ItrigInmux19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux19::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux19::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 20 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux20 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux20> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX20` reader - DMAC0 input trigger inmux 20 enable"]
pub type Dmac0ItrigInmux20R = crate::BitReader<Dmac0ItrigInmux20>;
impl Dmac0ItrigInmux20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux20 {
        match self.bits {
            false => Dmac0ItrigInmux20::Disabled,
            true => Dmac0ItrigInmux20::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux20::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux20::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX20` writer - DMAC0 input trigger inmux 20 enable"]
pub type Dmac0ItrigInmux20W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux20>;
impl<'a, REG> Dmac0ItrigInmux20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux20::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux20::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 21 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux21 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux21> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX21` reader - DMAC0 input trigger inmux 21 enable"]
pub type Dmac0ItrigInmux21R = crate::BitReader<Dmac0ItrigInmux21>;
impl Dmac0ItrigInmux21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux21 {
        match self.bits {
            false => Dmac0ItrigInmux21::Disabled,
            true => Dmac0ItrigInmux21::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux21::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux21::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX21` writer - DMAC0 input trigger inmux 21 enable"]
pub type Dmac0ItrigInmux21W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux21>;
impl<'a, REG> Dmac0ItrigInmux21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux21::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux21::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 22 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux22 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux22> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX22` reader - DMAC0 input trigger inmux 22 enable"]
pub type Dmac0ItrigInmux22R = crate::BitReader<Dmac0ItrigInmux22>;
impl Dmac0ItrigInmux22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux22 {
        match self.bits {
            false => Dmac0ItrigInmux22::Disabled,
            true => Dmac0ItrigInmux22::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux22::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux22::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX22` writer - DMAC0 input trigger inmux 22 enable"]
pub type Dmac0ItrigInmux22W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux22>;
impl<'a, REG> Dmac0ItrigInmux22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux22::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux22::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 23 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux23 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux23> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX23` reader - DMAC0 input trigger inmux 23 enable"]
pub type Dmac0ItrigInmux23R = crate::BitReader<Dmac0ItrigInmux23>;
impl Dmac0ItrigInmux23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux23 {
        match self.bits {
            false => Dmac0ItrigInmux23::Disabled,
            true => Dmac0ItrigInmux23::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux23::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux23::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX23` writer - DMAC0 input trigger inmux 23 enable"]
pub type Dmac0ItrigInmux23W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux23>;
impl<'a, REG> Dmac0ItrigInmux23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux23::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux23::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 24 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux24 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux24> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX24` reader - DMAC0 input trigger inmux 24 enable"]
pub type Dmac0ItrigInmux24R = crate::BitReader<Dmac0ItrigInmux24>;
impl Dmac0ItrigInmux24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux24 {
        match self.bits {
            false => Dmac0ItrigInmux24::Disabled,
            true => Dmac0ItrigInmux24::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux24::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux24::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX24` writer - DMAC0 input trigger inmux 24 enable"]
pub type Dmac0ItrigInmux24W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux24>;
impl<'a, REG> Dmac0ItrigInmux24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux24::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux24::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux25 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux25> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX25` reader - DMAC0 input trigger inmux 25 enable"]
pub type Dmac0ItrigInmux25R = crate::BitReader<Dmac0ItrigInmux25>;
impl Dmac0ItrigInmux25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux25 {
        match self.bits {
            false => Dmac0ItrigInmux25::Disabled,
            true => Dmac0ItrigInmux25::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux25::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux25::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX25` writer - DMAC0 input trigger inmux 25 enable"]
pub type Dmac0ItrigInmux25W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux25>;
impl<'a, REG> Dmac0ItrigInmux25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux25::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux25::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 26 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux26 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux26> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX26` reader - DMAC0 input trigger inmux 26 enable"]
pub type Dmac0ItrigInmux26R = crate::BitReader<Dmac0ItrigInmux26>;
impl Dmac0ItrigInmux26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux26 {
        match self.bits {
            false => Dmac0ItrigInmux26::Disabled,
            true => Dmac0ItrigInmux26::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux26::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux26::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX26` writer - DMAC0 input trigger inmux 26 enable"]
pub type Dmac0ItrigInmux26W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux26>;
impl<'a, REG> Dmac0ItrigInmux26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux26::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux26::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 27 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux27 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux27> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX27` reader - DMAC0 input trigger inmux 27 enable"]
pub type Dmac0ItrigInmux27R = crate::BitReader<Dmac0ItrigInmux27>;
impl Dmac0ItrigInmux27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux27 {
        match self.bits {
            false => Dmac0ItrigInmux27::Disabled,
            true => Dmac0ItrigInmux27::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux27::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux27::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX27` writer - DMAC0 input trigger inmux 27 enable"]
pub type Dmac0ItrigInmux27W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux27>;
impl<'a, REG> Dmac0ItrigInmux27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux27::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux27::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 28 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux28 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux28> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX28` reader - DMAC0 input trigger inmux 28 enable"]
pub type Dmac0ItrigInmux28R = crate::BitReader<Dmac0ItrigInmux28>;
impl Dmac0ItrigInmux28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux28 {
        match self.bits {
            false => Dmac0ItrigInmux28::Disabled,
            true => Dmac0ItrigInmux28::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux28::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux28::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX28` writer - DMAC0 input trigger inmux 28 enable"]
pub type Dmac0ItrigInmux28W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux28>;
impl<'a, REG> Dmac0ItrigInmux28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux28::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux28::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 29 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux29 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux29> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX29` reader - DMAC0 input trigger inmux 29 enable"]
pub type Dmac0ItrigInmux29R = crate::BitReader<Dmac0ItrigInmux29>;
impl Dmac0ItrigInmux29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux29 {
        match self.bits {
            false => Dmac0ItrigInmux29::Disabled,
            true => Dmac0ItrigInmux29::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux29::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux29::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX29` writer - DMAC0 input trigger inmux 29 enable"]
pub type Dmac0ItrigInmux29W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux29>;
impl<'a, REG> Dmac0ItrigInmux29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux29::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux29::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 30 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux30 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux30> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX30` reader - DMAC0 input trigger inmux 30 enable"]
pub type Dmac0ItrigInmux30R = crate::BitReader<Dmac0ItrigInmux30>;
impl Dmac0ItrigInmux30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux30 {
        match self.bits {
            false => Dmac0ItrigInmux30::Disabled,
            true => Dmac0ItrigInmux30::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux30::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux30::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX30` writer - DMAC0 input trigger inmux 30 enable"]
pub type Dmac0ItrigInmux30W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux30>;
impl<'a, REG> Dmac0ItrigInmux30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux30::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux30::Enabled)
    }
}
#[doc = "DMAC0 input trigger inmux 31 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux31 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac0ItrigInmux31> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX31` reader - DMAC0 input trigger inmux 31 enable"]
pub type Dmac0ItrigInmux31R = crate::BitReader<Dmac0ItrigInmux31>;
impl Dmac0ItrigInmux31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0ItrigInmux31 {
        match self.bits {
            false => Dmac0ItrigInmux31::Disabled,
            true => Dmac0ItrigInmux31::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0ItrigInmux31::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0ItrigInmux31::Enabled
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX31` writer - DMAC0 input trigger inmux 31 enable"]
pub type Dmac0ItrigInmux31W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux31>;
impl<'a, REG> Dmac0ItrigInmux31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux31::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux31::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC0 input trigger inmux 0 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux0(&self) -> Dmac0ItrigInmux0R {
        Dmac0ItrigInmux0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC0 input trigger inmux 1 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux1(&self) -> Dmac0ItrigInmux1R {
        Dmac0ItrigInmux1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC0 input trigger inmux 2 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux2(&self) -> Dmac0ItrigInmux2R {
        Dmac0ItrigInmux2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC0 input trigger inmux 3 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux3(&self) -> Dmac0ItrigInmux3R {
        Dmac0ItrigInmux3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC0 input trigger inmux 4 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux4(&self) -> Dmac0ItrigInmux4R {
        Dmac0ItrigInmux4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC0 input trigger inmux 5 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux5(&self) -> Dmac0ItrigInmux5R {
        Dmac0ItrigInmux5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC0 input trigger inmux 6 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux6(&self) -> Dmac0ItrigInmux6R {
        Dmac0ItrigInmux6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC0 input trigger inmux 7 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux7(&self) -> Dmac0ItrigInmux7R {
        Dmac0ItrigInmux7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC0 input trigger inmux 8 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux8(&self) -> Dmac0ItrigInmux8R {
        Dmac0ItrigInmux8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC0 input trigger inmux 9 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux9(&self) -> Dmac0ItrigInmux9R {
        Dmac0ItrigInmux9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAC0 input trigger inmux 10 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux10(&self) -> Dmac0ItrigInmux10R {
        Dmac0ItrigInmux10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC0 input trigger inmux 11 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux11(&self) -> Dmac0ItrigInmux11R {
        Dmac0ItrigInmux11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAC0 input trigger inmux 12 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux12(&self) -> Dmac0ItrigInmux12R {
        Dmac0ItrigInmux12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAC0 input trigger inmux 13 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux13(&self) -> Dmac0ItrigInmux13R {
        Dmac0ItrigInmux13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMAC0 input trigger inmux 14 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux14(&self) -> Dmac0ItrigInmux14R {
        Dmac0ItrigInmux14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAC0 input trigger inmux 15 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux15(&self) -> Dmac0ItrigInmux15R {
        Dmac0ItrigInmux15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAC0 input trigger inmux 16 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux16(&self) -> Dmac0ItrigInmux16R {
        Dmac0ItrigInmux16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAC0 input trigger inmux 17 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux17(&self) -> Dmac0ItrigInmux17R {
        Dmac0ItrigInmux17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAC0 input trigger inmux 18 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux18(&self) -> Dmac0ItrigInmux18R {
        Dmac0ItrigInmux18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMAC0 input trigger inmux 19 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux19(&self) -> Dmac0ItrigInmux19R {
        Dmac0ItrigInmux19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMAC0 input trigger inmux 20 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux20(&self) -> Dmac0ItrigInmux20R {
        Dmac0ItrigInmux20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMAC0 input trigger inmux 21 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux21(&self) -> Dmac0ItrigInmux21R {
        Dmac0ItrigInmux21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMAC0 input trigger inmux 22 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux22(&self) -> Dmac0ItrigInmux22R {
        Dmac0ItrigInmux22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC0 input trigger inmux 23 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux23(&self) -> Dmac0ItrigInmux23R {
        Dmac0ItrigInmux23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC0 input trigger inmux 24 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux24(&self) -> Dmac0ItrigInmux24R {
        Dmac0ItrigInmux24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMAC0 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux25(&self) -> Dmac0ItrigInmux25R {
        Dmac0ItrigInmux25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMAC0 input trigger inmux 26 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux26(&self) -> Dmac0ItrigInmux26R {
        Dmac0ItrigInmux26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMAC0 input trigger inmux 27 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux27(&self) -> Dmac0ItrigInmux27R {
        Dmac0ItrigInmux27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMAC0 input trigger inmux 28 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux28(&self) -> Dmac0ItrigInmux28R {
        Dmac0ItrigInmux28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAC0 input trigger inmux 29 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux29(&self) -> Dmac0ItrigInmux29R {
        Dmac0ItrigInmux29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMAC0 input trigger inmux 30 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux30(&self) -> Dmac0ItrigInmux30R {
        Dmac0ItrigInmux30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMAC0 input trigger inmux 31 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux31(&self) -> Dmac0ItrigInmux31R {
        Dmac0ItrigInmux31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0_ITRIG_ENA0")
            .field("dmac0_itrig_inmux0", &self.dmac0_itrig_inmux0())
            .field("dmac0_itrig_inmux1", &self.dmac0_itrig_inmux1())
            .field("dmac0_itrig_inmux2", &self.dmac0_itrig_inmux2())
            .field("dmac0_itrig_inmux3", &self.dmac0_itrig_inmux3())
            .field("dmac0_itrig_inmux4", &self.dmac0_itrig_inmux4())
            .field("dmac0_itrig_inmux5", &self.dmac0_itrig_inmux5())
            .field("dmac0_itrig_inmux6", &self.dmac0_itrig_inmux6())
            .field("dmac0_itrig_inmux7", &self.dmac0_itrig_inmux7())
            .field("dmac0_itrig_inmux8", &self.dmac0_itrig_inmux8())
            .field("dmac0_itrig_inmux9", &self.dmac0_itrig_inmux9())
            .field("dmac0_itrig_inmux10", &self.dmac0_itrig_inmux10())
            .field("dmac0_itrig_inmux11", &self.dmac0_itrig_inmux11())
            .field("dmac0_itrig_inmux12", &self.dmac0_itrig_inmux12())
            .field("dmac0_itrig_inmux13", &self.dmac0_itrig_inmux13())
            .field("dmac0_itrig_inmux14", &self.dmac0_itrig_inmux14())
            .field("dmac0_itrig_inmux15", &self.dmac0_itrig_inmux15())
            .field("dmac0_itrig_inmux16", &self.dmac0_itrig_inmux16())
            .field("dmac0_itrig_inmux17", &self.dmac0_itrig_inmux17())
            .field("dmac0_itrig_inmux18", &self.dmac0_itrig_inmux18())
            .field("dmac0_itrig_inmux19", &self.dmac0_itrig_inmux19())
            .field("dmac0_itrig_inmux20", &self.dmac0_itrig_inmux20())
            .field("dmac0_itrig_inmux21", &self.dmac0_itrig_inmux21())
            .field("dmac0_itrig_inmux22", &self.dmac0_itrig_inmux22())
            .field("dmac0_itrig_inmux23", &self.dmac0_itrig_inmux23())
            .field("dmac0_itrig_inmux24", &self.dmac0_itrig_inmux24())
            .field("dmac0_itrig_inmux25", &self.dmac0_itrig_inmux25())
            .field("dmac0_itrig_inmux26", &self.dmac0_itrig_inmux26())
            .field("dmac0_itrig_inmux27", &self.dmac0_itrig_inmux27())
            .field("dmac0_itrig_inmux28", &self.dmac0_itrig_inmux28())
            .field("dmac0_itrig_inmux29", &self.dmac0_itrig_inmux29())
            .field("dmac0_itrig_inmux30", &self.dmac0_itrig_inmux30())
            .field("dmac0_itrig_inmux31", &self.dmac0_itrig_inmux31())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMAC0 input trigger inmux 0 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux0(&mut self) -> Dmac0ItrigInmux0W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC0 input trigger inmux 1 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux1(&mut self) -> Dmac0ItrigInmux1W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC0 input trigger inmux 2 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux2(&mut self) -> Dmac0ItrigInmux2W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC0 input trigger inmux 3 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux3(&mut self) -> Dmac0ItrigInmux3W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC0 input trigger inmux 4 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux4(&mut self) -> Dmac0ItrigInmux4W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC0 input trigger inmux 5 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux5(&mut self) -> Dmac0ItrigInmux5W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC0 input trigger inmux 6 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux6(&mut self) -> Dmac0ItrigInmux6W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC0 input trigger inmux 7 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux7(&mut self) -> Dmac0ItrigInmux7W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC0 input trigger inmux 8 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux8(&mut self) -> Dmac0ItrigInmux8W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC0 input trigger inmux 9 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux9(&mut self) -> Dmac0ItrigInmux9W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC0 input trigger inmux 10 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux10(&mut self) -> Dmac0ItrigInmux10W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC0 input trigger inmux 11 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux11(&mut self) -> Dmac0ItrigInmux11W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC0 input trigger inmux 12 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux12(&mut self) -> Dmac0ItrigInmux12W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC0 input trigger inmux 13 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux13(&mut self) -> Dmac0ItrigInmux13W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC0 input trigger inmux 14 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux14(&mut self) -> Dmac0ItrigInmux14W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC0 input trigger inmux 15 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux15(&mut self) -> Dmac0ItrigInmux15W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMAC0 input trigger inmux 16 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux16(&mut self) -> Dmac0ItrigInmux16W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC0 input trigger inmux 17 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux17(&mut self) -> Dmac0ItrigInmux17W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC0 input trigger inmux 18 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux18(&mut self) -> Dmac0ItrigInmux18W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC0 input trigger inmux 19 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux19(&mut self) -> Dmac0ItrigInmux19W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC0 input trigger inmux 20 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux20(&mut self) -> Dmac0ItrigInmux20W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC0 input trigger inmux 21 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux21(&mut self) -> Dmac0ItrigInmux21W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC0 input trigger inmux 22 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux22(&mut self) -> Dmac0ItrigInmux22W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC0 input trigger inmux 23 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux23(&mut self) -> Dmac0ItrigInmux23W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC0 input trigger inmux 24 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux24(&mut self) -> Dmac0ItrigInmux24W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC0 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux25(&mut self) -> Dmac0ItrigInmux25W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC0 input trigger inmux 26 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux26(&mut self) -> Dmac0ItrigInmux26W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC0 input trigger inmux 27 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux27(&mut self) -> Dmac0ItrigInmux27W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC0 input trigger inmux 28 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux28(&mut self) -> Dmac0ItrigInmux28W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC0 input trigger inmux 29 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux29(&mut self) -> Dmac0ItrigInmux29W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC0 input trigger inmux 30 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux30(&mut self) -> Dmac0ItrigInmux30W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC0 input trigger inmux 31 enable"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux31(&mut self) -> Dmac0ItrigInmux31W<Dmac0ItrigEna0Spec> {
        Dmac0ItrigInmux31W::new(self, 31)
    }
}
#[doc = "DMAC0 input trigger enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_itrig_ena0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_ena0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ItrigEna0Spec;
impl crate::RegisterSpec for Dmac0ItrigEna0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac0_itrig_ena0::R`](R) reader structure"]
impl crate::Readable for Dmac0ItrigEna0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmac0_itrig_ena0::W`](W) writer structure"]
impl crate::Writable for Dmac0ItrigEna0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_ENA0 to value 0xffff_ffff"]
impl crate::Resettable for Dmac0ItrigEna0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
