#[doc = "Register `FFROCTL0` reader"]
pub type R = crate::R<Ffroctl0Spec>;
#[doc = "Register `FFROCTL0` writer"]
pub type W = crate::W<Ffroctl0Spec>;
#[doc = "Field `TRIM_TEMPCO` reader - Trims temperature compensation of FFRO."]
pub type TrimTempcoR = crate::FieldReader;
#[doc = "Field `TRIM_TEMPCO` writer - Trims temperature compensation of FFRO."]
pub type TrimTempcoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM_COARSE` reader - Trims coarse frequency of FFRO."]
pub type TrimCoarseR = crate::FieldReader;
#[doc = "Field `TRIM_COARSE` writer - Trims coarse frequency of FFRO."]
pub type TrimCoarseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRIM_FINE` reader - Trims fine frequency of FFRO."]
pub type TrimFineR = crate::FieldReader;
#[doc = "Field `TRIM_FINE` writer - Trims fine frequency of FFRO."]
pub type TrimFineW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Trims frequency range of FFRO.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrimRange {
    #[doc = "0: 48MHz."]
    Ffro48mhz = 0,
    #[doc = "3: 60MHz."]
    Ffro60mhz = 3,
}
impl From<TrimRange> for u8 {
    #[inline(always)]
    fn from(variant: TrimRange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TrimRange {
    type Ux = u8;
}
impl crate::IsEnum for TrimRange {}
#[doc = "Field `TRIM_RANGE` reader - Trims frequency range of FFRO."]
pub type TrimRangeR = crate::FieldReader<TrimRange>;
impl TrimRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TrimRange> {
        match self.bits {
            0 => Some(TrimRange::Ffro48mhz),
            3 => Some(TrimRange::Ffro60mhz),
            _ => None,
        }
    }
    #[doc = "48MHz."]
    #[inline(always)]
    pub fn is_ffro_48mhz(&self) -> bool {
        *self == TrimRange::Ffro48mhz
    }
    #[doc = "60MHz."]
    #[inline(always)]
    pub fn is_ffro_60mhz(&self) -> bool {
        *self == TrimRange::Ffro60mhz
    }
}
#[doc = "Field `TRIM_RANGE` writer - Trims frequency range of FFRO."]
pub type TrimRangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, TrimRange>;
impl<'a, REG> TrimRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48MHz."]
    #[inline(always)]
    pub fn ffro_48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(TrimRange::Ffro48mhz)
    }
    #[doc = "60MHz."]
    #[inline(always)]
    pub fn ffro_60mhz(self) -> &'a mut crate::W<REG> {
        self.variant(TrimRange::Ffro60mhz)
    }
}
impl R {
    #[doc = "Bits 0:4 - Trims temperature compensation of FFRO."]
    #[inline(always)]
    pub fn trim_tempco(&self) -> TrimTempcoR {
        TrimTempcoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:10 - Trims coarse frequency of FFRO."]
    #[inline(always)]
    pub fn trim_coarse(&self) -> TrimCoarseR {
        TrimCoarseR::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:17 - Trims fine frequency of FFRO."]
    #[inline(always)]
    pub fn trim_fine(&self) -> TrimFineR {
        TrimFineR::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:19 - Trims frequency range of FFRO."]
    #[inline(always)]
    pub fn trim_range(&self) -> TrimRangeR {
        TrimRangeR::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FFROCTL0")
            .field("trim_tempco", &self.trim_tempco())
            .field("trim_coarse", &self.trim_coarse())
            .field("trim_fine", &self.trim_fine())
            .field("trim_range", &self.trim_range())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Trims temperature compensation of FFRO."]
    #[inline(always)]
    #[must_use]
    pub fn trim_tempco(&mut self) -> TrimTempcoW<Ffroctl0Spec> {
        TrimTempcoW::new(self, 0)
    }
    #[doc = "Bits 5:10 - Trims coarse frequency of FFRO."]
    #[inline(always)]
    #[must_use]
    pub fn trim_coarse(&mut self) -> TrimCoarseW<Ffroctl0Spec> {
        TrimCoarseW::new(self, 5)
    }
    #[doc = "Bits 11:17 - Trims fine frequency of FFRO."]
    #[inline(always)]
    #[must_use]
    pub fn trim_fine(&mut self) -> TrimFineW<Ffroctl0Spec> {
        TrimFineW::new(self, 11)
    }
    #[doc = "Bits 18:19 - Trims frequency range of FFRO."]
    #[inline(always)]
    #[must_use]
    pub fn trim_range(&mut self) -> TrimRangeW<Ffroctl0Spec> {
        TrimRangeW::new(self, 18)
    }
}
#[doc = "FFRO control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ffroctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffroctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ffroctl0Spec;
impl crate::RegisterSpec for Ffroctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffroctl0::R`](R) reader structure"]
impl crate::Readable for Ffroctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ffroctl0::W`](W) writer structure"]
impl crate::Writable for Ffroctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFROCTL0 to value 0x0002_0410"]
impl crate::Resettable for Ffroctl0Spec {
    const RESET_VALUE: u32 = 0x0002_0410;
}
