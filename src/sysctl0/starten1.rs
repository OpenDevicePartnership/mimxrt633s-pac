#[doc = "Register `STARTEN1` reader"]
pub type R = crate::R<Starten1Spec>;
#[doc = "Register `STARTEN1` writer"]
pub type W = crate::W<Starten1Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcLite0AlarmOrWakeup {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<RtcLite0AlarmOrWakeup> for bool {
    #[inline(always)]
    fn from(variant: RtcLite0AlarmOrWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_LITE0_ALARM_or_WAKEUP` reader - no description available"]
pub type RtcLite0AlarmOrWakeupR = crate::BitReader<RtcLite0AlarmOrWakeup>;
impl RtcLite0AlarmOrWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcLite0AlarmOrWakeup {
        match self.bits {
            false => RtcLite0AlarmOrWakeup::Disabled,
            true => RtcLite0AlarmOrWakeup::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RtcLite0AlarmOrWakeup::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RtcLite0AlarmOrWakeup::Enabled
    }
}
#[doc = "Field `RTC_LITE0_ALARM_or_WAKEUP` writer - no description available"]
pub type RtcLite0AlarmOrWakeupW<'a, REG> = crate::BitWriter<'a, REG, RtcLite0AlarmOrWakeup>;
impl<'a, REG> RtcLite0AlarmOrWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLite0AlarmOrWakeup::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLite0AlarmOrWakeup::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mu {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Mu> for bool {
    #[inline(always)]
    fn from(variant: Mu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU` reader - no description available"]
pub type MuR = crate::BitReader<Mu>;
impl MuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mu {
        match self.bits {
            false => Mu::Disabled,
            true => Mu::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mu::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mu::Enabled
    }
}
#[doc = "Field `MU` writer - no description available"]
pub type MuW<'a, REG> = crate::BitWriter<'a, REG, Mu>;
impl<'a, REG> MuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq4 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq4> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` reader - no description available"]
pub type GpioInt0Irq4R = crate::BitReader<GpioInt0Irq4>;
impl GpioInt0Irq4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq4 {
        match self.bits {
            false => GpioInt0Irq4::Disabled,
            true => GpioInt0Irq4::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq4::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` writer - no description available"]
pub type GpioInt0Irq4W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq4>;
impl<'a, REG> GpioInt0Irq4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq5 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq5> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` reader - no description available"]
pub type GpioInt0Irq5R = crate::BitReader<GpioInt0Irq5>;
impl GpioInt0Irq5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq5 {
        match self.bits {
            false => GpioInt0Irq5::Disabled,
            true => GpioInt0Irq5::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq5::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq5::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` writer - no description available"]
pub type GpioInt0Irq5W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq5>;
impl<'a, REG> GpioInt0Irq5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq6 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq6> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` reader - no description available"]
pub type GpioInt0Irq6R = crate::BitReader<GpioInt0Irq6>;
impl GpioInt0Irq6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq6 {
        match self.bits {
            false => GpioInt0Irq6::Disabled,
            true => GpioInt0Irq6::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq6::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq6::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` writer - no description available"]
pub type GpioInt0Irq6W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq6>;
impl<'a, REG> GpioInt0Irq6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq7 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<GpioInt0Irq7> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` reader - no description available"]
pub type GpioInt0Irq7R = crate::BitReader<GpioInt0Irq7>;
impl GpioInt0Irq7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq7 {
        match self.bits {
            false => GpioInt0Irq7::Disabled,
            true => GpioInt0Irq7::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GpioInt0Irq7::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GpioInt0Irq7::Enabled
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` writer - no description available"]
pub type GpioInt0Irq7W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq7>;
impl<'a, REG> GpioInt0Irq7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ct32bit2> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2` reader - no description available"]
pub type Ct32bit2R = crate::BitReader<Ct32bit2>;
impl Ct32bit2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit2 {
        match self.bits {
            false => Ct32bit2::Disabled,
            true => Ct32bit2::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ct32bit2::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ct32bit2::Enabled
    }
}
#[doc = "Field `CT32BIT2` writer - no description available"]
pub type Ct32bit2W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2>;
impl<'a, REG> Ct32bit2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ct32bit4> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4` reader - no description available"]
pub type Ct32bit4R = crate::BitReader<Ct32bit4>;
impl Ct32bit4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit4 {
        match self.bits {
            false => Ct32bit4::Disabled,
            true => Ct32bit4::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ct32bit4::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ct32bit4::Enabled
    }
}
#[doc = "Field `CT32BIT4` writer - no description available"]
pub type Ct32bit4W<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4>;
impl<'a, REG> Ct32bit4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsEventTimerWu {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<OsEventTimerWu> for bool {
    #[inline(always)]
    fn from(variant: OsEventTimerWu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OS_EVENT_TIMER_WU` reader - no description available"]
pub type OsEventTimerWuR = crate::BitReader<OsEventTimerWu>;
impl OsEventTimerWuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OsEventTimerWu {
        match self.bits {
            false => OsEventTimerWu::Disabled,
            true => OsEventTimerWu::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OsEventTimerWu::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OsEventTimerWu::Enabled
    }
}
#[doc = "Field `OS_EVENT_TIMER_WU` writer - no description available"]
pub type OsEventTimerWuW<'a, REG> = crate::BitWriter<'a, REG, OsEventTimerWu>;
impl<'a, REG> OsEventTimerWuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerWu::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerWu::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexspi {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexspi> for bool {
    #[inline(always)]
    fn from(variant: Flexspi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI` reader - no description available"]
pub type FlexspiR = crate::BitReader<Flexspi>;
impl FlexspiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexspi {
        match self.bits {
            false => Flexspi::Disabled,
            true => Flexspi::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexspi::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexspi::Enabled
    }
}
#[doc = "Field `FLEXSPI` writer - no description available"]
pub type FlexspiW<'a, REG> = crate::BitWriter<'a, REG, Flexspi>;
impl<'a, REG> FlexspiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexspi::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexspi::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm6> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6` reader - no description available"]
pub type Flexcomm6R = crate::BitReader<Flexcomm6>;
impl Flexcomm6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6 {
        match self.bits {
            false => Flexcomm6::Disabled,
            true => Flexcomm6::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm6::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm6::Enabled
    }
}
#[doc = "Field `FLEXCOMM6` writer - no description available"]
pub type Flexcomm6W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6>;
impl<'a, REG> Flexcomm6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Flexcomm7> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7` reader - no description available"]
pub type Flexcomm7R = crate::BitReader<Flexcomm7>;
impl Flexcomm7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7 {
        match self.bits {
            false => Flexcomm7::Disabled,
            true => Flexcomm7::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcomm7::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcomm7::Enabled
    }
}
#[doc = "Field `FLEXCOMM7` writer - no description available"]
pub type Flexcomm7W<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7>;
impl<'a, REG> Flexcomm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Sdio0> for bool {
    #[inline(always)]
    fn from(variant: Sdio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0` reader - no description available"]
pub type Sdio0R = crate::BitReader<Sdio0>;
impl Sdio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio0 {
        match self.bits {
            false => Sdio0::Disabled,
            true => Sdio0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sdio0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sdio0::Enabled
    }
}
#[doc = "Field `SDIO0` writer - no description available"]
pub type Sdio0W<'a, REG> = crate::BitWriter<'a, REG, Sdio0>;
impl<'a, REG> Sdio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Sdio1> for bool {
    #[inline(always)]
    fn from(variant: Sdio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1` reader - no description available"]
pub type Sdio1R = crate::BitReader<Sdio1>;
impl Sdio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio1 {
        match self.bits {
            false => Sdio1::Disabled,
            true => Sdio1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sdio1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sdio1::Enabled
    }
}
#[doc = "Field `SDIO1` writer - no description available"]
pub type Sdio1W<'a, REG> = crate::BitWriter<'a, REG, Sdio1>;
impl<'a, REG> Sdio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShsgpioInt0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<ShsgpioInt0> for bool {
    #[inline(always)]
    fn from(variant: ShsgpioInt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO_INT0` reader - no description available"]
pub type ShsgpioInt0R = crate::BitReader<ShsgpioInt0>;
impl ShsgpioInt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShsgpioInt0 {
        match self.bits {
            false => ShsgpioInt0::Disabled,
            true => ShsgpioInt0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ShsgpioInt0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ShsgpioInt0::Enabled
    }
}
#[doc = "Field `SHSGPIO_INT0` writer - no description available"]
pub type ShsgpioInt0W<'a, REG> = crate::BitWriter<'a, REG, ShsgpioInt0>;
impl<'a, REG> ShsgpioInt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShsgpioInt1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<ShsgpioInt1> for bool {
    #[inline(always)]
    fn from(variant: ShsgpioInt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO_INT1` reader - no description available"]
pub type ShsgpioInt1R = crate::BitReader<ShsgpioInt1>;
impl ShsgpioInt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShsgpioInt1 {
        match self.bits {
            false => ShsgpioInt1::Disabled,
            true => ShsgpioInt1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ShsgpioInt1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ShsgpioInt1::Enabled
    }
}
#[doc = "Field `SHSGPIO_INT1` writer - no description available"]
pub type ShsgpioInt1W<'a, REG> = crate::BitWriter<'a, REG, ShsgpioInt1>;
impl<'a, REG> ShsgpioInt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShsgpioInt1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<I3c0> for bool {
    #[inline(always)]
    fn from(variant: I3c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0` reader - no description available"]
pub type I3c0R = crate::BitReader<I3c0>;
impl I3c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3c0 {
        match self.bits {
            false => I3c0::Disabled,
            true => I3c0::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I3c0::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I3c0::Enabled
    }
}
#[doc = "Field `I3C0` writer - no description available"]
pub type I3c0W<'a, REG> = crate::BitWriter<'a, REG, I3c0>;
impl<'a, REG> I3c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbIrq {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<UsbIrq> for bool {
    #[inline(always)]
    fn from(variant: UsbIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IRQ` reader - no description available"]
pub type UsbIrqR = crate::BitReader<UsbIrq>;
impl UsbIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbIrq {
        match self.bits {
            false => UsbIrq::Disabled,
            true => UsbIrq::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UsbIrq::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UsbIrq::Enabled
    }
}
#[doc = "Field `USB_IRQ` writer - no description available"]
pub type UsbIrqW<'a, REG> = crate::BitWriter<'a, REG, UsbIrq>;
impl<'a, REG> UsbIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbIrq::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbIrq::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbNeedclk {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<UsbNeedclk> for bool {
    #[inline(always)]
    fn from(variant: UsbNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_NEEDCLK` reader - no description available"]
pub type UsbNeedclkR = crate::BitReader<UsbNeedclk>;
impl UsbNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbNeedclk {
        match self.bits {
            false => UsbNeedclk::Disabled,
            true => UsbNeedclk::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UsbNeedclk::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UsbNeedclk::Enabled
    }
}
#[doc = "Field `USB_NEEDCLK` writer - no description available"]
pub type UsbNeedclkW<'a, REG> = crate::BitWriter<'a, REG, UsbNeedclk>;
impl<'a, REG> UsbNeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbNeedclk::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UsbNeedclk::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1 {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Dmac1> for bool {
    #[inline(always)]
    fn from(variant: Dmac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1` reader - no description available"]
pub type Dmac1R = crate::BitReader<Dmac1>;
impl Dmac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1 {
        match self.bits {
            false => Dmac1::Disabled,
            true => Dmac1::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1::Enabled
    }
}
#[doc = "Field `DMAC1` writer - no description available"]
pub type Dmac1W<'a, REG> = crate::BitWriter<'a, REG, Dmac1>;
impl<'a, REG> Dmac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puf {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Puf> for bool {
    #[inline(always)]
    fn from(variant: Puf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` reader - no description available"]
pub type PufR = crate::BitReader<Puf>;
impl PufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Puf {
        match self.bits {
            false => Puf::Disabled,
            true => Puf::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Puf::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Puf::Enabled
    }
}
#[doc = "Field `PUF` writer - no description available"]
pub type PufW<'a, REG> = crate::BitWriter<'a, REG, Puf>;
impl<'a, REG> PufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Puf::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powerquad {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Powerquad> for bool {
    #[inline(always)]
    fn from(variant: Powerquad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` reader - no description available"]
pub type PowerquadR = crate::BitReader<Powerquad>;
impl PowerquadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Powerquad {
        match self.bits {
            false => Powerquad::Disabled,
            true => Powerquad::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Powerquad::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Powerquad::Enabled
    }
}
#[doc = "Field `POWERQUAD` writer - no description available"]
pub type PowerquadW<'a, REG> = crate::BitWriter<'a, REG, Powerquad>;
impl<'a, REG> PowerquadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Powerquad::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` reader - no description available"]
pub type CasperR = crate::BitReader<Casper>;
impl CasperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Casper {
        match self.bits {
            false => Casper::Disabled,
            true => Casper::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Casper::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Casper::Enabled
    }
}
#[doc = "Field `CASPER` writer - no description available"]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmic {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Pmic> for bool {
    #[inline(always)]
    fn from(variant: Pmic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC` reader - no description available"]
pub type PmicR = crate::BitReader<Pmic>;
impl PmicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmic {
        match self.bits {
            false => Pmic::Disabled,
            true => Pmic::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pmic::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pmic::Enabled
    }
}
#[doc = "Field `PMIC` writer - no description available"]
pub type PmicW<'a, REG> = crate::BitWriter<'a, REG, Pmic>;
impl<'a, REG> PmicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pmic::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pmic::Enabled)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sha {
    #[doc = "0: disbale"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Sha> for bool {
    #[inline(always)]
    fn from(variant: Sha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHA` reader - no description available"]
pub type ShaR = crate::BitReader<Sha>;
impl ShaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sha {
        match self.bits {
            false => Sha::Disabled,
            true => Sha::Enabled,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sha::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sha::Enabled
    }
}
#[doc = "Field `SHA` writer - no description available"]
pub type ShaW<'a, REG> = crate::BitWriter<'a, REG, Sha>;
impl<'a, REG> ShaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sha::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sha::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn rtc_lite0_alarm_or_wakeup(&self) -> RtcLite0AlarmOrWakeupR {
        RtcLite0AlarmOrWakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq4(&self) -> GpioInt0Irq4R {
        GpioInt0Irq4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq5(&self) -> GpioInt0Irq5R {
        GpioInt0Irq5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq6(&self) -> GpioInt0Irq6R {
        GpioInt0Irq6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn gpio_int0_irq7(&self) -> GpioInt0Irq7R {
        GpioInt0Irq7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn ct32bit2(&self) -> Ct32bit2R {
        Ct32bit2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ct32bit4(&self) -> Ct32bit4R {
        Ct32bit4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn os_event_timer_wu(&self) -> OsEventTimerWuR {
        OsEventTimerWuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn flexspi(&self) -> FlexspiR {
        FlexspiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn flexcomm6(&self) -> Flexcomm6R {
        Flexcomm6R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn flexcomm7(&self) -> Flexcomm7R {
        Flexcomm7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sdio0(&self) -> Sdio0R {
        Sdio0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn sdio1(&self) -> Sdio1R {
        Sdio1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn shsgpio_int0(&self) -> ShsgpioInt0R {
        ShsgpioInt0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn shsgpio_int1(&self) -> ShsgpioInt1R {
        ShsgpioInt1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn i3c0(&self) -> I3c0R {
        I3c0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usb_irq(&self) -> UsbIrqR {
        UsbIrqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn usb_needclk(&self) -> UsbNeedclkR {
        UsbNeedclkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn dmac1(&self) -> Dmac1R {
        Dmac1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn puf(&self) -> PufR {
        PufR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn powerquad(&self) -> PowerquadR {
        PowerquadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn casper(&self) -> CasperR {
        CasperR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pmic(&self) -> PmicR {
        PmicR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn sha(&self) -> ShaR {
        ShaR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTEN1")
            .field(
                "rtc_lite0_alarm_or_wakeup",
                &self.rtc_lite0_alarm_or_wakeup(),
            )
            .field("mu", &self.mu())
            .field("gpio_int0_irq4", &self.gpio_int0_irq4())
            .field("gpio_int0_irq5", &self.gpio_int0_irq5())
            .field("gpio_int0_irq6", &self.gpio_int0_irq6())
            .field("gpio_int0_irq7", &self.gpio_int0_irq7())
            .field("ct32bit2", &self.ct32bit2())
            .field("ct32bit4", &self.ct32bit4())
            .field("os_event_timer_wu", &self.os_event_timer_wu())
            .field("flexspi", &self.flexspi())
            .field("flexcomm6", &self.flexcomm6())
            .field("flexcomm7", &self.flexcomm7())
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("shsgpio_int0", &self.shsgpio_int0())
            .field("shsgpio_int1", &self.shsgpio_int1())
            .field("i3c0", &self.i3c0())
            .field("usb_irq", &self.usb_irq())
            .field("usb_needclk", &self.usb_needclk())
            .field("dmac1", &self.dmac1())
            .field("puf", &self.puf())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("pmic", &self.pmic())
            .field("sha", &self.sha())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_lite0_alarm_or_wakeup(&mut self) -> RtcLite0AlarmOrWakeupW<Starten1Spec> {
        RtcLite0AlarmOrWakeupW::new(self, 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<Starten1Spec> {
        MuW::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq4(&mut self) -> GpioInt0Irq4W<Starten1Spec> {
        GpioInt0Irq4W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq5(&mut self) -> GpioInt0Irq5W<Starten1Spec> {
        GpioInt0Irq5W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq6(&mut self) -> GpioInt0Irq6W<Starten1Spec> {
        GpioInt0Irq6W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq7(&mut self) -> GpioInt0Irq7W<Starten1Spec> {
        GpioInt0Irq7W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2(&mut self) -> Ct32bit2W<Starten1Spec> {
        Ct32bit2W::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4(&mut self) -> Ct32bit4W<Starten1Spec> {
        Ct32bit4W::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn os_event_timer_wu(&mut self) -> OsEventTimerWuW<Starten1Spec> {
        OsEventTimerWuW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi(&mut self) -> FlexspiW<Starten1Spec> {
        FlexspiW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6(&mut self) -> Flexcomm6W<Starten1Spec> {
        Flexcomm6W::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7(&mut self) -> Flexcomm7W<Starten1Spec> {
        Flexcomm7W::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> Sdio0W<Starten1Spec> {
        Sdio0W::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> Sdio1W<Starten1Spec> {
        Sdio1W::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio_int0(&mut self) -> ShsgpioInt0W<Starten1Spec> {
        ShsgpioInt0W::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio_int1(&mut self) -> ShsgpioInt1W<Starten1Spec> {
        ShsgpioInt1W::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0(&mut self) -> I3c0W<Starten1Spec> {
        I3c0W::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usb_irq(&mut self) -> UsbIrqW<Starten1Spec> {
        UsbIrqW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usb_needclk(&mut self) -> UsbNeedclkW<Starten1Spec> {
        UsbNeedclkW::new(self, 19)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1(&mut self) -> Dmac1W<Starten1Spec> {
        Dmac1W::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PufW<Starten1Spec> {
        PufW::new(self, 23)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> PowerquadW<Starten1Spec> {
        PowerquadW::new(self, 24)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CasperW<Starten1Spec> {
        CasperW::new(self, 25)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmic(&mut self) -> PmicW<Starten1Spec> {
        PmicW::new(self, 26)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sha(&mut self) -> ShaW<Starten1Spec> {
        ShaW::new(self, 27)
    }
}
#[doc = "Start enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`starten1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`starten1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Starten1Spec;
impl crate::RegisterSpec for Starten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`starten1::R`](R) reader structure"]
impl crate::Readable for Starten1Spec {}
#[doc = "`write(|w| ..)` method takes [`starten1::W`](W) writer structure"]
impl crate::Writable for Starten1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTEN1 to value 0"]
impl crate::Resettable for Starten1Spec {
    const RESET_VALUE: u32 = 0;
}
