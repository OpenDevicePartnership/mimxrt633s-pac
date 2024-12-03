#[doc = "Register `OUTBASE` reader"]
pub type R = crate::R<OutbaseSpec>;
#[doc = "Register `OUTBASE` writer"]
pub type W = crate::W<OutbaseSpec>;
#[doc = "Field `outbase` reader - Base address register for the output region"]
pub type OutbaseR = crate::FieldReader<u32>;
#[doc = "Field `outbase` writer - Base address register for the output region"]
pub type OutbaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&self) -> OutbaseR {
        OutbaseR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTBASE")
            .field("outbase", &self.outbase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&mut self) -> OutbaseW<OutbaseSpec> {
        OutbaseW::new(self, 0)
    }
}
#[doc = "Base address register for output region\n\nYou can [`read`](crate::Reg::read) this register and get [`outbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutbaseSpec;
impl crate::RegisterSpec for OutbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outbase::R`](R) reader structure"]
impl crate::Readable for OutbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`outbase::W`](W) writer structure"]
impl crate::Writable for OutbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTBASE to value 0"]
impl crate::Resettable for OutbaseSpec {
    const RESET_VALUE: u32 = 0;
}
