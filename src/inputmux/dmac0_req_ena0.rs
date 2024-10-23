#[doc = "Register `DMAC0_REQ_ENA0` reader"]
pub type R = crate::R<Dmac0ReqEna0Spec>;
#[doc = "Register `DMAC0_REQ_ENA0` writer"]
pub type W = crate::W<Dmac0ReqEna0Spec>;
#[doc = "FLEXCOMM0 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm0Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_RX` reader - FLEXCOMM0 RX enable"]
pub type Flexcomm0RxR = crate::BitReader<Flexcomm0Rx>;
impl Flexcomm0RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0Rx {
        match self.bits {
            false => Flexcomm0Rx::Disabled,
            true => Flexcomm0Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm0Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm0Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM0_RX` writer - FLEXCOMM0 RX enable"]
pub type Flexcomm0RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Rx>;
impl<'a, REG> Flexcomm0RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rx::Enabled)
    }
}
#[doc = "FLEXCOMM0 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm0Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_TX` reader - FLEXCOMM0 TX enable"]
pub type Flexcomm0TxR = crate::BitReader<Flexcomm0Tx>;
impl Flexcomm0TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0Tx {
        match self.bits {
            false => Flexcomm0Tx::Disabled,
            true => Flexcomm0Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm0Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm0Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM0_TX` writer - FLEXCOMM0 TX enable"]
pub type Flexcomm0TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Tx>;
impl<'a, REG> Flexcomm0TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Tx::Enabled)
    }
}
#[doc = "FLEXCOMM1 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm1Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_RX` reader - FLEXCOMM1 RX enable"]
pub type Flexcomm1RxR = crate::BitReader<Flexcomm1Rx>;
impl Flexcomm1RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1Rx {
        match self.bits {
            false => Flexcomm1Rx::Disabled,
            true => Flexcomm1Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm1Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm1Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM1_RX` writer - FLEXCOMM1 RX enable"]
pub type Flexcomm1RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Rx>;
impl<'a, REG> Flexcomm1RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rx::Enabled)
    }
}
#[doc = "FLEXCOMM1 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm1Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_TX` reader - FLEXCOMM1 TX enable"]
pub type Flexcomm1TxR = crate::BitReader<Flexcomm1Tx>;
impl Flexcomm1TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1Tx {
        match self.bits {
            false => Flexcomm1Tx::Disabled,
            true => Flexcomm1Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm1Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm1Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM1_TX` writer - FLEXCOMM1 TX enable"]
pub type Flexcomm1TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Tx>;
impl<'a, REG> Flexcomm1TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Tx::Enabled)
    }
}
#[doc = "FLEXCOMM2 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm2Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_RX` reader - FLEXCOMM2 RX enable"]
pub type Flexcomm2RxR = crate::BitReader<Flexcomm2Rx>;
impl Flexcomm2RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2Rx {
        match self.bits {
            false => Flexcomm2Rx::Disabled,
            true => Flexcomm2Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm2Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm2Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM2_RX` writer - FLEXCOMM2 RX enable"]
pub type Flexcomm2RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Rx>;
impl<'a, REG> Flexcomm2RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rx::Enabled)
    }
}
#[doc = "FLEXCOMM2 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm2Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_TX` reader - FLEXCOMM2 TX enable"]
pub type Flexcomm2TxR = crate::BitReader<Flexcomm2Tx>;
impl Flexcomm2TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2Tx {
        match self.bits {
            false => Flexcomm2Tx::Disabled,
            true => Flexcomm2Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm2Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm2Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM2_TX` writer - FLEXCOMM2 TX enable"]
pub type Flexcomm2TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Tx>;
impl<'a, REG> Flexcomm2TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Tx::Enabled)
    }
}
#[doc = "FLEXCOMM3 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm3Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_RX` reader - FLEXCOMM3 RX enable"]
pub type Flexcomm3RxR = crate::BitReader<Flexcomm3Rx>;
impl Flexcomm3RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3Rx {
        match self.bits {
            false => Flexcomm3Rx::Disabled,
            true => Flexcomm3Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm3Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm3Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM3_RX` writer - FLEXCOMM3 RX enable"]
pub type Flexcomm3RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Rx>;
impl<'a, REG> Flexcomm3RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rx::Enabled)
    }
}
#[doc = "FLEXCOMM3 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm3Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_TX` reader - FLEXCOMM3 TX enable"]
pub type Flexcomm3TxR = crate::BitReader<Flexcomm3Tx>;
impl Flexcomm3TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3Tx {
        match self.bits {
            false => Flexcomm3Tx::Disabled,
            true => Flexcomm3Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm3Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm3Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM3_TX` writer - FLEXCOMM3 TX enable"]
pub type Flexcomm3TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Tx>;
impl<'a, REG> Flexcomm3TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Tx::Enabled)
    }
}
#[doc = "FLEXCOMM4 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm4Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_RX` reader - FLEXCOMM4 RX enable"]
pub type Flexcomm4RxR = crate::BitReader<Flexcomm4Rx>;
impl Flexcomm4RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4Rx {
        match self.bits {
            false => Flexcomm4Rx::Disabled,
            true => Flexcomm4Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm4Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm4Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM4_RX` writer - FLEXCOMM4 RX enable"]
pub type Flexcomm4RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Rx>;
impl<'a, REG> Flexcomm4RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rx::Enabled)
    }
}
#[doc = "FLEXCOMM4 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm4Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_TX` reader - FLEXCOMM4 TX enable"]
pub type Flexcomm4TxR = crate::BitReader<Flexcomm4Tx>;
impl Flexcomm4TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4Tx {
        match self.bits {
            false => Flexcomm4Tx::Disabled,
            true => Flexcomm4Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm4Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm4Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM4_TX` writer - FLEXCOMM4 TX enable"]
pub type Flexcomm4TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Tx>;
impl<'a, REG> Flexcomm4TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Tx::Enabled)
    }
}
#[doc = "FLEXCOMM5 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm5Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_RX` reader - FLEXCOMM5 RX enable"]
pub type Flexcomm5RxR = crate::BitReader<Flexcomm5Rx>;
impl Flexcomm5RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5Rx {
        match self.bits {
            false => Flexcomm5Rx::Disabled,
            true => Flexcomm5Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm5Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm5Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM5_RX` writer - FLEXCOMM5 RX enable"]
pub type Flexcomm5RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Rx>;
impl<'a, REG> Flexcomm5RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rx::Enabled)
    }
}
#[doc = "FLEXCOMM5 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm5Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_TX` reader - FLEXCOMM5 TX enable"]
pub type Flexcomm5TxR = crate::BitReader<Flexcomm5Tx>;
impl Flexcomm5TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5Tx {
        match self.bits {
            false => Flexcomm5Tx::Disabled,
            true => Flexcomm5Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm5Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm5Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM5_TX` writer - FLEXCOMM5 TX enable"]
pub type Flexcomm5TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Tx>;
impl<'a, REG> Flexcomm5TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Tx::Enabled)
    }
}
#[doc = "FLEXCOMM6 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm6Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_RX` reader - FLEXCOMM6 RX enable"]
pub type Flexcomm6RxR = crate::BitReader<Flexcomm6Rx>;
impl Flexcomm6RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6Rx {
        match self.bits {
            false => Flexcomm6Rx::Disabled,
            true => Flexcomm6Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm6Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm6Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM6_RX` writer - FLEXCOMM6 RX enable"]
pub type Flexcomm6RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Rx>;
impl<'a, REG> Flexcomm6RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rx::Enabled)
    }
}
#[doc = "FLEXCOMM6 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm6Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_TX` reader - FLEXCOMM6 TX enable"]
pub type Flexcomm6TxR = crate::BitReader<Flexcomm6Tx>;
impl Flexcomm6TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6Tx {
        match self.bits {
            false => Flexcomm6Tx::Disabled,
            true => Flexcomm6Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm6Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm6Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM6_TX` writer - FLEXCOMM6 TX enable"]
pub type Flexcomm6TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Tx>;
impl<'a, REG> Flexcomm6TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Tx::Enabled)
    }
}
#[doc = "FLEXCOMM7 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm7Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_RX` reader - FLEXCOMM7 RX enable"]
pub type Flexcomm7RxR = crate::BitReader<Flexcomm7Rx>;
impl Flexcomm7RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7Rx {
        match self.bits {
            false => Flexcomm7Rx::Disabled,
            true => Flexcomm7Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm7Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm7Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM7_RX` writer - FLEXCOMM7 RX enable"]
pub type Flexcomm7RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Rx>;
impl<'a, REG> Flexcomm7RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rx::Enabled)
    }
}
#[doc = "FLEXCOMM7 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm7Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_TX` reader - FLEXCOMM7 TX enable"]
pub type Flexcomm7TxR = crate::BitReader<Flexcomm7Tx>;
impl Flexcomm7TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7Tx {
        match self.bits {
            false => Flexcomm7Tx::Disabled,
            true => Flexcomm7Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm7Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm7Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM7_TX` writer - FLEXCOMM7 TX enable"]
pub type Flexcomm7TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Tx>;
impl<'a, REG> Flexcomm7TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Tx::Enabled)
    }
}
#[doc = "DMIC0 channel 0 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch0 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch0> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH0` reader - DMIC0 channel 0 enable"]
pub type Dmic0ch0R = crate::BitReader<Dmic0ch0>;
impl Dmic0ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch0 {
        match self.bits {
            false => Dmic0ch0::Disabled,
            true => Dmic0ch0::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch0::Enabled
    }
}
#[doc = "Field `DMIC0CH0` writer - DMIC0 channel 0 enable"]
pub type Dmic0ch0W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch0>;
impl<'a, REG> Dmic0ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch0::Enabled)
    }
}
#[doc = "DMIC0 channel 1 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch1 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch1> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH1` reader - DMIC0 channel 1 enable"]
pub type Dmic0ch1R = crate::BitReader<Dmic0ch1>;
impl Dmic0ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch1 {
        match self.bits {
            false => Dmic0ch1::Disabled,
            true => Dmic0ch1::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch1::Enabled
    }
}
#[doc = "Field `DMIC0CH1` writer - DMIC0 channel 1 enable"]
pub type Dmic0ch1W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch1>;
impl<'a, REG> Dmic0ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch1::Enabled)
    }
}
#[doc = "DMIC0 channel 2 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch2 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch2> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH2` reader - DMIC0 channel 2 enable"]
pub type Dmic0ch2R = crate::BitReader<Dmic0ch2>;
impl Dmic0ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch2 {
        match self.bits {
            false => Dmic0ch2::Disabled,
            true => Dmic0ch2::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch2::Enabled
    }
}
#[doc = "Field `DMIC0CH2` writer - DMIC0 channel 2 enable"]
pub type Dmic0ch2W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch2>;
impl<'a, REG> Dmic0ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch2::Enabled)
    }
}
#[doc = "DMIC0 channel 3 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch3 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch3> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH3` reader - DMIC0 channel 3 enable"]
pub type Dmic0ch3R = crate::BitReader<Dmic0ch3>;
impl Dmic0ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch3 {
        match self.bits {
            false => Dmic0ch3::Disabled,
            true => Dmic0ch3::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch3::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch3::Enabled
    }
}
#[doc = "Field `DMIC0CH3` writer - DMIC0 channel 3 enable"]
pub type Dmic0ch3W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch3>;
impl<'a, REG> Dmic0ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch3::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch3::Enabled)
    }
}
#[doc = "DMIC0 channel 4 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch4 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch4> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH4` reader - DMIC0 channel 4 enable"]
pub type Dmic0ch4R = crate::BitReader<Dmic0ch4>;
impl Dmic0ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch4 {
        match self.bits {
            false => Dmic0ch4::Disabled,
            true => Dmic0ch4::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch4::Enabled
    }
}
#[doc = "Field `DMIC0CH4` writer - DMIC0 channel 4 enable"]
pub type Dmic0ch4W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch4>;
impl<'a, REG> Dmic0ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch4::Enabled)
    }
}
#[doc = "DMIC0 channel 5 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch5 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch5> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH5` reader - DMIC0 channel 5 enable"]
pub type Dmic0ch5R = crate::BitReader<Dmic0ch5>;
impl Dmic0ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch5 {
        match self.bits {
            false => Dmic0ch5::Disabled,
            true => Dmic0ch5::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch5::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch5::Enabled
    }
}
#[doc = "Field `DMIC0CH5` writer - DMIC0 channel 5 enable"]
pub type Dmic0ch5W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch5>;
impl<'a, REG> Dmic0ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch5::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch5::Enabled)
    }
}
#[doc = "DMIC0 channel 6 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch6 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch6> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH6` reader - DMIC0 channel 6 enable"]
pub type Dmic0ch6R = crate::BitReader<Dmic0ch6>;
impl Dmic0ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch6 {
        match self.bits {
            false => Dmic0ch6::Disabled,
            true => Dmic0ch6::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch6::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch6::Enabled
    }
}
#[doc = "Field `DMIC0CH6` writer - DMIC0 channel 6 enable"]
pub type Dmic0ch6W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch6>;
impl<'a, REG> Dmic0ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch6::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch6::Enabled)
    }
}
#[doc = "DMIC0 channel 7 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch7 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmic0ch7> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH7` reader - DMIC0 channel 7 enable"]
pub type Dmic0ch7R = crate::BitReader<Dmic0ch7>;
impl Dmic0ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0ch7 {
        match self.bits {
            false => Dmic0ch7::Disabled,
            true => Dmic0ch7::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmic0ch7::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmic0ch7::Enabled
    }
}
#[doc = "Field `DMIC0CH7` writer - DMIC0 channel 7 enable"]
pub type Dmic0ch7W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch7>;
impl<'a, REG> Dmic0ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch7::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch7::Enabled)
    }
}
#[doc = "I3C RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<I3c0Rx> for bool {
    #[inline(always)]
    fn from(variant: I3c0Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_RX` reader - I3C RX enable"]
pub type I3c0RxR = crate::BitReader<I3c0Rx>;
impl I3c0RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3c0Rx {
        match self.bits {
            false => I3c0Rx::Disabled,
            true => I3c0Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I3c0Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I3c0Rx::Enabled
    }
}
#[doc = "Field `I3C0_RX` writer - I3C RX enable"]
pub type I3c0RxW<'a, REG> = crate::BitWriter<'a, REG, I3c0Rx>;
impl<'a, REG> I3c0RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rx::Enabled)
    }
}
#[doc = "I3C TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<I3c0Tx> for bool {
    #[inline(always)]
    fn from(variant: I3c0Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_TX` reader - I3C TX enable"]
pub type I3c0TxR = crate::BitReader<I3c0Tx>;
impl I3c0TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3c0Tx {
        match self.bits {
            false => I3c0Tx::Disabled,
            true => I3c0Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I3c0Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I3c0Tx::Enabled
    }
}
#[doc = "Field `I3C0_TX` writer - I3C TX enable"]
pub type I3c0TxW<'a, REG> = crate::BitWriter<'a, REG, I3c0Tx>;
impl<'a, REG> I3c0TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Tx::Enabled)
    }
}
#[doc = "FLEXCOMM14 RX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14Rx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm14Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_RX` reader - FLEXCOMM14 RX enable"]
pub type Flexcomm14RxR = crate::BitReader<Flexcomm14Rx>;
impl Flexcomm14RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm14Rx {
        match self.bits {
            false => Flexcomm14Rx::Disabled,
            true => Flexcomm14Rx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm14Rx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm14Rx::Enabled
    }
}
#[doc = "Field `FLEXCOMM14_RX` writer - FLEXCOMM14 RX enable"]
pub type Flexcomm14RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14Rx>;
impl<'a, REG> Flexcomm14RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Rx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Rx::Enabled)
    }
}
#[doc = "FLEXCOMM14 TX enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14Tx {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm14Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_TX` reader - FLEXCOMM14 TX enable"]
pub type Flexcomm14TxR = crate::BitReader<Flexcomm14Tx>;
impl Flexcomm14TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm14Tx {
        match self.bits {
            false => Flexcomm14Tx::Disabled,
            true => Flexcomm14Tx::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm14Tx::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm14Tx::Enabled
    }
}
#[doc = "Field `FLEXCOMM14_TX` writer - FLEXCOMM14 TX enable"]
pub type Flexcomm14TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14Tx>;
impl<'a, REG> Flexcomm14TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Tx::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Tx::Enabled)
    }
}
#[doc = "hash enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashcrypt {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Hashcrypt> for bool {
    #[inline(always)]
    fn from(variant: Hashcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` reader - hash enable"]
pub type HashcryptR = crate::BitReader<Hashcrypt>;
impl HashcryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashcrypt {
        match self.bits {
            false => Hashcrypt::Disabled,
            true => Hashcrypt::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hashcrypt::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hashcrypt::Enabled
    }
}
#[doc = "Field `HASHCRYPT` writer - hash enable"]
pub type HashcryptW<'a, REG> = crate::BitWriter<'a, REG, Hashcrypt>;
impl<'a, REG> HashcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::Enabled)
    }
}
#[doc = "ESPI Channel 0 enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EspiCh0 {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<EspiCh0> for bool {
    #[inline(always)]
    fn from(variant: EspiCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESPI_CH0` reader - ESPI Channel 0 enable"]
pub type EspiCh0R = crate::BitReader<EspiCh0>;
impl EspiCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EspiCh0 {
        match self.bits {
            false => EspiCh0::Disabled,
            true => EspiCh0::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EspiCh0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EspiCh0::Enabled
    }
}
#[doc = "Field `ESPI_CH0` writer - ESPI Channel 0 enable"]
pub type EspiCh0W<'a, REG> = crate::BitWriter<'a, REG, EspiCh0>;
impl<'a, REG> EspiCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh0::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub fn flexcomm0_rx(&self) -> Flexcomm0RxR {
        Flexcomm0RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub fn flexcomm0_tx(&self) -> Flexcomm0TxR {
        Flexcomm0TxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub fn flexcomm1_rx(&self) -> Flexcomm1RxR {
        Flexcomm1RxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub fn flexcomm1_tx(&self) -> Flexcomm1TxR {
        Flexcomm1TxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub fn flexcomm2_rx(&self) -> Flexcomm2RxR {
        Flexcomm2RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub fn flexcomm2_tx(&self) -> Flexcomm2TxR {
        Flexcomm2TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub fn flexcomm3_rx(&self) -> Flexcomm3RxR {
        Flexcomm3RxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub fn flexcomm3_tx(&self) -> Flexcomm3TxR {
        Flexcomm3TxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub fn flexcomm4_rx(&self) -> Flexcomm4RxR {
        Flexcomm4RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub fn flexcomm4_tx(&self) -> Flexcomm4TxR {
        Flexcomm4TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub fn flexcomm5_rx(&self) -> Flexcomm5RxR {
        Flexcomm5RxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub fn flexcomm5_tx(&self) -> Flexcomm5TxR {
        Flexcomm5TxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub fn flexcomm6_rx(&self) -> Flexcomm6RxR {
        Flexcomm6RxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub fn flexcomm6_tx(&self) -> Flexcomm6TxR {
        Flexcomm6TxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub fn flexcomm7_rx(&self) -> Flexcomm7RxR {
        Flexcomm7RxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub fn flexcomm7_tx(&self) -> Flexcomm7TxR {
        Flexcomm7TxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMIC0 channel 0 enable"]
    #[inline(always)]
    pub fn dmic0ch0(&self) -> Dmic0ch0R {
        Dmic0ch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMIC0 channel 1 enable"]
    #[inline(always)]
    pub fn dmic0ch1(&self) -> Dmic0ch1R {
        Dmic0ch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMIC0 channel 2 enable"]
    #[inline(always)]
    pub fn dmic0ch2(&self) -> Dmic0ch2R {
        Dmic0ch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMIC0 channel 3 enable"]
    #[inline(always)]
    pub fn dmic0ch3(&self) -> Dmic0ch3R {
        Dmic0ch3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMIC0 channel 4 enable"]
    #[inline(always)]
    pub fn dmic0ch4(&self) -> Dmic0ch4R {
        Dmic0ch4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMIC0 channel 5 enable"]
    #[inline(always)]
    pub fn dmic0ch5(&self) -> Dmic0ch5R {
        Dmic0ch5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMIC0 channel 6 enable"]
    #[inline(always)]
    pub fn dmic0ch6(&self) -> Dmic0ch6R {
        Dmic0ch6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMIC0 channel 7 enable"]
    #[inline(always)]
    pub fn dmic0ch7(&self) -> Dmic0ch7R {
        Dmic0ch7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I3C RX enable"]
    #[inline(always)]
    pub fn i3c0_rx(&self) -> I3c0RxR {
        I3c0RxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I3C TX enable"]
    #[inline(always)]
    pub fn i3c0_tx(&self) -> I3c0TxR {
        I3c0TxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub fn flexcomm14_rx(&self) -> Flexcomm14RxR {
        Flexcomm14RxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub fn flexcomm14_tx(&self) -> Flexcomm14TxR {
        Flexcomm14TxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - hash enable"]
    #[inline(always)]
    pub fn hashcrypt(&self) -> HashcryptR {
        HashcryptR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ESPI Channel 0 enable"]
    #[inline(always)]
    pub fn espi_ch0(&self) -> EspiCh0R {
        EspiCh0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0_REQ_ENA0")
            .field("flexcomm0_rx", &self.flexcomm0_rx())
            .field("flexcomm0_tx", &self.flexcomm0_tx())
            .field("flexcomm1_rx", &self.flexcomm1_rx())
            .field("flexcomm1_tx", &self.flexcomm1_tx())
            .field("flexcomm2_rx", &self.flexcomm2_rx())
            .field("flexcomm2_tx", &self.flexcomm2_tx())
            .field("flexcomm3_rx", &self.flexcomm3_rx())
            .field("flexcomm3_tx", &self.flexcomm3_tx())
            .field("flexcomm4_rx", &self.flexcomm4_rx())
            .field("flexcomm4_tx", &self.flexcomm4_tx())
            .field("flexcomm5_rx", &self.flexcomm5_rx())
            .field("flexcomm5_tx", &self.flexcomm5_tx())
            .field("flexcomm6_rx", &self.flexcomm6_rx())
            .field("flexcomm6_tx", &self.flexcomm6_tx())
            .field("flexcomm7_rx", &self.flexcomm7_rx())
            .field("flexcomm7_tx", &self.flexcomm7_tx())
            .field("dmic0ch0", &self.dmic0ch0())
            .field("dmic0ch1", &self.dmic0ch1())
            .field("dmic0ch2", &self.dmic0ch2())
            .field("dmic0ch3", &self.dmic0ch3())
            .field("dmic0ch4", &self.dmic0ch4())
            .field("dmic0ch5", &self.dmic0ch5())
            .field("dmic0ch6", &self.dmic0ch6())
            .field("dmic0ch7", &self.dmic0ch7())
            .field("i3c0_rx", &self.i3c0_rx())
            .field("i3c0_tx", &self.i3c0_tx())
            .field("flexcomm14_rx", &self.flexcomm14_rx())
            .field("flexcomm14_tx", &self.flexcomm14_tx())
            .field("hashcrypt", &self.hashcrypt())
            .field("espi_ch0", &self.espi_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FLEXCOMM0 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rx(&mut self) -> Flexcomm0RxW<Dmac0ReqEna0Spec> {
        Flexcomm0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - FLEXCOMM0 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_tx(&mut self) -> Flexcomm0TxW<Dmac0ReqEna0Spec> {
        Flexcomm0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - FLEXCOMM1 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rx(&mut self) -> Flexcomm1RxW<Dmac0ReqEna0Spec> {
        Flexcomm1RxW::new(self, 2)
    }
    #[doc = "Bit 3 - FLEXCOMM1 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_tx(&mut self) -> Flexcomm1TxW<Dmac0ReqEna0Spec> {
        Flexcomm1TxW::new(self, 3)
    }
    #[doc = "Bit 4 - FLEXCOMM2 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rx(&mut self) -> Flexcomm2RxW<Dmac0ReqEna0Spec> {
        Flexcomm2RxW::new(self, 4)
    }
    #[doc = "Bit 5 - FLEXCOMM2 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_tx(&mut self) -> Flexcomm2TxW<Dmac0ReqEna0Spec> {
        Flexcomm2TxW::new(self, 5)
    }
    #[doc = "Bit 6 - FLEXCOMM3 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rx(&mut self) -> Flexcomm3RxW<Dmac0ReqEna0Spec> {
        Flexcomm3RxW::new(self, 6)
    }
    #[doc = "Bit 7 - FLEXCOMM3 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_tx(&mut self) -> Flexcomm3TxW<Dmac0ReqEna0Spec> {
        Flexcomm3TxW::new(self, 7)
    }
    #[doc = "Bit 8 - FLEXCOMM4 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_rx(&mut self) -> Flexcomm4RxW<Dmac0ReqEna0Spec> {
        Flexcomm4RxW::new(self, 8)
    }
    #[doc = "Bit 9 - FLEXCOMM4 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_tx(&mut self) -> Flexcomm4TxW<Dmac0ReqEna0Spec> {
        Flexcomm4TxW::new(self, 9)
    }
    #[doc = "Bit 10 - FLEXCOMM5 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_rx(&mut self) -> Flexcomm5RxW<Dmac0ReqEna0Spec> {
        Flexcomm5RxW::new(self, 10)
    }
    #[doc = "Bit 11 - FLEXCOMM5 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_tx(&mut self) -> Flexcomm5TxW<Dmac0ReqEna0Spec> {
        Flexcomm5TxW::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCOMM6 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_rx(&mut self) -> Flexcomm6RxW<Dmac0ReqEna0Spec> {
        Flexcomm6RxW::new(self, 12)
    }
    #[doc = "Bit 13 - FLEXCOMM6 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_tx(&mut self) -> Flexcomm6TxW<Dmac0ReqEna0Spec> {
        Flexcomm6TxW::new(self, 13)
    }
    #[doc = "Bit 14 - FLEXCOMM7 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_rx(&mut self) -> Flexcomm7RxW<Dmac0ReqEna0Spec> {
        Flexcomm7RxW::new(self, 14)
    }
    #[doc = "Bit 15 - FLEXCOMM7 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_tx(&mut self) -> Flexcomm7TxW<Dmac0ReqEna0Spec> {
        Flexcomm7TxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMIC0 channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch0(&mut self) -> Dmic0ch0W<Dmac0ReqEna0Spec> {
        Dmic0ch0W::new(self, 16)
    }
    #[doc = "Bit 17 - DMIC0 channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch1(&mut self) -> Dmic0ch1W<Dmac0ReqEna0Spec> {
        Dmic0ch1W::new(self, 17)
    }
    #[doc = "Bit 18 - DMIC0 channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch2(&mut self) -> Dmic0ch2W<Dmac0ReqEna0Spec> {
        Dmic0ch2W::new(self, 18)
    }
    #[doc = "Bit 19 - DMIC0 channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch3(&mut self) -> Dmic0ch3W<Dmac0ReqEna0Spec> {
        Dmic0ch3W::new(self, 19)
    }
    #[doc = "Bit 20 - DMIC0 channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch4(&mut self) -> Dmic0ch4W<Dmac0ReqEna0Spec> {
        Dmic0ch4W::new(self, 20)
    }
    #[doc = "Bit 21 - DMIC0 channel 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch5(&mut self) -> Dmic0ch5W<Dmac0ReqEna0Spec> {
        Dmic0ch5W::new(self, 21)
    }
    #[doc = "Bit 22 - DMIC0 channel 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch6(&mut self) -> Dmic0ch6W<Dmac0ReqEna0Spec> {
        Dmic0ch6W::new(self, 22)
    }
    #[doc = "Bit 23 - DMIC0 channel 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0ch7(&mut self) -> Dmic0ch7W<Dmac0ReqEna0Spec> {
        Dmic0ch7W::new(self, 23)
    }
    #[doc = "Bit 24 - I3C RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_rx(&mut self) -> I3c0RxW<Dmac0ReqEna0Spec> {
        I3c0RxW::new(self, 24)
    }
    #[doc = "Bit 25 - I3C TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_tx(&mut self) -> I3c0TxW<Dmac0ReqEna0Spec> {
        I3c0TxW::new(self, 25)
    }
    #[doc = "Bit 26 - FLEXCOMM14 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_rx(&mut self) -> Flexcomm14RxW<Dmac0ReqEna0Spec> {
        Flexcomm14RxW::new(self, 26)
    }
    #[doc = "Bit 27 - FLEXCOMM14 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_tx(&mut self) -> Flexcomm14TxW<Dmac0ReqEna0Spec> {
        Flexcomm14TxW::new(self, 27)
    }
    #[doc = "Bit 30 - hash enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt(&mut self) -> HashcryptW<Dmac0ReqEna0Spec> {
        HashcryptW::new(self, 30)
    }
    #[doc = "Bit 31 - ESPI Channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn espi_ch0(&mut self) -> EspiCh0W<Dmac0ReqEna0Spec> {
        EspiCh0W::new(self, 31)
    }
}
#[doc = "DMAC0 request enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_req_ena0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ReqEna0Spec;
impl crate::RegisterSpec for Dmac0ReqEna0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac0_req_ena0::R`](R) reader structure"]
impl crate::Readable for Dmac0ReqEna0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmac0_req_ena0::W`](W) writer structure"]
impl crate::Writable for Dmac0ReqEna0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_REQ_ENA0 to value 0xffff_ffff"]
impl crate::Resettable for Dmac0ReqEna0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
