#[doc = "Register `INTREN` reader"]
pub type R = crate::R<IntrenSpec>;
#[doc = "Register `INTREN` writer"]
pub type W = crate::W<IntrenSpec>;
#[doc = "Field `intr_oflow` reader - 1 : Enable interrupt on Floating point overflow"]
pub type IntrOflowR = crate::BitReader;
#[doc = "Field `intr_oflow` writer - 1 : Enable interrupt on Floating point overflow"]
pub type IntrOflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_nan` reader - 1 : Enable interrupt on Floating point NaN"]
pub type IntrNanR = crate::BitReader;
#[doc = "Field `intr_nan` writer - 1 : Enable interrupt on Floating point NaN"]
pub type IntrNanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_fixed` reader - 1: Enable interrupt on Fixed point Overflow"]
pub type IntrFixedR = crate::BitReader;
#[doc = "Field `intr_fixed` writer - 1: Enable interrupt on Fixed point Overflow"]
pub type IntrFixedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_uflow` reader - 1 : Enable interrupt on Subnormal truncation"]
pub type IntrUflowR = crate::BitReader;
#[doc = "Field `intr_uflow` writer - 1 : Enable interrupt on Subnormal truncation"]
pub type IntrUflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_berr` reader - 1: Enable interrupt on AHBM Buss Error"]
pub type IntrBerrR = crate::BitReader;
#[doc = "Field `intr_berr` writer - 1: Enable interrupt on AHBM Buss Error"]
pub type IntrBerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intr_comp` reader - 1: Enable interrupt on instruction completion"]
pub type IntrCompR = crate::BitReader;
#[doc = "Field `intr_comp` writer - 1: Enable interrupt on instruction completion"]
pub type IntrCompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&self) -> IntrOflowR {
        IntrOflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&self) -> IntrNanR {
        IntrNanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn intr_fixed(&self) -> IntrFixedR {
        IntrFixedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&self) -> IntrUflowR {
        IntrUflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn intr_berr(&self) -> IntrBerrR {
        IntrBerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&self) -> IntrCompR {
        IntrCompR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTREN")
            .field("intr_oflow", &self.intr_oflow())
            .field("intr_nan", &self.intr_nan())
            .field("intr_fixed", &self.intr_fixed())
            .field("intr_uflow", &self.intr_uflow())
            .field("intr_berr", &self.intr_berr())
            .field("intr_comp", &self.intr_comp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn intr_oflow(&mut self) -> IntrOflowW<IntrenSpec> {
        IntrOflowW::new(self, 0)
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    #[must_use]
    pub fn intr_nan(&mut self) -> IntrNanW<IntrenSpec> {
        IntrNanW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn intr_fixed(&mut self) -> IntrFixedW<IntrenSpec> {
        IntrFixedW::new(self, 2)
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    #[must_use]
    pub fn intr_uflow(&mut self) -> IntrUflowW<IntrenSpec> {
        IntrUflowW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_berr(&mut self) -> IntrBerrW<IntrenSpec> {
        IntrBerrW::new(self, 4)
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn intr_comp(&mut self) -> IntrCompW<IntrenSpec> {
        IntrCompW::new(self, 7)
    }
}
#[doc = "INTERRUPT enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrenSpec;
impl crate::RegisterSpec for IntrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intren::R`](R) reader structure"]
impl crate::Readable for IntrenSpec {}
#[doc = "`write(|w| ..)` method takes [`intren::W`](W) writer structure"]
impl crate::Writable for IntrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTREN to value 0"]
impl crate::Resettable for IntrenSpec {
    const RESET_VALUE: u32 = 0;
}
