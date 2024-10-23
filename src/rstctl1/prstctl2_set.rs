#[doc = "Register `PRSTCTL2_SET` writer"]
pub type W = crate::W<Prstctl2SetSpec>;
#[doc = "CT32BIT0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Ct32bit0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_RST_SET` writer - CT32BIT0 reset set"]
pub type Ct32bit0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0RstSet>;
impl<'a, REG> Ct32bit0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0RstSet::SetReset)
    }
}
#[doc = "CT32BIT1 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Ct32bit1RstSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_RST_SET` writer - CT32BIT1 reset set"]
pub type Ct32bit1RstSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1RstSet>;
impl<'a, REG> Ct32bit1RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1RstSet::SetReset)
    }
}
#[doc = "CT32BIT2 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Ct32bit2RstSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_RST_SET` writer - CT32BIT2 reset set"]
pub type Ct32bit2RstSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2RstSet>;
impl<'a, REG> Ct32bit2RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2RstSet::SetReset)
    }
}
#[doc = "CT32BIT3 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Ct32bit3RstSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_RST_SET` writer - CT32BIT3 reset set"]
pub type Ct32bit3RstSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3RstSet>;
impl<'a, REG> Ct32bit3RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3RstSet::SetReset)
    }
}
#[doc = "CT32BIT4 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Ct32bit4RstSet> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_RST_SET` writer - CT32BIT4 reset set"]
pub type Ct32bit4RstSetW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4RstSet>;
impl<'a, REG> Ct32bit4RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4RstSet::SetReset)
    }
}
#[doc = "MRT0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Mrt0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Mrt0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_RST_SET` writer - MRT0 reset set"]
pub type Mrt0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Mrt0RstSet>;
impl<'a, REG> Mrt0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0RstSet::SetReset)
    }
}
#[doc = "WWDT1 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Wwdt1RstSet> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_RST_SET` writer - WWDT1 reset set"]
pub type Wwdt1RstSetW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1RstSet>;
impl<'a, REG> Wwdt1RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1RstSet::SetReset)
    }
}
#[doc = "I3C0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<I3c0RstSet> for bool {
    #[inline(always)]
    fn from(variant: I3c0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_RST_SET` writer - I3C0 reset set"]
pub type I3c0RstSetW<'a, REG> = crate::BitWriter<'a, REG, I3c0RstSet>;
impl<'a, REG> I3c0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0RstSet::SetReset)
    }
}
#[doc = "GPIOINTCTL reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<GpiointctlRstSet> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_RST_SET` writer - GPIOINTCTL reset set"]
pub type GpiointctlRstSetW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlRstSet>;
impl<'a, REG> GpiointctlRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRstSet::SetReset)
    }
}
#[doc = "PMC reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<PimctlRstSet> for bool {
    #[inline(always)]
    fn from(variant: PimctlRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_RST_SET` writer - PMC reset set"]
pub type PimctlRstSetW<'a, REG> = crate::BitWriter<'a, REG, PimctlRstSet>;
impl<'a, REG> PimctlRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRstSet::SetReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl2SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0_rst_set(&mut self) -> Ct32bit0RstSetW<Prstctl2SetSpec> {
        Ct32bit0RstSetW::new(self, 0)
    }
    #[doc = "Bit 1 - CT32BIT1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1_rst_set(&mut self) -> Ct32bit1RstSetW<Prstctl2SetSpec> {
        Ct32bit1RstSetW::new(self, 1)
    }
    #[doc = "Bit 2 - CT32BIT2 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2_rst_set(&mut self) -> Ct32bit2RstSetW<Prstctl2SetSpec> {
        Ct32bit2RstSetW::new(self, 2)
    }
    #[doc = "Bit 3 - CT32BIT3 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3_rst_set(&mut self) -> Ct32bit3RstSetW<Prstctl2SetSpec> {
        Ct32bit3RstSetW::new(self, 3)
    }
    #[doc = "Bit 4 - CT32BIT4 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4_rst_set(&mut self) -> Ct32bit4RstSetW<Prstctl2SetSpec> {
        Ct32bit4RstSetW::new(self, 4)
    }
    #[doc = "Bit 8 - MRT0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0_rst_set(&mut self) -> Mrt0RstSetW<Prstctl2SetSpec> {
        Mrt0RstSetW::new(self, 8)
    }
    #[doc = "Bit 10 - WWDT1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1_rst_set(&mut self) -> Wwdt1RstSetW<Prstctl2SetSpec> {
        Wwdt1RstSetW::new(self, 10)
    }
    #[doc = "Bit 16 - I3C0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_rst_set(&mut self) -> I3c0RstSetW<Prstctl2SetSpec> {
        I3c0RstSetW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset set"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl_rst_set(&mut self) -> GpiointctlRstSetW<Prstctl2SetSpec> {
        GpiointctlRstSetW::new(self, 30)
    }
    #[doc = "Bit 31 - PMC reset set"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl_rst_set(&mut self) -> PimctlRstSetW<Prstctl2SetSpec> {
        PimctlRstSetW::new(self, 31)
    }
}
#[doc = "peripheral reset set register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl2SetSpec;
impl crate::RegisterSpec for Prstctl2SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl2_set::W`](W) writer structure"]
impl crate::Writable for Prstctl2SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL2_SET to value 0"]
impl crate::Resettable for Prstctl2SetSpec {
    const RESET_VALUE: u32 = 0;
}
