#[doc = "Register `INAFORMAT` reader"]
pub type R = crate::R<InaformatSpec>;
#[doc = "Register `INAFORMAT` writer"]
pub type W = crate::W<InaformatSpec>;
#[doc = "Field `ina_formatint` reader - Input A Internal format (00: q15; 01:q31; 10:float)"]
pub type InaFormatintR = crate::FieldReader;
#[doc = "Field `ina_formatint` writer - Input A Internal format (00: q15; 01:q31; 10:float)"]
pub type InaFormatintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ina_formatext` reader - Input A External format (00: q15; 01:q31; 10:float)"]
pub type InaFormatextR = crate::FieldReader;
#[doc = "Field `ina_formatext` writer - Input A External format (00: q15; 01:q31; 10:float)"]
pub type InaFormatextW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ina_scaler` reader - Input A Scaler value (for scaled 'q31' formats)"]
pub type InaScalerR = crate::FieldReader;
#[doc = "Field `ina_scaler` writer - Input A Scaler value (for scaled 'q31' formats)"]
pub type InaScalerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatint(&self) -> InaFormatintR {
        InaFormatintR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn ina_formatext(&self) -> InaFormatextR {
        InaFormatextR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn ina_scaler(&self) -> InaScalerR {
        InaScalerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INAFORMAT")
            .field("ina_formatint", &self.ina_formatint())
            .field("ina_formatext", &self.ina_formatext())
            .field("ina_scaler", &self.ina_scaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    #[must_use]
    pub fn ina_formatint(&mut self) -> InaFormatintW<InaformatSpec> {
        InaFormatintW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    #[must_use]
    pub fn ina_formatext(&mut self) -> InaFormatextW<InaformatSpec> {
        InaFormatextW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    #[must_use]
    pub fn ina_scaler(&mut self) -> InaScalerW<InaformatSpec> {
        InaScalerW::new(self, 8)
    }
}
#[doc = "Input A format\n\nYou can [`read`](crate::Reg::read) this register and get [`inaformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inaformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InaformatSpec;
impl crate::RegisterSpec for InaformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inaformat::R`](R) reader structure"]
impl crate::Readable for InaformatSpec {}
#[doc = "`write(|w| ..)` method takes [`inaformat::W`](W) writer structure"]
impl crate::Writable for InaformatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INAFORMAT to value 0"]
impl crate::Resettable for InaformatSpec {
    const RESET_VALUE: u32 = 0;
}
