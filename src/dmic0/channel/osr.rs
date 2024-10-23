#[doc = "Register `OSR` reader"]
pub type R = crate::R<OsrSpec>;
#[doc = "Register `OSR` writer"]
pub type W = crate::W<OsrSpec>;
#[doc = "Field `OSR` reader - Selects the oversample rate for the related input channel."]
pub type OsrR = crate::FieldReader;
#[doc = "Field `OSR` writer - Selects the oversample rate for the related input channel."]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Selects the oversample rate for the related input channel."]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSR").field("osr", &self.osr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the oversample rate for the related input channel."]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<OsrSpec> {
        OsrW::new(self, 0)
    }
}
#[doc = "CIC Filter decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsrSpec;
impl crate::RegisterSpec for OsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osr::R`](R) reader structure"]
impl crate::Readable for OsrSpec {}
#[doc = "`write(|w| ..)` method takes [`osr::W`](W) writer structure"]
impl crate::Writable for OsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSR to value 0"]
impl crate::Resettable for OsrSpec {
    const RESET_VALUE: u32 = 0;
}
