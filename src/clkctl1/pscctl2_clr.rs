#[doc = "Register `PSCCTL2_CLR` writer"]
pub type W = crate::W<Pscctl2ClrSpec>;
#[doc = "ct32bit timer 0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Ct32bit0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_CLK_CLR` writer - ct32bit timer 0 clock clear"]
pub type Ct32bit0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0ClkClr>;
impl<'a, REG> Ct32bit0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0ClkClr::ClrClock)
    }
}
#[doc = "ct32bit timer 1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Ct32bit1ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_CLK_CLR` writer - ct32bit timer 1 clock clear"]
pub type Ct32bit1ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1ClkClr>;
impl<'a, REG> Ct32bit1ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1ClkClr::ClrClock)
    }
}
#[doc = "ct32bit timer 2 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Ct32bit2ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_CLK_CLR` writer - ct32bit timer 2 clock clear"]
pub type Ct32bit2ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2ClkClr>;
impl<'a, REG> Ct32bit2ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2ClkClr::ClrClock)
    }
}
#[doc = "ct32bit timer 3 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Ct32bit3ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_CLK_CLR` writer - ct32bit timer 3 clock clear"]
pub type Ct32bit3ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3ClkClr>;
impl<'a, REG> Ct32bit3ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3ClkClr::ClrClock)
    }
}
#[doc = "ct32bit timer 4 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Ct32bit4ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_CLK_CLR` writer - ct32bit timer 4 clock clear"]
pub type Ct32bit4ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4ClkClr>;
impl<'a, REG> Ct32bit4ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4ClkClr::ClrClock)
    }
}
#[doc = "rtc lite clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcLiteClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<RtcLiteClkClr> for bool {
    #[inline(always)]
    fn from(variant: RtcLiteClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_LITE_CLK_CLR` writer - rtc lite clock clear"]
pub type RtcLiteClkClrW<'a, REG> = crate::BitWriter<'a, REG, RtcLiteClkClr>;
impl<'a, REG> RtcLiteClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RtcLiteClkClr::ClrClock)
    }
}
#[doc = "mrt0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Mrt0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Mrt0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_CLK_CLR` writer - mrt0 clock clear"]
pub type Mrt0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Mrt0ClkClr>;
impl<'a, REG> Mrt0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0ClkClr::ClrClock)
    }
}
#[doc = "wdt1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Wwdt1ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_CLK_CLR` writer - wdt1 clock clear"]
pub type Wwdt1ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1ClkClr>;
impl<'a, REG> Wwdt1ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1ClkClr::ClrClock)
    }
}
#[doc = "i3c0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<I3c0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: I3c0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_CLK_CLR` writer - i3c0 clock clear"]
pub type I3c0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, I3c0ClkClr>;
impl<'a, REG> I3c0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0ClkClr::ClrClock)
    }
}
#[doc = "GPIOINTCTL clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<GpiointctlClkClr> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_CLK_CLR` writer - GPIOINTCTL clock clear"]
pub type GpiointctlClkClrW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlClkClr>;
impl<'a, REG> GpiointctlClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlClkClr::ClrClock)
    }
}
#[doc = "PIMCTL clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<PimctlClkClr> for bool {
    #[inline(always)]
    fn from(variant: PimctlClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_CLK_CLR` writer - PIMCTL clock clear"]
pub type PimctlClkClrW<'a, REG> = crate::BitWriter<'a, REG, PimctlClkClr>;
impl<'a, REG> PimctlClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlClkClr::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl2ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ct32bit timer 0 clock clear"]
    #[inline(always)]
    pub fn ct32bit0_clk_clr(&mut self) -> Ct32bit0ClkClrW<Pscctl2ClrSpec> {
        Ct32bit0ClkClrW::new(self, 0)
    }
    #[doc = "Bit 1 - ct32bit timer 1 clock clear"]
    #[inline(always)]
    pub fn ct32bit1_clk_clr(&mut self) -> Ct32bit1ClkClrW<Pscctl2ClrSpec> {
        Ct32bit1ClkClrW::new(self, 1)
    }
    #[doc = "Bit 2 - ct32bit timer 2 clock clear"]
    #[inline(always)]
    pub fn ct32bit2_clk_clr(&mut self) -> Ct32bit2ClkClrW<Pscctl2ClrSpec> {
        Ct32bit2ClkClrW::new(self, 2)
    }
    #[doc = "Bit 3 - ct32bit timer 3 clock clear"]
    #[inline(always)]
    pub fn ct32bit3_clk_clr(&mut self) -> Ct32bit3ClkClrW<Pscctl2ClrSpec> {
        Ct32bit3ClkClrW::new(self, 3)
    }
    #[doc = "Bit 4 - ct32bit timer 4 clock clear"]
    #[inline(always)]
    pub fn ct32bit4_clk_clr(&mut self) -> Ct32bit4ClkClrW<Pscctl2ClrSpec> {
        Ct32bit4ClkClrW::new(self, 4)
    }
    #[doc = "Bit 7 - rtc lite clock clear"]
    #[inline(always)]
    pub fn rtc_lite_clk_clr(&mut self) -> RtcLiteClkClrW<Pscctl2ClrSpec> {
        RtcLiteClkClrW::new(self, 7)
    }
    #[doc = "Bit 8 - mrt0 clock clear"]
    #[inline(always)]
    pub fn mrt0_clk_clr(&mut self) -> Mrt0ClkClrW<Pscctl2ClrSpec> {
        Mrt0ClkClrW::new(self, 8)
    }
    #[doc = "Bit 10 - wdt1 clock clear"]
    #[inline(always)]
    pub fn wwdt1_clk_clr(&mut self) -> Wwdt1ClkClrW<Pscctl2ClrSpec> {
        Wwdt1ClkClrW::new(self, 10)
    }
    #[doc = "Bit 16 - i3c0 clock clear"]
    #[inline(always)]
    pub fn i3c0_clk_clr(&mut self) -> I3c0ClkClrW<Pscctl2ClrSpec> {
        I3c0ClkClrW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL clock clear"]
    #[inline(always)]
    pub fn gpiointctl_clk_clr(&mut self) -> GpiointctlClkClrW<Pscctl2ClrSpec> {
        GpiointctlClkClrW::new(self, 30)
    }
    #[doc = "Bit 31 - PIMCTL clock clear"]
    #[inline(always)]
    pub fn pimctl_clk_clr(&mut self) -> PimctlClkClrW<Pscctl2ClrSpec> {
        PimctlClkClrW::new(self, 31)
    }
}
#[doc = "clock clear register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl2_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl2ClrSpec;
impl crate::RegisterSpec for Pscctl2ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl2_clr::W`](W) writer structure"]
impl crate::Writable for Pscctl2ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL2_CLR to value 0"]
impl crate::Resettable for Pscctl2ClrSpec {
    const RESET_VALUE: u32 = 0;
}
