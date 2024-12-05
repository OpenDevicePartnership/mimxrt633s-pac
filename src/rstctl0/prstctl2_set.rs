#[doc = "Register `PRSTCTL2_SET` writer"]
pub type W = crate::W<Prstctl2SetSpec>;
#[doc = "utick reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Utick0> for bool {
    #[inline(always)]
    fn from(variant: Utick0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0` writer - utick reset set"]
pub type Utick0W<'a, REG> = crate::BitWriter<'a, REG, Utick0>;
impl<'a, REG> Utick0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::SetReset)
    }
}
#[doc = "wdt reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    SetReset = 1,
}
impl From<Wwdt0> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0` writer - wdt reset set"]
pub type Wwdt0W<'a, REG> = crate::BitWriter<'a, REG, Wwdt0>;
impl<'a, REG> Wwdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::NoEffect)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::SetReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl2SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - utick reset set"]
    #[inline(always)]
    pub fn utick0(&mut self) -> Utick0W<Prstctl2SetSpec> {
        Utick0W::new(self, 0)
    }
    #[doc = "Bit 1 - wdt reset set"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<Prstctl2SetSpec> {
        Wwdt0W::new(self, 1)
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
