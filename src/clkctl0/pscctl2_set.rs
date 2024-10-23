#[doc = "Register `PSCCTL2_SET` writer"]
pub type W = crate::W<Pscctl2SetSpec>;
#[doc = "utick clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Utick0Clk> for bool {
    #[inline(always)]
    fn from(variant: Utick0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0_CLK` writer - utick clock set"]
pub type Utick0ClkW<'a, REG> = crate::BitWriter<'a, REG, Utick0Clk>;
impl<'a, REG> Utick0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0Clk::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0Clk::SetClock)
    }
}
#[doc = "wdt clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL2 Bit"]
    SetClock = 1,
}
impl From<Wwdt0Clk> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0_CLK` writer - wdt clock set"]
pub type Wwdt0ClkW<'a, REG> = crate::BitWriter<'a, REG, Wwdt0Clk>;
impl<'a, REG> Wwdt0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0Clk::NoEffect)
    }
    #[doc = "Sets the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0Clk::SetClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl2SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - utick clock set"]
    #[inline(always)]
    #[must_use]
    pub fn utick0_clk(&mut self) -> Utick0ClkW<Pscctl2SetSpec> {
        Utick0ClkW::new(self, 0)
    }
    #[doc = "Bit 1 - wdt clock set"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0_clk(&mut self) -> Wwdt0ClkW<Pscctl2SetSpec> {
        Wwdt0ClkW::new(self, 1)
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
