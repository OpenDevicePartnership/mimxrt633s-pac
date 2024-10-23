#[doc = "Register `INTPTDS` reader"]
pub type R = crate::R<IntptdsSpec>;
#[doc = "Register `INTPTDS` writer"]
pub type W = crate::W<IntptdsSpec>;
#[doc = "Field `INT_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type IntSkipR = crate::FieldReader<u32>;
#[doc = "Field `INT_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type IntSkipW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&self) -> IntSkipR {
        IntSkipR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTDS")
            .field("int_skip", &self.int_skip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    #[must_use]
    pub fn int_skip(&mut self) -> IntSkipW<IntptdsSpec> {
        IntSkipW::new(self, 0)
    }
}
#[doc = "Skip map for each INT PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`intptds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntptdsSpec;
impl crate::RegisterSpec for IntptdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intptds::R`](R) reader structure"]
impl crate::Readable for IntptdsSpec {}
#[doc = "`write(|w| ..)` method takes [`intptds::W`](W) writer structure"]
impl crate::Writable for IntptdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPTDS to value 0"]
impl crate::Resettable for IntptdsSpec {
    const RESET_VALUE: u32 = 0;
}
