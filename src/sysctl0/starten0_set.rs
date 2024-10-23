#[doc = "Register `STARTEN0_SET` writer"]
pub type W = crate::W<Starten0SetSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Wdt0> for bool {
    #[inline(always)]
    fn from(variant: Wdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT0` writer - no description available"]
pub type Wdt0W<'a, REG> = crate::BitWriter<'a, REG, Wdt0>;
impl<'a, REG> Wdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Dmac0> for bool {
    #[inline(always)]
    fn from(variant: Dmac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0` writer - no description available"]
pub type Dmac0W<'a, REG> = crate::BitWriter<'a, REG, Dmac0>;
impl<'a, REG> Dmac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NshsgpioInt0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<NshsgpioInt0> for bool {
    #[inline(always)]
    fn from(variant: NshsgpioInt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSHSGPIO_INT0` writer - no description available"]
pub type NshsgpioInt0W<'a, REG> = crate::BitWriter<'a, REG, NshsgpioInt0>;
impl<'a, REG> NshsgpioInt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NshsgpioInt1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<NshsgpioInt1> for bool {
    #[inline(always)]
    fn from(variant: NshsgpioInt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSHSGPIO_INT1` writer - no description available"]
pub type NshsgpioInt1W<'a, REG> = crate::BitWriter<'a, REG, NshsgpioInt1>;
impl<'a, REG> NshsgpioInt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt1::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(NshsgpioInt1::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<GpioInt0Irq0> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - no description available"]
pub type GpioInt0Irq0W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq0>;
impl<'a, REG> GpioInt0Irq0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<GpioInt0Irq1> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - no description available"]
pub type GpioInt0Irq1W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq1>;
impl<'a, REG> GpioInt0Irq1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq2 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<GpioInt0Irq2> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - no description available"]
pub type GpioInt0Irq2W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq2>;
impl<'a, REG> GpioInt0Irq2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq3 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<GpioInt0Irq3> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - no description available"]
pub type GpioInt0Irq3W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq3>;
impl<'a, REG> GpioInt0Irq3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Utick0> for bool {
    #[inline(always)]
    fn from(variant: Utick0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0` writer - no description available"]
pub type Utick0W<'a, REG> = crate::BitWriter<'a, REG, Utick0>;
impl<'a, REG> Utick0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Mrt0> for bool {
    #[inline(always)]
    fn from(variant: Mrt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0` writer - no description available"]
pub type Mrt0W<'a, REG> = crate::BitWriter<'a, REG, Mrt0>;
impl<'a, REG> Mrt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Ct32bit0> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0` writer - no description available"]
pub type Ct32bit0W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0>;
impl<'a, REG> Ct32bit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Ct32bit1> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1` writer - no description available"]
pub type Ct32bit1W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1>;
impl<'a, REG> Ct32bit1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sct0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Sct0> for bool {
    #[inline(always)]
    fn from(variant: Sct0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT0` writer - no description available"]
pub type Sct0W<'a, REG> = crate::BitWriter<'a, REG, Sct0>;
impl<'a, REG> Sct0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sct0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Sct0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Ct32bit3> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3` writer - no description available"]
pub type Ct32bit3W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3>;
impl<'a, REG> Ct32bit3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm0> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0` writer - no description available"]
pub type Flexcomm0W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0>;
impl<'a, REG> Flexcomm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm1> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1` writer - no description available"]
pub type Flexcomm1W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1>;
impl<'a, REG> Flexcomm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm2> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2` writer - no description available"]
pub type Flexcomm2W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2>;
impl<'a, REG> Flexcomm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm3> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3` writer - no description available"]
pub type Flexcomm3W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3>;
impl<'a, REG> Flexcomm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm4> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4` writer - no description available"]
pub type Flexcomm4W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4>;
impl<'a, REG> Flexcomm4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm5> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5` writer - no description available"]
pub type Flexcomm5W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5>;
impl<'a, REG> Flexcomm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm14> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14` writer - no description available"]
pub type Flexcomm14W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14>;
impl<'a, REG> Flexcomm14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm15 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Flexcomm15> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15` writer - no description available"]
pub type Flexcomm15W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm15>;
impl<'a, REG> Flexcomm15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` writer - no description available"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Acmp> for bool {
    #[inline(always)]
    fn from(variant: Acmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` writer - no description available"]
pub type AcmpW<'a, REG> = crate::BitWriter<'a, REG, Acmp>;
impl<'a, REG> AcmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Dmic0> for bool {
    #[inline(always)]
    fn from(variant: Dmic0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0` writer - no description available"]
pub type Dmic0W<'a, REG> = crate::BitWriter<'a, REG, Dmic0>;
impl<'a, REG> Dmic0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hypervisor {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Hypervisor> for bool {
    #[inline(always)]
    fn from(variant: Hypervisor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYPERVISOR` writer - no description available"]
pub type HypervisorW<'a, REG> = crate::BitWriter<'a, REG, Hypervisor>;
impl<'a, REG> HypervisorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hypervisor::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Hypervisor::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secureviolation {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Secureviolation> for bool {
    #[inline(always)]
    fn from(variant: Secureviolation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECUREVIOLATION` writer - no description available"]
pub type SecureviolationW<'a, REG> = crate::BitWriter<'a, REG, Secureviolation>;
impl<'a, REG> SecureviolationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Secureviolation::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Secureviolation::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwvad0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Hwvad0> for bool {
    #[inline(always)]
    fn from(variant: Hwvad0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWVAD0` writer - no description available"]
pub type Hwvad0W<'a, REG> = crate::BitWriter<'a, REG, Hwvad0>;
impl<'a, REG> Hwvad0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hwvad0::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Hwvad0::SetStartEn0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN0 Bit"]
    SetStartEn0 = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` writer - no description available"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::NoEffect)
    }
    #[doc = "Sets the START_EN0 Bit"]
    #[inline(always)]
    pub fn set_start_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::SetStartEn0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Starten0SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> Wdt0W<Starten0SetSpec> {
        Wdt0W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0(&mut self) -> Dmac0W<Starten0SetSpec> {
        Dmac0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nshsgpio_int0(&mut self) -> NshsgpioInt0W<Starten0SetSpec> {
        NshsgpioInt0W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn nshsgpio_int1(&mut self) -> NshsgpioInt1W<Starten0SetSpec> {
        NshsgpioInt1W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq0(&mut self) -> GpioInt0Irq0W<Starten0SetSpec> {
        GpioInt0Irq0W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq1(&mut self) -> GpioInt0Irq1W<Starten0SetSpec> {
        GpioInt0Irq1W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq2(&mut self) -> GpioInt0Irq2W<Starten0SetSpec> {
        GpioInt0Irq2W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq3(&mut self) -> GpioInt0Irq3W<Starten0SetSpec> {
        GpioInt0Irq3W::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn utick0(&mut self) -> Utick0W<Starten0SetSpec> {
        Utick0W::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0(&mut self) -> Mrt0W<Starten0SetSpec> {
        Mrt0W::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0(&mut self) -> Ct32bit0W<Starten0SetSpec> {
        Ct32bit0W::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1(&mut self) -> Ct32bit1W<Starten0SetSpec> {
        Ct32bit1W::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sct0(&mut self) -> Sct0W<Starten0SetSpec> {
        Sct0W::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3(&mut self) -> Ct32bit3W<Starten0SetSpec> {
        Ct32bit3W::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0(&mut self) -> Flexcomm0W<Starten0SetSpec> {
        Flexcomm0W::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1(&mut self) -> Flexcomm1W<Starten0SetSpec> {
        Flexcomm1W::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2(&mut self) -> Flexcomm2W<Starten0SetSpec> {
        Flexcomm2W::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3(&mut self) -> Flexcomm3W<Starten0SetSpec> {
        Flexcomm3W::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4(&mut self) -> Flexcomm4W<Starten0SetSpec> {
        Flexcomm4W::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5(&mut self) -> Flexcomm5W<Starten0SetSpec> {
        Flexcomm5W::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> Flexcomm14W<Starten0SetSpec> {
        Flexcomm14W::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15(&mut self) -> Flexcomm15W<Starten0SetSpec> {
        Flexcomm15W::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<Starten0SetSpec> {
        Adc0W::new(self, 22)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> AcmpW<Starten0SetSpec> {
        AcmpW::new(self, 24)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> Dmic0W<Starten0SetSpec> {
        Dmic0W::new(self, 25)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hypervisor(&mut self) -> HypervisorW<Starten0SetSpec> {
        HypervisorW::new(self, 27)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn secureviolation(&mut self) -> SecureviolationW<Starten0SetSpec> {
        SecureviolationW::new(self, 28)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hwvad0(&mut self) -> Hwvad0W<Starten0SetSpec> {
        Hwvad0W::new(self, 29)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<Starten0SetSpec> {
        RngW::new(self, 31)
    }
}
#[doc = "Start enable 0 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Starten0SetSpec;
impl crate::RegisterSpec for Starten0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`starten0_set::W`](W) writer structure"]
impl crate::Writable for Starten0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTEN0_SET to value 0"]
impl crate::Resettable for Starten0SetSpec {
    const RESET_VALUE: u32 = 0;
}
