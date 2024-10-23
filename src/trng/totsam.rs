#[doc = "Register `TOTSAM` reader"]
pub type R = crate::R<TotsamSpec>;
#[doc = "Field `TOT_SAM` reader - Total Samples"]
pub type TotSamR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Total Samples"]
    #[inline(always)]
    pub fn tot_sam(&self) -> TotSamR {
        TotSamR::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOTSAM")
            .field("tot_sam", &self.tot_sam())
            .finish()
    }
}
#[doc = "Total Samples Register\n\nYou can [`read`](crate::Reg::read) this register and get [`totsam::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotsamSpec;
impl crate::RegisterSpec for TotsamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`totsam::R`](R) reader structure"]
impl crate::Readable for TotsamSpec {}
#[doc = "`reset()` method sets TOTSAM to value 0"]
impl crate::Resettable for TotsamSpec {
    const RESET_VALUE: u32 = 0;
}
