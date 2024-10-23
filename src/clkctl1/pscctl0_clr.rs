#[doc = "Register `PSCCTL0_CLR` writer"]
pub type W = crate::W<Pscctl0ClrSpec>;
#[doc = "flexcomm 0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0_CLK_CLR` writer - flexcomm 0 clock clear"]
pub type Fc0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc0ClkClr>;
impl<'a, REG> Fc0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc1ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc1ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1_CLK_CLR` writer - flexcomm 1 clock clear"]
pub type Fc1ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc1ClkClr>;
impl<'a, REG> Fc1ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 2 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc2ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc2ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2_CLK_CLR` writer - flexcomm 2 clock clear"]
pub type Fc2ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc2ClkClr>;
impl<'a, REG> Fc2ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 3 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc3ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc3ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3_CLK_CLR` writer - flexcomm 3 clock clear"]
pub type Fc3ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc3ClkClr>;
impl<'a, REG> Fc3ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 4 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc4ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc4ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4_CLK_CLR` writer - flexcomm 4 clock clear"]
pub type Fc4ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc4ClkClr>;
impl<'a, REG> Fc4ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 5 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc5ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc5ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5_CLK_CLR` writer - flexcomm 5 clock clear"]
pub type Fc5ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc5ClkClr>;
impl<'a, REG> Fc5ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 6 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc6ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc6ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6_CLK_CLR` writer - flexcomm 6 clock clear"]
pub type Fc6ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc6ClkClr>;
impl<'a, REG> Fc6ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 7 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc7ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc7ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7_CLK_CLR` writer - flexcomm 7 clock clear"]
pub type Fc7ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc7ClkClr>;
impl<'a, REG> Fc7ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7ClkClr::ClrClock)
    }
}
#[doc = "flexcomm 14 spi clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc14SpiClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc14SpiClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc14SpiClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC14_SPI_CLK_CLR` writer - flexcomm 14 spi clock clear"]
pub type Fc14SpiClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc14SpiClkClr>;
impl<'a, REG> Fc14SpiClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClkClr::ClrClock)
    }
}
#[doc = "flexcomm 15 i2c clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc15I2cClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Fc15I2cClkClr> for bool {
    #[inline(always)]
    fn from(variant: Fc15I2cClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC15_I2C_CLK_CLR` writer - flexcomm 15 i2c clock clear"]
pub type Fc15I2cClkClrW<'a, REG> = crate::BitWriter<'a, REG, Fc15I2cClkClr>;
impl<'a, REG> Fc15I2cClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClkClr::ClrClock)
    }
}
#[doc = "DMIC0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<Dmic0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_CLK_CLR` writer - DMIC0 clock set"]
pub type Dmic0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Dmic0ClkClr>;
impl<'a, REG> Dmic0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ClkClr::ClrClock)
    }
}
#[doc = "OS event timer clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OseventTimerClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL0 Bit"]
    ClrClock = 1,
}
impl From<OseventTimerClkClr> for bool {
    #[inline(always)]
    fn from(variant: OseventTimerClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVENT_TIMER_CLK_CLR` writer - OS event timer clock clear"]
pub type OseventTimerClkClrW<'a, REG> = crate::BitWriter<'a, REG, OseventTimerClkClr>;
impl<'a, REG> OseventTimerClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClkClr::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 8 - flexcomm 0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_clk_clr(&mut self) -> Fc0ClkClrW<Pscctl0ClrSpec> {
        Fc0ClkClrW::new(self, 8)
    }
    #[doc = "Bit 9 - flexcomm 1 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc1_clk_clr(&mut self) -> Fc1ClkClrW<Pscctl0ClrSpec> {
        Fc1ClkClrW::new(self, 9)
    }
    #[doc = "Bit 10 - flexcomm 2 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc2_clk_clr(&mut self) -> Fc2ClkClrW<Pscctl0ClrSpec> {
        Fc2ClkClrW::new(self, 10)
    }
    #[doc = "Bit 11 - flexcomm 3 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc3_clk_clr(&mut self) -> Fc3ClkClrW<Pscctl0ClrSpec> {
        Fc3ClkClrW::new(self, 11)
    }
    #[doc = "Bit 12 - flexcomm 4 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc4_clk_clr(&mut self) -> Fc4ClkClrW<Pscctl0ClrSpec> {
        Fc4ClkClrW::new(self, 12)
    }
    #[doc = "Bit 13 - flexcomm 5 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc5_clk_clr(&mut self) -> Fc5ClkClrW<Pscctl0ClrSpec> {
        Fc5ClkClrW::new(self, 13)
    }
    #[doc = "Bit 14 - flexcomm 6 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc6_clk_clr(&mut self) -> Fc6ClkClrW<Pscctl0ClrSpec> {
        Fc6ClkClrW::new(self, 14)
    }
    #[doc = "Bit 15 - flexcomm 7 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc7_clk_clr(&mut self) -> Fc7ClkClrW<Pscctl0ClrSpec> {
        Fc7ClkClrW::new(self, 15)
    }
    #[doc = "Bit 22 - flexcomm 14 spi clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc14_spi_clk_clr(&mut self) -> Fc14SpiClkClrW<Pscctl0ClrSpec> {
        Fc14SpiClkClrW::new(self, 22)
    }
    #[doc = "Bit 23 - flexcomm 15 i2c clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn fc15_i2c_clk_clr(&mut self) -> Fc15I2cClkClrW<Pscctl0ClrSpec> {
        Fc15I2cClkClrW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_clk_clr(&mut self) -> Dmic0ClkClrW<Pscctl0ClrSpec> {
        Dmic0ClkClrW::new(self, 24)
    }
    #[doc = "Bit 27 - OS event timer clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_timer_clk_clr(&mut self) -> OseventTimerClkClrW<Pscctl0ClrSpec> {
        OseventTimerClkClrW::new(self, 27)
    }
}
#[doc = "clock clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0ClrSpec;
impl crate::RegisterSpec for Pscctl0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl0_clr::W`](W) writer structure"]
impl crate::Writable for Pscctl0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0_CLR to value 0"]
impl crate::Resettable for Pscctl0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
