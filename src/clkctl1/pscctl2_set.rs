#[doc = "Register `PSCCTL2_SET` writer"]
pub type W = crate::W<Pscctl2SetSpec>;
#[doc = "ct32bit timer 0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Ct32bit0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_CLK_SET` writer - ct32bit timer 0 clock set"]
pub type Ct32bit0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0ClkSet>;
impl<'a, REG> Ct32bit0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0ClkSet::SetClock)
    }
}
#[doc = "ct32bit timer 1 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Ct32bit1ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_CLK_SET` writer - ct32bit timer 1 clock set"]
pub type Ct32bit1ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1ClkSet>;
impl<'a, REG> Ct32bit1ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1ClkSet::SetClock)
    }
}
#[doc = "ct32bit timer 2 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Ct32bit2ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_CLK_SET` writer - ct32bit timer 2 clock set"]
pub type Ct32bit2ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2ClkSet>;
impl<'a, REG> Ct32bit2ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2ClkSet::SetClock)
    }
}
#[doc = "ct32bit timer 3 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Ct32bit3ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_CLK_SET` writer - ct32bit timer 3 clock set"]
pub type Ct32bit3ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3ClkSet>;
impl<'a, REG> Ct32bit3ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3ClkSet::SetClock)
    }
}
#[doc = "ct32bit timer 4 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Ct32bit4ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_CLK_SET` writer - ct32bit timer 4 clock set"]
pub type Ct32bit4ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4ClkSet>;
impl<'a, REG> Ct32bit4ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4ClkSet::SetClock)
    }
}
#[doc = "rtc lite clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcLiteClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<RtcLiteClkSet> for bool {
    #[inline(always)]
    fn from(variant: RtcLiteClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_LITE_CLK_SET` writer - rtc lite clock set"]
pub type RtcLiteClkSetW<'a, REG> = crate::BitWriter<'a, REG, RtcLiteClkSet>;
impl<'a, REG> RtcLiteClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClkSet::SetClock)
    }
}
#[doc = "mrt0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Mrt0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Mrt0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_CLK_SET` writer - mrt0 clock set"]
pub type Mrt0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Mrt0ClkSet>;
impl<'a, REG> Mrt0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0ClkSet::SetClock)
    }
}
#[doc = "wdt1 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Wwdt1ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_CLK_SET` writer - wdt1 clock set"]
pub type Wwdt1ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1ClkSet>;
impl<'a, REG> Wwdt1ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1ClkSet::SetClock)
    }
}
#[doc = "i3c0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<I3c0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: I3c0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_CLK_SET` writer - i3c0 clock set"]
pub type I3c0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, I3c0ClkSet>;
impl<'a, REG> I3c0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0ClkSet::SetClock)
    }
}
#[doc = "GPIOINTCTL clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<GpiointctlClkSet> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_CLK_SET` writer - GPIOINTCTL clock set"]
pub type GpiointctlClkSetW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlClkSet>;
impl<'a, REG> GpiointctlClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClkSet::SetClock)
    }
}
#[doc = "PIMCTL clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<PimctlClkSet> for bool {
    #[inline(always)]
    fn from(variant: PimctlClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_CLK_SET` writer - PIMCTL clock set"]
pub type PimctlClkSetW<'a, REG> = crate::BitWriter<'a, REG, PimctlClkSet>;
impl<'a, REG> PimctlClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClkSet::SetClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl2SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ct32bit timer 0 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0_clk_set(&mut self) -> Ct32bit0ClkSetW<Pscctl2SetSpec> {
        Ct32bit0ClkSetW::new(self, 0)
    }
    #[doc = "Bit 1 - ct32bit timer 1 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1_clk_set(&mut self) -> Ct32bit1ClkSetW<Pscctl2SetSpec> {
        Ct32bit1ClkSetW::new(self, 1)
    }
    #[doc = "Bit 2 - ct32bit timer 2 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2_clk_set(&mut self) -> Ct32bit2ClkSetW<Pscctl2SetSpec> {
        Ct32bit2ClkSetW::new(self, 2)
    }
    #[doc = "Bit 3 - ct32bit timer 3 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3_clk_set(&mut self) -> Ct32bit3ClkSetW<Pscctl2SetSpec> {
        Ct32bit3ClkSetW::new(self, 3)
    }
    #[doc = "Bit 4 - ct32bit timer 4 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4_clk_set(&mut self) -> Ct32bit4ClkSetW<Pscctl2SetSpec> {
        Ct32bit4ClkSetW::new(self, 4)
    }
    #[doc = "Bit 7 - rtc lite clock set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_lite_clk_set(&mut self) -> RtcLiteClkSetW<Pscctl2SetSpec> {
        RtcLiteClkSetW::new(self, 7)
    }
    #[doc = "Bit 8 - mrt0 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0_clk_set(&mut self) -> Mrt0ClkSetW<Pscctl2SetSpec> {
        Mrt0ClkSetW::new(self, 8)
    }
    #[doc = "Bit 10 - wdt1 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1_clk_set(&mut self) -> Wwdt1ClkSetW<Pscctl2SetSpec> {
        Wwdt1ClkSetW::new(self, 10)
    }
    #[doc = "Bit 16 - i3c0 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_clk_set(&mut self) -> I3c0ClkSetW<Pscctl2SetSpec> {
        I3c0ClkSetW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL clock set"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl_clk_set(&mut self) -> GpiointctlClkSetW<Pscctl2SetSpec> {
        GpiointctlClkSetW::new(self, 30)
    }
    #[doc = "Bit 31 - PIMCTL clock set"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl_clk_set(&mut self) -> PimctlClkSetW<Pscctl2SetSpec> {
        PimctlClkSetW::new(self, 31)
    }
}
#[doc = "clock set register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl2SetSpec;
impl crate::RegisterSpec for Pscctl2SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl2_set::W`](W) writer structure"]
impl crate::Writable for Pscctl2SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL2_SET to value 0"]
impl crate::Resettable for Pscctl2SetSpec {
    const RESET_VALUE: u32 = 0;
}
