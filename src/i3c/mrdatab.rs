#[doc = "Register `MRDATAB` reader"]
pub type R = crate::R<MrdatabSpec>;
#[doc = "Field `VALUE` reader - VALUE"]
pub type ValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRDATAB")
            .field("value", &self.value())
            .finish()
    }
}
#[doc = "Master Read Data Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatab::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrdatabSpec;
impl crate::RegisterSpec for MrdatabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdatab::R`](R) reader structure"]
impl crate::Readable for MrdatabSpec {}
#[doc = "`reset()` method sets MRDATAB to value 0"]
impl crate::Resettable for MrdatabSpec {
    const RESET_VALUE: u32 = 0;
}
