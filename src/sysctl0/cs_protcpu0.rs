#[doc = "Register `CS_PROTCPU0` reader"]
pub type R = crate::R<CsProtcpu0Spec>;
#[doc = "Register `CS_PROTCPU0` writer"]
pub type W = crate::W<CsProtcpu0Spec>;
#[doc = "Field `CS_PROTCPU0` reader - Controls M33 AP Enable. Magic Value: 0x1234 5678"]
pub type CsProtcpu0R = crate::FieldReader<u32>;
#[doc = "Field `CS_PROTCPU0` writer - Controls M33 AP Enable. Magic Value: 0x1234 5678"]
pub type CsProtcpu0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls M33 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    pub fn cs_protcpu0(&self) -> CsProtcpu0R {
        CsProtcpu0R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS_PROTCPU0")
            .field("cs_protcpu0", &self.cs_protcpu0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls M33 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    #[must_use]
    pub fn cs_protcpu0(&mut self) -> CsProtcpu0W<CsProtcpu0Spec> {
        CsProtcpu0W::new(self, 0)
    }
}
#[doc = "Code Security for CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_protcpu0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs_protcpu0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsProtcpu0Spec;
impl crate::RegisterSpec for CsProtcpu0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs_protcpu0::R`](R) reader structure"]
impl crate::Readable for CsProtcpu0Spec {}
#[doc = "`write(|w| ..)` method takes [`cs_protcpu0::W`](W) writer structure"]
impl crate::Writable for CsProtcpu0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS_PROTCPU0 to value 0"]
impl crate::Resettable for CsProtcpu0Spec {
    const RESET_VALUE: u32 = 0;
}
