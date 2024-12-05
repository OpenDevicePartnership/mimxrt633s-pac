#[doc = "Register `CTX_KEY%s` reader"]
pub type R = crate::R<CtxKeySpec>;
#[doc = "Register `CTX_KEY%s` writer"]
pub type W = crate::W<CtxKeySpec>;
#[doc = "Field `KEY` reader - AES Key"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - AES Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTX_KEY").field("key", &self.key()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CtxKeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "AES Key Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKeySpec;
impl crate::RegisterSpec for CtxKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key::R`](R) reader structure"]
impl crate::Readable for CtxKeySpec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key::W`](W) writer structure"]
impl crate::Writable for CtxKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTX_KEY%s to value 0"]
impl crate::Resettable for CtxKeySpec {
    const RESET_VALUE: u32 = 0;
}
