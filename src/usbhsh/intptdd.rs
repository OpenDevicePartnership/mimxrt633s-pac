#[doc = "Register `INTPTDD` reader"]
pub type R = crate::R<IntptddSpec>;
#[doc = "Register `INTPTDD` writer"]
pub type W = crate::W<IntptddSpec>;
#[doc = "Field `INT_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IntDoneR = crate::FieldReader<u32>;
#[doc = "Field `INT_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub type IntDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn int_done(&self) -> IntDoneR {
        IntDoneR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTDD")
            .field("int_done", &self.int_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn int_done(&mut self) -> IntDoneW<IntptddSpec> {
        IntDoneW::new(self, 0)
    }
}
#[doc = "Done map for each INT PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`intptdd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptdd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntptddSpec;
impl crate::RegisterSpec for IntptddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intptdd::R`](R) reader structure"]
impl crate::Readable for IntptddSpec {}
#[doc = "`write(|w| ..)` method takes [`intptdd::W`](W) writer structure"]
impl crate::Writable for IntptddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPTDD to value 0"]
impl crate::Resettable for IntptddSpec {
    const RESET_VALUE: u32 = 0;
}
