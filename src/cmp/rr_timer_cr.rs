#[doc = "Register `RR_TIMER_CR` reader"]
pub type R = crate::R<RrTimerCrSpec>;
#[doc = "Register `RR_TIMER_CR` writer"]
pub type W = crate::W<RrTimerCrSpec>;
#[doc = "Field `RR_TIMER_RELOAD` reader - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
pub type RrTimerReloadR = crate::FieldReader<u32>;
#[doc = "Field `RR_TIMER_RELOAD` writer - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
pub type RrTimerReloadW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `RR_TIMER_ENA` reader - RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
pub type RrTimerEnaR = crate::BitReader;
#[doc = "Field `RR_TIMER_ENA` writer - RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
pub type RrTimerEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[inline(always)]
    pub fn rr_timer_reload(&self) -> RrTimerReloadR {
        RrTimerReloadR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
    #[inline(always)]
    pub fn rr_timer_ena(&self) -> RrTimerEnaR {
        RrTimerEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RR_TIMER_CR")
            .field("rr_timer_reload", &self.rr_timer_reload())
            .field("rr_timer_ena", &self.rr_timer_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[inline(always)]
    pub fn rr_timer_reload(&mut self) -> RrTimerReloadW<RrTimerCrSpec> {
        RrTimerReloadW::new(self, 0)
    }
    #[doc = "Bit 31 - RR_TIMER enable. When low, rr_timer count will be held at zero. When set, timer will commence continuous, repetitive counting beginning with the 1st or 2nd rising edge of the 32 KHz rr_clock.1"]
    #[inline(always)]
    pub fn rr_timer_ena(&mut self) -> RrTimerEnaW<RrTimerCrSpec> {
        RrTimerEnaW::new(self, 31)
    }
}
#[doc = "Round-Robin Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rr_timer_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr_timer_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrTimerCrSpec;
impl crate::RegisterSpec for RrTimerCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr_timer_cr::R`](R) reader structure"]
impl crate::Readable for RrTimerCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rr_timer_cr::W`](W) writer structure"]
impl crate::Writable for RrTimerCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RR_TIMER_CR to value 0"]
impl crate::Resettable for RrTimerCrSpec {
    const RESET_VALUE: u32 = 0;
}
