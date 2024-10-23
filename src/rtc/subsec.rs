#[doc = "Register `SUBSEC` reader"]
pub type R = crate::R<SubsecSpec>;
#[doc = "Register `SUBSEC` writer"]
pub type W = crate::W<SubsecSpec>;
#[doc = "Field `RTC_SUBSEC` reader - A read reflects the current value of the 32Khz sub-second counter. This counter will be cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32 KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep_powerdown mode or after the main RTC module has been disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
pub type RtcSubsecR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - A read reflects the current value of the 32Khz sub-second counter. This counter will be cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32 KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep_powerdown mode or after the main RTC module has been disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub fn rtc_subsec(&self) -> RtcSubsecR {
        RtcSubsecR::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBSEC")
            .field("rtc_subsec", &self.rtc_subsec())
            .finish()
    }
}
impl W {}
#[doc = "RTC Sub-second Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`subsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsecSpec;
impl crate::RegisterSpec for SubsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsec::R`](R) reader structure"]
impl crate::Readable for SubsecSpec {}
#[doc = "`write(|w| ..)` method takes [`subsec::W`](W) writer structure"]
impl crate::Writable for SubsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SubsecSpec {
    const RESET_VALUE: u32 = 0;
}
