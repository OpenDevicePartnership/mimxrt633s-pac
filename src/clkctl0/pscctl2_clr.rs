#[doc = "Register `PSCCTL2_CLR` writer"]
pub type W = crate::W<Pscctl2ClrSpec>;
#[doc = "utick clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Utick0Clk> for bool {
    #[inline(always)]
    fn from(variant: Utick0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0_CLK` writer - utick clock clear"]
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
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0Clk::ClrClock)
    }
}
#[doc = "wdt clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL2 Bit"]
    ClrClock = 1,
}
impl From<Wwdt0Clk> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0_CLK` writer - wdt clock clear"]
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
    #[doc = "Clears the PSCCTL2 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0Clk::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl2ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - utick clock clear"]
    #[inline(always)]
    pub fn utick0_clk(&mut self) -> Utick0ClkW<Pscctl2ClrSpec> {
        Utick0ClkW::new(self, 0)
    }
    #[doc = "Bit 1 - wdt clock clear"]
    #[inline(always)]
    pub fn wwdt0_clk(&mut self) -> Wwdt0ClkW<Pscctl2ClrSpec> {
        Wwdt0ClkW::new(self, 1)
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
