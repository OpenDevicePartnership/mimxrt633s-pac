#[doc = "Register `PRSTCTL0_CLR` writer"]
pub type W = crate::W<Prstctl0ClrSpec>;
#[doc = "FLEXCOMM0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_RST_CLR` writer - FLEXCOMM0 reset clear"]
pub type Flexcomm0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0RstClr>;
impl<'a, REG> Flexcomm0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm1RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_RST_CLR` writer - FLEXCOMM1 reset clear"]
pub type Flexcomm1RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1RstClr>;
impl<'a, REG> Flexcomm1RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM2 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm2RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_RST_CLR` writer - FLEXCOMM2 reset clear"]
pub type Flexcomm2RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2RstClr>;
impl<'a, REG> Flexcomm2RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM3 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm3RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_RST_CLR` writer - FLEXCOMM3 reset clear"]
pub type Flexcomm3RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3RstClr>;
impl<'a, REG> Flexcomm3RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM4 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm4RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_RST_CLR` writer - FLEXCOMM4 reset clear"]
pub type Flexcomm4RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4RstClr>;
impl<'a, REG> Flexcomm4RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM5 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm5RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_RST_CLR` writer - FLEXCOMM5 reset clear"]
pub type Flexcomm5RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5RstClr>;
impl<'a, REG> Flexcomm5RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM6 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm6RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_RST_CLR` writer - FLEXCOMM6 reset clear"]
pub type Flexcomm6RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6RstClr>;
impl<'a, REG> Flexcomm6RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM7 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm7RstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_RST_CLR` writer - FLEXCOMM7 reset clear"]
pub type Flexcomm7RstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7RstClr>;
impl<'a, REG> Flexcomm7RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7RstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM14 SPI reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14SpiRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm14SpiRstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14SpiRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_SPI_RST_CLR` writer - FLEXCOMM14 SPI reset clear"]
pub type Flexcomm14SpiRstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14SpiRstClr>;
impl<'a, REG> Flexcomm14SpiRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRstClr::ClrReset)
    }
}
#[doc = "FLEXCOMM15 I2C reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm15I2cRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Flexcomm15I2cRstClr> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm15I2cRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15_I2C_RST_CLR` writer - FLEXCOMM15 I2C reset clear"]
pub type Flexcomm15I2cRstClrW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm15I2cRstClr>;
impl<'a, REG> Flexcomm15I2cRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRstClr::ClrReset)
    }
}
#[doc = "DMIC0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<Dmic0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Dmic0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_RST_CLR` writer - DMIC0 reset clear"]
pub type Dmic0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Dmic0RstClr>;
impl<'a, REG> Dmic0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0RstClr::ClrReset)
    }
}
#[doc = "osevent timer reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsevtTimerRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL0 Bit"]
    ClrReset = 1,
}
impl From<OsevtTimerRstClr> for bool {
    #[inline(always)]
    fn from(variant: OsevtTimerRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVT_TIMER_RST_CLR` writer - osevent timer reset clear"]
pub type OsevtTimerRstClrW<'a, REG> = crate::BitWriter<'a, REG, OsevtTimerRstClr>;
impl<'a, REG> OsevtTimerRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRstClr::ClrReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 8 - FLEXCOMM0 reset clear"]
    #[inline(always)]
    pub fn flexcomm0_rst_clr(&mut self) -> Flexcomm0RstClrW<Prstctl0ClrSpec> {
        Flexcomm0RstClrW::new(self, 8)
    }
    #[doc = "Bit 9 - FLEXCOMM1 reset clear"]
    #[inline(always)]
    pub fn flexcomm1_rst_clr(&mut self) -> Flexcomm1RstClrW<Prstctl0ClrSpec> {
        Flexcomm1RstClrW::new(self, 9)
    }
    #[doc = "Bit 10 - FLEXCOMM2 reset clear"]
    #[inline(always)]
    pub fn flexcomm2_rst_clr(&mut self) -> Flexcomm2RstClrW<Prstctl0ClrSpec> {
        Flexcomm2RstClrW::new(self, 10)
    }
    #[doc = "Bit 11 - FLEXCOMM3 reset clear"]
    #[inline(always)]
    pub fn flexcomm3_rst_clr(&mut self) -> Flexcomm3RstClrW<Prstctl0ClrSpec> {
        Flexcomm3RstClrW::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCOMM4 reset clear"]
    #[inline(always)]
    pub fn flexcomm4_rst_clr(&mut self) -> Flexcomm4RstClrW<Prstctl0ClrSpec> {
        Flexcomm4RstClrW::new(self, 12)
    }
    #[doc = "Bit 13 - FLEXCOMM5 reset clear"]
    #[inline(always)]
    pub fn flexcomm5_rst_clr(&mut self) -> Flexcomm5RstClrW<Prstctl0ClrSpec> {
        Flexcomm5RstClrW::new(self, 13)
    }
    #[doc = "Bit 14 - FLEXCOMM6 reset clear"]
    #[inline(always)]
    pub fn flexcomm6_rst_clr(&mut self) -> Flexcomm6RstClrW<Prstctl0ClrSpec> {
        Flexcomm6RstClrW::new(self, 14)
    }
    #[doc = "Bit 15 - FLEXCOMM7 reset clear"]
    #[inline(always)]
    pub fn flexcomm7_rst_clr(&mut self) -> Flexcomm7RstClrW<Prstctl0ClrSpec> {
        Flexcomm7RstClrW::new(self, 15)
    }
    #[doc = "Bit 22 - FLEXCOMM14 SPI reset clear"]
    #[inline(always)]
    pub fn flexcomm14_spi_rst_clr(&mut self) -> Flexcomm14SpiRstClrW<Prstctl0ClrSpec> {
        Flexcomm14SpiRstClrW::new(self, 22)
    }
    #[doc = "Bit 23 - FLEXCOMM15 I2C reset clear"]
    #[inline(always)]
    pub fn flexcomm15_i2c_rst_clr(&mut self) -> Flexcomm15I2cRstClrW<Prstctl0ClrSpec> {
        Flexcomm15I2cRstClrW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 reset clear"]
    #[inline(always)]
    pub fn dmic0_rst_clr(&mut self) -> Dmic0RstClrW<Prstctl0ClrSpec> {
        Dmic0RstClrW::new(self, 24)
    }
    #[doc = "Bit 27 - osevent timer reset clear"]
    #[inline(always)]
    pub fn osevt_timer_rst_clr(&mut self) -> OsevtTimerRstClrW<Prstctl0ClrSpec> {
        OsevtTimerRstClrW::new(self, 27)
    }
}
#[doc = "peripheral reset clear register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0ClrSpec;
impl crate::RegisterSpec for Prstctl0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl0_clr::W`](W) writer structure"]
impl crate::Writable for Prstctl0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_CLR to value 0"]
impl crate::Resettable for Prstctl0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
