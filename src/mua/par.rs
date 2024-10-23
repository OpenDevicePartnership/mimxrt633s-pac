#[doc = "Register `PAR` reader"]
pub type R = crate::R<ParSpec>;
#[doc = "Field `PARAMETER` reader - no description available"]
pub type ParameterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn parameter(&self) -> ParameterR {
        ParameterR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAR")
            .field("parameter", &self.parameter())
            .finish()
    }
}
#[doc = "Use Parameter register to determine the parameter settings of MUA.\n\nYou can [`read`](crate::Reg::read) this register and get [`par::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParSpec;
impl crate::RegisterSpec for ParSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`par::R`](R) reader structure"]
impl crate::Readable for ParSpec {}
#[doc = "`reset()` method sets PAR to value 0"]
impl crate::Resettable for ParSpec {
    const RESET_VALUE: u32 = 0;
}
