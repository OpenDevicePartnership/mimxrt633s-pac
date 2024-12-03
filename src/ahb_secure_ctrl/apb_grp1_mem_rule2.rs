#[doc = "Register `APB_GRP1_MEM_RULE2` reader"]
pub type R = crate::R<ApbGrp1MemRule2Spec>;
#[doc = "Register `APB_GRP1_MEM_RULE2` writer"]
pub type W = crate::W<ApbGrp1MemRule2Spec>;
#[doc = "Field `RTC_RULE` reader - 0x4003 0000--0x4003 0FFF"]
pub type RtcRuleR = crate::FieldReader;
#[doc = "Field `RTC_RULE` writer - 0x4003 0000--0x4003 0FFF"]
pub type RtcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I3C0_RULE` reader - 0x4003 6000--0x4003 6FFF"]
pub type I3c0RuleR = crate::FieldReader;
#[doc = "Field `I3C0_RULE` writer - 0x4003 6000--0x4003 6FFF"]
pub type I3c0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4003 0000--0x4003 0FFF"]
    #[inline(always)]
    pub fn rtc_rule(&self) -> RtcRuleR {
        RtcRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x4003 6000--0x4003 6FFF"]
    #[inline(always)]
    pub fn i3c0_rule(&self) -> I3c0RuleR {
        I3c0RuleR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_GRP1_MEM_RULE2")
            .field("rtc_rule", &self.rtc_rule())
            .field("i3c0_rule", &self.i3c0_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4003 0000--0x4003 0FFF"]
    #[inline(always)]
    pub fn rtc_rule(&mut self) -> RtcRuleW<ApbGrp1MemRule2Spec> {
        RtcRuleW::new(self, 0)
    }
    #[doc = "Bits 24:25 - 0x4003 6000--0x4003 6FFF"]
    #[inline(always)]
    pub fn i3c0_rule(&mut self) -> I3c0RuleW<ApbGrp1MemRule2Spec> {
        I3c0RuleW::new(self, 24)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbGrp1MemRule2Spec;
impl crate::RegisterSpec for ApbGrp1MemRule2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_grp1_mem_rule2::R`](R) reader structure"]
impl crate::Readable for ApbGrp1MemRule2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_grp1_mem_rule2::W`](W) writer structure"]
impl crate::Writable for ApbGrp1MemRule2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_GRP1_MEM_RULE2 to value 0"]
impl crate::Resettable for ApbGrp1MemRule2Spec {
    const RESET_VALUE: u32 = 0;
}
