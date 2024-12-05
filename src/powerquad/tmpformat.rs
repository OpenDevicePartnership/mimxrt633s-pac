#[doc = "Register `TMPFORMAT` reader"]
pub type R = crate::R<TmpformatSpec>;
#[doc = "Register `TMPFORMAT` writer"]
pub type W = crate::W<TmpformatSpec>;
#[doc = "Field `tmp_formatint` reader - Temp Internal format (00: q15; 01:q31; 10:float)"]
pub type TmpFormatintR = crate::FieldReader;
#[doc = "Field `tmp_formatint` writer - Temp Internal format (00: q15; 01:q31; 10:float)"]
pub type TmpFormatintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tmp_formatext` reader - Temp External format (00: q15; 01:q31; 10:float)"]
pub type TmpFormatextR = crate::FieldReader;
#[doc = "Field `tmp_formatext` writer - Temp External format (00: q15; 01:q31; 10:float)"]
pub type TmpFormatextW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tmp_scaler` reader - Temp Scaler value (for scaled 'q31' formats)"]
pub type TmpScalerR = crate::FieldReader;
#[doc = "Field `tmp_scaler` writer - Temp Scaler value (for scaled 'q31' formats)"]
pub type TmpScalerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatint(&self) -> TmpFormatintR {
        TmpFormatintR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatext(&self) -> TmpFormatextR {
        TmpFormatextR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn tmp_scaler(&self) -> TmpScalerR {
        TmpScalerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMPFORMAT")
            .field("tmp_formatint", &self.tmp_formatint())
            .field("tmp_formatext", &self.tmp_formatext())
            .field("tmp_scaler", &self.tmp_scaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatint(&mut self) -> TmpFormatintW<TmpformatSpec> {
        TmpFormatintW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn tmp_formatext(&mut self) -> TmpFormatextW<TmpformatSpec> {
        TmpFormatextW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn tmp_scaler(&mut self) -> TmpScalerW<TmpformatSpec> {
        TmpScalerW::new(self, 8)
    }
}
#[doc = "Temp format\n\nYou can [`read`](crate::Reg::read) this register and get [`tmpformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmpformatSpec;
impl crate::RegisterSpec for TmpformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmpformat::R`](R) reader structure"]
impl crate::Readable for TmpformatSpec {}
#[doc = "`write(|w| ..)` method takes [`tmpformat::W`](W) writer structure"]
impl crate::Writable for TmpformatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMPFORMAT to value 0"]
impl crate::Resettable for TmpformatSpec {
    const RESET_VALUE: u32 = 0;
}
