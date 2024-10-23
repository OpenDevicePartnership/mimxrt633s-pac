#[doc = "Register `EVTIMERL` reader"]
pub type R = crate::R<EvtimerlSpec>;
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - A read reflects the current value of the lower 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
pub type EvtimerCountValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - A read reflects the current value of the lower 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EvtimerCountValueR {
        EvtimerCountValueR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTIMERL")
            .field("evtimer_count_value", &self.evtimer_count_value())
            .finish()
    }
}
#[doc = "EVTIMER Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evtimerl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtimerlSpec;
impl crate::RegisterSpec for EvtimerlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtimerl::R`](R) reader structure"]
impl crate::Readable for EvtimerlSpec {}
#[doc = "`reset()` method sets EVTIMERL to value 0"]
impl crate::Resettable for EvtimerlSpec {
    const RESET_VALUE: u32 = 0;
}
