#[doc = "Register `PRSTCTL1_CLR` writer"]
pub type W = crate::W<Prstctl1ClrSpec>;
#[doc = "SDIO0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Sdio0> for bool {
    #[inline(always)]
    fn from(variant: Sdio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0` writer - SDIO0 reset clear"]
pub type Sdio0W<'a, REG> = crate::BitWriter<'a, REG, Sdio0>;
impl<'a, REG> Sdio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::NoEffect)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::ClrReset)
    }
}
#[doc = "SDIO1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Sdio1> for bool {
    #[inline(always)]
    fn from(variant: Sdio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1` writer - SDIO1 reset clear"]
pub type Sdio1W<'a, REG> = crate::BitWriter<'a, REG, Sdio1>;
impl<'a, REG> Sdio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::NoEffect)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::ClrReset)
    }
}
#[doc = "Analog comparator reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Acmp0> for bool {
    #[inline(always)]
    fn from(variant: Acmp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0` writer - Analog comparator reset clear"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG, Acmp0>;
impl<'a, REG> Acmp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0::NoEffect)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0::ClrReset)
    }
}
#[doc = "ADC reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` writer - ADC reset clear"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::NoEffect)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::ClrReset)
    }
}
#[doc = "SHSGPIO0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shsgpio0 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Shsgpio0> for bool {
    #[inline(always)]
    fn from(variant: Shsgpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0` writer - SHSGPIO0 reset clear"]
pub type Shsgpio0W<'a, REG> = crate::BitWriter<'a, REG, Shsgpio0>;
impl<'a, REG> Shsgpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0::NoEffect)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0::ClrReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 reset clear"]
    #[inline(always)]
    pub fn sdio0(&mut self) -> Sdio0W<Prstctl1ClrSpec> {
        Sdio0W::new(self, 2)
    }
    #[doc = "Bit 3 - SDIO1 reset clear"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> Sdio1W<Prstctl1ClrSpec> {
        Sdio1W::new(self, 3)
    }
    #[doc = "Bit 15 - Analog comparator reset clear"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<Prstctl1ClrSpec> {
        Acmp0W::new(self, 15)
    }
    #[doc = "Bit 16 - ADC reset clear"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<Prstctl1ClrSpec> {
        Adc0W::new(self, 16)
    }
    #[doc = "Bit 24 - SHSGPIO0 reset clear"]
    #[inline(always)]
    pub fn shsgpio0(&mut self) -> Shsgpio0W<Prstctl1ClrSpec> {
        Shsgpio0W::new(self, 24)
    }
}
#[doc = "peripheral reset clear register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl1ClrSpec;
impl crate::RegisterSpec for Prstctl1ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl1_clr::W`](W) writer structure"]
impl crate::Writable for Prstctl1ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL1_CLR to value 0"]
impl crate::Resettable for Prstctl1ClrSpec {
    const RESET_VALUE: u32 = 0;
}
