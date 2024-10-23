#[doc = "Register `CODEINPUT` writer"]
pub type W = crate::W<CodeinputSpec>;
#[doc = "Field `CODEIN` writer - AC/KC Input Data"]
pub type CodeinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CodeinputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - AC/KC Input Data"]
    #[inline(always)]
    #[must_use]
    pub fn codein(&mut self) -> CodeinW<CodeinputSpec> {
        CodeinW::new(self, 0)
    }
}
#[doc = "PUF Code Input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codeinput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeinputSpec;
impl crate::RegisterSpec for CodeinputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`codeinput::W`](W) writer structure"]
impl crate::Writable for CodeinputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CODEINPUT to value 0"]
impl crate::Resettable for CodeinputSpec {
    const RESET_VALUE: u32 = 0;
}
