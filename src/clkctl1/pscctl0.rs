#[doc = "Register `PSCCTL0` reader"]
pub type R = crate::R<Pscctl0Spec>;
#[doc = "Register `PSCCTL0` writer"]
pub type W = crate::W<Pscctl0Spec>;
#[doc = "flexcomm 0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc0Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC0_CLK` reader - flexcomm 0 clock control"]
pub type Fc0ClkR = crate::BitReader<Fc0Clk>;
impl Fc0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc0Clk {
        match self.bits {
            false => Fc0Clk::DisableClock,
            true => Fc0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc0Clk::EnableClock
    }
}
#[doc = "Field `FC0_CLK` writer - flexcomm 0 clock control"]
pub type Fc0ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc0Clk>;
impl<'a, REG> Fc0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc0Clk::EnableClock)
    }
}
#[doc = "flexcomm 1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc1Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC1_CLK` reader - flexcomm 1 clock control"]
pub type Fc1ClkR = crate::BitReader<Fc1Clk>;
impl Fc1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc1Clk {
        match self.bits {
            false => Fc1Clk::DisableClock,
            true => Fc1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc1Clk::EnableClock
    }
}
#[doc = "Field `FC1_CLK` writer - flexcomm 1 clock control"]
pub type Fc1ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc1Clk>;
impl<'a, REG> Fc1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc1Clk::EnableClock)
    }
}
#[doc = "flexcomm 2 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc2Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc2Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc2Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC2_CLK` reader - flexcomm 2 clock control"]
pub type Fc2ClkR = crate::BitReader<Fc2Clk>;
impl Fc2ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc2Clk {
        match self.bits {
            false => Fc2Clk::DisableClock,
            true => Fc2Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc2Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc2Clk::EnableClock
    }
}
#[doc = "Field `FC2_CLK` writer - flexcomm 2 clock control"]
pub type Fc2ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc2Clk>;
impl<'a, REG> Fc2ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc2Clk::EnableClock)
    }
}
#[doc = "flexcomm 3 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc3Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc3Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc3Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC3_CLK` reader - flexcomm 3 clock control"]
pub type Fc3ClkR = crate::BitReader<Fc3Clk>;
impl Fc3ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc3Clk {
        match self.bits {
            false => Fc3Clk::DisableClock,
            true => Fc3Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc3Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc3Clk::EnableClock
    }
}
#[doc = "Field `FC3_CLK` writer - flexcomm 3 clock control"]
pub type Fc3ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc3Clk>;
impl<'a, REG> Fc3ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc3Clk::EnableClock)
    }
}
#[doc = "flexcomm 4 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc4Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc4Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc4Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC4_CLK` reader - flexcomm 4 clock control"]
pub type Fc4ClkR = crate::BitReader<Fc4Clk>;
impl Fc4ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc4Clk {
        match self.bits {
            false => Fc4Clk::DisableClock,
            true => Fc4Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc4Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc4Clk::EnableClock
    }
}
#[doc = "Field `FC4_CLK` writer - flexcomm 4 clock control"]
pub type Fc4ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc4Clk>;
impl<'a, REG> Fc4ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc4Clk::EnableClock)
    }
}
#[doc = "flexcomm 5 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc5Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc5Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc5Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC5_CLK` reader - flexcomm 5 clock control"]
pub type Fc5ClkR = crate::BitReader<Fc5Clk>;
impl Fc5ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc5Clk {
        match self.bits {
            false => Fc5Clk::DisableClock,
            true => Fc5Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc5Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc5Clk::EnableClock
    }
}
#[doc = "Field `FC5_CLK` writer - flexcomm 5 clock control"]
pub type Fc5ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc5Clk>;
impl<'a, REG> Fc5ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc5Clk::EnableClock)
    }
}
#[doc = "flexcomm 6 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc6Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc6Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc6Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC6_CLK` reader - flexcomm 6 clock control"]
pub type Fc6ClkR = crate::BitReader<Fc6Clk>;
impl Fc6ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc6Clk {
        match self.bits {
            false => Fc6Clk::DisableClock,
            true => Fc6Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc6Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc6Clk::EnableClock
    }
}
#[doc = "Field `FC6_CLK` writer - flexcomm 6 clock control"]
pub type Fc6ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc6Clk>;
impl<'a, REG> Fc6ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc6Clk::EnableClock)
    }
}
#[doc = "flexcomm 7 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc7Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc7Clk> for bool {
    #[inline(always)]
    fn from(variant: Fc7Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC7_CLK` reader - flexcomm 7 clock control"]
pub type Fc7ClkR = crate::BitReader<Fc7Clk>;
impl Fc7ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc7Clk {
        match self.bits {
            false => Fc7Clk::DisableClock,
            true => Fc7Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc7Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc7Clk::EnableClock
    }
}
#[doc = "Field `FC7_CLK` writer - flexcomm 7 clock control"]
pub type Fc7ClkW<'a, REG> = crate::BitWriter<'a, REG, Fc7Clk>;
impl<'a, REG> Fc7ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc7Clk::EnableClock)
    }
}
#[doc = "flexcomm 14 spi clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc14SpiClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc14SpiClk> for bool {
    #[inline(always)]
    fn from(variant: Fc14SpiClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC14_SPI_CLK` reader - flexcomm 14 spi clock control"]
pub type Fc14SpiClkR = crate::BitReader<Fc14SpiClk>;
impl Fc14SpiClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc14SpiClk {
        match self.bits {
            false => Fc14SpiClk::DisableClock,
            true => Fc14SpiClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc14SpiClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc14SpiClk::EnableClock
    }
}
#[doc = "Field `FC14_SPI_CLK` writer - flexcomm 14 spi clock control"]
pub type Fc14SpiClkW<'a, REG> = crate::BitWriter<'a, REG, Fc14SpiClk>;
impl<'a, REG> Fc14SpiClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc14SpiClk::EnableClock)
    }
}
#[doc = "flexcomm 15 i2c clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fc15I2cClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Fc15I2cClk> for bool {
    #[inline(always)]
    fn from(variant: Fc15I2cClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FC15_I2C_CLK` reader - flexcomm 15 i2c clock control"]
pub type Fc15I2cClkR = crate::BitReader<Fc15I2cClk>;
impl Fc15I2cClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc15I2cClk {
        match self.bits {
            false => Fc15I2cClk::DisableClock,
            true => Fc15I2cClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Fc15I2cClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Fc15I2cClk::EnableClock
    }
}
#[doc = "Field `FC15_I2C_CLK` writer - flexcomm 15 i2c clock control"]
pub type Fc15I2cClkW<'a, REG> = crate::BitWriter<'a, REG, Fc15I2cClk>;
impl<'a, REG> Fc15I2cClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Fc15I2cClk::EnableClock)
    }
}
#[doc = "DMIC0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Dmic0Clk> for bool {
    #[inline(always)]
    fn from(variant: Dmic0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_CLK` reader - DMIC0 clock control"]
pub type Dmic0ClkR = crate::BitReader<Dmic0Clk>;
impl Dmic0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0Clk {
        match self.bits {
            false => Dmic0Clk::DisableClock,
            true => Dmic0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Dmic0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Dmic0Clk::EnableClock
    }
}
#[doc = "Field `DMIC0_CLK` writer - DMIC0 clock control"]
pub type Dmic0ClkW<'a, REG> = crate::BitWriter<'a, REG, Dmic0Clk>;
impl<'a, REG> Dmic0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0Clk::EnableClock)
    }
}
#[doc = "OS event timer clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OseventTimerClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<OseventTimerClk> for bool {
    #[inline(always)]
    fn from(variant: OseventTimerClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVENT_TIMER_CLK` reader - OS event timer clock control"]
pub type OseventTimerClkR = crate::BitReader<OseventTimerClk>;
impl OseventTimerClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OseventTimerClk {
        match self.bits {
            false => OseventTimerClk::DisableClock,
            true => OseventTimerClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == OseventTimerClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == OseventTimerClk::EnableClock
    }
}
#[doc = "Field `OSEVENT_TIMER_CLK` writer - OS event timer clock control"]
pub type OseventTimerClkW<'a, REG> = crate::BitWriter<'a, REG, OseventTimerClk>;
impl<'a, REG> OseventTimerClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(OseventTimerClk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 8 - flexcomm 0 clock control"]
    #[inline(always)]
    pub fn fc0_clk(&self) -> Fc0ClkR {
        Fc0ClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - flexcomm 1 clock control"]
    #[inline(always)]
    pub fn fc1_clk(&self) -> Fc1ClkR {
        Fc1ClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - flexcomm 2 clock control"]
    #[inline(always)]
    pub fn fc2_clk(&self) -> Fc2ClkR {
        Fc2ClkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - flexcomm 3 clock control"]
    #[inline(always)]
    pub fn fc3_clk(&self) -> Fc3ClkR {
        Fc3ClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - flexcomm 4 clock control"]
    #[inline(always)]
    pub fn fc4_clk(&self) -> Fc4ClkR {
        Fc4ClkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - flexcomm 5 clock control"]
    #[inline(always)]
    pub fn fc5_clk(&self) -> Fc5ClkR {
        Fc5ClkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - flexcomm 6 clock control"]
    #[inline(always)]
    pub fn fc6_clk(&self) -> Fc6ClkR {
        Fc6ClkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - flexcomm 7 clock control"]
    #[inline(always)]
    pub fn fc7_clk(&self) -> Fc7ClkR {
        Fc7ClkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - flexcomm 14 spi clock control"]
    #[inline(always)]
    pub fn fc14_spi_clk(&self) -> Fc14SpiClkR {
        Fc14SpiClkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - flexcomm 15 i2c clock control"]
    #[inline(always)]
    pub fn fc15_i2c_clk(&self) -> Fc15I2cClkR {
        Fc15I2cClkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMIC0 clock control"]
    #[inline(always)]
    pub fn dmic0_clk(&self) -> Dmic0ClkR {
        Dmic0ClkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - OS event timer clock control"]
    #[inline(always)]
    pub fn osevent_timer_clk(&self) -> OseventTimerClkR {
        OseventTimerClkR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL0")
            .field("fc0_clk", &self.fc0_clk())
            .field("fc1_clk", &self.fc1_clk())
            .field("fc2_clk", &self.fc2_clk())
            .field("fc3_clk", &self.fc3_clk())
            .field("fc4_clk", &self.fc4_clk())
            .field("fc5_clk", &self.fc5_clk())
            .field("fc6_clk", &self.fc6_clk())
            .field("fc7_clk", &self.fc7_clk())
            .field("fc14_spi_clk", &self.fc14_spi_clk())
            .field("fc15_i2c_clk", &self.fc15_i2c_clk())
            .field("dmic0_clk", &self.dmic0_clk())
            .field("osevent_timer_clk", &self.osevent_timer_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - flexcomm 0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_clk(&mut self) -> Fc0ClkW<Pscctl0Spec> {
        Fc0ClkW::new(self, 8)
    }
    #[doc = "Bit 9 - flexcomm 1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc1_clk(&mut self) -> Fc1ClkW<Pscctl0Spec> {
        Fc1ClkW::new(self, 9)
    }
    #[doc = "Bit 10 - flexcomm 2 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc2_clk(&mut self) -> Fc2ClkW<Pscctl0Spec> {
        Fc2ClkW::new(self, 10)
    }
    #[doc = "Bit 11 - flexcomm 3 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc3_clk(&mut self) -> Fc3ClkW<Pscctl0Spec> {
        Fc3ClkW::new(self, 11)
    }
    #[doc = "Bit 12 - flexcomm 4 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc4_clk(&mut self) -> Fc4ClkW<Pscctl0Spec> {
        Fc4ClkW::new(self, 12)
    }
    #[doc = "Bit 13 - flexcomm 5 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc5_clk(&mut self) -> Fc5ClkW<Pscctl0Spec> {
        Fc5ClkW::new(self, 13)
    }
    #[doc = "Bit 14 - flexcomm 6 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc6_clk(&mut self) -> Fc6ClkW<Pscctl0Spec> {
        Fc6ClkW::new(self, 14)
    }
    #[doc = "Bit 15 - flexcomm 7 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc7_clk(&mut self) -> Fc7ClkW<Pscctl0Spec> {
        Fc7ClkW::new(self, 15)
    }
    #[doc = "Bit 22 - flexcomm 14 spi clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc14_spi_clk(&mut self) -> Fc14SpiClkW<Pscctl0Spec> {
        Fc14SpiClkW::new(self, 22)
    }
    #[doc = "Bit 23 - flexcomm 15 i2c clock control"]
    #[inline(always)]
    #[must_use]
    pub fn fc15_i2c_clk(&mut self) -> Fc15I2cClkW<Pscctl0Spec> {
        Fc15I2cClkW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_clk(&mut self) -> Dmic0ClkW<Pscctl0Spec> {
        Dmic0ClkW::new(self, 24)
    }
    #[doc = "Bit 27 - OS event timer clock control"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_timer_clk(&mut self) -> OseventTimerClkW<Pscctl0Spec> {
        OseventTimerClkW::new(self, 27)
    }
}
#[doc = "clock control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pscctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl0Spec;
impl crate::RegisterSpec for Pscctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscctl0::R`](R) reader structure"]
impl crate::Readable for Pscctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pscctl0::W`](W) writer structure"]
impl crate::Writable for Pscctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL0 to value 0"]
impl crate::Resettable for Pscctl0Spec {
    const RESET_VALUE: u32 = 0;
}
