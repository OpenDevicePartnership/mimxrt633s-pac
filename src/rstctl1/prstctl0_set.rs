#[doc = "Register `PRSTCTL0_SET` writer"]
pub type W = crate::W<Prstctl0SetSpec>;
#[doc = "FLEXCOMM0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_RST_SET` writer - FLEXCOMM0 reset set"]
pub type Flexcomm0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0RstSet>;
impl<'a, REG> Flexcomm0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM1 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm1RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_RST_SET` writer - FLEXCOMM1 reset set"]
pub type Flexcomm1RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1RstSet>;
impl<'a, REG> Flexcomm1RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM2 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm2RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_RST_SET` writer - FLEXCOMM2 reset set"]
pub type Flexcomm2RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2RstSet>;
impl<'a, REG> Flexcomm2RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM3 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm3RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_RST_SET` writer - FLEXCOMM3 reset set"]
pub type Flexcomm3RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3RstSet>;
impl<'a, REG> Flexcomm3RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM4 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm4RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_RST_SET` writer - FLEXCOMM4 reset set"]
pub type Flexcomm4RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4RstSet>;
impl<'a, REG> Flexcomm4RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM5 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm5RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_RST_SET` writer - FLEXCOMM5 reset set"]
pub type Flexcomm5RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5RstSet>;
impl<'a, REG> Flexcomm5RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM6 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm6RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_RST_SET` writer - FLEXCOMM6 reset set"]
pub type Flexcomm6RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6RstSet>;
impl<'a, REG> Flexcomm6RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM7 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm7RstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_RST_SET` writer - FLEXCOMM7 reset set"]
pub type Flexcomm7RstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7RstSet>;
impl<'a, REG> Flexcomm7RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7RstSet::SetReset)
    }
}
#[doc = "FLEXCOMM14 SPI reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14SpiRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm14SpiRstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14SpiRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_SPI_RST_SET` writer - FLEXCOMM14 SPI reset set"]
pub type Flexcomm14SpiRstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14SpiRstSet>;
impl<'a, REG> Flexcomm14SpiRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRstSet::SetReset)
    }
}
#[doc = "FLEXCOMM15 I2C reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm15I2cRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Flexcomm15I2cRstSet> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm15I2cRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15_I2C_RST_SET` writer - FLEXCOMM15 I2C reset set"]
pub type Flexcomm15I2cRstSetW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm15I2cRstSet>;
impl<'a, REG> Flexcomm15I2cRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRstSet::SetReset)
    }
}
#[doc = "DMIC0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<Dmic0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Dmic0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_RST_SET` writer - DMIC0 reset set"]
pub type Dmic0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Dmic0RstSet>;
impl<'a, REG> Dmic0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0RstSet::SetReset)
    }
}
#[doc = "osevent timer reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsevtTimerRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SetReset = 1,
}
impl From<OsevtTimerRstSet> for bool {
    #[inline(always)]
    fn from(variant: OsevtTimerRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVT_TIMER_RST_SET` writer - osevent timer reset set"]
pub type OsevtTimerRstSetW<'a, REG> = crate::BitWriter<'a, REG, OsevtTimerRstSet>;
impl<'a, REG> OsevtTimerRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRstSet::SetReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl0SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 8 - FLEXCOMM0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rst_set(&mut self) -> Flexcomm0RstSetW<Prstctl0SetSpec> {
        Flexcomm0RstSetW::new(self, 8)
    }
    #[doc = "Bit 9 - FLEXCOMM1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rst_set(&mut self) -> Flexcomm1RstSetW<Prstctl0SetSpec> {
        Flexcomm1RstSetW::new(self, 9)
    }
    #[doc = "Bit 10 - FLEXCOMM2 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rst_set(&mut self) -> Flexcomm2RstSetW<Prstctl0SetSpec> {
        Flexcomm2RstSetW::new(self, 10)
    }
    #[doc = "Bit 11 - FLEXCOMM3 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rst_set(&mut self) -> Flexcomm3RstSetW<Prstctl0SetSpec> {
        Flexcomm3RstSetW::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCOMM4 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_rst_set(&mut self) -> Flexcomm4RstSetW<Prstctl0SetSpec> {
        Flexcomm4RstSetW::new(self, 12)
    }
    #[doc = "Bit 13 - FLEXCOMM5 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_rst_set(&mut self) -> Flexcomm5RstSetW<Prstctl0SetSpec> {
        Flexcomm5RstSetW::new(self, 13)
    }
    #[doc = "Bit 14 - FLEXCOMM6 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_rst_set(&mut self) -> Flexcomm6RstSetW<Prstctl0SetSpec> {
        Flexcomm6RstSetW::new(self, 14)
    }
    #[doc = "Bit 15 - FLEXCOMM7 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_rst_set(&mut self) -> Flexcomm7RstSetW<Prstctl0SetSpec> {
        Flexcomm7RstSetW::new(self, 15)
    }
    #[doc = "Bit 22 - FLEXCOMM14 SPI reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_spi_rst_set(&mut self) -> Flexcomm14SpiRstSetW<Prstctl0SetSpec> {
        Flexcomm14SpiRstSetW::new(self, 22)
    }
    #[doc = "Bit 23 - FLEXCOMM15 I2C reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15_i2c_rst_set(&mut self) -> Flexcomm15I2cRstSetW<Prstctl0SetSpec> {
        Flexcomm15I2cRstSetW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_rst_set(&mut self) -> Dmic0RstSetW<Prstctl0SetSpec> {
        Dmic0RstSetW::new(self, 24)
    }
    #[doc = "Bit 27 - osevent timer reset set"]
    #[inline(always)]
    #[must_use]
    pub fn osevt_timer_rst_set(&mut self) -> OsevtTimerRstSetW<Prstctl0SetSpec> {
        OsevtTimerRstSetW::new(self, 27)
    }
}
#[doc = "peripheral reset set register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0SetSpec;
impl crate::RegisterSpec for Prstctl0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl0_set::W`](W) writer structure"]
impl crate::Writable for Prstctl0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_SET to value 0"]
impl crate::Resettable for Prstctl0SetSpec {
    const RESET_VALUE: u32 = 0;
}
