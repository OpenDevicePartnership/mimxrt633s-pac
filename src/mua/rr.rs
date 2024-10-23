#[doc = "Register `RR[%s]` reader"]
pub type R = crate::R<RrSpec>;
#[doc = "Register `RR[%s]` writer"]
pub type W = crate::W<RrSpec>;
#[doc = "Field `DATA` reader - DATA"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RR").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<RrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Receive Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrSpec;
impl crate::RegisterSpec for RrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr::R`](R) reader structure"]
impl crate::Readable for RrSpec {}
#[doc = "`write(|w| ..)` method takes [`rr::W`](W) writer structure"]
impl crate::Writable for RrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RrSpec {
    const RESET_VALUE: u32 = 0;
}
