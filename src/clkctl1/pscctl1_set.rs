#[doc = "Register `PSCCTL1_SET` writer"]
pub type W = crate::W<Pscctl1SetSpec>;
#[doc = "HSGPIO0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_CLK_SET` writer - HSGPIO0 clock set"]
pub type Hsgpio0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0ClkSet>;
impl<'a, REG> Hsgpio0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0ClkSet::SetClock)
    }
}
#[doc = "HSGPIO1 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio1ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_CLK_SET` writer - HSGPIO1 clock set"]
pub type Hsgpio1ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1ClkSet>;
impl<'a, REG> Hsgpio1ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1ClkSet::SetClock)
    }
}
#[doc = "HSGPIO2 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio2ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_CLK_SET` writer - HSGPIO2 clock set"]
pub type Hsgpio2ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2ClkSet>;
impl<'a, REG> Hsgpio2ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2ClkSet::SetClock)
    }
}
#[doc = "HSGPIO3 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio3ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_CLK_SET` writer - HSGPIO3 clock set"]
pub type Hsgpio3ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3ClkSet>;
impl<'a, REG> Hsgpio3ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3ClkSet::SetClock)
    }
}
#[doc = "HSGPIO4 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio4ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_CLK_SET` writer - HSGPIO4 clock set"]
pub type Hsgpio4ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4ClkSet>;
impl<'a, REG> Hsgpio4ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4ClkSet::SetClock)
    }
}
#[doc = "HSGPIO5 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio5ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_CLK_SET` writer - HSGPIO5 clock set"]
pub type Hsgpio5ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5ClkSet>;
impl<'a, REG> Hsgpio5ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5ClkSet::SetClock)
    }
}
#[doc = "HSGPIO6 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio6ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_CLK_SET` writer - HSGPIO6 clock set"]
pub type Hsgpio6ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6ClkSet>;
impl<'a, REG> Hsgpio6ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6ClkSet::SetClock)
    }
}
#[doc = "HSGPIO7 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Hsgpio7ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_CLK_SET` writer - HSGPIO7 clock set"]
pub type Hsgpio7ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7ClkSet>;
impl<'a, REG> Hsgpio7ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7ClkSet::SetClock)
    }
}
#[doc = "CRC clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<CrcClkSet> for bool {
    #[inline(always)]
    fn from(variant: CrcClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_CLK_SET` writer - CRC clock set"]
pub type CrcClkSetW<'a, REG> = crate::BitWriter<'a, REG, CrcClkSet>;
impl<'a, REG> CrcClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CrcClkSet::SetClock)
    }
}
#[doc = "DMAC0 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Dmac0ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Dmac0ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_CLK_SET` writer - DMAC0 clock set"]
pub type Dmac0ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Dmac0ClkSet>;
impl<'a, REG> Dmac0ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0ClkSet::SetClock)
    }
}
#[doc = "DMAC1 clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1ClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<Dmac1ClkSet> for bool {
    #[inline(always)]
    fn from(variant: Dmac1ClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_CLK_SET` writer - DMAC1 clock set"]
pub type Dmac1ClkSetW<'a, REG> = crate::BitWriter<'a, REG, Dmac1ClkSet>;
impl<'a, REG> Dmac1ClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1ClkSet::SetClock)
    }
}
#[doc = "MU clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<MuClkSet> for bool {
    #[inline(always)]
    fn from(variant: MuClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_CLK_SET` writer - MU clock set"]
pub type MuClkSetW<'a, REG> = crate::BitWriter<'a, REG, MuClkSet>;
impl<'a, REG> MuClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MuClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(MuClkSet::SetClock)
    }
}
#[doc = "SEMA clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<SemaClkSet> for bool {
    #[inline(always)]
    fn from(variant: SemaClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_CLK_SET` writer - SEMA clock set"]
pub type SemaClkSetW<'a, REG> = crate::BitWriter<'a, REG, SemaClkSet>;
impl<'a, REG> SemaClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(SemaClkSet::SetClock)
    }
}
#[doc = "FREQME clock set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeClkSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PSCCTL1 Bit"]
    SetClock = 1,
}
impl From<FreqmeClkSet> for bool {
    #[inline(always)]
    fn from(variant: FreqmeClkSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_CLK_SET` writer - FREQME clock set"]
pub type FreqmeClkSetW<'a, REG> = crate::BitWriter<'a, REG, FreqmeClkSet>;
impl<'a, REG> FreqmeClkSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClkSet::NoEffect)
    }
    #[doc = "Sets the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn set_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeClkSet::SetClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl1SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 clock set"]
    #[inline(always)]
    pub fn hsgpio0_clk_set(&mut self) -> Hsgpio0ClkSetW<Pscctl1SetSpec> {
        Hsgpio0ClkSetW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 clock set"]
    #[inline(always)]
    pub fn hsgpio1_clk_set(&mut self) -> Hsgpio1ClkSetW<Pscctl1SetSpec> {
        Hsgpio1ClkSetW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 clock set"]
    #[inline(always)]
    pub fn hsgpio2_clk_set(&mut self) -> Hsgpio2ClkSetW<Pscctl1SetSpec> {
        Hsgpio2ClkSetW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 clock set"]
    #[inline(always)]
    pub fn hsgpio3_clk_set(&mut self) -> Hsgpio3ClkSetW<Pscctl1SetSpec> {
        Hsgpio3ClkSetW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 clock set"]
    #[inline(always)]
    pub fn hsgpio4_clk_set(&mut self) -> Hsgpio4ClkSetW<Pscctl1SetSpec> {
        Hsgpio4ClkSetW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 clock set"]
    #[inline(always)]
    pub fn hsgpio5_clk_set(&mut self) -> Hsgpio5ClkSetW<Pscctl1SetSpec> {
        Hsgpio5ClkSetW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 clock set"]
    #[inline(always)]
    pub fn hsgpio6_clk_set(&mut self) -> Hsgpio6ClkSetW<Pscctl1SetSpec> {
        Hsgpio6ClkSetW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 clock set"]
    #[inline(always)]
    pub fn hsgpio7_clk_set(&mut self) -> Hsgpio7ClkSetW<Pscctl1SetSpec> {
        Hsgpio7ClkSetW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC clock set"]
    #[inline(always)]
    pub fn crc_clk_set(&mut self) -> CrcClkSetW<Pscctl1SetSpec> {
        CrcClkSetW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 clock set"]
    #[inline(always)]
    pub fn dmac0_clk_set(&mut self) -> Dmac0ClkSetW<Pscctl1SetSpec> {
        Dmac0ClkSetW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 clock set"]
    #[inline(always)]
    pub fn dmac1_clk_set(&mut self) -> Dmac1ClkSetW<Pscctl1SetSpec> {
        Dmac1ClkSetW::new(self, 24)
    }
    #[doc = "Bit 28 - MU clock set"]
    #[inline(always)]
    pub fn mu_clk_set(&mut self) -> MuClkSetW<Pscctl1SetSpec> {
        MuClkSetW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA clock set"]
    #[inline(always)]
    pub fn sema_clk_set(&mut self) -> SemaClkSetW<Pscctl1SetSpec> {
        SemaClkSetW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME clock set"]
    #[inline(always)]
    pub fn freqme_clk_set(&mut self) -> FreqmeClkSetW<Pscctl1SetSpec> {
        FreqmeClkSetW::new(self, 31)
    }
}
#[doc = "clock set register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl1SetSpec;
impl crate::RegisterSpec for Pscctl1SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl1_set::W`](W) writer structure"]
impl crate::Writable for Pscctl1SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL1_SET to value 0"]
impl crate::Resettable for Pscctl1SetSpec {
    const RESET_VALUE: u32 = 0;
}
