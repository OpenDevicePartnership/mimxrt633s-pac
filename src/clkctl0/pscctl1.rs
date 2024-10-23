#[doc = "Register `PSCCTL1` reader"]
pub type R = crate::R<Pscctl1Spec>;
#[doc = "Register `PSCCTL1` writer"]
pub type W = crate::W<Pscctl1Spec>;
#[doc = "SDIO0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Sdio0Clk> for bool {
    #[inline(always)]
    fn from(variant: Sdio0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0_CLK` reader - SDIO0 clock control"]
pub type Sdio0ClkR = crate::BitReader<Sdio0Clk>;
impl Sdio0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio0Clk {
        match self.bits {
            false => Sdio0Clk::DisableClock,
            true => Sdio0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Sdio0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Sdio0Clk::EnableClock
    }
}
#[doc = "Field `SDIO0_CLK` writer - SDIO0 clock control"]
pub type Sdio0ClkW<'a, REG> = crate::BitWriter<'a, REG, Sdio0Clk>;
impl<'a, REG> Sdio0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Clk::EnableClock)
    }
}
#[doc = "SDIO1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Sdio1Clk> for bool {
    #[inline(always)]
    fn from(variant: Sdio1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1_CLK` reader - SDIO1 clock control"]
pub type Sdio1ClkR = crate::BitReader<Sdio1Clk>;
impl Sdio1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio1Clk {
        match self.bits {
            false => Sdio1Clk::DisableClock,
            true => Sdio1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Sdio1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Sdio1Clk::EnableClock
    }
}
#[doc = "Field `SDIO1_CLK` writer - SDIO1 clock control"]
pub type Sdio1ClkW<'a, REG> = crate::BitWriter<'a, REG, Sdio1Clk>;
impl<'a, REG> Sdio1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Clk::EnableClock)
    }
}
#[doc = "Analog comparator clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Acmp0Clk> for bool {
    #[inline(always)]
    fn from(variant: Acmp0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0_CLK` reader - Analog comparator clock control"]
pub type Acmp0ClkR = crate::BitReader<Acmp0Clk>;
impl Acmp0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp0Clk {
        match self.bits {
            false => Acmp0Clk::DisableClock,
            true => Acmp0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Acmp0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Acmp0Clk::EnableClock
    }
}
#[doc = "Field `ACMP0_CLK` writer - Analog comparator clock control"]
pub type Acmp0ClkW<'a, REG> = crate::BitWriter<'a, REG, Acmp0Clk>;
impl<'a, REG> Acmp0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0Clk::EnableClock)
    }
}
#[doc = "ADC clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Adc0Clk> for bool {
    #[inline(always)]
    fn from(variant: Adc0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0_CLK` reader - ADC clock control"]
pub type Adc0ClkR = crate::BitReader<Adc0Clk>;
impl Adc0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0Clk {
        match self.bits {
            false => Adc0Clk::DisableClock,
            true => Adc0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Adc0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Adc0Clk::EnableClock
    }
}
#[doc = "Field `ADC0_CLK` writer - ADC clock control"]
pub type Adc0ClkW<'a, REG> = crate::BitWriter<'a, REG, Adc0Clk>;
impl<'a, REG> Adc0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Clk::EnableClock)
    }
}
#[doc = "SHSGPIO0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shsgpio0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Shsgpio0Clk> for bool {
    #[inline(always)]
    fn from(variant: Shsgpio0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0_CLK` reader - SHSGPIO0 clock control"]
pub type Shsgpio0ClkR = crate::BitReader<Shsgpio0Clk>;
impl Shsgpio0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shsgpio0Clk {
        match self.bits {
            false => Shsgpio0Clk::DisableClock,
            true => Shsgpio0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Shsgpio0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Shsgpio0Clk::EnableClock
    }
}
#[doc = "Field `SHSGPIO0_CLK` writer - SHSGPIO0 clock control"]
pub type Shsgpio0ClkW<'a, REG> = crate::BitWriter<'a, REG, Shsgpio0Clk>;
impl<'a, REG> Shsgpio0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0Clk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 2 - SDIO0 clock control"]
    #[inline(always)]
    pub fn sdio0_clk(&self) -> Sdio0ClkR {
        Sdio0ClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO1 clock control"]
    #[inline(always)]
    pub fn sdio1_clk(&self) -> Sdio1ClkR {
        Sdio1ClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog comparator clock control"]
    #[inline(always)]
    pub fn acmp0_clk(&self) -> Acmp0ClkR {
        Acmp0ClkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC clock control"]
    #[inline(always)]
    pub fn adc0_clk(&self) -> Adc0ClkR {
        Adc0ClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock control"]
    #[inline(always)]
    pub fn shsgpio0_clk(&self) -> Shsgpio0ClkR {
        Shsgpio0ClkR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL1")
            .field("sdio0_clk", &self.sdio0_clk())
            .field("sdio1_clk", &self.sdio1_clk())
            .field("acmp0_clk", &self.acmp0_clk())
            .field("adc0_clk", &self.adc0_clk())
            .field("shsgpio0_clk", &self.shsgpio0_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_clk(&mut self) -> Sdio0ClkW<Pscctl1Spec> {
        Sdio0ClkW::new(self, 2)
    }
    #[doc = "Bit 3 - SDIO1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_clk(&mut self) -> Sdio1ClkW<Pscctl1Spec> {
        Sdio1ClkW::new(self, 3)
    }
    #[doc = "Bit 15 - Analog comparator clock control"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0_clk(&mut self) -> Acmp0ClkW<Pscctl1Spec> {
        Acmp0ClkW::new(self, 15)
    }
    #[doc = "Bit 16 - ADC clock control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_clk(&mut self) -> Adc0ClkW<Pscctl1Spec> {
        Adc0ClkW::new(self, 16)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio0_clk(&mut self) -> Shsgpio0ClkW<Pscctl1Spec> {
        Shsgpio0ClkW::new(self, 24)
    }
}
#[doc = "clock control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl1Spec;
impl crate::RegisterSpec for Pscctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscctl1::R`](R) reader structure"]
impl crate::Readable for Pscctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pscctl1::W`](W) writer structure"]
impl crate::Writable for Pscctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL1 to value 0"]
impl crate::Resettable for Pscctl1Spec {
    const RESET_VALUE: u32 = 0;
}
