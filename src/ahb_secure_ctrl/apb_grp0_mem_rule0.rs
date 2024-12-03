#[doc = "Register `APB_GRP0_MEM_RULE0` reader"]
pub type R = crate::R<ApbGrp0MemRule0Spec>;
#[doc = "Register `APB_GRP0_MEM_RULE0` writer"]
pub type W = crate::W<ApbGrp0MemRule0Spec>;
#[doc = "Field `RSTCTL0_RULE` reader - 0x4000 0000--0x4000 0FFF"]
pub type Rstctl0RuleR = crate::FieldReader;
#[doc = "Field `RSTCTL0_RULE` writer - 0x4000 0000--0x4000 0FFF"]
pub type Rstctl0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKCTL0_RULE` reader - 0x4000 1000--0x4000 1FFF"]
pub type Clkctl0RuleR = crate::FieldReader;
#[doc = "Field `CLKCTL0_RULE` writer - 0x4000 1000--0x4000 1FFF"]
pub type Clkctl0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYSCTL0_RULE` reader - 0x4000 2000--0x4000 2FFF"]
pub type Sysctl0RuleR = crate::FieldReader;
#[doc = "Field `SYSCTL0_RULE` writer - 0x4000 2000--0x4000 2FFF"]
pub type Sysctl0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOPCTL_RULE` reader - 0x4000 4000--0x4000 4FFF"]
pub type IopctlRuleR = crate::FieldReader;
#[doc = "Field `IOPCTL_RULE` writer - 0x4000 4000--0x4000 4FFF"]
pub type IopctlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUFCTRL_RULE` reader - 0x4000 6000--0x4000 6FFF"]
pub type PufctrlRuleR = crate::FieldReader;
#[doc = "Field `PUFCTRL_RULE` writer - 0x4000 6000--0x4000 6FFF"]
pub type PufctrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4000 0000--0x4000 0FFF"]
    #[inline(always)]
    pub fn rstctl0_rule(&self) -> Rstctl0RuleR {
        Rstctl0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4000 1000--0x4000 1FFF"]
    #[inline(always)]
    pub fn clkctl0_rule(&self) -> Clkctl0RuleR {
        Clkctl0RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4000 2000--0x4000 2FFF"]
    #[inline(always)]
    pub fn sysctl0_rule(&self) -> Sysctl0RuleR {
        Sysctl0RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x4000 4000--0x4000 4FFF"]
    #[inline(always)]
    pub fn iopctl_rule(&self) -> IopctlRuleR {
        IopctlRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x4000 6000--0x4000 6FFF"]
    #[inline(always)]
    pub fn pufctrl_rule(&self) -> PufctrlRuleR {
        PufctrlRuleR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_GRP0_MEM_RULE0")
            .field("rstctl0_rule", &self.rstctl0_rule())
            .field("clkctl0_rule", &self.clkctl0_rule())
            .field("sysctl0_rule", &self.sysctl0_rule())
            .field("iopctl_rule", &self.iopctl_rule())
            .field("pufctrl_rule", &self.pufctrl_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4000 0000--0x4000 0FFF"]
    #[inline(always)]
    pub fn rstctl0_rule(&mut self) -> Rstctl0RuleW<ApbGrp0MemRule0Spec> {
        Rstctl0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4000 1000--0x4000 1FFF"]
    #[inline(always)]
    pub fn clkctl0_rule(&mut self) -> Clkctl0RuleW<ApbGrp0MemRule0Spec> {
        Clkctl0RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4000 2000--0x4000 2FFF"]
    #[inline(always)]
    pub fn sysctl0_rule(&mut self) -> Sysctl0RuleW<ApbGrp0MemRule0Spec> {
        Sysctl0RuleW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 0x4000 4000--0x4000 4FFF"]
    #[inline(always)]
    pub fn iopctl_rule(&mut self) -> IopctlRuleW<ApbGrp0MemRule0Spec> {
        IopctlRuleW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 0x4000 6000--0x4000 6FFF"]
    #[inline(always)]
    pub fn pufctrl_rule(&mut self) -> PufctrlRuleW<ApbGrp0MemRule0Spec> {
        PufctrlRuleW::new(self, 24)
    }
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp0_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp0_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbGrp0MemRule0Spec;
impl crate::RegisterSpec for ApbGrp0MemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_grp0_mem_rule0::R`](R) reader structure"]
impl crate::Readable for ApbGrp0MemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_grp0_mem_rule0::W`](W) writer structure"]
impl crate::Writable for ApbGrp0MemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_GRP0_MEM_RULE0 to value 0"]
impl crate::Resettable for ApbGrp0MemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
