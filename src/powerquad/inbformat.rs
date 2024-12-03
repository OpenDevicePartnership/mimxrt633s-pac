#[doc = "Register `INBFORMAT` reader"]
pub type R = crate::R<InbformatSpec>;
#[doc = "Register `INBFORMAT` writer"]
pub type W = crate::W<InbformatSpec>;
#[doc = "Field `inb_formatint` reader - Input B Internal format (00: q15; 01:q31; 10:float)"]
pub type InbFormatintR = crate::FieldReader;
#[doc = "Field `inb_formatint` writer - Input B Internal format (00: q15; 01:q31; 10:float)"]
pub type InbFormatintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `inb_formatext` reader - Input B External format (00: q15; 01:q31; 10:float)"]
pub type InbFormatextR = crate::FieldReader;
#[doc = "Field `inb_formatext` writer - Input B External format (00: q15; 01:q31; 10:float)"]
pub type InbFormatextW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `inb_scaler` reader - Input B Scaler value (for scaled 'q31' formats)"]
pub type InbScalerR = crate::FieldReader;
#[doc = "Field `inb_scaler` writer - Input B Scaler value (for scaled 'q31' formats)"]
pub type InbScalerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatint(&self) -> InbFormatintR {
        InbFormatintR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatext(&self) -> InbFormatextR {
        InbFormatextR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn inb_scaler(&self) -> InbScalerR {
        InbScalerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INBFORMAT")
            .field("inb_formatint", &self.inb_formatint())
            .field("inb_formatext", &self.inb_formatext())
            .field("inb_scaler", &self.inb_scaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatint(&mut self) -> InbFormatintW<InbformatSpec> {
        InbFormatintW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn inb_formatext(&mut self) -> InbFormatextW<InbformatSpec> {
        InbFormatextW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn inb_scaler(&mut self) -> InbScalerW<InbformatSpec> {
        InbScalerW::new(self, 8)
    }
}
#[doc = "Input B format\n\nYou can [`read`](crate::Reg::read) this register and get [`inbformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inbformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InbformatSpec;
impl crate::RegisterSpec for InbformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inbformat::R`](R) reader structure"]
impl crate::Readable for InbformatSpec {}
#[doc = "`write(|w| ..)` method takes [`inbformat::W`](W) writer structure"]
impl crate::Writable for InbformatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INBFORMAT to value 0"]
impl crate::Resettable for InbformatSpec {
    const RESET_VALUE: u32 = 0;
}
