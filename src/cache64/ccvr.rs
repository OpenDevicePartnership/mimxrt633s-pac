#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CcvrSpec>;
#[doc = "Register `CCVR` writer"]
pub type W = crate::W<CcvrSpec>;
#[doc = "Field `DATA` reader - Cache read/write Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Cache read/write Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache read/write Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVR").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache read/write Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<CcvrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cache read/write value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcvrSpec;
impl crate::RegisterSpec for CcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CcvrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccvr::W`](W) writer structure"]
impl crate::Writable for CcvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CcvrSpec {
    const RESET_VALUE: u32 = 0;
}
