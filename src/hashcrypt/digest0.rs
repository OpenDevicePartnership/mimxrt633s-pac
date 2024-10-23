#[doc = "Register `DIGEST0[%s]` reader"]
pub type R = crate::R<Digest0Spec>;
#[doc = "Field `DIGEST` reader - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
pub type DigestR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub fn digest(&self) -> DigestR {
        DigestR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIGEST0")
            .field("digest", &self.digest())
            .finish()
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`digest0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Digest0Spec;
impl crate::RegisterSpec for Digest0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digest0::R`](R) reader structure"]
impl crate::Readable for Digest0Spec {}
#[doc = "`reset()` method sets DIGEST0[%s]
to value 0"]
impl crate::Resettable for Digest0Spec {
    const RESET_VALUE: u32 = 0;
}
