#[doc = "Register `PADVRANGE` reader"]
pub type R = crate::R<PadvrangeSpec>;
#[doc = "Register `PADVRANGE` writer"]
pub type W = crate::W<PadvrangeSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vddio0range {
    #[doc = "0: 1.71 - 3.6V. Consumes static current to detect VDDE0 level"]
    Vddio0range0 = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    Vddio0range1 = 1,
    #[doc = "2: 3.00 - 3.6V, vdde detector off"]
    Vddio0range2 = 2,
    #[doc = "3: Not allowed (hardware should translate to 10)"]
    Vddio0range3 = 3,
}
impl From<Vddio0range> for u8 {
    #[inline(always)]
    fn from(variant: Vddio0range) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vddio0range {
    type Ux = u8;
}
impl crate::IsEnum for Vddio0range {}
#[doc = "Field `VDDIO_0RANGE` reader - no description available"]
pub type Vddio0rangeR = crate::FieldReader<Vddio0range>;
impl Vddio0rangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddio0range {
        match self.bits {
            0 => Vddio0range::Vddio0range0,
            1 => Vddio0range::Vddio0range1,
            2 => Vddio0range::Vddio0range2,
            3 => Vddio0range::Vddio0range3,
            _ => unreachable!(),
        }
    }
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE0 level"]
    #[inline(always)]
    pub fn is_vddio_0range_0(&self) -> bool {
        *self == Vddio0range::Vddio0range0
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_0range_1(&self) -> bool {
        *self == Vddio0range::Vddio0range1
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_0range_2(&self) -> bool {
        *self == Vddio0range::Vddio0range2
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn is_vddio_0range_3(&self) -> bool {
        *self == Vddio0range::Vddio0range3
    }
}
#[doc = "Field `VDDIO_0RANGE` writer - no description available"]
pub type Vddio0rangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vddio0range, crate::Safe>;
impl<'a, REG> Vddio0rangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE0 level"]
    #[inline(always)]
    pub fn vddio_0range_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio0range::Vddio0range0)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_0range_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio0range::Vddio0range1)
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_0range_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio0range::Vddio0range2)
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn vddio_0range_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio0range::Vddio0range3)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vddio1range {
    #[doc = "0: 1.71-3.6V. Consumes static current to detect VDDE1 level"]
    Vddio1range0 = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    Vddio1range1 = 1,
    #[doc = "2: 3.00 - 3.6V, vdde detector off"]
    Vddio1range2 = 2,
    #[doc = "3: Not allowed (hardware should translate to 10)"]
    Vddio1range3 = 3,
}
impl From<Vddio1range> for u8 {
    #[inline(always)]
    fn from(variant: Vddio1range) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vddio1range {
    type Ux = u8;
}
impl crate::IsEnum for Vddio1range {}
#[doc = "Field `VDDIO_1RANGE` reader - no description available"]
pub type Vddio1rangeR = crate::FieldReader<Vddio1range>;
impl Vddio1rangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddio1range {
        match self.bits {
            0 => Vddio1range::Vddio1range0,
            1 => Vddio1range::Vddio1range1,
            2 => Vddio1range::Vddio1range2,
            3 => Vddio1range::Vddio1range3,
            _ => unreachable!(),
        }
    }
    #[doc = "1.71-3.6V. Consumes static current to detect VDDE1 level"]
    #[inline(always)]
    pub fn is_vddio_1range_0(&self) -> bool {
        *self == Vddio1range::Vddio1range0
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_1range_1(&self) -> bool {
        *self == Vddio1range::Vddio1range1
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_1range_2(&self) -> bool {
        *self == Vddio1range::Vddio1range2
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn is_vddio_1range_3(&self) -> bool {
        *self == Vddio1range::Vddio1range3
    }
}
#[doc = "Field `VDDIO_1RANGE` writer - no description available"]
pub type Vddio1rangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vddio1range, crate::Safe>;
impl<'a, REG> Vddio1rangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.71-3.6V. Consumes static current to detect VDDE1 level"]
    #[inline(always)]
    pub fn vddio_1range_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio1range::Vddio1range0)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_1range_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio1range::Vddio1range1)
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_1range_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio1range::Vddio1range2)
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn vddio_1range_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio1range::Vddio1range3)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vddio2range {
    #[doc = "0: 1.71 - 3.6V. Consumes static current to detect VDDE2 level"]
    Vddio2range0 = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    Vddio2range1 = 1,
    #[doc = "2: 3.00 - 3.6V, vdde detector off"]
    Vddio2range2 = 2,
    #[doc = "3: Not allowed (hardware should translate to 10)"]
    Vddio2range3 = 3,
}
impl From<Vddio2range> for u8 {
    #[inline(always)]
    fn from(variant: Vddio2range) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vddio2range {
    type Ux = u8;
}
impl crate::IsEnum for Vddio2range {}
#[doc = "Field `VDDIO_2RANGE` reader - no description available"]
pub type Vddio2rangeR = crate::FieldReader<Vddio2range>;
impl Vddio2rangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddio2range {
        match self.bits {
            0 => Vddio2range::Vddio2range0,
            1 => Vddio2range::Vddio2range1,
            2 => Vddio2range::Vddio2range2,
            3 => Vddio2range::Vddio2range3,
            _ => unreachable!(),
        }
    }
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE2 level"]
    #[inline(always)]
    pub fn is_vddio_2range_0(&self) -> bool {
        *self == Vddio2range::Vddio2range0
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_2range_1(&self) -> bool {
        *self == Vddio2range::Vddio2range1
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn is_vddio_2range_2(&self) -> bool {
        *self == Vddio2range::Vddio2range2
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn is_vddio_2range_3(&self) -> bool {
        *self == Vddio2range::Vddio2range3
    }
}
#[doc = "Field `VDDIO_2RANGE` writer - no description available"]
pub type Vddio2rangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vddio2range, crate::Safe>;
impl<'a, REG> Vddio2rangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE2 level"]
    #[inline(always)]
    pub fn vddio_2range_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2range::Vddio2range0)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_2range_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2range::Vddio2range1)
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn vddio_2range_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2range::Vddio2range2)
    }
    #[doc = "Not allowed (hardware should translate to 10)"]
    #[inline(always)]
    pub fn vddio_2range_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vddio2range::Vddio2range3)
    }
}
impl R {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn vddio_0range(&self) -> Vddio0rangeR {
        Vddio0rangeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - no description available"]
    #[inline(always)]
    pub fn vddio_1range(&self) -> Vddio1rangeR {
        Vddio1rangeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    pub fn vddio_2range(&self) -> Vddio2rangeR {
        Vddio2rangeR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADVRANGE")
            .field("vddio_0range", &self.vddio_0range())
            .field("vddio_1range", &self.vddio_1range())
            .field("vddio_2range", &self.vddio_2range())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_0range(&mut self) -> Vddio0rangeW<PadvrangeSpec> {
        Vddio0rangeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_1range(&mut self) -> Vddio1rangeW<PadvrangeSpec> {
        Vddio1rangeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_2range(&mut self) -> Vddio2rangeW<PadvrangeSpec> {
        Vddio2rangeW::new(self, 4)
    }
}
#[doc = "GPIO vdde range selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`padvrange::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padvrange::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadvrangeSpec;
impl crate::RegisterSpec for PadvrangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padvrange::R`](R) reader structure"]
impl crate::Readable for PadvrangeSpec {}
#[doc = "`write(|w| ..)` method takes [`padvrange::W`](W) writer structure"]
impl crate::Writable for PadvrangeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADVRANGE to value 0"]
impl crate::Resettable for PadvrangeSpec {
    const RESET_VALUE: u32 = 0;
}
