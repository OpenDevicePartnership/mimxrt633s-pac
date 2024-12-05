#[doc = "Register `STARTEN1_SET` writer"]
pub type W = crate::W<Starten1SetSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcLite0AlarmOrWakeup {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<RtcLite0AlarmOrWakeup> for bool {
    #[inline(always)]
    fn from(variant: RtcLite0AlarmOrWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_LITE0_ALARM_or_WAKEUP` writer - no description available"]
pub type RtcLite0AlarmOrWakeupW<'a, REG> = crate::BitWriter<'a, REG, RtcLite0AlarmOrWakeup>;
impl<'a, REG> RtcLite0AlarmOrWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLite0AlarmOrWakeup::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLite0AlarmOrWakeup::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mu {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Mu> for bool {
    #[inline(always)]
    fn from(variant: Mu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU` writer - no description available"]
pub type MuW<'a, REG> = crate::BitWriter<'a, REG, Mu>;
impl<'a, REG> MuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq4 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<GpioInt0Irq4> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` writer - no description available"]
pub type GpioInt0Irq4W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq4>;
impl<'a, REG> GpioInt0Irq4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq5 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<GpioInt0Irq5> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` writer - no description available"]
pub type GpioInt0Irq5W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq5>;
impl<'a, REG> GpioInt0Irq5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq6 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<GpioInt0Irq6> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` writer - no description available"]
pub type GpioInt0Irq6W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq6>;
impl<'a, REG> GpioInt0Irq6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq7 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<GpioInt0Irq7> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` writer - no description available"]
pub type GpioInt0Irq7W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq7>;
impl<'a, REG> GpioInt0Irq7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Ct32bit2> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2` writer - no description available"]
pub type Ct32bit2W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2>;
impl<'a, REG> Ct32bit2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Ct32bit4> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4` writer - no description available"]
pub type Ct32bit4W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4>;
impl<'a, REG> Ct32bit4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsEventTimerWu {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<OsEventTimerWu> for bool {
    #[inline(always)]
    fn from(variant: OsEventTimerWu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OS_EVENT_TIMER_WU` writer - no description available"]
pub type OsEventTimerWuW<'a, REG> = crate::BitWriter<'a, REG, OsEventTimerWu>;
impl<'a, REG> OsEventTimerWuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerWu::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerWu::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexspi {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Flexspi> for bool {
    #[inline(always)]
    fn from(variant: Flexspi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI` writer - no description available"]
pub type FlexspiW<'a, REG> = crate::BitWriter<'a, REG, Flexspi>;
impl<'a, REG> FlexspiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexspi::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Flexspi::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Flexcomm6> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6` writer - no description available"]
pub type Flexcomm6W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6>;
impl<'a, REG> Flexcomm6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Flexcomm7> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7` writer - no description available"]
pub type Flexcomm7W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7>;
impl<'a, REG> Flexcomm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Sdio0> for bool {
    #[inline(always)]
    fn from(variant: Sdio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0` writer - no description available"]
pub type Sdio0W<'a, REG> = crate::BitWriter<'a, REG, Sdio0>;
impl<'a, REG> Sdio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Sdio1> for bool {
    #[inline(always)]
    fn from(variant: Sdio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1` writer - no description available"]
pub type Sdio1W<'a, REG> = crate::BitWriter<'a, REG, Sdio1>;
impl<'a, REG> Sdio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShsgpioInt0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<ShsgpioInt0> for bool {
    #[inline(always)]
    fn from(variant: ShsgpioInt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO_INT0` writer - no description available"]
pub type ShsgpioInt0W<'a, REG> = crate::BitWriter<'a, REG, ShsgpioInt0>;
impl<'a, REG> ShsgpioInt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt0::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt0::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShsgpioInt1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<ShsgpioInt1> for bool {
    #[inline(always)]
    fn from(variant: ShsgpioInt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO_INT1` writer - no description available"]
pub type ShsgpioInt1W<'a, REG> = crate::BitWriter<'a, REG, ShsgpioInt1>;
impl<'a, REG> ShsgpioInt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt1::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt1::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<I3c0> for bool {
    #[inline(always)]
    fn from(variant: I3c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0` writer - no description available"]
pub type I3c0W<'a, REG> = crate::BitWriter<'a, REG, I3c0>;
impl<'a, REG> I3c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbIrq {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<UsbIrq> for bool {
    #[inline(always)]
    fn from(variant: UsbIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IRQ` writer - no description available"]
pub type UsbIrqW<'a, REG> = crate::BitWriter<'a, REG, UsbIrq>;
impl<'a, REG> UsbIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbIrq::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbIrq::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbNeedclk {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<UsbNeedclk> for bool {
    #[inline(always)]
    fn from(variant: UsbNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_NEEDCLK` writer - no description available"]
pub type UsbNeedclkW<'a, REG> = crate::BitWriter<'a, REG, UsbNeedclk>;
impl<'a, REG> UsbNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UsbNeedclk::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbNeedclk::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Dmac1> for bool {
    #[inline(always)]
    fn from(variant: Dmac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1` writer - no description available"]
pub type Dmac1W<'a, REG> = crate::BitWriter<'a, REG, Dmac1>;
impl<'a, REG> Dmac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` writer - no description available"]
pub type PufW<'a, REG> = crate::BitWriter<'a, REG, Puf>;
impl<'a, REG> PufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powerquad {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Powerquad> for bool {
    #[inline(always)]
    fn from(variant: Powerquad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` writer - no description available"]
pub type PowerquadW<'a, REG> = crate::BitWriter<'a, REG, Powerquad>;
impl<'a, REG> PowerquadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` writer - no description available"]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmic {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Pmic> for bool {
    #[inline(always)]
    fn from(variant: Pmic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC` writer - no description available"]
pub type PmicW<'a, REG> = crate::BitWriter<'a, REG, Pmic>;
impl<'a, REG> PmicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Pmic::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmic::SetStartEn1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sha {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the START_EN1 Bit"]
    SetStartEn1 = 1,
}
impl From<Sha> for bool {
    #[inline(always)]
    fn from(variant: Sha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHA` writer - no description available"]
pub type ShaW<'a, REG> = crate::BitWriter<'a, REG, Sha>;
impl<'a, REG> ShaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sha::NoEffect)
    }
    #[doc = "Sets the START_EN1 Bit"]
    #[inline(always)]
    pub fn set_start_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Sha::SetStartEn1)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Starten1SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn rtc_lite0_alarm_or_wakeup(&mut self) -> RtcLite0AlarmOrWakeupW<Starten1SetSpec> {
        RtcLite0AlarmOrWakeupW::new(self, 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<Starten1SetSpec> {
        MuW::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq4(&mut self) -> GpioInt0Irq4W<Starten1SetSpec> {
        GpioInt0Irq4W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq5(&mut self) -> GpioInt0Irq5W<Starten1SetSpec> {
        GpioInt0Irq5W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq6(&mut self) -> GpioInt0Irq6W<Starten1SetSpec> {
        GpioInt0Irq6W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq7(&mut self) -> GpioInt0Irq7W<Starten1SetSpec> {
        GpioInt0Irq7W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn ct32bit2(&mut self) -> Ct32bit2W<Starten1SetSpec> {
        Ct32bit2W::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ct32bit4(&mut self) -> Ct32bit4W<Starten1SetSpec> {
        Ct32bit4W::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn os_event_timer_wu(&mut self) -> OsEventTimerWuW<Starten1SetSpec> {
        OsEventTimerWuW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn flexspi(&mut self) -> FlexspiW<Starten1SetSpec> {
        FlexspiW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> Flexcomm6W<Starten1SetSpec> {
        Flexcomm6W::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> Flexcomm7W<Starten1SetSpec> {
        Flexcomm7W::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sdio0(&mut self) -> Sdio0W<Starten1SetSpec> {
        Sdio0W::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> Sdio1W<Starten1SetSpec> {
        Sdio1W::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn shsgpio_int0(&mut self) -> ShsgpioInt0W<Starten1SetSpec> {
        ShsgpioInt0W::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn shsgpio_int1(&mut self) -> ShsgpioInt1W<Starten1SetSpec> {
        ShsgpioInt1W::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn i3c0(&mut self) -> I3c0W<Starten1SetSpec> {
        I3c0W::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usb_irq(&mut self) -> UsbIrqW<Starten1SetSpec> {
        UsbIrqW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn usb_needclk(&mut self) -> UsbNeedclkW<Starten1SetSpec> {
        UsbNeedclkW::new(self, 19)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn dmac1(&mut self) -> Dmac1W<Starten1SetSpec> {
        Dmac1W::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn puf(&mut self) -> PufW<Starten1SetSpec> {
        PufW::new(self, 23)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn powerquad(&mut self) -> PowerquadW<Starten1SetSpec> {
        PowerquadW::new(self, 24)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn casper(&mut self) -> CasperW<Starten1SetSpec> {
        CasperW::new(self, 25)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pmic(&mut self) -> PmicW<Starten1SetSpec> {
        PmicW::new(self, 26)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn sha(&mut self) -> ShaW<Starten1SetSpec> {
        ShaW::new(self, 27)
    }
}
#[doc = "Start enable 1 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Starten1SetSpec;
impl crate::RegisterSpec for Starten1SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`starten1_set::W`](W) writer structure"]
impl crate::Writable for Starten1SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTEN1_SET to value 0"]
impl crate::Resettable for Starten1SetSpec {
    const RESET_VALUE: u32 = 0;
}
