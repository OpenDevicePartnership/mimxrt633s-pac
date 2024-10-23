#[doc = "Register `PSCCTL1_CLR` writer"]
pub type W = crate::W<Pscctl1ClrSpec>;
#[doc = "HSGPIO0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_CLK_CLR` writer - HSGPIO0 clock clear"]
pub type Hsgpio0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0ClkClr>;
impl<'a, REG> Hsgpio0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio1ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_CLK_CLR` writer - HSGPIO1 clock clear"]
pub type Hsgpio1ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1ClkClr>;
impl<'a, REG> Hsgpio1ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO2 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio2ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_CLK_CLR` writer - HSGPIO2 clock clear"]
pub type Hsgpio2ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2ClkClr>;
impl<'a, REG> Hsgpio2ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO3 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio3ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_CLK_CLR` writer - HSGPIO3 clock clear"]
pub type Hsgpio3ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3ClkClr>;
impl<'a, REG> Hsgpio3ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO4 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio4ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_CLK_CLR` writer - HSGPIO4 clock clear"]
pub type Hsgpio4ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4ClkClr>;
impl<'a, REG> Hsgpio4ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO5 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio5ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_CLK_CLR` writer - HSGPIO5 clock clear"]
pub type Hsgpio5ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5ClkClr>;
impl<'a, REG> Hsgpio5ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO6 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio6ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_CLK_CLR` writer - HSGPIO6 clock clear"]
pub type Hsgpio6ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6ClkClr>;
impl<'a, REG> Hsgpio6ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6ClkClr::ClrClock)
    }
}
#[doc = "HSGPIO7 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Hsgpio7ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_CLK_CLR` writer - HSGPIO7 clock clear"]
pub type Hsgpio7ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7ClkClr>;
impl<'a, REG> Hsgpio7ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7ClkClr::ClrClock)
    }
}
#[doc = "CRC clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<CrcClkClr> for bool {
    #[inline(always)]
    fn from(variant: CrcClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_CLK_CLR` writer - CRC clock clear"]
pub type CrcClkClrW<'a, REG> = crate::BitWriter<'a, REG, CrcClkClr>;
impl<'a, REG> CrcClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClkClr::ClrClock)
    }
}
#[doc = "DMAC0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Dmac0ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_CLK_CLR` writer - DMAC0 clock clear"]
pub type Dmac0ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Dmac0ClkClr>;
impl<'a, REG> Dmac0ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ClkClr::ClrClock)
    }
}
#[doc = "DMAC1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Dmac1ClkClr> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_CLK_CLR` writer - DMAC1 clock clear"]
pub type Dmac1ClkClrW<'a, REG> = crate::BitWriter<'a, REG, Dmac1ClkClr>;
impl<'a, REG> Dmac1ClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ClkClr::ClrClock)
    }
}
#[doc = "MU clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<MuClkClr> for bool {
    #[inline(always)]
    fn from(variant: MuClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_CLK_CLR` writer - MU clock clear"]
pub type MuClkClrW<'a, REG> = crate::BitWriter<'a, REG, MuClkClr>;
impl<'a, REG> MuClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MuClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MuClkClr::ClrClock)
    }
}
#[doc = "SEMA clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<SemaClkClr> for bool {
    #[inline(always)]
    fn from(variant: SemaClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_CLK_CLR` writer - SEMA clock clear"]
pub type SemaClkClrW<'a, REG> = crate::BitWriter<'a, REG, SemaClkClr>;
impl<'a, REG> SemaClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClkClr::ClrClock)
    }
}
#[doc = "FREQME clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeClkClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<FreqmeClkClr> for bool {
    #[inline(always)]
    fn from(variant: FreqmeClkClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_CLK_CLR` writer - FREQME clock clear"]
pub type FreqmeClkClrW<'a, REG> = crate::BitWriter<'a, REG, FreqmeClkClr>;
impl<'a, REG> FreqmeClkClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClkClr::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClkClr::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0_clk_clr(&mut self) -> Hsgpio0ClkClrW<Pscctl1ClrSpec> {
        Hsgpio0ClkClrW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1_clk_clr(&mut self) -> Hsgpio1ClkClrW<Pscctl1ClrSpec> {
        Hsgpio1ClkClrW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2_clk_clr(&mut self) -> Hsgpio2ClkClrW<Pscctl1ClrSpec> {
        Hsgpio2ClkClrW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3_clk_clr(&mut self) -> Hsgpio3ClkClrW<Pscctl1ClrSpec> {
        Hsgpio3ClkClrW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4_clk_clr(&mut self) -> Hsgpio4ClkClrW<Pscctl1ClrSpec> {
        Hsgpio4ClkClrW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5_clk_clr(&mut self) -> Hsgpio5ClkClrW<Pscctl1ClrSpec> {
        Hsgpio5ClkClrW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6_clk_clr(&mut self) -> Hsgpio6ClkClrW<Pscctl1ClrSpec> {
        Hsgpio6ClkClrW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7_clk_clr(&mut self) -> Hsgpio7ClkClrW<Pscctl1ClrSpec> {
        Hsgpio7ClkClrW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn crc_clk_clr(&mut self) -> CrcClkClrW<Pscctl1ClrSpec> {
        CrcClkClrW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_clk_clr(&mut self) -> Dmac0ClkClrW<Pscctl1ClrSpec> {
        Dmac0ClkClrW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_clk_clr(&mut self) -> Dmac1ClkClrW<Pscctl1ClrSpec> {
        Dmac1ClkClrW::new(self, 24)
    }
    #[doc = "Bit 28 - MU clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn mu_clk_clr(&mut self) -> MuClkClrW<Pscctl1ClrSpec> {
        MuClkClrW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn sema_clk_clr(&mut self) -> SemaClkClrW<Pscctl1ClrSpec> {
        SemaClkClrW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_clk_clr(&mut self) -> FreqmeClkClrW<Pscctl1ClrSpec> {
        FreqmeClkClrW::new(self, 31)
    }
}
#[doc = "clock clear register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl1ClrSpec;
impl crate::RegisterSpec for Pscctl1ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl1_clr::W`](W) writer structure"]
impl crate::Writable for Pscctl1ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL1_CLR to value 0"]
impl crate::Resettable for Pscctl1ClrSpec {
    const RESET_VALUE: u32 = 0;
}
