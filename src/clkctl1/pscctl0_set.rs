#[doc = "Register `PSCCTL0_SET` writer"]
pub type W = crate::W<Pscctl0SetSpec>;
#[doc = "flexcomm 0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0_CLK_SET` writer - flexcomm 0 clock set"]
pub type Fc0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc0ClkSet>;
impl<'a, REG> Fc0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0ClkSet::SetClock)
    }
}
#[doc = "flexcomm 1 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc1ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc1ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1_CLK_SET` writer - flexcomm 1 clock set"]
pub type Fc1ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc1ClkSet>;
impl<'a, REG> Fc1ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1ClkSet::SetClock)
    }
}
#[doc = "flexcomm 2 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc2ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc2ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2_CLK_SET` writer - flexcomm 2 clock set"]
pub type Fc2ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc2ClkSet>;
impl<'a, REG> Fc2ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2ClkSet::SetClock)
    }
}
#[doc = "flexcomm 3 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc3ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc3ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3_CLK_SET` writer - flexcomm 3 clock set"]
pub type Fc3ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc3ClkSet>;
impl<'a, REG> Fc3ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3ClkSet::SetClock)
    }
}
#[doc = "flexcomm 4 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc4ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc4ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4_CLK_SET` writer - flexcomm 4 clock set"]
pub type Fc4ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc4ClkSet>;
impl<'a, REG> Fc4ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4ClkSet::SetClock)
    }
}
#[doc = "flexcomm 5 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc5ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc5ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5_CLK_SET` writer - flexcomm 5 clock set"]
pub type Fc5ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc5ClkSet>;
impl<'a, REG> Fc5ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5ClkSet::SetClock)
    }
}
#[doc = "flexcomm 6 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc6ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc6ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6_CLK_SET` writer - flexcomm 6 clock set"]
pub type Fc6ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc6ClkSet>;
impl<'a, REG> Fc6ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6ClkSet::SetClock)
    }
}
#[doc = "flexcomm 7 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc7ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc7ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7_CLK_SET` writer - flexcomm 7 clock set"]
pub type Fc7ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc7ClkSet>;
impl<'a, REG> Fc7ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7ClkSet::SetClock)
    }
}
#[doc = "flexcomm 14 spi clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc14SpiClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc14SpiClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc14SpiClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC14_SPI_CLK_SET` writer - flexcomm 14 spi clock set"]
pub type Fc14SpiClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc14SpiClkSet>;
impl<'a, REG> Fc14SpiClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClkSet::SetClock)
    }
}
#[doc = "flexcomm 15 i2c clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc15I2cClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Fc15I2cClkSet> for bool {
    #[inline(always)]
    fn from(variant: Fc15I2cClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC15_I2C_CLK_SET` writer - flexcomm 15 i2c clock set"]
pub type Fc15I2cClkSetW<'a, REG> = crate::BitWriter<'a, REG, Fc15I2cClkSet>;
impl<'a, REG> Fc15I2cClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClkSet::SetClock)
    }
}
#[doc = "DMIC0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<Dmic0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Dmic0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_CLK_SET` writer - DMIC0 clock set"]
pub type Dmic0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Dmic0ClkSet>;
impl<'a, REG> Dmic0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0ClkSet::SetClock)
    }
}
#[doc = "OS event timer clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OseventTimerClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL0 Bit"]
    SetClock = 1,
}
impl From<OseventTimerClkSet> for bool {
    #[inline(always)]
    fn from(variant: OseventTimerClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVENT_TIMER_CLK_SET` writer - OS event timer clock set"]
pub type OseventTimerClkSetW<'a, REG> = crate::BitWriter<'a, REG, OseventTimerClkSet>;
impl<'a, REG> OseventTimerClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL0 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClkSet::SetClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl0SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 8 - flexcomm 0 clock set"]
    #[inline(always)]
    pub fn fc0_clk_set(&mut self) -> Fc0ClkSetW<Pscctl0SetSpec> {
        Fc0ClkSetW::new(self, 8)
    }
    #[doc = "Bit 9 - flexcomm 1 clock set"]
    #[inline(always)]
    pub fn fc1_clk_set(&mut self) -> Fc1ClkSetW<Pscctl0SetSpec> {
        Fc1ClkSetW::new(self, 9)
    }
    #[doc = "Bit 10 - flexcomm 2 clock set"]
    #[inline(always)]
    pub fn fc2_clk_set(&mut self) -> Fc2ClkSetW<Pscctl0SetSpec> {
        Fc2ClkSetW::new(self, 10)
    }
    #[doc = "Bit 11 - flexcomm 3 clock set"]
    #[inline(always)]
    pub fn fc3_clk_set(&mut self) -> Fc3ClkSetW<Pscctl0SetSpec> {
        Fc3ClkSetW::new(self, 11)
    }
    #[doc = "Bit 12 - flexcomm 4 clock set"]
    #[inline(always)]
    pub fn fc4_clk_set(&mut self) -> Fc4ClkSetW<Pscctl0SetSpec> {
        Fc4ClkSetW::new(self, 12)
    }
    #[doc = "Bit 13 - flexcomm 5 clock set"]
    #[inline(always)]
    pub fn fc5_clk_set(&mut self) -> Fc5ClkSetW<Pscctl0SetSpec> {
        Fc5ClkSetW::new(self, 13)
    }
    #[doc = "Bit 14 - flexcomm 6 clock set"]
    #[inline(always)]
    pub fn fc6_clk_set(&mut self) -> Fc6ClkSetW<Pscctl0SetSpec> {
        Fc6ClkSetW::new(self, 14)
    }
    #[doc = "Bit 15 - flexcomm 7 clock set"]
    #[inline(always)]
    pub fn fc7_clk_set(&mut self) -> Fc7ClkSetW<Pscctl0SetSpec> {
        Fc7ClkSetW::new(self, 15)
    }
    #[doc = "Bit 22 - flexcomm 14 spi clock set"]
    #[inline(always)]
    pub fn fc14_spi_clk_set(&mut self) -> Fc14SpiClkSetW<Pscctl0SetSpec> {
        Fc14SpiClkSetW::new(self, 22)
    }
    #[doc = "Bit 23 - flexcomm 15 i2c clock set"]
    #[inline(always)]
    pub fn fc15_i2c_clk_set(&mut self) -> Fc15I2cClkSetW<Pscctl0SetSpec> {
        Fc15I2cClkSetW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 clock set"]
    #[inline(always)]
    pub fn dmic0_clk_set(&mut self) -> Dmic0ClkSetW<Pscctl0SetSpec> {
        Dmic0ClkSetW::new(self, 24)
    }
    #[doc = "Bit 27 - OS event timer clock set"]
    #[inline(always)]
    pub fn osevent_timer_clk_set(&mut self) -> OseventTimerClkSetW<Pscctl0SetSpec> {
        OseventTimerClkSetW::new(self, 27)
    }
}
#[doc = "clock set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0SetSpec;
impl crate::RegisterSpec for Pscctl0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl0_set::W`](W) writer structure"]
impl crate::Writable for Pscctl0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0_SET to value 0"]
impl crate::Resettable for Pscctl0SetSpec {
    const RESET_VALUE: u32 = 0;
}
