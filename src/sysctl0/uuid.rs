#[doc = "Register `UUID%s` reader"]
pub type R = crate::R<UuidSpec>;
#[doc = "Field `uuid` reader - no description available"]
pub type UuidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn uuid(&self) -> UuidR {
        UuidR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID").field("uuid", &self.uuid()).finish()
    }
}
#[doc = "UUIDn 32-Bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UuidSpec;
impl crate::RegisterSpec for UuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uuid::R`](R) reader structure"]
impl crate::Readable for UuidSpec {}
#[doc = "`reset()` method sets UUID%s to value 0"]
impl crate::Resettable for UuidSpec {
    const RESET_VALUE: u32 = 0;
}
