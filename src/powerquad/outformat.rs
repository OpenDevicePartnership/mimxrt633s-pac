#[doc = "Register `OUTFORMAT` reader"]
pub type R = crate::R<OutformatSpec>;
#[doc = "Register `OUTFORMAT` writer"]
pub type W = crate::W<OutformatSpec>;
#[doc = "Field `out_formatint` reader - Output Internal format (00: q15; 01:q31; 10:float)"]
pub type OutFormatintR = crate::FieldReader;
#[doc = "Field `out_formatint` writer - Output Internal format (00: q15; 01:q31; 10:float)"]
pub type OutFormatintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `out_formatext` reader - Output External format (00: q15; 01:q31; 10:float)"]
pub type OutFormatextR = crate::FieldReader;
#[doc = "Field `out_formatext` writer - Output External format (00: q15; 01:q31; 10:float)"]
pub type OutFormatextW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `out_scaler` reader - Output Scaler value (for scaled 'q31' formats)"]
pub type OutScalerR = crate::FieldReader;
#[doc = "Field `out_scaler` writer - Output Scaler value (for scaled 'q31' formats)"]
pub type OutScalerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatint(&self) -> OutFormatintR {
        OutFormatintR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn out_formatext(&self) -> OutFormatextR {
        OutFormatextR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn out_scaler(&self) -> OutScalerR {
        OutScalerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFORMAT")
            .field("out_formatint", &self.out_formatint())
            .field("out_formatext", &self.out_formatext())
            .field("out_scaler", &self.out_scaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    #[must_use]
    pub fn out_formatint(&mut self) -> OutFormatintW<OutformatSpec> {
        OutFormatintW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    #[must_use]
    pub fn out_formatext(&mut self) -> OutFormatextW<OutformatSpec> {
        OutFormatextW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    #[must_use]
    pub fn out_scaler(&mut self) -> OutScalerW<OutformatSpec> {
        OutScalerW::new(self, 8)
    }
}
#[doc = "Output format\n\nYou can [`read`](crate::Reg::read) this register and get [`outformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutformatSpec;
impl crate::RegisterSpec for OutformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outformat::R`](R) reader structure"]
impl crate::Readable for OutformatSpec {}
#[doc = "`write(|w| ..)` method takes [`outformat::W`](W) writer structure"]
impl crate::Writable for OutformatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTFORMAT to value 0"]
impl crate::Resettable for OutformatSpec {
    const RESET_VALUE: u32 = 0;
}
