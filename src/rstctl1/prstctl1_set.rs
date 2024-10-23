#[doc = "Register `PRSTCTL1_SET` writer"]
pub type W = crate::W<Prstctl1SetSpec>;
#[doc = "HSGPIO0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_RST_SET` writer - HSGPIO0 reset set"]
pub type Hsgpio0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0RstSet>;
impl<'a, REG> Hsgpio0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0RstSet::SetReset)
    }
}
#[doc = "HSGPIO1 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio1RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_RST_SET` writer - HSGPIO1 reset set"]
pub type Hsgpio1RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1RstSet>;
impl<'a, REG> Hsgpio1RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1RstSet::SetReset)
    }
}
#[doc = "HSGPIO2 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio2RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_RST_SET` writer - HSGPIO2 reset set"]
pub type Hsgpio2RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2RstSet>;
impl<'a, REG> Hsgpio2RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2RstSet::SetReset)
    }
}
#[doc = "HSGPIO3 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio3RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_RST_SET` writer - HSGPIO3 reset set"]
pub type Hsgpio3RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3RstSet>;
impl<'a, REG> Hsgpio3RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3RstSet::SetReset)
    }
}
#[doc = "HSGPIO4 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio4RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_RST_SET` writer - HSGPIO4 reset set"]
pub type Hsgpio4RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4RstSet>;
impl<'a, REG> Hsgpio4RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4RstSet::SetReset)
    }
}
#[doc = "HSGPIO5 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio5RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_RST_SET` writer - HSGPIO5 reset set"]
pub type Hsgpio5RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5RstSet>;
impl<'a, REG> Hsgpio5RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5RstSet::SetReset)
    }
}
#[doc = "HSGPIO6 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio6RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_RST_SET` writer - HSGPIO6 reset set"]
pub type Hsgpio6RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6RstSet>;
impl<'a, REG> Hsgpio6RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6RstSet::SetReset)
    }
}
#[doc = "HSGPIO7 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Hsgpio7RstSet> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_RST_SET` writer - HSGPIO7 reset set"]
pub type Hsgpio7RstSetW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7RstSet>;
impl<'a, REG> Hsgpio7RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7RstSet::SetReset)
    }
}
#[doc = "CRC reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<CrcRstSet> for bool {
    #[inline(always)]
    fn from(variant: CrcRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_RST_SET` writer - CRC reset set"]
pub type CrcRstSetW<'a, REG> = crate::BitWriter<'a, REG, CrcRstSet>;
impl<'a, REG> CrcRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRstSet::SetReset)
    }
}
#[doc = "DMAC0 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Dmac0RstSet> for bool {
    #[inline(always)]
    fn from(variant: Dmac0RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_RST_SET` writer - DMAC0 reset set"]
pub type Dmac0RstSetW<'a, REG> = crate::BitWriter<'a, REG, Dmac0RstSet>;
impl<'a, REG> Dmac0RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0RstSet::SetReset)
    }
}
#[doc = "DMAC1 reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1RstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<Dmac1RstSet> for bool {
    #[inline(always)]
    fn from(variant: Dmac1RstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_RST_SET` writer - DMAC1 reset set"]
pub type Dmac1RstSetW<'a, REG> = crate::BitWriter<'a, REG, Dmac1RstSet>;
impl<'a, REG> Dmac1RstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1RstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1RstSet::SetReset)
    }
}
#[doc = "MU reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<MuRstSet> for bool {
    #[inline(always)]
    fn from(variant: MuRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_RST_SET` writer - MU reset set"]
pub type MuRstSetW<'a, REG> = crate::BitWriter<'a, REG, MuRstSet>;
impl<'a, REG> MuRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MuRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(MuRstSet::SetReset)
    }
}
#[doc = "SEMA reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<SemaRstSet> for bool {
    #[inline(always)]
    fn from(variant: SemaRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_RST_SET` writer - SEMA reset set"]
pub type SemaRstSetW<'a, REG> = crate::BitWriter<'a, REG, SemaRstSet>;
impl<'a, REG> SemaRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRstSet::SetReset)
    }
}
#[doc = "FREQME reset set\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeRstSet {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SetReset = 1,
}
impl From<FreqmeRstSet> for bool {
    #[inline(always)]
    fn from(variant: FreqmeRstSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_RST_SET` writer - FREQME reset set"]
pub type FreqmeRstSetW<'a, REG> = crate::BitWriter<'a, REG, FreqmeRstSet>;
impl<'a, REG> FreqmeRstSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRstSet::NoEffect)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRstSet::SetReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl1SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0_rst_set(&mut self) -> Hsgpio0RstSetW<Prstctl1SetSpec> {
        Hsgpio0RstSetW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1_rst_set(&mut self) -> Hsgpio1RstSetW<Prstctl1SetSpec> {
        Hsgpio1RstSetW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2_rst_set(&mut self) -> Hsgpio2RstSetW<Prstctl1SetSpec> {
        Hsgpio2RstSetW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3_rst_set(&mut self) -> Hsgpio3RstSetW<Prstctl1SetSpec> {
        Hsgpio3RstSetW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4_rst_set(&mut self) -> Hsgpio4RstSetW<Prstctl1SetSpec> {
        Hsgpio4RstSetW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5_rst_set(&mut self) -> Hsgpio5RstSetW<Prstctl1SetSpec> {
        Hsgpio5RstSetW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6_rst_set(&mut self) -> Hsgpio6RstSetW<Prstctl1SetSpec> {
        Hsgpio6RstSetW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7_rst_set(&mut self) -> Hsgpio7RstSetW<Prstctl1SetSpec> {
        Hsgpio7RstSetW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC reset set"]
    #[inline(always)]
    #[must_use]
    pub fn crc_rst_set(&mut self) -> CrcRstSetW<Prstctl1SetSpec> {
        CrcRstSetW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_rst_set(&mut self) -> Dmac0RstSetW<Prstctl1SetSpec> {
        Dmac0RstSetW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_rst_set(&mut self) -> Dmac1RstSetW<Prstctl1SetSpec> {
        Dmac1RstSetW::new(self, 24)
    }
    #[doc = "Bit 28 - MU reset set"]
    #[inline(always)]
    #[must_use]
    pub fn mu_rst_set(&mut self) -> MuRstSetW<Prstctl1SetSpec> {
        MuRstSetW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA reset set"]
    #[inline(always)]
    #[must_use]
    pub fn sema_rst_set(&mut self) -> SemaRstSetW<Prstctl1SetSpec> {
        SemaRstSetW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME reset set"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_rst_set(&mut self) -> FreqmeRstSetW<Prstctl1SetSpec> {
        FreqmeRstSetW::new(self, 31)
    }
}
#[doc = "peripheral reset set register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl1SetSpec;
impl crate::RegisterSpec for Prstctl1SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prstctl1_set::W`](W) writer structure"]
impl crate::Writable for Prstctl1SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL1_SET to value 0"]
impl crate::Resettable for Prstctl1SetSpec {
    const RESET_VALUE: u32 = 0;
}
