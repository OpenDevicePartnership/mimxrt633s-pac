#[doc = "Register `DMAC0_ITRIG_ENA0_CLR` writer"]
pub type W = crate::W<Dmac0ItrigEna0ClrSpec>;
#[doc = "DMAC0 input trigger inmux 0 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux0> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX0` writer - DMAC0 input trigger inmux 0 enable clear"]
pub type Dmac0ItrigInmux0W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux0>;
impl<'a, REG> Dmac0ItrigInmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux0::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux0::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 1 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux1> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX1` writer - DMAC0 input trigger inmux 1 enable clear"]
pub type Dmac0ItrigInmux1W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux1>;
impl<'a, REG> Dmac0ItrigInmux1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux1::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux1::ClrEna0Bit)
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX2` writer - DMAC0 input trigger inmux 2 enable clear"]
pub type Dmac0ItrigInmux2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMAC0 input trigger inmux 3 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux3 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux3> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX3` writer - DMAC0 input trigger inmux 3 enable clear"]
pub type Dmac0ItrigInmux3W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux3>;
impl<'a, REG> Dmac0ItrigInmux3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux3::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux3::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 4 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux4 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux4> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX4` writer - DMAC0 input trigger inmux 4 enable clear"]
pub type Dmac0ItrigInmux4W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux4>;
impl<'a, REG> Dmac0ItrigInmux4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux4::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux4::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 5 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux5 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux5> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX5` writer - DMAC0 input trigger inmux 5 enable clear"]
pub type Dmac0ItrigInmux5W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux5>;
impl<'a, REG> Dmac0ItrigInmux5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux5::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux5::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 6 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux6 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux6> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX6` writer - DMAC0 input trigger inmux 6 enable clear"]
pub type Dmac0ItrigInmux6W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux6>;
impl<'a, REG> Dmac0ItrigInmux6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux6::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux6::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 7 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux7 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux7> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX7` writer - DMAC0 input trigger inmux 7 enable clear"]
pub type Dmac0ItrigInmux7W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux7>;
impl<'a, REG> Dmac0ItrigInmux7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux7::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux7::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 8 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux8 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux8> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX8` writer - DMAC0 input trigger inmux 8 enable clear"]
pub type Dmac0ItrigInmux8W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux8>;
impl<'a, REG> Dmac0ItrigInmux8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux8::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux8::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 9 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux9 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux9> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX9` writer - DMAC0 input trigger inmux 9 enable clear"]
pub type Dmac0ItrigInmux9W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux9>;
impl<'a, REG> Dmac0ItrigInmux9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux9::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux9::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 10 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux10 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux10> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX10` writer - DMAC0 input trigger inmux 10 enable clear"]
pub type Dmac0ItrigInmux10W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux10>;
impl<'a, REG> Dmac0ItrigInmux10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux10::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux10::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 11 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux11 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux11> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX11` writer - DMAC0 input trigger inmux 11 enable clear"]
pub type Dmac0ItrigInmux11W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux11>;
impl<'a, REG> Dmac0ItrigInmux11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux11::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux11::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 12 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux12 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux12> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX12` writer - DMAC0 input trigger inmux 12 enable clear"]
pub type Dmac0ItrigInmux12W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux12>;
impl<'a, REG> Dmac0ItrigInmux12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux12::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux12::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 13 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux13 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux13> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX13` writer - DMAC0 input trigger inmux 13 enable clear"]
pub type Dmac0ItrigInmux13W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux13>;
impl<'a, REG> Dmac0ItrigInmux13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux13::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux13::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 14 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux14 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux14> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX14` writer - DMAC0 input trigger inmux 14 enable clear"]
pub type Dmac0ItrigInmux14W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux14>;
impl<'a, REG> Dmac0ItrigInmux14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux14::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux14::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 15 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux15 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux15> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX15` writer - DMAC0 input trigger inmux 15 enable clear"]
pub type Dmac0ItrigInmux15W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux15>;
impl<'a, REG> Dmac0ItrigInmux15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux15::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux15::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 16 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux16 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux16> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX16` writer - DMAC0 input trigger inmux 16 enable clear"]
pub type Dmac0ItrigInmux16W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux16>;
impl<'a, REG> Dmac0ItrigInmux16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux16::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux16::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 17 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux17 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux17> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX17` writer - DMAC0 input trigger inmux 17 enable clear"]
pub type Dmac0ItrigInmux17W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux17>;
impl<'a, REG> Dmac0ItrigInmux17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux17::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux17::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 18 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux18 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux18> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX18` writer - DMAC0 input trigger inmux 18 enable clear"]
pub type Dmac0ItrigInmux18W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux18>;
impl<'a, REG> Dmac0ItrigInmux18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux18::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux18::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 19 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux19 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux19> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX19` writer - DMAC0 input trigger inmux 19 enable clear"]
pub type Dmac0ItrigInmux19W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux19>;
impl<'a, REG> Dmac0ItrigInmux19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux19::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux19::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 20 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux20 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux20> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX20` writer - DMAC0 input trigger inmux 20 enable clear"]
pub type Dmac0ItrigInmux20W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux20>;
impl<'a, REG> Dmac0ItrigInmux20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux20::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux20::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 21 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux21 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux21> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX21` writer - DMAC0 input trigger inmux 21 enable clear"]
pub type Dmac0ItrigInmux21W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux21>;
impl<'a, REG> Dmac0ItrigInmux21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux21::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux21::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 22 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux22 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux22> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX22` writer - DMAC0 input trigger inmux 22 enable clear"]
pub type Dmac0ItrigInmux22W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux22>;
impl<'a, REG> Dmac0ItrigInmux22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux22::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux22::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 23 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux23 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux23> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX23` writer - DMAC0 input trigger inmux 23 enable clear"]
pub type Dmac0ItrigInmux23W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux23>;
impl<'a, REG> Dmac0ItrigInmux23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux23::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux23::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 24 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux24 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux24> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX24` writer - DMAC0 input trigger inmux 24 enable clear"]
pub type Dmac0ItrigInmux24W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux24>;
impl<'a, REG> Dmac0ItrigInmux24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux24::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux24::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 25 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux25 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux25> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX25` writer - DMAC0 input trigger inmux 25 enable clear"]
pub type Dmac0ItrigInmux25W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux25>;
impl<'a, REG> Dmac0ItrigInmux25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux25::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux25::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 26 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux26 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux26> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX26` writer - DMAC0 input trigger inmux 26 enable clear"]
pub type Dmac0ItrigInmux26W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux26>;
impl<'a, REG> Dmac0ItrigInmux26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux26::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux26::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 27 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux27 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux27> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX27` writer - DMAC0 input trigger inmux 27 enable clear"]
pub type Dmac0ItrigInmux27W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux27>;
impl<'a, REG> Dmac0ItrigInmux27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux27::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux27::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 28 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux28 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux28> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX28` writer - DMAC0 input trigger inmux 28 enable clear"]
pub type Dmac0ItrigInmux28W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux28>;
impl<'a, REG> Dmac0ItrigInmux28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux28::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux28::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 29 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux29 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux29> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX29` writer - DMAC0 input trigger inmux 29 enable clear"]
pub type Dmac0ItrigInmux29W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux29>;
impl<'a, REG> Dmac0ItrigInmux29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux29::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux29::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 30 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux30 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux30> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX30` writer - DMAC0 input trigger inmux 30 enable clear"]
pub type Dmac0ItrigInmux30W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux30>;
impl<'a, REG> Dmac0ItrigInmux30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux30::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux30::ClrEna0Bit)
    }
}
#[doc = "DMAC0 input trigger inmux 31 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ItrigInmux31 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmac0ItrigInmux31> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ItrigInmux31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_ITRIG_INMUX31` writer - DMAC0 input trigger inmux 31 enable clear"]
pub type Dmac0ItrigInmux31W<'a, REG> = crate::BitWriter<'a, REG, Dmac0ItrigInmux31>;
impl<'a, REG> Dmac0ItrigInmux31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux31::NoEffect)
    }
    #[doc = "clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ItrigInmux31::ClrEna0Bit)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dmac0ItrigEna0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DMAC0 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux0(&mut self) -> Dmac0ItrigInmux0W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC0 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux1(&mut self) -> Dmac0ItrigInmux1W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC0 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux2(&mut self) -> Dmac0ItrigInmux2W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC0 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux3(&mut self) -> Dmac0ItrigInmux3W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC0 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux4(&mut self) -> Dmac0ItrigInmux4W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC0 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux5(&mut self) -> Dmac0ItrigInmux5W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC0 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux6(&mut self) -> Dmac0ItrigInmux6W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC0 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux7(&mut self) -> Dmac0ItrigInmux7W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC0 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux8(&mut self) -> Dmac0ItrigInmux8W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC0 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux9(&mut self) -> Dmac0ItrigInmux9W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC0 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux10(&mut self) -> Dmac0ItrigInmux10W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC0 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux11(&mut self) -> Dmac0ItrigInmux11W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC0 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux12(&mut self) -> Dmac0ItrigInmux12W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC0 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux13(&mut self) -> Dmac0ItrigInmux13W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC0 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux14(&mut self) -> Dmac0ItrigInmux14W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC0 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux15(&mut self) -> Dmac0ItrigInmux15W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMAC0 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux16(&mut self) -> Dmac0ItrigInmux16W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC0 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux17(&mut self) -> Dmac0ItrigInmux17W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC0 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux18(&mut self) -> Dmac0ItrigInmux18W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC0 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux19(&mut self) -> Dmac0ItrigInmux19W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC0 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux20(&mut self) -> Dmac0ItrigInmux20W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC0 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux21(&mut self) -> Dmac0ItrigInmux21W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC0 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux22(&mut self) -> Dmac0ItrigInmux22W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC0 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux23(&mut self) -> Dmac0ItrigInmux23W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC0 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux24(&mut self) -> Dmac0ItrigInmux24W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC0 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux25(&mut self) -> Dmac0ItrigInmux25W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC0 input trigger inmux 26 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux26(&mut self) -> Dmac0ItrigInmux26W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC0 input trigger inmux 27 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux27(&mut self) -> Dmac0ItrigInmux27W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC0 input trigger inmux 28 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux28(&mut self) -> Dmac0ItrigInmux28W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC0 input trigger inmux 29 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux29(&mut self) -> Dmac0ItrigInmux29W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC0 input trigger inmux 30 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux30(&mut self) -> Dmac0ItrigInmux30W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC0 input trigger inmux 31 enable clear"]
    #[inline(always)]
    pub fn dmac0_itrig_inmux31(&mut self) -> Dmac0ItrigInmux31W<Dmac0ItrigEna0ClrSpec> {
        Dmac0ItrigInmux31W::new(self, 31)
    }
}
#[doc = "DMAC0 input trigger enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_ena0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ItrigEna0ClrSpec;
impl crate::RegisterSpec for Dmac0ItrigEna0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac0_itrig_ena0_clr::W`](W) writer structure"]
impl crate::Writable for Dmac0ItrigEna0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_ENA0_CLR to value 0"]
impl crate::Resettable for Dmac0ItrigEna0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
