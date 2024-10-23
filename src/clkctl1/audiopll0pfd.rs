#[doc = "Register `AUDIOPLL0PFD` reader"]
pub type R = crate::R<Audiopll0pfdSpec>;
#[doc = "Register `AUDIOPLL0PFD` writer"]
pub type W = crate::W<Audiopll0pfdSpec>;
#[doc = "Field `PFD0` reader - PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd0R = crate::FieldReader;
#[doc = "Field `PFD0` writer - PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PFD0_CLKRDY` reader - PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd0ClkrdyR = crate::BitReader;
#[doc = "Field `PFD0_CLKRDY` writer - PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd0ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfd0Clkgate {
    #[doc = "0: PFD0 clock is not gated."]
    NotGated = 0,
    #[doc = "1: PFD0 clock is gated."]
    Gated = 1,
}
impl From<Pfd0Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Pfd0Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFD0_CLKGATE` reader - PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
pub type Pfd0ClkgateR = crate::BitReader<Pfd0Clkgate>;
impl Pfd0ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfd0Clkgate {
        match self.bits {
            false => Pfd0Clkgate::NotGated,
            true => Pfd0Clkgate::Gated,
        }
    }
    #[doc = "PFD0 clock is not gated."]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == Pfd0Clkgate::NotGated
    }
    #[doc = "PFD0 clock is gated."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Pfd0Clkgate::Gated
    }
}
#[doc = "Field `PFD0_CLKGATE` writer - PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
pub type Pfd0ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Pfd0Clkgate>;
impl<'a, REG> Pfd0ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFD0 clock is not gated."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd0Clkgate::NotGated)
    }
    #[doc = "PFD0 clock is gated."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd0Clkgate::Gated)
    }
}
#[doc = "Field `PFD1` reader - PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd1R = crate::FieldReader;
#[doc = "Field `PFD1` writer - PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PFD1_CLKRDY` reader - PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd1ClkrdyR = crate::BitReader;
#[doc = "Field `PFD1_CLKRDY` writer - PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd1ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfd1Clkgate {
    #[doc = "0: PFD1 clock is not gated."]
    NotGated = 0,
    #[doc = "1: PFD1 clock is gated."]
    Gated = 1,
}
impl From<Pfd1Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Pfd1Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFD1_CLKGATE` reader - PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
pub type Pfd1ClkgateR = crate::BitReader<Pfd1Clkgate>;
impl Pfd1ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfd1Clkgate {
        match self.bits {
            false => Pfd1Clkgate::NotGated,
            true => Pfd1Clkgate::Gated,
        }
    }
    #[doc = "PFD1 clock is not gated."]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == Pfd1Clkgate::NotGated
    }
    #[doc = "PFD1 clock is gated."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Pfd1Clkgate::Gated
    }
}
#[doc = "Field `PFD1_CLKGATE` writer - PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
pub type Pfd1ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Pfd1Clkgate>;
impl<'a, REG> Pfd1ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFD1 clock is not gated."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd1Clkgate::NotGated)
    }
    #[doc = "PFD1 clock is gated."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd1Clkgate::Gated)
    }
}
#[doc = "Field `PFD2` reader - PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd2R = crate::FieldReader;
#[doc = "Field `PFD2` writer - PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PFD2_CLKRDY` reader - PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd2ClkrdyR = crate::BitReader;
#[doc = "Field `PFD2_CLKRDY` writer - PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd2ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfd2Clkgate {
    #[doc = "0: PFD2 clock is not gated."]
    NotGated = 0,
    #[doc = "1: PFD2 clock is gated."]
    Gated = 1,
}
impl From<Pfd2Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Pfd2Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFD2_CLKGATE` reader - PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
pub type Pfd2ClkgateR = crate::BitReader<Pfd2Clkgate>;
impl Pfd2ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfd2Clkgate {
        match self.bits {
            false => Pfd2Clkgate::NotGated,
            true => Pfd2Clkgate::Gated,
        }
    }
    #[doc = "PFD2 clock is not gated."]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == Pfd2Clkgate::NotGated
    }
    #[doc = "PFD2 clock is gated."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Pfd2Clkgate::Gated
    }
}
#[doc = "Field `PFD2_CLKGATE` writer - PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
pub type Pfd2ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Pfd2Clkgate>;
impl<'a, REG> Pfd2ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFD2 clock is not gated."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd2Clkgate::NotGated)
    }
    #[doc = "PFD2 clock is gated."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd2Clkgate::Gated)
    }
}
#[doc = "Field `PFD3` reader - PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd3R = crate::FieldReader;
#[doc = "Field `PFD3` writer - PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
pub type Pfd3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PFD3_CLKRDY` reader - PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd3ClkrdyR = crate::BitReader;
#[doc = "Field `PFD3_CLKRDY` writer - PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
pub type Pfd3ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfd3Clkgate {
    #[doc = "0: PFD3 clock is not gated."]
    NotGated = 0,
    #[doc = "1: PFD3 clock is gated."]
    Gated = 1,
}
impl From<Pfd3Clkgate> for bool {
    #[inline(always)]
    fn from(variant: Pfd3Clkgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFD3_CLKGATE` reader - PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
pub type Pfd3ClkgateR = crate::BitReader<Pfd3Clkgate>;
impl Pfd3ClkgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfd3Clkgate {
        match self.bits {
            false => Pfd3Clkgate::NotGated,
            true => Pfd3Clkgate::Gated,
        }
    }
    #[doc = "PFD3 clock is not gated."]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == Pfd3Clkgate::NotGated
    }
    #[doc = "PFD3 clock is gated."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Pfd3Clkgate::Gated
    }
}
#[doc = "Field `PFD3_CLKGATE` writer - PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
pub type Pfd3ClkgateW<'a, REG> = crate::BitWriter<'a, REG, Pfd3Clkgate>;
impl<'a, REG> Pfd3ClkgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFD3 clock is not gated."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd3Clkgate::NotGated)
    }
    #[doc = "PFD3 clock is gated."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Pfd3Clkgate::Gated)
    }
}
impl R {
    #[doc = "Bits 0:5 - PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn pfd0(&self) -> Pfd0R {
        Pfd0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub fn pfd0_clkrdy(&self) -> Pfd0ClkrdyR {
        Pfd0ClkrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[inline(always)]
    pub fn pfd0_clkgate(&self) -> Pfd0ClkgateR {
        Pfd0ClkgateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn pfd1(&self) -> Pfd1R {
        Pfd1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub fn pfd1_clkrdy(&self) -> Pfd1ClkrdyR {
        Pfd1ClkrdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[inline(always)]
    pub fn pfd1_clkgate(&self) -> Pfd1ClkgateR {
        Pfd1ClkgateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn pfd2(&self) -> Pfd2R {
        Pfd2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub fn pfd2_clkrdy(&self) -> Pfd2ClkrdyR {
        Pfd2ClkrdyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[inline(always)]
    pub fn pfd2_clkgate(&self) -> Pfd2ClkgateR {
        Pfd2ClkgateR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn pfd3(&self) -> Pfd3R {
        Pfd3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub fn pfd3_clkrdy(&self) -> Pfd3ClkrdyR {
        Pfd3ClkrdyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[inline(always)]
    pub fn pfd3_clkgate(&self) -> Pfd3ClkgateR {
        Pfd3ClkgateR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIOPLL0PFD")
            .field("pfd0", &self.pfd0())
            .field("pfd0_clkrdy", &self.pfd0_clkrdy())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1", &self.pfd1())
            .field("pfd1_clkrdy", &self.pfd1_clkrdy())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2", &self.pfd2())
            .field("pfd2_clkrdy", &self.pfd2_clkrdy())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3", &self.pfd3())
            .field("pfd3_clkrdy", &self.pfd3_clkrdy())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    #[must_use]
    pub fn pfd0(&mut self) -> Pfd0W<Audiopll0pfdSpec> {
        Pfd0W::new(self, 0)
    }
    #[doc = "Bit 6 - PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_clkrdy(&mut self) -> Pfd0ClkrdyW<Audiopll0pfdSpec> {
        Pfd0ClkrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_clkgate(&mut self) -> Pfd0ClkgateW<Audiopll0pfdSpec> {
        Pfd0ClkgateW::new(self, 7)
    }
    #[doc = "Bits 8:13 - PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    #[must_use]
    pub fn pfd1(&mut self) -> Pfd1W<Audiopll0pfdSpec> {
        Pfd1W::new(self, 8)
    }
    #[doc = "Bit 14 - PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_clkrdy(&mut self) -> Pfd1ClkrdyW<Audiopll0pfdSpec> {
        Pfd1ClkrdyW::new(self, 14)
    }
    #[doc = "Bit 15 - PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_clkgate(&mut self) -> Pfd1ClkgateW<Audiopll0pfdSpec> {
        Pfd1ClkgateW::new(self, 15)
    }
    #[doc = "Bits 16:21 - PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    #[must_use]
    pub fn pfd2(&mut self) -> Pfd2W<Audiopll0pfdSpec> {
        Pfd2W::new(self, 16)
    }
    #[doc = "Bit 22 - PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_clkrdy(&mut self) -> Pfd2ClkrdyW<Audiopll0pfdSpec> {
        Pfd2ClkrdyW::new(self, 22)
    }
    #[doc = "Bit 23 - PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_clkgate(&mut self) -> Pfd2ClkgateW<Audiopll0pfdSpec> {
        Pfd2ClkgateW::new(self, 23)
    }
    #[doc = "Bits 24:29 - PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    #[must_use]
    pub fn pfd3(&mut self) -> Pfd3W<Audiopll0pfdSpec> {
        Pfd3W::new(self, 24)
    }
    #[doc = "Bit 30 - PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_clkrdy(&mut self) -> Pfd3ClkrdyW<Audiopll0pfdSpec> {
        Pfd3ClkrdyW::new(self, 30)
    }
    #[doc = "Bit 31 - PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_clkgate(&mut self) -> Pfd3ClkgateW<Audiopll0pfdSpec> {
        Pfd3ClkgateW::new(self, 31)
    }
}
#[doc = "audio pll0 PFD\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0pfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0pfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audiopll0pfdSpec;
impl crate::RegisterSpec for Audiopll0pfdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiopll0pfd::R`](R) reader structure"]
impl crate::Readable for Audiopll0pfdSpec {}
#[doc = "`write(|w| ..)` method takes [`audiopll0pfd::W`](W) writer structure"]
impl crate::Writable for Audiopll0pfdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0PFD to value 0x8080_8080"]
impl crate::Resettable for Audiopll0pfdSpec {
    const RESET_VALUE: u32 = 0x8080_8080;
}
