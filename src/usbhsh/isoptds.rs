#[doc = "Register `ISOPTDS` reader"]
pub type R = crate::R<IsoptdsSpec>;
#[doc = "Register `ISOPTDS` writer"]
pub type W = crate::W<IsoptdsSpec>;
#[doc = "Field `ISO_SKIP` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IsoSkipR = crate::FieldReader<u32>;
#[doc = "Field `ISO_SKIP` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IsoSkipW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_skip(&self) -> IsoSkipR {
        IsoSkipR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTDS")
            .field("iso_skip", &self.iso_skip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    #[must_use]
    pub fn iso_skip(&mut self) -> IsoSkipW<IsoptdsSpec> {
        IsoSkipW::new(self, 0)
    }
}
#[doc = "Skip map for each ISO PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoptdsSpec;
impl crate::RegisterSpec for IsoptdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoptds::R`](R) reader structure"]
impl crate::Readable for IsoptdsSpec {}
#[doc = "`write(|w| ..)` method takes [`isoptds::W`](W) writer structure"]
impl crate::Writable for IsoptdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISOPTDS to value 0"]
impl crate::Resettable for IsoptdsSpec {
    const RESET_VALUE: u32 = 0;
}
