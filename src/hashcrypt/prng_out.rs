#[doc = "Register `PRNG_OUT` reader"]
pub type R = crate::R<PrngOutSpec>;
#[doc = "Field `PRNG_OUT` reader - Random output value from the PRNG. The PRNG output is disabled and this register is set to 0x00000000 when the AES is enabled."]
pub type PrngOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random output value from the PRNG. The PRNG output is disabled and this register is set to 0x00000000 when the AES is enabled."]
    #[inline(always)]
    pub fn prng_out(&self) -> PrngOutR {
        PrngOutR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRNG_OUT")
            .field("prng_out", &self.prng_out())
            .finish()
    }
}
#[doc = "PRNG software-accessable random output value\n\nYou can [`read`](crate::Reg::read) this register and get [`prng_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrngOutSpec;
impl crate::RegisterSpec for PrngOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prng_out::R`](R) reader structure"]
impl crate::Readable for PrngOutSpec {}
#[doc = "`reset()` method sets PRNG_OUT to value 0"]
impl crate::Resettable for PrngOutSpec {
    const RESET_VALUE: u32 = 0;
}
