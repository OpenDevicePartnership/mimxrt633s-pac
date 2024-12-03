#[doc = "Register `compreg[%s]` reader"]
pub type R = crate::R<CompregSpec>;
#[doc = "Register `compreg[%s]` writer"]
pub type W = crate::W<CompregSpec>;
#[doc = "Field `compreg` reader - Compute register bank"]
pub type CompregR = crate::FieldReader<u32>;
#[doc = "Field `compreg` writer - Compute register bank"]
pub type CompregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compute register bank"]
    #[inline(always)]
    pub fn compreg(&self) -> CompregR {
        CompregR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("compreg")
            .field("compreg", &self.compreg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Compute register bank"]
    #[inline(always)]
    pub fn compreg(&mut self) -> CompregW<CompregSpec> {
        CompregW::new(self, 0)
    }
}
#[doc = "Compute register bank\n\nYou can [`read`](crate::Reg::read) this register and get [`compreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompregSpec;
impl crate::RegisterSpec for CompregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compreg::R`](R) reader structure"]
impl crate::Readable for CompregSpec {}
#[doc = "`write(|w| ..)` method takes [`compreg::W`](W) writer structure"]
impl crate::Writable for CompregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets compreg[%s]
to value 0"]
impl crate::Resettable for CompregSpec {
    const RESET_VALUE: u32 = 0;
}
