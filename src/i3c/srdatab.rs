#[doc = "Register `SRDATAB` reader"]
pub type R = crate::R<SrdatabSpec>;
#[doc = "Field `DATA0` reader - Byte read from the master"]
pub type Data0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte read from the master"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRDATAB")
            .field("data0", &self.data0())
            .finish()
    }
}
#[doc = "Slave Read Data Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srdatab::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrdatabSpec;
impl crate::RegisterSpec for SrdatabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdatab::R`](R) reader structure"]
impl crate::Readable for SrdatabSpec {}
#[doc = "`reset()` method sets SRDATAB to value 0"]
impl crate::Resettable for SrdatabSpec {
    const RESET_VALUE: u32 = 0;
}
