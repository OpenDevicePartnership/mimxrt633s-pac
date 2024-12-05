#[doc = "Register `CTX_CTR%s` reader"]
pub type R = crate::R<CtxCtrSpec>;
#[doc = "Register `CTX_CTR%s` writer"]
pub type W = crate::W<CtxCtrSpec>;
#[doc = "Field `CTR` reader - AES Counter"]
pub type CtrR = crate::FieldReader<u32>;
#[doc = "Field `CTR` writer - AES Counter"]
pub type CtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTX_CTR").field("ctr", &self.ctr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES Counter"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<CtxCtrSpec> {
        CtrW::new(self, 0)
    }
}
#[doc = "AES Counter Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxCtrSpec;
impl crate::RegisterSpec for CtxCtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_ctr::R`](R) reader structure"]
impl crate::Readable for CtxCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctx_ctr::W`](W) writer structure"]
impl crate::Writable for CtxCtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTX_CTR%s to value 0"]
impl crate::Resettable for CtxCtrSpec {
    const RESET_VALUE: u32 = 0;
}
