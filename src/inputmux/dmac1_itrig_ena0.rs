#[doc = "Register `DMAC1_ITRIG_ENA0` reader"]
pub type R = crate::R<Dmac1ItrigEna0Spec>;
#[doc = "Register `DMAC1_ITRIG_ENA0` writer"]
pub type W = crate::W<Dmac1ItrigEna0Spec>;
#[doc = "DMAC1 input trigger inmux 0 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux0 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux0> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX0` reader - DMAC1 input trigger inmux 0 enable"]
pub type Dmac1ItrigInmux0R = crate::BitReader<Dmac1ItrigInmux0>;
impl Dmac1ItrigInmux0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux0 {
        match self.bits {
            false => Dmac1ItrigInmux0::Disabled,
            true => Dmac1ItrigInmux0::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux0::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX0` writer - DMAC1 input trigger inmux 0 enable"]
pub type Dmac1ItrigInmux0W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux0>;
impl<'a, REG> Dmac1ItrigInmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux0::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 1 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux1 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux1> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX1` reader - DMAC1 input trigger inmux 1 enable"]
pub type Dmac1ItrigInmux1R = crate::BitReader<Dmac1ItrigInmux1>;
impl Dmac1ItrigInmux1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux1 {
        match self.bits {
            false => Dmac1ItrigInmux1::Disabled,
            true => Dmac1ItrigInmux1::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux1::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX1` writer - DMAC1 input trigger inmux 1 enable"]
pub type Dmac1ItrigInmux1W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux1>;
impl<'a, REG> Dmac1ItrigInmux1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux1::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 2 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux2 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux2> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX2` reader - DMAC1 input trigger inmux 2 enable"]
pub type Dmac1ItrigInmux2R = crate::BitReader<Dmac1ItrigInmux2>;
impl Dmac1ItrigInmux2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux2 {
        match self.bits {
            false => Dmac1ItrigInmux2::Disabled,
            true => Dmac1ItrigInmux2::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux2::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX2` writer - DMAC1 input trigger inmux 2 enable"]
pub type Dmac1ItrigInmux2W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux2>;
impl<'a, REG> Dmac1ItrigInmux2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux2::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 3 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux3 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux3> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX3` reader - DMAC1 input trigger inmux 3 enable"]
pub type Dmac1ItrigInmux3R = crate::BitReader<Dmac1ItrigInmux3>;
impl Dmac1ItrigInmux3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux3 {
        match self.bits {
            false => Dmac1ItrigInmux3::Disabled,
            true => Dmac1ItrigInmux3::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux3::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX3` writer - DMAC1 input trigger inmux 3 enable"]
pub type Dmac1ItrigInmux3W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux3>;
impl<'a, REG> Dmac1ItrigInmux3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux3::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 4 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux4 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux4> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX4` reader - DMAC1 input trigger inmux 4 enable"]
pub type Dmac1ItrigInmux4R = crate::BitReader<Dmac1ItrigInmux4>;
impl Dmac1ItrigInmux4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux4 {
        match self.bits {
            false => Dmac1ItrigInmux4::Disabled,
            true => Dmac1ItrigInmux4::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux4::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX4` writer - DMAC1 input trigger inmux 4 enable"]
pub type Dmac1ItrigInmux4W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux4>;
impl<'a, REG> Dmac1ItrigInmux4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux4::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 5 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux5 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux5> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX5` reader - DMAC1 input trigger inmux 5 enable"]
pub type Dmac1ItrigInmux5R = crate::BitReader<Dmac1ItrigInmux5>;
impl Dmac1ItrigInmux5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux5 {
        match self.bits {
            false => Dmac1ItrigInmux5::Disabled,
            true => Dmac1ItrigInmux5::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux5::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux5::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX5` writer - DMAC1 input trigger inmux 5 enable"]
pub type Dmac1ItrigInmux5W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux5>;
impl<'a, REG> Dmac1ItrigInmux5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux5::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux5::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 6 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux6 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux6> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX6` reader - DMAC1 input trigger inmux 6 enable"]
pub type Dmac1ItrigInmux6R = crate::BitReader<Dmac1ItrigInmux6>;
impl Dmac1ItrigInmux6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux6 {
        match self.bits {
            false => Dmac1ItrigInmux6::Disabled,
            true => Dmac1ItrigInmux6::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux6::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux6::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX6` writer - DMAC1 input trigger inmux 6 enable"]
pub type Dmac1ItrigInmux6W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux6>;
impl<'a, REG> Dmac1ItrigInmux6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux6::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux6::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 7 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux7 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux7> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX7` reader - DMAC1 input trigger inmux 7 enable"]
pub type Dmac1ItrigInmux7R = crate::BitReader<Dmac1ItrigInmux7>;
impl Dmac1ItrigInmux7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux7 {
        match self.bits {
            false => Dmac1ItrigInmux7::Disabled,
            true => Dmac1ItrigInmux7::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux7::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux7::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX7` writer - DMAC1 input trigger inmux 7 enable"]
pub type Dmac1ItrigInmux7W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux7>;
impl<'a, REG> Dmac1ItrigInmux7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux7::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux7::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 8 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux8 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux8> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX8` reader - DMAC1 input trigger inmux 8 enable"]
pub type Dmac1ItrigInmux8R = crate::BitReader<Dmac1ItrigInmux8>;
impl Dmac1ItrigInmux8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux8 {
        match self.bits {
            false => Dmac1ItrigInmux8::Disabled,
            true => Dmac1ItrigInmux8::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux8::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux8::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX8` writer - DMAC1 input trigger inmux 8 enable"]
pub type Dmac1ItrigInmux8W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux8>;
impl<'a, REG> Dmac1ItrigInmux8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux8::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux8::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 9 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux9 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux9> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX9` reader - DMAC1 input trigger inmux 9 enable"]
pub type Dmac1ItrigInmux9R = crate::BitReader<Dmac1ItrigInmux9>;
impl Dmac1ItrigInmux9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux9 {
        match self.bits {
            false => Dmac1ItrigInmux9::Disabled,
            true => Dmac1ItrigInmux9::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux9::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux9::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX9` writer - DMAC1 input trigger inmux 9 enable"]
pub type Dmac1ItrigInmux9W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux9>;
impl<'a, REG> Dmac1ItrigInmux9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux9::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux9::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 10 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux10 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux10> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX10` reader - DMAC1 input trigger inmux 10 enable"]
pub type Dmac1ItrigInmux10R = crate::BitReader<Dmac1ItrigInmux10>;
impl Dmac1ItrigInmux10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux10 {
        match self.bits {
            false => Dmac1ItrigInmux10::Disabled,
            true => Dmac1ItrigInmux10::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux10::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux10::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX10` writer - DMAC1 input trigger inmux 10 enable"]
pub type Dmac1ItrigInmux10W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux10>;
impl<'a, REG> Dmac1ItrigInmux10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux10::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux10::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 11 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux11 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux11> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX11` reader - DMAC1 input trigger inmux 11 enable"]
pub type Dmac1ItrigInmux11R = crate::BitReader<Dmac1ItrigInmux11>;
impl Dmac1ItrigInmux11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux11 {
        match self.bits {
            false => Dmac1ItrigInmux11::Disabled,
            true => Dmac1ItrigInmux11::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux11::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux11::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX11` writer - DMAC1 input trigger inmux 11 enable"]
pub type Dmac1ItrigInmux11W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux11>;
impl<'a, REG> Dmac1ItrigInmux11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux11::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux11::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 12 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux12 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux12> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX12` reader - DMAC1 input trigger inmux 12 enable"]
pub type Dmac1ItrigInmux12R = crate::BitReader<Dmac1ItrigInmux12>;
impl Dmac1ItrigInmux12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux12 {
        match self.bits {
            false => Dmac1ItrigInmux12::Disabled,
            true => Dmac1ItrigInmux12::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux12::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux12::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX12` writer - DMAC1 input trigger inmux 12 enable"]
pub type Dmac1ItrigInmux12W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux12>;
impl<'a, REG> Dmac1ItrigInmux12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux12::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux12::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 13 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux13 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux13> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX13` reader - DMAC1 input trigger inmux 13 enable"]
pub type Dmac1ItrigInmux13R = crate::BitReader<Dmac1ItrigInmux13>;
impl Dmac1ItrigInmux13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux13 {
        match self.bits {
            false => Dmac1ItrigInmux13::Disabled,
            true => Dmac1ItrigInmux13::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux13::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux13::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX13` writer - DMAC1 input trigger inmux 13 enable"]
pub type Dmac1ItrigInmux13W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux13>;
impl<'a, REG> Dmac1ItrigInmux13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux13::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux13::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 14 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux14 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux14> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX14` reader - DMAC1 input trigger inmux 14 enable"]
pub type Dmac1ItrigInmux14R = crate::BitReader<Dmac1ItrigInmux14>;
impl Dmac1ItrigInmux14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux14 {
        match self.bits {
            false => Dmac1ItrigInmux14::Disabled,
            true => Dmac1ItrigInmux14::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux14::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux14::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX14` writer - DMAC1 input trigger inmux 14 enable"]
pub type Dmac1ItrigInmux14W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux14>;
impl<'a, REG> Dmac1ItrigInmux14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux14::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux14::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 15 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux15 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux15> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX15` reader - DMAC1 input trigger inmux 15 enable"]
pub type Dmac1ItrigInmux15R = crate::BitReader<Dmac1ItrigInmux15>;
impl Dmac1ItrigInmux15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux15 {
        match self.bits {
            false => Dmac1ItrigInmux15::Disabled,
            true => Dmac1ItrigInmux15::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux15::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux15::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX15` writer - DMAC1 input trigger inmux 15 enable"]
pub type Dmac1ItrigInmux15W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux15>;
impl<'a, REG> Dmac1ItrigInmux15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux15::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux15::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 16 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux16 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux16> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX16` reader - DMAC1 input trigger inmux 16 enable"]
pub type Dmac1ItrigInmux16R = crate::BitReader<Dmac1ItrigInmux16>;
impl Dmac1ItrigInmux16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux16 {
        match self.bits {
            false => Dmac1ItrigInmux16::Disabled,
            true => Dmac1ItrigInmux16::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux16::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux16::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX16` writer - DMAC1 input trigger inmux 16 enable"]
pub type Dmac1ItrigInmux16W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux16>;
impl<'a, REG> Dmac1ItrigInmux16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux16::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux16::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 17 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux17 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux17> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX17` reader - DMAC1 input trigger inmux 17 enable"]
pub type Dmac1ItrigInmux17R = crate::BitReader<Dmac1ItrigInmux17>;
impl Dmac1ItrigInmux17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux17 {
        match self.bits {
            false => Dmac1ItrigInmux17::Disabled,
            true => Dmac1ItrigInmux17::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux17::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux17::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX17` writer - DMAC1 input trigger inmux 17 enable"]
pub type Dmac1ItrigInmux17W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux17>;
impl<'a, REG> Dmac1ItrigInmux17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux17::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux17::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 18 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux18 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux18> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX18` reader - DMAC1 input trigger inmux 18 enable"]
pub type Dmac1ItrigInmux18R = crate::BitReader<Dmac1ItrigInmux18>;
impl Dmac1ItrigInmux18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux18 {
        match self.bits {
            false => Dmac1ItrigInmux18::Disabled,
            true => Dmac1ItrigInmux18::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux18::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux18::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX18` writer - DMAC1 input trigger inmux 18 enable"]
pub type Dmac1ItrigInmux18W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux18>;
impl<'a, REG> Dmac1ItrigInmux18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux18::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux18::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 19 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux19 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux19> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX19` reader - DMAC1 input trigger inmux 19 enable"]
pub type Dmac1ItrigInmux19R = crate::BitReader<Dmac1ItrigInmux19>;
impl Dmac1ItrigInmux19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux19 {
        match self.bits {
            false => Dmac1ItrigInmux19::Disabled,
            true => Dmac1ItrigInmux19::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux19::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux19::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX19` writer - DMAC1 input trigger inmux 19 enable"]
pub type Dmac1ItrigInmux19W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux19>;
impl<'a, REG> Dmac1ItrigInmux19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux19::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux19::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 20 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux20 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux20> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX20` reader - DMAC1 input trigger inmux 20 enable"]
pub type Dmac1ItrigInmux20R = crate::BitReader<Dmac1ItrigInmux20>;
impl Dmac1ItrigInmux20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux20 {
        match self.bits {
            false => Dmac1ItrigInmux20::Disabled,
            true => Dmac1ItrigInmux20::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux20::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux20::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX20` writer - DMAC1 input trigger inmux 20 enable"]
pub type Dmac1ItrigInmux20W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux20>;
impl<'a, REG> Dmac1ItrigInmux20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux20::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux20::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 21 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux21 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux21> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX21` reader - DMAC1 input trigger inmux 21 enable"]
pub type Dmac1ItrigInmux21R = crate::BitReader<Dmac1ItrigInmux21>;
impl Dmac1ItrigInmux21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux21 {
        match self.bits {
            false => Dmac1ItrigInmux21::Disabled,
            true => Dmac1ItrigInmux21::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux21::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux21::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX21` writer - DMAC1 input trigger inmux 21 enable"]
pub type Dmac1ItrigInmux21W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux21>;
impl<'a, REG> Dmac1ItrigInmux21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux21::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux21::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 22 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux22 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux22> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX22` reader - DMAC1 input trigger inmux 22 enable"]
pub type Dmac1ItrigInmux22R = crate::BitReader<Dmac1ItrigInmux22>;
impl Dmac1ItrigInmux22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux22 {
        match self.bits {
            false => Dmac1ItrigInmux22::Disabled,
            true => Dmac1ItrigInmux22::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux22::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux22::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX22` writer - DMAC1 input trigger inmux 22 enable"]
pub type Dmac1ItrigInmux22W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux22>;
impl<'a, REG> Dmac1ItrigInmux22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux22::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux22::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 23 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux23 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux23> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX23` reader - DMAC1 input trigger inmux 23 enable"]
pub type Dmac1ItrigInmux23R = crate::BitReader<Dmac1ItrigInmux23>;
impl Dmac1ItrigInmux23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux23 {
        match self.bits {
            false => Dmac1ItrigInmux23::Disabled,
            true => Dmac1ItrigInmux23::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux23::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux23::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX23` writer - DMAC1 input trigger inmux 23 enable"]
pub type Dmac1ItrigInmux23W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux23>;
impl<'a, REG> Dmac1ItrigInmux23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux23::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux23::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 24 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux24 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux24> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX24` reader - DMAC1 input trigger inmux 24 enable"]
pub type Dmac1ItrigInmux24R = crate::BitReader<Dmac1ItrigInmux24>;
impl Dmac1ItrigInmux24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux24 {
        match self.bits {
            false => Dmac1ItrigInmux24::Disabled,
            true => Dmac1ItrigInmux24::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux24::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux24::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX24` writer - DMAC1 input trigger inmux 24 enable"]
pub type Dmac1ItrigInmux24W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux24>;
impl<'a, REG> Dmac1ItrigInmux24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux24::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux24::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux25 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux25> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX25` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux25R = crate::BitReader<Dmac1ItrigInmux25>;
impl Dmac1ItrigInmux25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux25 {
        match self.bits {
            false => Dmac1ItrigInmux25::Disabled,
            true => Dmac1ItrigInmux25::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux25::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux25::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX25` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux25W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux25>;
impl<'a, REG> Dmac1ItrigInmux25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux25::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux25::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux26 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux26> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX26` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux26R = crate::BitReader<Dmac1ItrigInmux26>;
impl Dmac1ItrigInmux26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux26 {
        match self.bits {
            false => Dmac1ItrigInmux26::Disabled,
            true => Dmac1ItrigInmux26::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux26::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux26::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX26` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux26W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux26>;
impl<'a, REG> Dmac1ItrigInmux26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux26::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux26::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux27 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux27> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX27` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux27R = crate::BitReader<Dmac1ItrigInmux27>;
impl Dmac1ItrigInmux27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux27 {
        match self.bits {
            false => Dmac1ItrigInmux27::Disabled,
            true => Dmac1ItrigInmux27::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux27::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux27::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX27` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux27W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux27>;
impl<'a, REG> Dmac1ItrigInmux27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux27::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux27::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux28 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux28> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX28` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux28R = crate::BitReader<Dmac1ItrigInmux28>;
impl Dmac1ItrigInmux28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux28 {
        match self.bits {
            false => Dmac1ItrigInmux28::Disabled,
            true => Dmac1ItrigInmux28::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux28::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux28::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX28` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux28W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux28>;
impl<'a, REG> Dmac1ItrigInmux28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux28::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux28::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux29 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux29> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX29` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux29R = crate::BitReader<Dmac1ItrigInmux29>;
impl Dmac1ItrigInmux29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux29 {
        match self.bits {
            false => Dmac1ItrigInmux29::Disabled,
            true => Dmac1ItrigInmux29::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux29::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux29::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX29` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux29W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux29>;
impl<'a, REG> Dmac1ItrigInmux29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux29::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux29::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux30 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux30> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX30` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux30R = crate::BitReader<Dmac1ItrigInmux30>;
impl Dmac1ItrigInmux30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux30 {
        match self.bits {
            false => Dmac1ItrigInmux30::Disabled,
            true => Dmac1ItrigInmux30::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux30::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux30::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX30` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux30W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux30>;
impl<'a, REG> Dmac1ItrigInmux30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux30::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux30::Enabled)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux31 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1ItrigInmux31> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX31` reader - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux31R = crate::BitReader<Dmac1ItrigInmux31>;
impl Dmac1ItrigInmux31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1ItrigInmux31 {
        match self.bits {
            false => Dmac1ItrigInmux31::Disabled,
            true => Dmac1ItrigInmux31::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1ItrigInmux31::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1ItrigInmux31::Enabled
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX31` writer - DMAC1 input trigger inmux 25 enable"]
pub type Dmac1ItrigInmux31W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux31>;
impl<'a, REG> Dmac1ItrigInmux31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux31::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux31::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC1 input trigger inmux 0 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux0(&self) -> Dmac1ItrigInmux0R {
        Dmac1ItrigInmux0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC1 input trigger inmux 1 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux1(&self) -> Dmac1ItrigInmux1R {
        Dmac1ItrigInmux1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC1 input trigger inmux 2 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux2(&self) -> Dmac1ItrigInmux2R {
        Dmac1ItrigInmux2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC1 input trigger inmux 3 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux3(&self) -> Dmac1ItrigInmux3R {
        Dmac1ItrigInmux3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC1 input trigger inmux 4 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux4(&self) -> Dmac1ItrigInmux4R {
        Dmac1ItrigInmux4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC1 input trigger inmux 5 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux5(&self) -> Dmac1ItrigInmux5R {
        Dmac1ItrigInmux5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC1 input trigger inmux 6 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux6(&self) -> Dmac1ItrigInmux6R {
        Dmac1ItrigInmux6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC1 input trigger inmux 7 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux7(&self) -> Dmac1ItrigInmux7R {
        Dmac1ItrigInmux7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC1 input trigger inmux 8 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux8(&self) -> Dmac1ItrigInmux8R {
        Dmac1ItrigInmux8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC1 input trigger inmux 9 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux9(&self) -> Dmac1ItrigInmux9R {
        Dmac1ItrigInmux9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAC1 input trigger inmux 10 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux10(&self) -> Dmac1ItrigInmux10R {
        Dmac1ItrigInmux10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC1 input trigger inmux 11 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux11(&self) -> Dmac1ItrigInmux11R {
        Dmac1ItrigInmux11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAC1 input trigger inmux 12 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux12(&self) -> Dmac1ItrigInmux12R {
        Dmac1ItrigInmux12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAC1 input trigger inmux 13 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux13(&self) -> Dmac1ItrigInmux13R {
        Dmac1ItrigInmux13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMAC1 input trigger inmux 14 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux14(&self) -> Dmac1ItrigInmux14R {
        Dmac1ItrigInmux14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAC1 input trigger inmux 15 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux15(&self) -> Dmac1ItrigInmux15R {
        Dmac1ItrigInmux15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAC1 input trigger inmux 16 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux16(&self) -> Dmac1ItrigInmux16R {
        Dmac1ItrigInmux16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAC1 input trigger inmux 17 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux17(&self) -> Dmac1ItrigInmux17R {
        Dmac1ItrigInmux17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAC1 input trigger inmux 18 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux18(&self) -> Dmac1ItrigInmux18R {
        Dmac1ItrigInmux18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMAC1 input trigger inmux 19 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux19(&self) -> Dmac1ItrigInmux19R {
        Dmac1ItrigInmux19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMAC1 input trigger inmux 20 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux20(&self) -> Dmac1ItrigInmux20R {
        Dmac1ItrigInmux20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMAC1 input trigger inmux 21 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux21(&self) -> Dmac1ItrigInmux21R {
        Dmac1ItrigInmux21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMAC1 input trigger inmux 22 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux22(&self) -> Dmac1ItrigInmux22R {
        Dmac1ItrigInmux22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC1 input trigger inmux 23 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux23(&self) -> Dmac1ItrigInmux23R {
        Dmac1ItrigInmux23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC1 input trigger inmux 24 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux24(&self) -> Dmac1ItrigInmux24R {
        Dmac1ItrigInmux24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux25(&self) -> Dmac1ItrigInmux25R {
        Dmac1ItrigInmux25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux26(&self) -> Dmac1ItrigInmux26R {
        Dmac1ItrigInmux26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux27(&self) -> Dmac1ItrigInmux27R {
        Dmac1ItrigInmux27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux28(&self) -> Dmac1ItrigInmux28R {
        Dmac1ItrigInmux28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux29(&self) -> Dmac1ItrigInmux29R {
        Dmac1ItrigInmux29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux30(&self) -> Dmac1ItrigInmux30R {
        Dmac1ItrigInmux30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux31(&self) -> Dmac1ItrigInmux31R {
        Dmac1ItrigInmux31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1_ITRIG_ENA0")
            .field("dmac1_itrig_inmux0", &self.dmac1_itrig_inmux0())
            .field("dmac1_itrig_inmux1", &self.dmac1_itrig_inmux1())
            .field("dmac1_itrig_inmux2", &self.dmac1_itrig_inmux2())
            .field("dmac1_itrig_inmux3", &self.dmac1_itrig_inmux3())
            .field("dmac1_itrig_inmux4", &self.dmac1_itrig_inmux4())
            .field("dmac1_itrig_inmux5", &self.dmac1_itrig_inmux5())
            .field("dmac1_itrig_inmux6", &self.dmac1_itrig_inmux6())
            .field("dmac1_itrig_inmux7", &self.dmac1_itrig_inmux7())
            .field("dmac1_itrig_inmux8", &self.dmac1_itrig_inmux8())
            .field("dmac1_itrig_inmux9", &self.dmac1_itrig_inmux9())
            .field("dmac1_itrig_inmux10", &self.dmac1_itrig_inmux10())
            .field("dmac1_itrig_inmux11", &self.dmac1_itrig_inmux11())
            .field("dmac1_itrig_inmux12", &self.dmac1_itrig_inmux12())
            .field("dmac1_itrig_inmux13", &self.dmac1_itrig_inmux13())
            .field("dmac1_itrig_inmux14", &self.dmac1_itrig_inmux14())
            .field("dmac1_itrig_inmux15", &self.dmac1_itrig_inmux15())
            .field("dmac1_itrig_inmux16", &self.dmac1_itrig_inmux16())
            .field("dmac1_itrig_inmux17", &self.dmac1_itrig_inmux17())
            .field("dmac1_itrig_inmux18", &self.dmac1_itrig_inmux18())
            .field("dmac1_itrig_inmux19", &self.dmac1_itrig_inmux19())
            .field("dmac1_itrig_inmux20", &self.dmac1_itrig_inmux20())
            .field("dmac1_itrig_inmux21", &self.dmac1_itrig_inmux21())
            .field("dmac1_itrig_inmux22", &self.dmac1_itrig_inmux22())
            .field("dmac1_itrig_inmux23", &self.dmac1_itrig_inmux23())
            .field("dmac1_itrig_inmux24", &self.dmac1_itrig_inmux24())
            .field("dmac1_itrig_inmux25", &self.dmac1_itrig_inmux25())
            .field("dmac1_itrig_inmux26", &self.dmac1_itrig_inmux26())
            .field("dmac1_itrig_inmux27", &self.dmac1_itrig_inmux27())
            .field("dmac1_itrig_inmux28", &self.dmac1_itrig_inmux28())
            .field("dmac1_itrig_inmux29", &self.dmac1_itrig_inmux29())
            .field("dmac1_itrig_inmux30", &self.dmac1_itrig_inmux30())
            .field("dmac1_itrig_inmux31", &self.dmac1_itrig_inmux31())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMAC1 input trigger inmux 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux0(&mut self) -> Dmac1ItrigInmux0W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC1 input trigger inmux 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux1(&mut self) -> Dmac1ItrigInmux1W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC1 input trigger inmux 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux2(&mut self) -> Dmac1ItrigInmux2W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC1 input trigger inmux 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux3(&mut self) -> Dmac1ItrigInmux3W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC1 input trigger inmux 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux4(&mut self) -> Dmac1ItrigInmux4W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC1 input trigger inmux 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux5(&mut self) -> Dmac1ItrigInmux5W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC1 input trigger inmux 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux6(&mut self) -> Dmac1ItrigInmux6W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC1 input trigger inmux 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux7(&mut self) -> Dmac1ItrigInmux7W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC1 input trigger inmux 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux8(&mut self) -> Dmac1ItrigInmux8W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC1 input trigger inmux 9 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux9(&mut self) -> Dmac1ItrigInmux9W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC1 input trigger inmux 10 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux10(&mut self) -> Dmac1ItrigInmux10W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC1 input trigger inmux 11 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux11(&mut self) -> Dmac1ItrigInmux11W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC1 input trigger inmux 12 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux12(&mut self) -> Dmac1ItrigInmux12W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC1 input trigger inmux 13 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux13(&mut self) -> Dmac1ItrigInmux13W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC1 input trigger inmux 14 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux14(&mut self) -> Dmac1ItrigInmux14W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC1 input trigger inmux 15 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux15(&mut self) -> Dmac1ItrigInmux15W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMAC1 input trigger inmux 16 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux16(&mut self) -> Dmac1ItrigInmux16W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC1 input trigger inmux 17 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux17(&mut self) -> Dmac1ItrigInmux17W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC1 input trigger inmux 18 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux18(&mut self) -> Dmac1ItrigInmux18W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC1 input trigger inmux 19 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux19(&mut self) -> Dmac1ItrigInmux19W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC1 input trigger inmux 20 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux20(&mut self) -> Dmac1ItrigInmux20W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC1 input trigger inmux 21 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux21(&mut self) -> Dmac1ItrigInmux21W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC1 input trigger inmux 22 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux22(&mut self) -> Dmac1ItrigInmux22W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC1 input trigger inmux 23 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux23(&mut self) -> Dmac1ItrigInmux23W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 input trigger inmux 24 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux24(&mut self) -> Dmac1ItrigInmux24W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux25(&mut self) -> Dmac1ItrigInmux25W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux26(&mut self) -> Dmac1ItrigInmux26W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux27(&mut self) -> Dmac1ItrigInmux27W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux28(&mut self) -> Dmac1ItrigInmux28W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux29(&mut self) -> Dmac1ItrigInmux29W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux30(&mut self) -> Dmac1ItrigInmux30W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_itrig_inmux31(&mut self) -> Dmac1ItrigInmux31W<Dmac1ItrigEna0Spec> {
        Dmac1ItrigInmux31W::new(self, 31)
    }
}
#[doc = "DMAC1 input trigger enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_itrig_ena0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_ena0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1ItrigEna0Spec;
impl crate::RegisterSpec for Dmac1ItrigEna0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac1_itrig_ena0::R`](R) reader structure"]
impl crate::Readable for Dmac1ItrigEna0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmac1_itrig_ena0::W`](W) writer structure"]
impl crate::Writable for Dmac1ItrigEna0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_ITRIG_ENA0 to value 0xffff_ffff"]
impl crate::Resettable for Dmac1ItrigEna0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
