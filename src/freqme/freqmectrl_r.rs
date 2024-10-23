#[doc = "Register `FREQMECTRL_R` reader"]
pub type R = crate::R<FreqmectrlRSpec>;
#[doc = "Field `RESULT` reader - Result"]
pub type ResultR = crate::FieldReader<u32>;
#[doc = "Measure in Progress\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasureInProgress {
    #[doc = "0: Process complete. Measurement cycle is complete. The results are ready in the RESULT field."]
    CycleDone = 0,
    #[doc = "1: In Progress. Measurement cycle is in progress."]
    InProgress = 1,
}
impl From<MeasureInProgress> for bool {
    #[inline(always)]
    fn from(variant: MeasureInProgress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEASURE_IN_PROGRESS` reader - Measure in Progress"]
pub type MeasureInProgressR = crate::BitReader<MeasureInProgress>;
impl MeasureInProgressR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MeasureInProgress {
        match self.bits {
            false => MeasureInProgress::CycleDone,
            true => MeasureInProgress::InProgress,
        }
    }
    #[doc = "Process complete. Measurement cycle is complete. The results are ready in the RESULT field."]
    #[inline(always)]
    pub fn is_cycle_done(&self) -> bool {
        *self == MeasureInProgress::CycleDone
    }
    #[doc = "In Progress. Measurement cycle is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MeasureInProgress::InProgress
    }
}
impl R {
    #[doc = "Bits 0:30 - Result"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Measure in Progress"]
    #[inline(always)]
    pub fn measure_in_progress(&self) -> MeasureInProgressR {
        MeasureInProgressR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQMECTRL_R")
            .field("result", &self.result())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[doc = "Frequency Measurement (in Read mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`freqmectrl_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqmectrlRSpec;
impl crate::RegisterSpec for FreqmectrlRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqmectrl_r::R`](R) reader structure"]
impl crate::Readable for FreqmectrlRSpec {}
#[doc = "`reset()` method sets FREQMECTRL_R to value 0"]
impl crate::Resettable for FreqmectrlRSpec {
    const RESET_VALUE: u32 = 0;
}
