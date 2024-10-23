#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "Field `inst_misc` reader - Misc register. For Matrix : Used for scale factor"]
pub type InstMiscR = crate::FieldReader<u32>;
#[doc = "Field `inst_misc` writer - Misc register. For Matrix : Used for scale factor"]
pub type InstMiscW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Misc register. For Matrix : Used for scale factor"]
    #[inline(always)]
    pub fn inst_misc(&self) -> InstMiscR {
        InstMiscR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("inst_misc", &self.inst_misc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Misc register. For Matrix : Used for scale factor"]
    #[inline(always)]
    #[must_use]
    pub fn inst_misc(&mut self) -> InstMiscW<MiscSpec> {
        InstMiscW::new(self, 0)
    }
}
#[doc = "Misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MiscSpec {
    const RESET_VALUE: u32 = 0;
}
