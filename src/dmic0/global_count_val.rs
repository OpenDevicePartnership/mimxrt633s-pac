#[doc = "Register `GLOBAL_COUNT_VAL` reader"]
pub type R = crate::R<GlobalCountValSpec>;
#[doc = "Register `GLOBAL_COUNT_VAL` writer"]
pub type W = crate::W<GlobalCountValSpec>;
#[doc = "Field `CCOUNTVAL` reader - 32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
pub type CcountvalR = crate::FieldReader<u32>;
#[doc = "Field `CCOUNTVAL` writer - 32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
pub type CcountvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
    #[inline(always)]
    pub fn ccountval(&self) -> CcountvalR {
        CcountvalR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLOBAL_COUNT_VAL")
            .field("ccountval", &self.ccountval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
    #[inline(always)]
    #[must_use]
    pub fn ccountval(&mut self) -> CcountvalW<GlobalCountValSpec> {
        CcountvalW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`global_count_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`global_count_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobalCountValSpec;
impl crate::RegisterSpec for GlobalCountValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`global_count_val::R`](R) reader structure"]
impl crate::Readable for GlobalCountValSpec {}
#[doc = "`write(|w| ..)` method takes [`global_count_val::W`](W) writer structure"]
impl crate::Writable for GlobalCountValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBAL_COUNT_VAL to value 0"]
impl crate::Resettable for GlobalCountValSpec {
    const RESET_VALUE: u32 = 0;
}
