#[doc = "Register `PSCCTL2` reader"]
pub type R = crate::R<Pscctl2Spec>;
#[doc = "Register `PSCCTL2` writer"]
pub type W = crate::W<Pscctl2Spec>;
#[doc = "ct32bit timer 0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Ct32bit0Clk> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_CLK` reader - ct32bit timer 0 clock control"]
pub type Ct32bit0ClkR = crate::BitReader<Ct32bit0Clk>;
impl Ct32bit0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit0Clk {
        match self.bits {
            false => Ct32bit0Clk::DisableClock,
            true => Ct32bit0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Ct32bit0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Ct32bit0Clk::EnableClock
    }
}
#[doc = "Field `CT32BIT0_CLK` writer - ct32bit timer 0 clock control"]
pub type Ct32bit0ClkW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0Clk>;
impl<'a, REG> Ct32bit0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0Clk::EnableClock)
    }
}
#[doc = "ct32bit timer 1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Ct32bit1Clk> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_CLK` reader - ct32bit timer 1 clock control"]
pub type Ct32bit1ClkR = crate::BitReader<Ct32bit1Clk>;
impl Ct32bit1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit1Clk {
        match self.bits {
            false => Ct32bit1Clk::DisableClock,
            true => Ct32bit1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Ct32bit1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Ct32bit1Clk::EnableClock
    }
}
#[doc = "Field `CT32BIT1_CLK` writer - ct32bit timer 1 clock control"]
pub type Ct32bit1ClkW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1Clk>;
impl<'a, REG> Ct32bit1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1Clk::EnableClock)
    }
}
#[doc = "ct32bit timer 2 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Ct32bit2Clk> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_CLK` reader - ct32bit timer 2 clock control"]
pub type Ct32bit2ClkR = crate::BitReader<Ct32bit2Clk>;
impl Ct32bit2ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit2Clk {
        match self.bits {
            false => Ct32bit2Clk::DisableClock,
            true => Ct32bit2Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Ct32bit2Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Ct32bit2Clk::EnableClock
    }
}
#[doc = "Field `CT32BIT2_CLK` writer - ct32bit timer 2 clock control"]
pub type Ct32bit2ClkW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2Clk>;
impl<'a, REG> Ct32bit2ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2Clk::EnableClock)
    }
}
#[doc = "ct32bit timer 3 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Ct32bit3Clk> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_CLK` reader - ct32bit timer 3 clock control"]
pub type Ct32bit3ClkR = crate::BitReader<Ct32bit3Clk>;
impl Ct32bit3ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit3Clk {
        match self.bits {
            false => Ct32bit3Clk::DisableClock,
            true => Ct32bit3Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Ct32bit3Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Ct32bit3Clk::EnableClock
    }
}
#[doc = "Field `CT32BIT3_CLK` writer - ct32bit timer 3 clock control"]
pub type Ct32bit3ClkW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3Clk>;
impl<'a, REG> Ct32bit3ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3Clk::EnableClock)
    }
}
#[doc = "ct32bit timer 4 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Ct32bit4Clk> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_CLK` reader - ct32bit timer 4 clock control"]
pub type Ct32bit4ClkR = crate::BitReader<Ct32bit4Clk>;
impl Ct32bit4ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit4Clk {
        match self.bits {
            false => Ct32bit4Clk::DisableClock,
            true => Ct32bit4Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Ct32bit4Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Ct32bit4Clk::EnableClock
    }
}
#[doc = "Field `CT32BIT4_CLK` writer - ct32bit timer 4 clock control"]
pub type Ct32bit4ClkW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4Clk>;
impl<'a, REG> Ct32bit4ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4Clk::EnableClock)
    }
}
#[doc = "rtc lite clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcLiteClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<RtcLiteClk> for bool {
    #[inline(always)]
    fn from(variant: RtcLiteClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_LITE_CLK` reader - rtc lite clock control"]
pub type RtcLiteClkR = crate::BitReader<RtcLiteClk>;
impl RtcLiteClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcLiteClk {
        match self.bits {
            false => RtcLiteClk::DisableClock,
            true => RtcLiteClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == RtcLiteClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == RtcLiteClk::EnableClock
    }
}
#[doc = "Field `RTC_LITE_CLK` writer - rtc lite clock control"]
pub type RtcLiteClkW<'a, REG> = crate::BitWriter<'a, REG, RtcLiteClk>;
impl<'a, REG> RtcLiteClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClk::EnableClock)
    }
}
#[doc = "mrt0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Mrt0Clk> for bool {
    #[inline(always)]
    fn from(variant: Mrt0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_CLK` reader - mrt0 clock control"]
pub type Mrt0ClkR = crate::BitReader<Mrt0Clk>;
impl Mrt0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrt0Clk {
        match self.bits {
            false => Mrt0Clk::DisableClock,
            true => Mrt0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Mrt0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Mrt0Clk::EnableClock
    }
}
#[doc = "Field `MRT0_CLK` writer - mrt0 clock control"]
pub type Mrt0ClkW<'a, REG> = crate::BitWriter<'a, REG, Mrt0Clk>;
impl<'a, REG> Mrt0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0Clk::EnableClock)
    }
}
#[doc = "wdt1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Wwdt1Clk> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_CLK` reader - wdt1 clock control"]
pub type Wwdt1ClkR = crate::BitReader<Wwdt1Clk>;
impl Wwdt1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt1Clk {
        match self.bits {
            false => Wwdt1Clk::DisableClock,
            true => Wwdt1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Wwdt1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Wwdt1Clk::EnableClock
    }
}
#[doc = "Field `WWDT1_CLK` writer - wdt1 clock control"]
pub type Wwdt1ClkW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1Clk>;
impl<'a, REG> Wwdt1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1Clk::EnableClock)
    }
}
#[doc = "i3c0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<I3c0Clk> for bool {
    #[inline(always)]
    fn from(variant: I3c0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_CLK` reader - i3c0 clock control"]
pub type I3c0ClkR = crate::BitReader<I3c0Clk>;
impl I3c0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3c0Clk {
        match self.bits {
            false => I3c0Clk::DisableClock,
            true => I3c0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == I3c0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == I3c0Clk::EnableClock
    }
}
#[doc = "Field `I3C0_CLK` writer - i3c0 clock control"]
pub type I3c0ClkW<'a, REG> = crate::BitWriter<'a, REG, I3c0Clk>;
impl<'a, REG> I3c0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Clk::EnableClock)
    }
}
#[doc = "GPIOINTCTL clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<GpiointctlClk> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_CLK` reader - GPIOINTCTL clock control"]
pub type GpiointctlClkR = crate::BitReader<GpiointctlClk>;
impl GpiointctlClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpiointctlClk {
        match self.bits {
            false => GpiointctlClk::DisableClock,
            true => GpiointctlClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == GpiointctlClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == GpiointctlClk::EnableClock
    }
}
#[doc = "Field `GPIOINTCTL_CLK` writer - GPIOINTCTL clock control"]
pub type GpiointctlClkW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlClk>;
impl<'a, REG> GpiointctlClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClk::EnableClock)
    }
}
#[doc = "PIMCTL clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<PimctlClk> for bool {
    #[inline(always)]
    fn from(variant: PimctlClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_CLK` reader - PIMCTL clock control"]
pub type PimctlClkR = crate::BitReader<PimctlClk>;
impl PimctlClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PimctlClk {
        match self.bits {
            false => PimctlClk::DisableClock,
            true => PimctlClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == PimctlClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == PimctlClk::EnableClock
    }
}
#[doc = "Field `PIMCTL_CLK` writer - PIMCTL clock control"]
pub type PimctlClkW<'a, REG> = crate::BitWriter<'a, REG, PimctlClk>;
impl<'a, REG> PimctlClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 0 - ct32bit timer 0 clock control"]
    #[inline(always)]
    pub fn ct32bit0_clk(&self) -> Ct32bit0ClkR {
        Ct32bit0ClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ct32bit timer 1 clock control"]
    #[inline(always)]
    pub fn ct32bit1_clk(&self) -> Ct32bit1ClkR {
        Ct32bit1ClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ct32bit timer 2 clock control"]
    #[inline(always)]
    pub fn ct32bit2_clk(&self) -> Ct32bit2ClkR {
        Ct32bit2ClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ct32bit timer 3 clock control"]
    #[inline(always)]
    pub fn ct32bit3_clk(&self) -> Ct32bit3ClkR {
        Ct32bit3ClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ct32bit timer 4 clock control"]
    #[inline(always)]
    pub fn ct32bit4_clk(&self) -> Ct32bit4ClkR {
        Ct32bit4ClkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - rtc lite clock control"]
    #[inline(always)]
    pub fn rtc_lite_clk(&self) -> RtcLiteClkR {
        RtcLiteClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - mrt0 clock control"]
    #[inline(always)]
    pub fn mrt0_clk(&self) -> Mrt0ClkR {
        Mrt0ClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - wdt1 clock control"]
    #[inline(always)]
    pub fn wwdt1_clk(&self) -> Wwdt1ClkR {
        Wwdt1ClkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - i3c0 clock control"]
    #[inline(always)]
    pub fn i3c0_clk(&self) -> I3c0ClkR {
        I3c0ClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIOINTCTL clock control"]
    #[inline(always)]
    pub fn gpiointctl_clk(&self) -> GpiointctlClkR {
        GpiointctlClkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PIMCTL clock control"]
    #[inline(always)]
    pub fn pimctl_clk(&self) -> PimctlClkR {
        PimctlClkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL2")
            .field("ct32bit0_clk", &self.ct32bit0_clk())
            .field("ct32bit1_clk", &self.ct32bit1_clk())
            .field("ct32bit2_clk", &self.ct32bit2_clk())
            .field("ct32bit3_clk", &self.ct32bit3_clk())
            .field("ct32bit4_clk", &self.ct32bit4_clk())
            .field("rtc_lite_clk", &self.rtc_lite_clk())
            .field("mrt0_clk", &self.mrt0_clk())
            .field("wwdt1_clk", &self.wwdt1_clk())
            .field("i3c0_clk", &self.i3c0_clk())
            .field("gpiointctl_clk", &self.gpiointctl_clk())
            .field("pimctl_clk", &self.pimctl_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ct32bit timer 0 clock control"]
    #[inline(always)]
    pub fn ct32bit0_clk(&mut self) -> Ct32bit0ClkW<Pscctl2Spec> {
        Ct32bit0ClkW::new(self, 0)
    }
    #[doc = "Bit 1 - ct32bit timer 1 clock control"]
    #[inline(always)]
    pub fn ct32bit1_clk(&mut self) -> Ct32bit1ClkW<Pscctl2Spec> {
        Ct32bit1ClkW::new(self, 1)
    }
    #[doc = "Bit 2 - ct32bit timer 2 clock control"]
    #[inline(always)]
    pub fn ct32bit2_clk(&mut self) -> Ct32bit2ClkW<Pscctl2Spec> {
        Ct32bit2ClkW::new(self, 2)
    }
    #[doc = "Bit 3 - ct32bit timer 3 clock control"]
    #[inline(always)]
    pub fn ct32bit3_clk(&mut self) -> Ct32bit3ClkW<Pscctl2Spec> {
        Ct32bit3ClkW::new(self, 3)
    }
    #[doc = "Bit 4 - ct32bit timer 4 clock control"]
    #[inline(always)]
    pub fn ct32bit4_clk(&mut self) -> Ct32bit4ClkW<Pscctl2Spec> {
        Ct32bit4ClkW::new(self, 4)
    }
    #[doc = "Bit 7 - rtc lite clock control"]
    #[inline(always)]
    pub fn rtc_lite_clk(&mut self) -> RtcLiteClkW<Pscctl2Spec> {
        RtcLiteClkW::new(self, 7)
    }
    #[doc = "Bit 8 - mrt0 clock control"]
    #[inline(always)]
    pub fn mrt0_clk(&mut self) -> Mrt0ClkW<Pscctl2Spec> {
        Mrt0ClkW::new(self, 8)
    }
    #[doc = "Bit 10 - wdt1 clock control"]
    #[inline(always)]
    pub fn wwdt1_clk(&mut self) -> Wwdt1ClkW<Pscctl2Spec> {
        Wwdt1ClkW::new(self, 10)
    }
    #[doc = "Bit 16 - i3c0 clock control"]
    #[inline(always)]
    pub fn i3c0_clk(&mut self) -> I3c0ClkW<Pscctl2Spec> {
        I3c0ClkW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL clock control"]
    #[inline(always)]
    pub fn gpiointctl_clk(&mut self) -> GpiointctlClkW<Pscctl2Spec> {
        GpiointctlClkW::new(self, 30)
    }
    #[doc = "Bit 31 - PIMCTL clock control"]
    #[inline(always)]
    pub fn pimctl_clk(&mut self) -> PimctlClkW<Pscctl2Spec> {
        PimctlClkW::new(self, 31)
    }
}
#[doc = "clock control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl2Spec;
impl crate::RegisterSpec for Pscctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscctl2::R`](R) reader structure"]
impl crate::Readable for Pscctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pscctl2::W`](W) writer structure"]
impl crate::Writable for Pscctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL2 to value 0"]
impl crate::Resettable for Pscctl2Spec {
    const RESET_VALUE: u32 = 0;
}
