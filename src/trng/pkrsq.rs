#[doc = "Register `PKRSQ` reader"]
pub type R = crate::R<PkrsqSpec>;
#[doc = "Field `PKR_SQ` reader - Poker Square Calculation Result."]
pub type PkrSqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Square Calculation Result."]
    #[inline(always)]
    pub fn pkr_sq(&self) -> PkrSqR {
        PkrSqR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKRSQ")
            .field("pkr_sq", &self.pkr_sq())
            .finish()
    }
}
#[doc = "Poker Square Calculation Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrsq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkrsqSpec;
impl crate::RegisterSpec for PkrsqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkrsq::R`](R) reader structure"]
impl crate::Readable for PkrsqSpec {}
#[doc = "`reset()` method sets PKRSQ to value 0"]
impl crate::Resettable for PkrsqSpec {
    const RESET_VALUE: u32 = 0;
}
