#[doc = "Register `ISOPTDD` reader"]
pub type R = crate::R<IsoptddSpec>;
#[doc = "Register `ISOPTDD` writer"]
pub type W = crate::W<IsoptddSpec>;
#[doc = "Field `ISO_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IsoDoneR = crate::FieldReader<u32>;
#[doc = "Field `ISO_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IsoDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_done(&self) -> IsoDoneR {
        IsoDoneR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTDD")
            .field("iso_done", &self.iso_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_done(&mut self) -> IsoDoneW<IsoptddSpec> {
        IsoDoneW::new(self, 0)
    }
}
#[doc = "Done map for each ISO PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptdd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptdd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoptddSpec;
impl crate::RegisterSpec for IsoptddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoptdd::R`](R) reader structure"]
impl crate::Readable for IsoptddSpec {}
#[doc = "`write(|w| ..)` method takes [`isoptdd::W`](W) writer structure"]
impl crate::Writable for IsoptddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISOPTDD to value 0"]
impl crate::Resettable for IsoptddSpec {
    const RESET_VALUE: u32 = 0;
}
