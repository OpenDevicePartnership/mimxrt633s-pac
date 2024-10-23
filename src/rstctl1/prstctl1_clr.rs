#[doc = "Register `PRSTCTL1_CLR` writer"]
pub type W = crate::W<Prstctl1ClrSpec>;
#[doc = "HSGPIO0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_RST_CLR` writer - HSGPIO0 reset clear"]
pub type Hsgpio0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0RstClr>;
impl<'a, REG> Hsgpio0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0RstClr::ClrReset)
    }
}
#[doc = "HSGPIO1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio1RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_RST_CLR` writer - HSGPIO1 reset clear"]
pub type Hsgpio1RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1RstClr>;
impl<'a, REG> Hsgpio1RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1RstClr::ClrReset)
    }
}
#[doc = "HSGPIO2 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio2RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_RST_CLR` writer - HSGPIO2 reset clear"]
pub type Hsgpio2RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2RstClr>;
impl<'a, REG> Hsgpio2RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2RstClr::ClrReset)
    }
}
#[doc = "HSGPIO3 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio3RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_RST_CLR` writer - HSGPIO3 reset clear"]
pub type Hsgpio3RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3RstClr>;
impl<'a, REG> Hsgpio3RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3RstClr::ClrReset)
    }
}
#[doc = "HSGPIO4 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio4RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_RST_CLR` writer - HSGPIO4 reset clear"]
pub type Hsgpio4RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4RstClr>;
impl<'a, REG> Hsgpio4RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4RstClr::ClrReset)
    }
}
#[doc = "HSGPIO5 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio5RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_RST_CLR` writer - HSGPIO5 reset clear"]
pub type Hsgpio5RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5RstClr>;
impl<'a, REG> Hsgpio5RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5RstClr::ClrReset)
    }
}
#[doc = "HSGPIO6 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio6RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_RST_CLR` writer - HSGPIO6 reset clear"]
pub type Hsgpio6RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6RstClr>;
impl<'a, REG> Hsgpio6RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6RstClr::ClrReset)
    }
}
#[doc = "HSGPIO7 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Hsgpio7RstClr> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_RST_CLR` writer - HSGPIO7 reset clear"]
pub type Hsgpio7RstClrW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7RstClr>;
impl<'a, REG> Hsgpio7RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7RstClr::ClrReset)
    }
}
#[doc = "CRC reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<CrcRstClr> for bool {
    #[inline(always)]
    fn from(variant: CrcRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_RST_CLR` writer - CRC reset clear"]
pub type CrcRstClrW<'a, REG> = crate::BitWriter<'a, REG, CrcRstClr>;
impl<'a, REG> CrcRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRstClr::ClrReset)
    }
}
#[doc = "DMAC0 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Dmac0RstClr> for bool {
    #[inline(always)]
    fn from(variant: Dmac0RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_RST_CLR` writer - DMAC0 reset clear"]
pub type Dmac0RstClrW<'a, REG> = crate::BitWriter<'a, REG, Dmac0RstClr>;
impl<'a, REG> Dmac0RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0RstClr::ClrReset)
    }
}
#[doc = "DMAC1 reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1RstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<Dmac1RstClr> for bool {
    #[inline(always)]
    fn from(variant: Dmac1RstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_RST_CLR` writer - DMAC1 reset clear"]
pub type Dmac1RstClrW<'a, REG> = crate::BitWriter<'a, REG, Dmac1RstClr>;
impl<'a, REG> Dmac1RstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1RstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1RstClr::ClrReset)
    }
}
#[doc = "MU reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<MuRstClr> for bool {
    #[inline(always)]
    fn from(variant: MuRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_RST_CLR` writer - MU reset clear"]
pub type MuRstClrW<'a, REG> = crate::BitWriter<'a, REG, MuRstClr>;
impl<'a, REG> MuRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MuRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(MuRstClr::ClrReset)
    }
}
#[doc = "SEMA reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<SemaRstClr> for bool {
    #[inline(always)]
    fn from(variant: SemaRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_RST_CLR` writer - SEMA reset clear"]
pub type SemaRstClrW<'a, REG> = crate::BitWriter<'a, REG, SemaRstClr>;
impl<'a, REG> SemaRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRstClr::ClrReset)
    }
}
#[doc = "FREQME reset clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeRstClr {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: clears the PRSTCTL1 Bit"]
    ClrReset = 1,
}
impl From<FreqmeRstClr> for bool {
    #[inline(always)]
    fn from(variant: FreqmeRstClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_RST_CLR` writer - FREQME reset clear"]
pub type FreqmeRstClrW<'a, REG> = crate::BitWriter<'a, REG, FreqmeRstClr>;
impl<'a, REG> FreqmeRstClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRstClr::NoEffect)
    }
    #[doc = "clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn clr_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRstClr::ClrReset)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Prstctl1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0_rst_clr(&mut self) -> Hsgpio0RstClrW<Prstctl1ClrSpec> {
        Hsgpio0RstClrW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1_rst_clr(&mut self) -> Hsgpio1RstClrW<Prstctl1ClrSpec> {
        Hsgpio1RstClrW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2_rst_clr(&mut self) -> Hsgpio2RstClrW<Prstctl1ClrSpec> {
        Hsgpio2RstClrW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3_rst_clr(&mut self) -> Hsgpio3RstClrW<Prstctl1ClrSpec> {
        Hsgpio3RstClrW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4_rst_clr(&mut self) -> Hsgpio4RstClrW<Prstctl1ClrSpec> {
        Hsgpio4RstClrW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5_rst_clr(&mut self) -> Hsgpio5RstClrW<Prstctl1ClrSpec> {
        Hsgpio5RstClrW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6_rst_clr(&mut self) -> Hsgpio6RstClrW<Prstctl1ClrSpec> {
        Hsgpio6RstClrW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7_rst_clr(&mut self) -> Hsgpio7RstClrW<Prstctl1ClrSpec> {
        Hsgpio7RstClrW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn crc_rst_clr(&mut self) -> CrcRstClrW<Prstctl1ClrSpec> {
        CrcRstClrW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_rst_clr(&mut self) -> Dmac0RstClrW<Prstctl1ClrSpec> {
        Dmac0RstClrW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_rst_clr(&mut self) -> Dmac1RstClrW<Prstctl1ClrSpec> {
        Dmac1RstClrW::new(self, 24)
    }
    #[doc = "Bit 28 - MU reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn mu_rst_clr(&mut self) -> MuRstClrW<Prstctl1ClrSpec> {
        MuRstClrW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn sema_rst_clr(&mut self) -> SemaRstClrW<Prstctl1ClrSpec> {
        SemaRstClrW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_rst_clr(&mut self) -> FreqmeRstClrW<Prstctl1ClrSpec> {
        FreqmeRstClrW::new(self, 31)
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
