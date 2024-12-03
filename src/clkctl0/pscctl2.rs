#[doc = "Register `PSCCTL2` reader"]
pub type R = crate::R<Pscctl2Spec>;
#[doc = "Register `PSCCTL2` writer"]
pub type W = crate::W<Pscctl2Spec>;
#[doc = "utick clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Utick0Clk> for bool {
    #[inline(always)]
    fn from(variant: Utick0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0_CLK` reader - utick clock control"]
pub type Utick0ClkR = crate::BitReader<Utick0Clk>;
impl Utick0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Utick0Clk {
        match self.bits {
            false => Utick0Clk::DisableClock,
            true => Utick0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Utick0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Utick0Clk::EnableClock
    }
}
#[doc = "Field `UTICK0_CLK` writer - utick clock control"]
pub type Utick0ClkW<'a, REG> = crate::BitWriter<'a, REG, Utick0Clk>;
impl<'a, REG> Utick0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0Clk::EnableClock)
    }
}
#[doc = "wdt clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Wwdt0Clk> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0_CLK` reader - wdt clock control"]
pub type Wwdt0ClkR = crate::BitReader<Wwdt0Clk>;
impl Wwdt0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt0Clk {
        match self.bits {
            false => Wwdt0Clk::DisableClock,
            true => Wwdt0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Wwdt0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Wwdt0Clk::EnableClock
    }
}
#[doc = "Field `WWDT0_CLK` writer - wdt clock control"]
pub type Wwdt0ClkW<'a, REG> = crate::BitWriter<'a, REG, Wwdt0Clk>;
impl<'a, REG> Wwdt0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0Clk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 0 - utick clock control"]
    #[inline(always)]
    pub fn utick0_clk(&self) -> Utick0ClkR {
        Utick0ClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt clock control"]
    #[inline(always)]
    pub fn wwdt0_clk(&self) -> Wwdt0ClkR {
        Wwdt0ClkR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL2")
            .field("utick0_clk", &self.utick0_clk())
            .field("wwdt0_clk", &self.wwdt0_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - utick clock control"]
    #[inline(always)]
    pub fn utick0_clk(&mut self) -> Utick0ClkW<Pscctl2Spec> {
        Utick0ClkW::new(self, 0)
    }
    #[doc = "Bit 1 - wdt clock control"]
    #[inline(always)]
    pub fn wwdt0_clk(&mut self) -> Wwdt0ClkW<Pscctl2Spec> {
        Wwdt0ClkW::new(self, 1)
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
