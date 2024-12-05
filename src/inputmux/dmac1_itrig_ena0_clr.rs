#[doc = "Register `DMAC1_ITRIG_ENA0_CLR` writer"]
pub type W = crate::W<Dmac1ItrigEna0ClrSpec>;
#[doc = "DMAC1 input trigger inmux 0 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux0> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX0` writer - DMAC1 input trigger inmux 0 enable clear"]
pub type Dmac1ItrigInmux0W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux0>;
impl<'a, REG> Dmac1ItrigInmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux0::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux0::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 1 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux1> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX1` writer - DMAC1 input trigger inmux 1 enable clear"]
pub type Dmac1ItrigInmux1W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux1>;
impl<'a, REG> Dmac1ItrigInmux1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux1::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux1::ClrEna0Bit)
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX2` writer - DMAC1 input trigger inmux 2 enable clear"]
pub type Dmac1ItrigInmux2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMAC1 input trigger inmux 3 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux3 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux3> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX3` writer - DMAC1 input trigger inmux 3 enable clear"]
pub type Dmac1ItrigInmux3W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux3>;
impl<'a, REG> Dmac1ItrigInmux3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux3::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux3::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 4 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux4 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux4> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX4` writer - DMAC1 input trigger inmux 4 enable clear"]
pub type Dmac1ItrigInmux4W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux4>;
impl<'a, REG> Dmac1ItrigInmux4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux4::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux4::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 5 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux5 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux5> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX5` writer - DMAC1 input trigger inmux 5 enable clear"]
pub type Dmac1ItrigInmux5W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux5>;
impl<'a, REG> Dmac1ItrigInmux5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux5::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux5::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 6 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux6 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux6> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX6` writer - DMAC1 input trigger inmux 6 enable clear"]
pub type Dmac1ItrigInmux6W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux6>;
impl<'a, REG> Dmac1ItrigInmux6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux6::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux6::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 7 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux7 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux7> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX7` writer - DMAC1 input trigger inmux 7 enable clear"]
pub type Dmac1ItrigInmux7W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux7>;
impl<'a, REG> Dmac1ItrigInmux7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux7::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux7::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 8 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux8 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux8> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX8` writer - DMAC1 input trigger inmux 8 enable clear"]
pub type Dmac1ItrigInmux8W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux8>;
impl<'a, REG> Dmac1ItrigInmux8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux8::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux8::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 9 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux9 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux9> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX9` writer - DMAC1 input trigger inmux 9 enable clear"]
pub type Dmac1ItrigInmux9W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux9>;
impl<'a, REG> Dmac1ItrigInmux9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux9::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux9::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 10 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux10 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux10> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX10` writer - DMAC1 input trigger inmux 10 enable clear"]
pub type Dmac1ItrigInmux10W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux10>;
impl<'a, REG> Dmac1ItrigInmux10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux10::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux10::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 11 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux11 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux11> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX11` writer - DMAC1 input trigger inmux 11 enable clear"]
pub type Dmac1ItrigInmux11W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux11>;
impl<'a, REG> Dmac1ItrigInmux11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux11::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux11::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 12 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux12 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux12> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX12` writer - DMAC1 input trigger inmux 12 enable clear"]
pub type Dmac1ItrigInmux12W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux12>;
impl<'a, REG> Dmac1ItrigInmux12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux12::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux12::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 13 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux13 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux13> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX13` writer - DMAC1 input trigger inmux 13 enable clear"]
pub type Dmac1ItrigInmux13W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux13>;
impl<'a, REG> Dmac1ItrigInmux13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux13::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux13::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 14 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux14 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux14> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX14` writer - DMAC1 input trigger inmux 14 enable clear"]
pub type Dmac1ItrigInmux14W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux14>;
impl<'a, REG> Dmac1ItrigInmux14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux14::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux14::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 15 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux15 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux15> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX15` writer - DMAC1 input trigger inmux 15 enable clear"]
pub type Dmac1ItrigInmux15W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux15>;
impl<'a, REG> Dmac1ItrigInmux15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux15::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux15::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 16 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux16 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux16> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX16` writer - DMAC1 input trigger inmux 16 enable clear"]
pub type Dmac1ItrigInmux16W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux16>;
impl<'a, REG> Dmac1ItrigInmux16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux16::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux16::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 17 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux17 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux17> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX17` writer - DMAC1 input trigger inmux 17 enable clear"]
pub type Dmac1ItrigInmux17W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux17>;
impl<'a, REG> Dmac1ItrigInmux17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux17::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux17::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 18 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux18 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux18> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX18` writer - DMAC1 input trigger inmux 18 enable clear"]
pub type Dmac1ItrigInmux18W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux18>;
impl<'a, REG> Dmac1ItrigInmux18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux18::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux18::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 19 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux19 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux19> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX19` writer - DMAC1 input trigger inmux 19 enable clear"]
pub type Dmac1ItrigInmux19W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux19>;
impl<'a, REG> Dmac1ItrigInmux19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux19::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux19::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 20 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux20 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux20> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX20` writer - DMAC1 input trigger inmux 20 enable clear"]
pub type Dmac1ItrigInmux20W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux20>;
impl<'a, REG> Dmac1ItrigInmux20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux20::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux20::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 21 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux21 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux21> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX21` writer - DMAC1 input trigger inmux 21 enable clear"]
pub type Dmac1ItrigInmux21W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux21>;
impl<'a, REG> Dmac1ItrigInmux21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux21::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux21::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 22 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux22 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux22> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX22` writer - DMAC1 input trigger inmux 22 enable clear"]
pub type Dmac1ItrigInmux22W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux22>;
impl<'a, REG> Dmac1ItrigInmux22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux22::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux22::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 23 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux23 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux23> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX23` writer - DMAC1 input trigger inmux 23 enable clear"]
pub type Dmac1ItrigInmux23W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux23>;
impl<'a, REG> Dmac1ItrigInmux23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux23::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux23::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 24 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux24 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux24> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX24` writer - DMAC1 input trigger inmux 24 enable clear"]
pub type Dmac1ItrigInmux24W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux24>;
impl<'a, REG> Dmac1ItrigInmux24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux24::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux24::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux25 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux25> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX25` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux25W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux25>;
impl<'a, REG> Dmac1ItrigInmux25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux25::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux25::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux26 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux26> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX26` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux26W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux26>;
impl<'a, REG> Dmac1ItrigInmux26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux26::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux26::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux27 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux27> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX27` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux27W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux27>;
impl<'a, REG> Dmac1ItrigInmux27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux27::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux27::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux28 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux28> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX28` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux28W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux28>;
impl<'a, REG> Dmac1ItrigInmux28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux28::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux28::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux29 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux29> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX29` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux29W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux29>;
impl<'a, REG> Dmac1ItrigInmux29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux29::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux29::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux30 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux30> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX30` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux30W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux30>;
impl<'a, REG> Dmac1ItrigInmux30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux30::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux30::ClrEna0Bit)
    }
}
#[doc = "DMAC1 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ItrigInmux31 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac1ItrigInmux31> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ItrigInmux31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_ITRIG_INMUX31` writer - DMAC1 input trigger inmux 25 enable clear"]
pub type Dmac1ItrigInmux31W<'a, REG> = crate::BitWriter<'a, REG, Dmac1ItrigInmux31>;
impl<'a, REG> Dmac1ItrigInmux31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux31::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ItrigInmux31::ClrEna0Bit)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dmac1ItrigEna0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DMAC1 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux0(&mut self) -> Dmac1ItrigInmux0W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC1 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux1(&mut self) -> Dmac1ItrigInmux1W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC1 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux2(&mut self) -> Dmac1ItrigInmux2W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC1 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux3(&mut self) -> Dmac1ItrigInmux3W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC1 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux4(&mut self) -> Dmac1ItrigInmux4W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC1 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux5(&mut self) -> Dmac1ItrigInmux5W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC1 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux6(&mut self) -> Dmac1ItrigInmux6W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC1 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux7(&mut self) -> Dmac1ItrigInmux7W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC1 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux8(&mut self) -> Dmac1ItrigInmux8W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC1 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux9(&mut self) -> Dmac1ItrigInmux9W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC1 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux10(&mut self) -> Dmac1ItrigInmux10W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC1 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux11(&mut self) -> Dmac1ItrigInmux11W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC1 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux12(&mut self) -> Dmac1ItrigInmux12W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC1 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux13(&mut self) -> Dmac1ItrigInmux13W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC1 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux14(&mut self) -> Dmac1ItrigInmux14W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC1 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux15(&mut self) -> Dmac1ItrigInmux15W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMAC1 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux16(&mut self) -> Dmac1ItrigInmux16W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC1 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux17(&mut self) -> Dmac1ItrigInmux17W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC1 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux18(&mut self) -> Dmac1ItrigInmux18W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC1 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux19(&mut self) -> Dmac1ItrigInmux19W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC1 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux20(&mut self) -> Dmac1ItrigInmux20W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC1 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux21(&mut self) -> Dmac1ItrigInmux21W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC1 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux22(&mut self) -> Dmac1ItrigInmux22W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC1 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux23(&mut self) -> Dmac1ItrigInmux23W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux24(&mut self) -> Dmac1ItrigInmux24W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux25(&mut self) -> Dmac1ItrigInmux25W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux26(&mut self) -> Dmac1ItrigInmux26W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux27(&mut self) -> Dmac1ItrigInmux27W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux28(&mut self) -> Dmac1ItrigInmux28W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux29(&mut self) -> Dmac1ItrigInmux29W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux30(&mut self) -> Dmac1ItrigInmux30W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac1_itrig_inmux31(&mut self) -> Dmac1ItrigInmux31W<Dmac1ItrigEna0ClrSpec> {
        Dmac1ItrigInmux31W::new(self, 31)
    }
}
#[doc = "DMAC1 input trigger enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_ena0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1ItrigEna0ClrSpec;
impl crate::RegisterSpec for Dmac1ItrigEna0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac1_itrig_ena0_clr::W`](W) writer structure"]
impl crate::Writable for Dmac1ItrigEna0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_ITRIG_ENA0_CLR to value 0"]
impl crate::Resettable for Dmac1ItrigEna0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
