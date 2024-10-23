#[doc = "Register `APB_GRP1_MEM_RULE0` reader"]
pub type R = crate::R<ApbGrp1MemRule0Spec>;
#[doc = "Register `APB_GRP1_MEM_RULE0` writer"]
pub type W = crate::W<ApbGrp1MemRule0Spec>;
#[doc = "Field `RSTCTL1_RULE` reader - 0x4002 0000--0x4002 0FFF"]
pub type Rstctl1RuleR = crate::FieldReader;
#[doc = "Field `RSTCTL1_RULE` writer - 0x4002 0000--0x4002 0FFF"]
pub type Rstctl1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKCTL1_RULE` reader - 0x4002 1000--0x4002 1FFF"]
pub type Clkctl1RuleR = crate::FieldReader;
#[doc = "Field `CLKCTL1_RULE` writer - 0x4002 1000--0x4002 1FFF"]
pub type Clkctl1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYSCTL1_RULE` reader - 0x4002 2000--0x4002 2FFF"]
pub type Sysctl1RuleR = crate::FieldReader;
#[doc = "Field `SYSCTL1_RULE` writer - 0x4002 2000--0x4002 2FFF"]
pub type Sysctl1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO_INTR_CTRL_RULE` reader - 0x4002 5000--0x4002 5FFF"]
pub type GpioIntrCtrlRuleR = crate::FieldReader;
#[doc = "Field `GPIO_INTR_CTRL_RULE` writer - 0x4002 5000--0x4002 5FFF"]
pub type GpioIntrCtrlRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERIPH_INPUT_MUX_RULE` reader - 0x4002 6000--0x4002 6FFF"]
pub type PeriphInputMuxRuleR = crate::FieldReader;
#[doc = "Field `PERIPH_INPUT_MUX_RULE` writer - 0x4002 6000--0x4002 6FFF"]
pub type PeriphInputMuxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4002 0000--0x4002 0FFF"]
    #[inline(always)]
    pub fn rstctl1_rule(&self) -> Rstctl1RuleR {
        Rstctl1RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4002 1000--0x4002 1FFF"]
    #[inline(always)]
    pub fn clkctl1_rule(&self) -> Clkctl1RuleR {
        Clkctl1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4002 2000--0x4002 2FFF"]
    #[inline(always)]
    pub fn sysctl1_rule(&self) -> Sysctl1RuleR {
        Sysctl1RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 0x4002 5000--0x4002 5FFF"]
    #[inline(always)]
    pub fn gpio_intr_ctrl_rule(&self) -> GpioIntrCtrlRuleR {
        GpioIntrCtrlRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x4002 6000--0x4002 6FFF"]
    #[inline(always)]
    pub fn periph_input_mux_rule(&self) -> PeriphInputMuxRuleR {
        PeriphInputMuxRuleR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_GRP1_MEM_RULE0")
            .field("rstctl1_rule", &self.rstctl1_rule())
            .field("clkctl1_rule", &self.clkctl1_rule())
            .field("sysctl1_rule", &self.sysctl1_rule())
            .field("gpio_intr_ctrl_rule", &self.gpio_intr_ctrl_rule())
            .field("periph_input_mux_rule", &self.periph_input_mux_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4002 0000--0x4002 0FFF"]
    #[inline(always)]
    #[must_use]
    pub fn rstctl1_rule(&mut self) -> Rstctl1RuleW<ApbGrp1MemRule0Spec> {
        Rstctl1RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4002 1000--0x4002 1FFF"]
    #[inline(always)]
    #[must_use]
    pub fn clkctl1_rule(&mut self) -> Clkctl1RuleW<ApbGrp1MemRule0Spec> {
        Clkctl1RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4002 2000--0x4002 2FFF"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl1_rule(&mut self) -> Sysctl1RuleW<ApbGrp1MemRule0Spec> {
        Sysctl1RuleW::new(self, 8)
    }
    #[doc = "Bits 20:21 - 0x4002 5000--0x4002 5FFF"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intr_ctrl_rule(&mut self) -> GpioIntrCtrlRuleW<ApbGrp1MemRule0Spec> {
        GpioIntrCtrlRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - 0x4002 6000--0x4002 6FFF"]
    #[inline(always)]
    #[must_use]
    pub fn periph_input_mux_rule(&mut self) -> PeriphInputMuxRuleW<ApbGrp1MemRule0Spec> {
        PeriphInputMuxRuleW::new(self, 24)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbGrp1MemRule0Spec;
impl crate::RegisterSpec for ApbGrp1MemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_grp1_mem_rule0::R`](R) reader structure"]
impl crate::Readable for ApbGrp1MemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_grp1_mem_rule0::W`](W) writer structure"]
impl crate::Writable for ApbGrp1MemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_GRP1_MEM_RULE0 to value 0"]
impl crate::Resettable for ApbGrp1MemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
