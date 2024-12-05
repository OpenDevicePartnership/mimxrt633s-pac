#[doc = "Register `CS_PROTCPU1` reader"]
pub type R = crate::R<CsProtcpu1Spec>;
#[doc = "Register `CS_PROTCPU1` writer"]
pub type W = crate::W<CsProtcpu1Spec>;
#[doc = "Field `CS_PROTCPU1` reader - Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
pub type CsProtcpu1R = crate::FieldReader<u32>;
#[doc = "Field `CS_PROTCPU1` writer - Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
pub type CsProtcpu1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    pub fn cs_protcpu1(&self) -> CsProtcpu1R {
        CsProtcpu1R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS_PROTCPU1")
            .field("cs_protcpu1", &self.cs_protcpu1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    pub fn cs_protcpu1(&mut self) -> CsProtcpu1W<CsProtcpu1Spec> {
        CsProtcpu1W::new(self, 0)
    }
}
#[doc = "Code Security for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_protcpu1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs_protcpu1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsProtcpu1Spec;
impl crate::RegisterSpec for CsProtcpu1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs_protcpu1::R`](R) reader structure"]
impl crate::Readable for CsProtcpu1Spec {}
#[doc = "`write(|w| ..)` method takes [`cs_protcpu1::W`](W) writer structure"]
impl crate::Writable for CsProtcpu1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS_PROTCPU1 to value 0"]
impl crate::Resettable for CsProtcpu1Spec {
    const RESET_VALUE: u32 = 0;
}
