#[doc = "Register `MATCH` reader"]
pub type R = crate::R<MatchSpec>;
#[doc = "Register `MATCH` writer"]
pub type W = crate::W<MatchSpec>;
#[doc = "Field `MATVAL` reader - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
pub type MatvalR = crate::FieldReader<u32>;
#[doc = "Field `MATVAL` writer - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
pub type MatvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub fn matval(&self) -> MatvalR {
        MatvalR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH")
            .field("matval", &self.matval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub fn matval(&mut self) -> MatvalW<MatchSpec> {
        MatvalW::new(self, 0)
    }
}
#[doc = "RTC match register\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchSpec;
impl crate::RegisterSpec for MatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_::R`](R) reader structure"]
impl crate::Readable for MatchSpec {}
#[doc = "`write(|w| ..)` method takes [`match_::W`](W) writer structure"]
impl crate::Writable for MatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCH to value 0xffff_ffff"]
impl crate::Resettable for MatchSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
