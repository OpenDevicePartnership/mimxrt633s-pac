#[doc = "Register `PRSTCTL2_CLR` writer"]
pub type W = crate::W<Prstctl2ClrSpec>;
#[doc = "CT32BIT0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Ct32bit0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_RST_CLR` writer - CT32BIT0 reset clear"]
pub type Ct32bit0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0RstClr>;
impl<'a, REG> Ct32bit0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0RstClr::ClrReset)
    }
}
#[doc = "CT32BIT1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Ct32bit1RstClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_RST_CLR` writer - CT32BIT1 reset clear"]
pub type Ct32bit1RstClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1RstClr>;
impl<'a, REG> Ct32bit1RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1RstClr::ClrReset)
    }
}
#[doc = "CT32BIT2 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Ct32bit2RstClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_RST_CLR` writer - CT32BIT2 reset clear"]
pub type Ct32bit2RstClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2RstClr>;
impl<'a, REG> Ct32bit2RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2RstClr::ClrReset)
    }
}
#[doc = "CT32BIT3 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Ct32bit3RstClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_RST_CLR` writer - CT32BIT3 reset clear"]
pub type Ct32bit3RstClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3RstClr>;
impl<'a, REG> Ct32bit3RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3RstClr::ClrReset)
    }
}
#[doc = "CT32BIT4 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Ct32bit4RstClr> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_RST_CLR` writer - CT32BIT4 reset clear"]
pub type Ct32bit4RstClrW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4RstClr>;
impl<'a, REG> Ct32bit4RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4RstClr::ClrReset)
    }
}
#[doc = "MRT0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Mrt0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Mrt0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_RST_CLR` writer - MRT0 reset clear"]
pub type Mrt0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Mrt0RstClr>;
impl<'a, REG> Mrt0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0RstClr::ClrReset)
    }
}
#[doc = "WWDT1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<Wwdt1RstClr> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_RST_CLR` writer - WWDT1 reset clear"]
pub type Wwdt1RstClrW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1RstClr>;
impl<'a, REG> Wwdt1RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1RstClr::ClrReset)
    }
}
#[doc = "I3C0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0RstClr {
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<I3c0RstClr> for bool {
    #[inline(always)]
    fn from(variant: I3c0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_RST_CLR` writer - I3C0 reset clear"]
pub type I3c0RstClrW<'a, REG> = crate::BitWriter<'a, REG, I3c0RstClr>;
impl<'a, REG> I3c0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0RstClr::SetReset)
    }
}
#[doc = "GPIOINTCTL reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<GpiointctlRstClr> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_RST_CLR` writer - GPIOINTCTL reset clear"]
pub type GpiointctlRstClrW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlRstClr>;
impl<'a, REG> GpiointctlRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRstClr::ClrReset)
    }
}
#[doc = "PMC reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL2 Bit"]
    ClrReset = 1,
}
impl From<PimctlRstClr> for bool {
    #[inline(always)]
    fn from(variant: PimctlRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_RST_CLR` writer - PMC reset clear"]
pub type PimctlRstClrW<'a, REG> = crate::BitWriter<'a, REG, PimctlRstClr>;
impl<'a, REG> PimctlRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRstClr::ClrReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl2ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0_rst_clr(&mut self) -> Ct32bit0RstClrW<Prstctl2ClrSpec> {
        Ct32bit0RstClrW::new(self, 0)
    }
    #[doc = "Bit 1 - CT32BIT1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1_rst_clr(&mut self) -> Ct32bit1RstClrW<Prstctl2ClrSpec> {
        Ct32bit1RstClrW::new(self, 1)
    }
    #[doc = "Bit 2 - CT32BIT2 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2_rst_clr(&mut self) -> Ct32bit2RstClrW<Prstctl2ClrSpec> {
        Ct32bit2RstClrW::new(self, 2)
    }
    #[doc = "Bit 3 - CT32BIT3 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3_rst_clr(&mut self) -> Ct32bit3RstClrW<Prstctl2ClrSpec> {
        Ct32bit3RstClrW::new(self, 3)
    }
    #[doc = "Bit 4 - CT32BIT4 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4_rst_clr(&mut self) -> Ct32bit4RstClrW<Prstctl2ClrSpec> {
        Ct32bit4RstClrW::new(self, 4)
    }
    #[doc = "Bit 8 - MRT0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0_rst_clr(&mut self) -> Mrt0RstClrW<Prstctl2ClrSpec> {
        Mrt0RstClrW::new(self, 8)
    }
    #[doc = "Bit 10 - WWDT1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1_rst_clr(&mut self) -> Wwdt1RstClrW<Prstctl2ClrSpec> {
        Wwdt1RstClrW::new(self, 10)
    }
    #[doc = "Bit 16 - I3C0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_rst_clr(&mut self) -> I3c0RstClrW<Prstctl2ClrSpec> {
        I3c0RstClrW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl_rst_clr(&mut self) -> GpiointctlRstClrW<Prstctl2ClrSpec> {
        GpiointctlRstClrW::new(self, 30)
    }
    #[doc = "Bit 31 - PMC reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl_rst_clr(&mut self) -> PimctlRstClrW<Prstctl2ClrSpec> {
        PimctlRstClrW::new(self, 31)
    }
}
#[doc = "peripheral reset clear register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl2ClrSpec;
impl crate::RegisterSpec for Prstctl2ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl2_clr::W`](W) writer structure"]
impl crate::Writable for Prstctl2ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL2_CLR to value 0"]
impl crate::Resettable for Prstctl2ClrSpec {
    const RESET_VALUE: u32 = 0;
}
