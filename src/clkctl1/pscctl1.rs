#[doc = "Register `PSCCTL1` reader"]
pub type R = crate::R<Pscctl1Spec>;
#[doc = "Register `PSCCTL1` writer"]
pub type W = crate::W<Pscctl1Spec>;
#[doc = "HSGPIO0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio0Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_CLK` reader - HSGPIO0 clock control"]
pub type Hsgpio0ClkR = crate::BitReader<Hsgpio0Clk>;
impl Hsgpio0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio0Clk {
        match self.bits {
            false => Hsgpio0Clk::DisableClock,
            true => Hsgpio0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio0Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO0_CLK` writer - HSGPIO0 clock control"]
pub type Hsgpio0ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0Clk>;
impl<'a, REG> Hsgpio0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0Clk::EnableClock)
    }
}
#[doc = "HSGPIO1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio1Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_CLK` reader - HSGPIO1 clock control"]
pub type Hsgpio1ClkR = crate::BitReader<Hsgpio1Clk>;
impl Hsgpio1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio1Clk {
        match self.bits {
            false => Hsgpio1Clk::DisableClock,
            true => Hsgpio1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio1Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO1_CLK` writer - HSGPIO1 clock control"]
pub type Hsgpio1ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1Clk>;
impl<'a, REG> Hsgpio1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1Clk::EnableClock)
    }
}
#[doc = "HSGPIO2 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio2Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_CLK` reader - HSGPIO2 clock control"]
pub type Hsgpio2ClkR = crate::BitReader<Hsgpio2Clk>;
impl Hsgpio2ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio2Clk {
        match self.bits {
            false => Hsgpio2Clk::DisableClock,
            true => Hsgpio2Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio2Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio2Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO2_CLK` writer - HSGPIO2 clock control"]
pub type Hsgpio2ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2Clk>;
impl<'a, REG> Hsgpio2ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2Clk::EnableClock)
    }
}
#[doc = "HSGPIO3 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio3Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_CLK` reader - HSGPIO3 clock control"]
pub type Hsgpio3ClkR = crate::BitReader<Hsgpio3Clk>;
impl Hsgpio3ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio3Clk {
        match self.bits {
            false => Hsgpio3Clk::DisableClock,
            true => Hsgpio3Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio3Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio3Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO3_CLK` writer - HSGPIO3 clock control"]
pub type Hsgpio3ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3Clk>;
impl<'a, REG> Hsgpio3ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3Clk::EnableClock)
    }
}
#[doc = "HSGPIO4 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio4Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_CLK` reader - HSGPIO4 clock control"]
pub type Hsgpio4ClkR = crate::BitReader<Hsgpio4Clk>;
impl Hsgpio4ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio4Clk {
        match self.bits {
            false => Hsgpio4Clk::DisableClock,
            true => Hsgpio4Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio4Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio4Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO4_CLK` writer - HSGPIO4 clock control"]
pub type Hsgpio4ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4Clk>;
impl<'a, REG> Hsgpio4ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4Clk::EnableClock)
    }
}
#[doc = "HSGPIO5 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio5Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_CLK` reader - HSGPIO5 clock control"]
pub type Hsgpio5ClkR = crate::BitReader<Hsgpio5Clk>;
impl Hsgpio5ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio5Clk {
        match self.bits {
            false => Hsgpio5Clk::DisableClock,
            true => Hsgpio5Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio5Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio5Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO5_CLK` writer - HSGPIO5 clock control"]
pub type Hsgpio5ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5Clk>;
impl<'a, REG> Hsgpio5ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5Clk::EnableClock)
    }
}
#[doc = "HSGPIO6 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio6Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_CLK` reader - HSGPIO6 clock control"]
pub type Hsgpio6ClkR = crate::BitReader<Hsgpio6Clk>;
impl Hsgpio6ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio6Clk {
        match self.bits {
            false => Hsgpio6Clk::DisableClock,
            true => Hsgpio6Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio6Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio6Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO6_CLK` writer - HSGPIO6 clock control"]
pub type Hsgpio6ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6Clk>;
impl<'a, REG> Hsgpio6ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6Clk::EnableClock)
    }
}
#[doc = "HSGPIO7 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Hsgpio7Clk> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_CLK` reader - HSGPIO7 clock control"]
pub type Hsgpio7ClkR = crate::BitReader<Hsgpio7Clk>;
impl Hsgpio7ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio7Clk {
        match self.bits {
            false => Hsgpio7Clk::DisableClock,
            true => Hsgpio7Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Hsgpio7Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Hsgpio7Clk::EnableClock
    }
}
#[doc = "Field `HSGPIO7_CLK` writer - HSGPIO7 clock control"]
pub type Hsgpio7ClkW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7Clk>;
impl<'a, REG> Hsgpio7ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7Clk::EnableClock)
    }
}
#[doc = "CRC clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<CrcClk> for bool {
    #[inline(always)]
    fn from(variant: CrcClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_CLK` reader - CRC clock control"]
pub type CrcClkR = crate::BitReader<CrcClk>;
impl CrcClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcClk {
        match self.bits {
            false => CrcClk::DisableClock,
            true => CrcClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == CrcClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == CrcClk::EnableClock
    }
}
#[doc = "Field `CRC_CLK` writer - CRC clock control"]
pub type CrcClkW<'a, REG> = crate::BitWriter<'a, REG, CrcClk>;
impl<'a, REG> CrcClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClk::EnableClock)
    }
}
#[doc = "DMAC0 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Dmac0Clk> for bool {
    #[inline(always)]
    fn from(variant: Dmac0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_CLK` reader - DMAC0 clock control"]
pub type Dmac0ClkR = crate::BitReader<Dmac0Clk>;
impl Dmac0ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0Clk {
        match self.bits {
            false => Dmac0Clk::DisableClock,
            true => Dmac0Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Dmac0Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Dmac0Clk::EnableClock
    }
}
#[doc = "Field `DMAC0_CLK` writer - DMAC0 clock control"]
pub type Dmac0ClkW<'a, REG> = crate::BitWriter<'a, REG, Dmac0Clk>;
impl<'a, REG> Dmac0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0Clk::EnableClock)
    }
}
#[doc = "DMAC1 clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1Clk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<Dmac1Clk> for bool {
    #[inline(always)]
    fn from(variant: Dmac1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_CLK` reader - DMAC1 clock control"]
pub type Dmac1ClkR = crate::BitReader<Dmac1Clk>;
impl Dmac1ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1Clk {
        match self.bits {
            false => Dmac1Clk::DisableClock,
            true => Dmac1Clk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == Dmac1Clk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == Dmac1Clk::EnableClock
    }
}
#[doc = "Field `DMAC1_CLK` writer - DMAC1 clock control"]
pub type Dmac1ClkW<'a, REG> = crate::BitWriter<'a, REG, Dmac1Clk>;
impl<'a, REG> Dmac1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1Clk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1Clk::EnableClock)
    }
}
#[doc = "MU clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<MuClk> for bool {
    #[inline(always)]
    fn from(variant: MuClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_CLK` reader - MU clock control"]
pub type MuClkR = crate::BitReader<MuClk>;
impl MuClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuClk {
        match self.bits {
            false => MuClk::DisableClock,
            true => MuClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == MuClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == MuClk::EnableClock
    }
}
#[doc = "Field `MU_CLK` writer - MU clock control"]
pub type MuClkW<'a, REG> = crate::BitWriter<'a, REG, MuClk>;
impl<'a, REG> MuClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MuClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MuClk::EnableClock)
    }
}
#[doc = "SEMA clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<SemaClk> for bool {
    #[inline(always)]
    fn from(variant: SemaClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_CLK` reader - SEMA clock control"]
pub type SemaClkR = crate::BitReader<SemaClk>;
impl SemaClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SemaClk {
        match self.bits {
            false => SemaClk::DisableClock,
            true => SemaClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == SemaClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == SemaClk::EnableClock
    }
}
#[doc = "Field `SEMA_CLK` writer - SEMA clock control"]
pub type SemaClkW<'a, REG> = crate::BitWriter<'a, REG, SemaClk>;
impl<'a, REG> SemaClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClk::EnableClock)
    }
}
#[doc = "FREQME clock control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeClk {
    #[doc = "0: Disable Clock"]
    DisableClock = 0,
    #[doc = "1: Enable Clock"]
    EnableClock = 1,
}
impl From<FreqmeClk> for bool {
    #[inline(always)]
    fn from(variant: FreqmeClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_CLK` reader - FREQME clock control"]
pub type FreqmeClkR = crate::BitReader<FreqmeClk>;
impl FreqmeClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FreqmeClk {
        match self.bits {
            false => FreqmeClk::DisableClock,
            true => FreqmeClk::EnableClock,
        }
    }
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn is_disable_clock(&self) -> bool {
        *self == FreqmeClk::DisableClock
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn is_enable_clock(&self) -> bool {
        *self == FreqmeClk::EnableClock
    }
}
#[doc = "Field `FREQME_CLK` writer - FREQME clock control"]
pub type FreqmeClkW<'a, REG> = crate::BitWriter<'a, REG, FreqmeClk>;
impl<'a, REG> FreqmeClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClk::DisableClock)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClk::EnableClock)
    }
}
impl R {
    #[doc = "Bit 0 - HSGPIO0 clock control"]
    #[inline(always)]
    pub fn hsgpio0_clk(&self) -> Hsgpio0ClkR {
        Hsgpio0ClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSGPIO1 clock control"]
    #[inline(always)]
    pub fn hsgpio1_clk(&self) -> Hsgpio1ClkR {
        Hsgpio1ClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSGPIO2 clock control"]
    #[inline(always)]
    pub fn hsgpio2_clk(&self) -> Hsgpio2ClkR {
        Hsgpio2ClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSGPIO3 clock control"]
    #[inline(always)]
    pub fn hsgpio3_clk(&self) -> Hsgpio3ClkR {
        Hsgpio3ClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSGPIO4 clock control"]
    #[inline(always)]
    pub fn hsgpio4_clk(&self) -> Hsgpio4ClkR {
        Hsgpio4ClkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSGPIO5 clock control"]
    #[inline(always)]
    pub fn hsgpio5_clk(&self) -> Hsgpio5ClkR {
        Hsgpio5ClkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSGPIO6 clock control"]
    #[inline(always)]
    pub fn hsgpio6_clk(&self) -> Hsgpio6ClkR {
        Hsgpio6ClkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSGPIO7 clock control"]
    #[inline(always)]
    pub fn hsgpio7_clk(&self) -> Hsgpio7ClkR {
        Hsgpio7ClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC clock control"]
    #[inline(always)]
    pub fn crc_clk(&self) -> CrcClkR {
        CrcClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC0 clock control"]
    #[inline(always)]
    pub fn dmac0_clk(&self) -> Dmac0ClkR {
        Dmac0ClkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC1 clock control"]
    #[inline(always)]
    pub fn dmac1_clk(&self) -> Dmac1ClkR {
        Dmac1ClkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - MU clock control"]
    #[inline(always)]
    pub fn mu_clk(&self) -> MuClkR {
        MuClkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SEMA clock control"]
    #[inline(always)]
    pub fn sema_clk(&self) -> SemaClkR {
        SemaClkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - FREQME clock control"]
    #[inline(always)]
    pub fn freqme_clk(&self) -> FreqmeClkR {
        FreqmeClkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCCTL1")
            .field("hsgpio0_clk", &self.hsgpio0_clk())
            .field("hsgpio1_clk", &self.hsgpio1_clk())
            .field("hsgpio2_clk", &self.hsgpio2_clk())
            .field("hsgpio3_clk", &self.hsgpio3_clk())
            .field("hsgpio4_clk", &self.hsgpio4_clk())
            .field("hsgpio5_clk", &self.hsgpio5_clk())
            .field("hsgpio6_clk", &self.hsgpio6_clk())
            .field("hsgpio7_clk", &self.hsgpio7_clk())
            .field("crc_clk", &self.crc_clk())
            .field("dmac0_clk", &self.dmac0_clk())
            .field("dmac1_clk", &self.dmac1_clk())
            .field("mu_clk", &self.mu_clk())
            .field("sema_clk", &self.sema_clk())
            .field("freqme_clk", &self.freqme_clk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0_clk(&mut self) -> Hsgpio0ClkW<Pscctl1Spec> {
        Hsgpio0ClkW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1_clk(&mut self) -> Hsgpio1ClkW<Pscctl1Spec> {
        Hsgpio1ClkW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2_clk(&mut self) -> Hsgpio2ClkW<Pscctl1Spec> {
        Hsgpio2ClkW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3_clk(&mut self) -> Hsgpio3ClkW<Pscctl1Spec> {
        Hsgpio3ClkW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4_clk(&mut self) -> Hsgpio4ClkW<Pscctl1Spec> {
        Hsgpio4ClkW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5_clk(&mut self) -> Hsgpio5ClkW<Pscctl1Spec> {
        Hsgpio5ClkW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6_clk(&mut self) -> Hsgpio6ClkW<Pscctl1Spec> {
        Hsgpio6ClkW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7_clk(&mut self) -> Hsgpio7ClkW<Pscctl1Spec> {
        Hsgpio7ClkW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC clock control"]
    #[inline(always)]
    #[must_use]
    pub fn crc_clk(&mut self) -> CrcClkW<Pscctl1Spec> {
        CrcClkW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_clk(&mut self) -> Dmac0ClkW<Pscctl1Spec> {
        Dmac0ClkW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_clk(&mut self) -> Dmac1ClkW<Pscctl1Spec> {
        Dmac1ClkW::new(self, 24)
    }
    #[doc = "Bit 28 - MU clock control"]
    #[inline(always)]
    #[must_use]
    pub fn mu_clk(&mut self) -> MuClkW<Pscctl1Spec> {
        MuClkW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sema_clk(&mut self) -> SemaClkW<Pscctl1Spec> {
        SemaClkW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME clock control"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_clk(&mut self) -> FreqmeClkW<Pscctl1Spec> {
        FreqmeClkW::new(self, 31)
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
