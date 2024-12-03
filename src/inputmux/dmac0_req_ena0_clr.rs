#[doc = "Register `DMAC0_REQ_ENA0_CLR` writer"]
pub type W = crate::W<Dmac0ReqEna0ClrSpec>;
#[doc = "FLEXCOMM0 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm0Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_RX` writer - FLEXCOMM0 RX enable clear"]
pub type Flexcomm0RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Rx>;
impl<'a, REG> Flexcomm0RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM0 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm0Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_TX` writer - FLEXCOMM0 TX enable clear"]
pub type Flexcomm0TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Tx>;
impl<'a, REG> Flexcomm0TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM1 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm1Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_RX` writer - FLEXCOMM1 RX enable clear"]
pub type Flexcomm1RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Rx>;
impl<'a, REG> Flexcomm1RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM1 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm1Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_TX` writer - FLEXCOMM1 TX enable clear"]
pub type Flexcomm1TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Tx>;
impl<'a, REG> Flexcomm1TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM2 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm2Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_RX` writer - FLEXCOMM2 RX enable clear"]
pub type Flexcomm2RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Rx>;
impl<'a, REG> Flexcomm2RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM2 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm2Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_TX` writer - FLEXCOMM2 TX enable clear"]
pub type Flexcomm2TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Tx>;
impl<'a, REG> Flexcomm2TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM3 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm3Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_RX` writer - FLEXCOMM3 RX enable clear"]
pub type Flexcomm3RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Rx>;
impl<'a, REG> Flexcomm3RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM3 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm3Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_TX` writer - FLEXCOMM3 TX enable clear"]
pub type Flexcomm3TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Tx>;
impl<'a, REG> Flexcomm3TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM4 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm4Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_RX` writer - FLEXCOMM4 RX enable clear"]
pub type Flexcomm4RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Rx>;
impl<'a, REG> Flexcomm4RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM4 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm4Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_TX` writer - FLEXCOMM4 TX enable clear"]
pub type Flexcomm4TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Tx>;
impl<'a, REG> Flexcomm4TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM5 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm5Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_RX` writer - FLEXCOMM5 RX enable clear"]
pub type Flexcomm5RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Rx>;
impl<'a, REG> Flexcomm5RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM5 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm5Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_TX` writer - FLEXCOMM5 TX enable clear"]
pub type Flexcomm5TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Tx>;
impl<'a, REG> Flexcomm5TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM6 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm6Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_RX` writer - FLEXCOMM6 RX enable clear"]
pub type Flexcomm6RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Rx>;
impl<'a, REG> Flexcomm6RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM6 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm6Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_TX` writer - FLEXCOMM6 TX enable clear"]
pub type Flexcomm6TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Tx>;
impl<'a, REG> Flexcomm6TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM7 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm7Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_RX` writer - FLEXCOMM7 RX enable clear"]
pub type Flexcomm7RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Rx>;
impl<'a, REG> Flexcomm7RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM7 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm7Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_TX` writer - FLEXCOMM7 TX enable clear"]
pub type Flexcomm7TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Tx>;
impl<'a, REG> Flexcomm7TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Tx::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 0 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch0> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH0` writer - DMIC0 channel 0 enable clear"]
pub type Dmic0ch0W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch0>;
impl<'a, REG> Dmic0ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch0::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch0::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 1 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch1> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH1` writer - DMIC0 channel 1 enable clear"]
pub type Dmic0ch1W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch1>;
impl<'a, REG> Dmic0ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch1::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch1::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 2 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch2 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch2> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH2` writer - DMIC0 channel 2 enable clear"]
pub type Dmic0ch2W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch2>;
impl<'a, REG> Dmic0ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch2::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch2::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 3 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch3 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch3> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH3` writer - DMIC0 channel 3 enable clear"]
pub type Dmic0ch3W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch3>;
impl<'a, REG> Dmic0ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch3::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch3::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 4 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch4 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch4> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH4` writer - DMIC0 channel 4 enable clear"]
pub type Dmic0ch4W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch4>;
impl<'a, REG> Dmic0ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch4::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch4::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 5 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch5 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch5> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH5` writer - DMIC0 channel 5 enable clear"]
pub type Dmic0ch5W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch5>;
impl<'a, REG> Dmic0ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch5::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch5::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 6 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch6 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch6> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH6` writer - DMIC0 channel 6 enable clear"]
pub type Dmic0ch6W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch6>;
impl<'a, REG> Dmic0ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch6::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch6::ClrEna0Bit)
    }
}
#[doc = "DMIC0 channel 7 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ch7 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Dmic0ch7> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0CH7` writer - DMIC0 channel 7 enable clear"]
pub type Dmic0ch7W<'a, REG> = crate::BitWriter<'a, REG, Dmic0ch7>;
impl<'a, REG> Dmic0ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch7::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ch7::ClrEna0Bit)
    }
}
#[doc = "I3C RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<I3c0Rx> for bool {
    #[inline(always)]
    fn from(variant: I3c0Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_RX` writer - I3C RX enable clear"]
pub type I3c0RxW<'a, REG> = crate::BitWriter<'a, REG, I3c0Rx>;
impl<'a, REG> I3c0RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rx::ClrEna0Bit)
    }
}
#[doc = "I3C TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<I3c0Tx> for bool {
    #[inline(always)]
    fn from(variant: I3c0Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_TX` writer - I3C TX enable clear"]
pub type I3c0TxW<'a, REG> = crate::BitWriter<'a, REG, I3c0Tx>;
impl<'a, REG> I3c0TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Tx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM14 RX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14Rx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm14Rx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_RX` writer - FLEXCOMM14 RX enable clear"]
pub type Flexcomm14RxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14Rx>;
impl<'a, REG> Flexcomm14RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Rx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Rx::ClrEna0Bit)
    }
}
#[doc = "FLEXCOMM14 TX enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14Tx {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Flexcomm14Tx> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_TX` writer - FLEXCOMM14 TX enable clear"]
pub type Flexcomm14TxW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14Tx>;
impl<'a, REG> Flexcomm14TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Tx::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14Tx::ClrEna0Bit)
    }
}
#[doc = "Hash enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashcrypt {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<Hashcrypt> for bool {
    #[inline(always)]
    fn from(variant: Hashcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` writer - Hash enable clear"]
pub type HashcryptW<'a, REG> = crate::BitWriter<'a, REG, Hashcrypt>;
impl<'a, REG> HashcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Hashcrypt::ClrEna0Bit)
    }
}
#[doc = "ESPI Channel 0 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EspiCh0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA0 Bit"]
    ClrEna0Bit = 1,
}
impl From<EspiCh0> for bool {
    #[inline(always)]
    fn from(variant: EspiCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESPI_CH0` writer - ESPI Channel 0 enable clear"]
pub type EspiCh0W<'a, REG> = crate::BitWriter<'a, REG, EspiCh0>;
impl<'a, REG> EspiCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh0::NoEffect)
    }
    #[doc = "Clears the ENA0 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh0::ClrEna0Bit)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dmac0ReqEna0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - FLEXCOMM0 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm0_rx(&mut self) -> Flexcomm0RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - FLEXCOMM0 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm0_tx(&mut self) -> Flexcomm0TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - FLEXCOMM1 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm1_rx(&mut self) -> Flexcomm1RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm1RxW::new(self, 2)
    }
    #[doc = "Bit 3 - FLEXCOMM1 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm1_tx(&mut self) -> Flexcomm1TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm1TxW::new(self, 3)
    }
    #[doc = "Bit 4 - FLEXCOMM2 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm2_rx(&mut self) -> Flexcomm2RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm2RxW::new(self, 4)
    }
    #[doc = "Bit 5 - FLEXCOMM2 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm2_tx(&mut self) -> Flexcomm2TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm2TxW::new(self, 5)
    }
    #[doc = "Bit 6 - FLEXCOMM3 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm3_rx(&mut self) -> Flexcomm3RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm3RxW::new(self, 6)
    }
    #[doc = "Bit 7 - FLEXCOMM3 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm3_tx(&mut self) -> Flexcomm3TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm3TxW::new(self, 7)
    }
    #[doc = "Bit 8 - FLEXCOMM4 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm4_rx(&mut self) -> Flexcomm4RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm4RxW::new(self, 8)
    }
    #[doc = "Bit 9 - FLEXCOMM4 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm4_tx(&mut self) -> Flexcomm4TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm4TxW::new(self, 9)
    }
    #[doc = "Bit 10 - FLEXCOMM5 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm5_rx(&mut self) -> Flexcomm5RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm5RxW::new(self, 10)
    }
    #[doc = "Bit 11 - FLEXCOMM5 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm5_tx(&mut self) -> Flexcomm5TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm5TxW::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCOMM6 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm6_rx(&mut self) -> Flexcomm6RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm6RxW::new(self, 12)
    }
    #[doc = "Bit 13 - FLEXCOMM6 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm6_tx(&mut self) -> Flexcomm6TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm6TxW::new(self, 13)
    }
    #[doc = "Bit 14 - FLEXCOMM7 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm7_rx(&mut self) -> Flexcomm7RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm7RxW::new(self, 14)
    }
    #[doc = "Bit 15 - FLEXCOMM7 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm7_tx(&mut self) -> Flexcomm7TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm7TxW::new(self, 15)
    }
    #[doc = "Bit 16 - DMIC0 channel 0 enable clear"]
    #[inline(always)]
    pub fn dmic0ch0(&mut self) -> Dmic0ch0W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch0W::new(self, 16)
    }
    #[doc = "Bit 17 - DMIC0 channel 1 enable clear"]
    #[inline(always)]
    pub fn dmic0ch1(&mut self) -> Dmic0ch1W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch1W::new(self, 17)
    }
    #[doc = "Bit 18 - DMIC0 channel 2 enable clear"]
    #[inline(always)]
    pub fn dmic0ch2(&mut self) -> Dmic0ch2W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch2W::new(self, 18)
    }
    #[doc = "Bit 19 - DMIC0 channel 3 enable clear"]
    #[inline(always)]
    pub fn dmic0ch3(&mut self) -> Dmic0ch3W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch3W::new(self, 19)
    }
    #[doc = "Bit 20 - DMIC0 channel 4 enable clear"]
    #[inline(always)]
    pub fn dmic0ch4(&mut self) -> Dmic0ch4W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch4W::new(self, 20)
    }
    #[doc = "Bit 21 - DMIC0 channel 5 enable clear"]
    #[inline(always)]
    pub fn dmic0ch5(&mut self) -> Dmic0ch5W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch5W::new(self, 21)
    }
    #[doc = "Bit 22 - DMIC0 channel 6 enable clear"]
    #[inline(always)]
    pub fn dmic0ch6(&mut self) -> Dmic0ch6W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch6W::new(self, 22)
    }
    #[doc = "Bit 23 - DMIC0 channel 7 enable clear"]
    #[inline(always)]
    pub fn dmic0ch7(&mut self) -> Dmic0ch7W<Dmac0ReqEna0ClrSpec> {
        Dmic0ch7W::new(self, 23)
    }
    #[doc = "Bit 24 - I3C RX enable clear"]
    #[inline(always)]
    pub fn i3c0_rx(&mut self) -> I3c0RxW<Dmac0ReqEna0ClrSpec> {
        I3c0RxW::new(self, 24)
    }
    #[doc = "Bit 25 - I3C TX enable clear"]
    #[inline(always)]
    pub fn i3c0_tx(&mut self) -> I3c0TxW<Dmac0ReqEna0ClrSpec> {
        I3c0TxW::new(self, 25)
    }
    #[doc = "Bit 26 - FLEXCOMM14 RX enable clear"]
    #[inline(always)]
    pub fn flexcomm14_rx(&mut self) -> Flexcomm14RxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm14RxW::new(self, 26)
    }
    #[doc = "Bit 27 - FLEXCOMM14 TX enable clear"]
    #[inline(always)]
    pub fn flexcomm14_tx(&mut self) -> Flexcomm14TxW<Dmac0ReqEna0ClrSpec> {
        Flexcomm14TxW::new(self, 27)
    }
    #[doc = "Bit 30 - Hash enable clear"]
    #[inline(always)]
    pub fn hashcrypt(&mut self) -> HashcryptW<Dmac0ReqEna0ClrSpec> {
        HashcryptW::new(self, 30)
    }
    #[doc = "Bit 31 - ESPI Channel 0 enable clear"]
    #[inline(always)]
    pub fn espi_ch0(&mut self) -> EspiCh0W<Dmac0ReqEna0ClrSpec> {
        EspiCh0W::new(self, 31)
    }
}
#[doc = "DMAC0 request enable clear 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_req_ena0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ReqEna0ClrSpec;
impl crate::RegisterSpec for Dmac0ReqEna0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac0_req_ena0_clr::W`](W) writer structure"]
impl crate::Writable for Dmac0ReqEna0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_REQ_ENA0_CLR to value 0"]
impl crate::Resettable for Dmac0ReqEna0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
