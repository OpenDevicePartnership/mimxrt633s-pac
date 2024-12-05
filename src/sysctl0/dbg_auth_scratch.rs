#[doc = "Register `DBG_AUTH_SCRATCH` reader"]
pub type R = crate::R<DbgAuthScratchSpec>;
#[doc = "Register `DBG_AUTH_SCRATCH` writer"]
pub type W = crate::W<DbgAuthScratchSpec>;
#[doc = "Field `DBG_AUTH_SCRATCH` reader - Debug authorization scratch register for S/W."]
pub type DbgAuthScratchR = crate::FieldReader<u32>;
#[doc = "Field `DBG_AUTH_SCRATCH` writer - Debug authorization scratch register for S/W."]
pub type DbgAuthScratchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug authorization scratch register for S/W."]
    #[inline(always)]
    pub fn dbg_auth_scratch(&self) -> DbgAuthScratchR {
        DbgAuthScratchR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_SCRATCH")
            .field("dbg_auth_scratch", &self.dbg_auth_scratch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug authorization scratch register for S/W."]
    #[inline(always)]
    pub fn dbg_auth_scratch(&mut self) -> DbgAuthScratchW<DbgAuthScratchSpec> {
        DbgAuthScratchW::new(self, 0)
    }
}
#[doc = "Debug authorization scratch\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_scratch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_scratch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgAuthScratchSpec;
impl crate::RegisterSpec for DbgAuthScratchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_auth_scratch::R`](R) reader structure"]
impl crate::Readable for DbgAuthScratchSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_auth_scratch::W`](W) writer structure"]
impl crate::Writable for DbgAuthScratchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_AUTH_SCRATCH to value 0"]
impl crate::Resettable for DbgAuthScratchSpec {
    const RESET_VALUE: u32 = 0;
}
