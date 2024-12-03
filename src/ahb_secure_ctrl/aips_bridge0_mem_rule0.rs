#[doc = "Register `AIPS_BRIDGE0_MEM_RULE0` reader"]
pub type R = crate::R<AipsBridge0MemRule0Spec>;
#[doc = "Register `AIPS_BRIDGE0_MEM_RULE0` writer"]
pub type W = crate::W<AipsBridge0MemRule0Spec>;
#[doc = "Field `MU0_M33_RULE` reader - 0x4011 0000--0x4011 0FFF"]
pub type Mu0M33RuleR = crate::FieldReader;
#[doc = "Field `MU0_M33_RULE` writer - 0x4011 0000--0x4011 0FFF"]
pub type Mu0M33RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU0_DSP_RULE` reader - 0x4011 1000--0x4011 1FFF"]
pub type Mu0DspRuleR = crate::FieldReader;
#[doc = "Field `MU0_DSP_RULE` writer - 0x4011 1000--0x4011 1FFF"]
pub type Mu0DspRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEMAPHORE_RULE` reader - 0x4011 2000--0x4011 2FFF"]
pub type SemaphoreRuleR = crate::FieldReader;
#[doc = "Field `SEMAPHORE_RULE` writer - 0x4011 2000--0x4011 2FFF"]
pub type SemaphoreRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OS_EVENT_TIMER_M33_RULE` reader - 0x4011 3000--0x4011 3FFF"]
pub type OsEventTimerM33RuleR = crate::FieldReader;
#[doc = "Field `OS_EVENT_TIMER_M33_RULE` writer - 0x4011 3000--0x4011 3FFF"]
pub type OsEventTimerM33RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OS_EVENT_TIMER_DSP_RULE` reader - 0x4011 4000--0x4011 4FFF"]
pub type OsEventTimerDspRuleR = crate::FieldReader;
#[doc = "Field `OS_EVENT_TIMER_DSP_RULE` writer - 0x4011 4000--0x4011 4FFF"]
pub type OsEventTimerDspRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4011 0000--0x4011 0FFF"]
    #[inline(always)]
    pub fn mu0_m33_rule(&self) -> Mu0M33RuleR {
        Mu0M33RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4011 1000--0x4011 1FFF"]
    #[inline(always)]
    pub fn mu0_dsp_rule(&self) -> Mu0DspRuleR {
        Mu0DspRuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4011 2000--0x4011 2FFF"]
    #[inline(always)]
    pub fn semaphore_rule(&self) -> SemaphoreRuleR {
        SemaphoreRuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x4011 3000--0x4011 3FFF"]
    #[inline(always)]
    pub fn os_event_timer_m33_rule(&self) -> OsEventTimerM33RuleR {
        OsEventTimerM33RuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x4011 4000--0x4011 4FFF"]
    #[inline(always)]
    pub fn os_event_timer_dsp_rule(&self) -> OsEventTimerDspRuleR {
        OsEventTimerDspRuleR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIPS_BRIDGE0_MEM_RULE0")
            .field("mu0_m33_rule", &self.mu0_m33_rule())
            .field("mu0_dsp_rule", &self.mu0_dsp_rule())
            .field("semaphore_rule", &self.semaphore_rule())
            .field("os_event_timer_m33_rule", &self.os_event_timer_m33_rule())
            .field("os_event_timer_dsp_rule", &self.os_event_timer_dsp_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4011 0000--0x4011 0FFF"]
    #[inline(always)]
    pub fn mu0_m33_rule(&mut self) -> Mu0M33RuleW<AipsBridge0MemRule0Spec> {
        Mu0M33RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4011 1000--0x4011 1FFF"]
    #[inline(always)]
    pub fn mu0_dsp_rule(&mut self) -> Mu0DspRuleW<AipsBridge0MemRule0Spec> {
        Mu0DspRuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4011 2000--0x4011 2FFF"]
    #[inline(always)]
    pub fn semaphore_rule(&mut self) -> SemaphoreRuleW<AipsBridge0MemRule0Spec> {
        SemaphoreRuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x4011 3000--0x4011 3FFF"]
    #[inline(always)]
    pub fn os_event_timer_m33_rule(&mut self) -> OsEventTimerM33RuleW<AipsBridge0MemRule0Spec> {
        OsEventTimerM33RuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x4011 4000--0x4011 4FFF"]
    #[inline(always)]
    pub fn os_event_timer_dsp_rule(&mut self) -> OsEventTimerDspRuleW<AipsBridge0MemRule0Spec> {
        OsEventTimerDspRuleW::new(self, 16)
    }
}
#[doc = "0x40110000--0x4011FFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge0_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge0_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AipsBridge0MemRule0Spec;
impl crate::RegisterSpec for AipsBridge0MemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aips_bridge0_mem_rule0::R`](R) reader structure"]
impl crate::Readable for AipsBridge0MemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`aips_bridge0_mem_rule0::W`](W) writer structure"]
impl crate::Writable for AipsBridge0MemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE0_MEM_RULE0 to value 0"]
impl crate::Resettable for AipsBridge0MemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
