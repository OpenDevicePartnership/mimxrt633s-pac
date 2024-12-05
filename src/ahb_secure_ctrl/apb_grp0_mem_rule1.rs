#[doc = "Register `APB_GRP0_MEM_RULE1` reader"]
pub type R = crate::R<ApbGrp0MemRule1Spec>;
#[doc = "Register `APB_GRP0_MEM_RULE1` writer"]
pub type W = crate::W<ApbGrp0MemRule1Spec>;
#[doc = "Field `WWDT0_RULE` reader - 0x4000 E000--0x4000 EFFF"]
pub type Wwdt0RuleR = crate::FieldReader;
#[doc = "Field `WWDT0_RULE` writer - 0x4000 E000--0x4000 EFFF"]
pub type Wwdt0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTICK_RULE` reader - 0x4000 F000--0x4000 FFFF"]
pub type UtickRuleR = crate::FieldReader;
#[doc = "Field `UTICK_RULE` writer - 0x4000 F000--0x4000 FFFF"]
pub type UtickRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 24:25 - 0x4000 E000--0x4000 EFFF"]
    #[inline(always)]
    pub fn wwdt0_rule(&self) -> Wwdt0RuleR {
        Wwdt0RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 0x4000 F000--0x4000 FFFF"]
    #[inline(always)]
    pub fn utick_rule(&self) -> UtickRuleR {
        UtickRuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_GRP0_MEM_RULE1")
            .field("wwdt0_rule", &self.wwdt0_rule())
            .field("utick_rule", &self.utick_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:25 - 0x4000 E000--0x4000 EFFF"]
    #[inline(always)]
    pub fn wwdt0_rule(&mut self) -> Wwdt0RuleW<ApbGrp0MemRule1Spec> {
        Wwdt0RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 0x4000 F000--0x4000 FFFF"]
    #[inline(always)]
    pub fn utick_rule(&mut self) -> UtickRuleW<ApbGrp0MemRule1Spec> {
        UtickRuleW::new(self, 28)
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp0_mem_rule1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp0_mem_rule1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbGrp0MemRule1Spec;
impl crate::RegisterSpec for ApbGrp0MemRule1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_grp0_mem_rule1::R`](R) reader structure"]
impl crate::Readable for ApbGrp0MemRule1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_grp0_mem_rule1::W`](W) writer structure"]
impl crate::Writable for ApbGrp0MemRule1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_GRP0_MEM_RULE1 to value 0"]
impl crate::Resettable for ApbGrp0MemRule1Spec {
    const RESET_VALUE: u32 = 0;
}
