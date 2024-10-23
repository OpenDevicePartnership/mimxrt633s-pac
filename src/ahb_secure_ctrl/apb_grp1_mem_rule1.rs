#[doc = "Register `APB_GRP1_MEM_RULE1` reader"]
pub type R = crate::R<ApbGrp1MemRule1Spec>;
#[doc = "Register `APB_GRP1_MEM_RULE1` writer"]
pub type W = crate::W<ApbGrp1MemRule1Spec>;
#[doc = "Field `CT32BIT0_RULE` reader - 0x4002 8000--0x4002 8FFF"]
pub type Ct32bit0RuleR = crate::FieldReader;
#[doc = "Field `CT32BIT0_RULE` writer - 0x4002 8000--0x4002 8FFF"]
pub type Ct32bit0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT32BIT1_RULE` reader - 0x4002 9000--0x4002 9FFF"]
pub type Ct32bit1RuleR = crate::FieldReader;
#[doc = "Field `CT32BIT1_RULE` writer - 0x4002 9000--0x4002 9FFF"]
pub type Ct32bit1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT32BIT2_RULE` reader - 0x4002 A000--0x4002 AFFF"]
pub type Ct32bit2RuleR = crate::FieldReader;
#[doc = "Field `CT32BIT2_RULE` writer - 0x4002 A000--0x4002 AFFF"]
pub type Ct32bit2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT32BIT3_RULE` reader - 0x4002 B000--0x4002 BFFF"]
pub type Ct32bit3RuleR = crate::FieldReader;
#[doc = "Field `CT32BIT3_RULE` writer - 0x4002 B000--0x4002 BFFF"]
pub type Ct32bit3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CT32BIT4_RULE` reader - 0x4002 C000--0x4002 CFFF"]
pub type Ct32bit4RuleR = crate::FieldReader;
#[doc = "Field `CT32BIT4_RULE` writer - 0x4002 C000--0x4002 CFFF"]
pub type Ct32bit4RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MRT_RULE` reader - 0x4002 D000--0x4002 DFFF"]
pub type MrtRuleR = crate::FieldReader;
#[doc = "Field `MRT_RULE` writer - 0x4002 D000--0x4002 DFFF"]
pub type MrtRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WWDT1_RULE` reader - 0x4002 E000--0x4002 EFFF"]
pub type Wwdt1RuleR = crate::FieldReader;
#[doc = "Field `WWDT1_RULE` writer - 0x4002 E000--0x4002 EFFF"]
pub type Wwdt1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQME_RULE` reader - 0x4002 F000--0x4002 FFFF"]
pub type FreqmeRuleR = crate::FieldReader;
#[doc = "Field `FREQME_RULE` writer - 0x4002 F000--0x4002 FFFF"]
pub type FreqmeRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4002 8000--0x4002 8FFF"]
    #[inline(always)]
    pub fn ct32bit0_rule(&self) -> Ct32bit0RuleR {
        Ct32bit0RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4002 9000--0x4002 9FFF"]
    #[inline(always)]
    pub fn ct32bit1_rule(&self) -> Ct32bit1RuleR {
        Ct32bit1RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4002 A000--0x4002 AFFF"]
    #[inline(always)]
    pub fn ct32bit2_rule(&self) -> Ct32bit2RuleR {
        Ct32bit2RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x4002 B000--0x4002 BFFF"]
    #[inline(always)]
    pub fn ct32bit3_rule(&self) -> Ct32bit3RuleR {
        Ct32bit3RuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x4002 C000--0x4002 CFFF"]
    #[inline(always)]
    pub fn ct32bit4_rule(&self) -> Ct32bit4RuleR {
        Ct32bit4RuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 0x4002 D000--0x4002 DFFF"]
    #[inline(always)]
    pub fn mrt_rule(&self) -> MrtRuleR {
        MrtRuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x4002 E000--0x4002 EFFF"]
    #[inline(always)]
    pub fn wwdt1_rule(&self) -> Wwdt1RuleR {
        Wwdt1RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 0x4002 F000--0x4002 FFFF"]
    #[inline(always)]
    pub fn freqme_rule(&self) -> FreqmeRuleR {
        FreqmeRuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_GRP1_MEM_RULE1")
            .field("ct32bit0_rule", &self.ct32bit0_rule())
            .field("ct32bit1_rule", &self.ct32bit1_rule())
            .field("ct32bit2_rule", &self.ct32bit2_rule())
            .field("ct32bit3_rule", &self.ct32bit3_rule())
            .field("ct32bit4_rule", &self.ct32bit4_rule())
            .field("mrt_rule", &self.mrt_rule())
            .field("wwdt1_rule", &self.wwdt1_rule())
            .field("freqme_rule", &self.freqme_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4002 8000--0x4002 8FFF"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0_rule(&mut self) -> Ct32bit0RuleW<ApbGrp1MemRule1Spec> {
        Ct32bit0RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4002 9000--0x4002 9FFF"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1_rule(&mut self) -> Ct32bit1RuleW<ApbGrp1MemRule1Spec> {
        Ct32bit1RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4002 A000--0x4002 AFFF"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2_rule(&mut self) -> Ct32bit2RuleW<ApbGrp1MemRule1Spec> {
        Ct32bit2RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x4002 B000--0x4002 BFFF"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3_rule(&mut self) -> Ct32bit3RuleW<ApbGrp1MemRule1Spec> {
        Ct32bit3RuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x4002 C000--0x4002 CFFF"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4_rule(&mut self) -> Ct32bit4RuleW<ApbGrp1MemRule1Spec> {
        Ct32bit4RuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 0x4002 D000--0x4002 DFFF"]
    #[inline(always)]
    #[must_use]
    pub fn mrt_rule(&mut self) -> MrtRuleW<ApbGrp1MemRule1Spec> {
        MrtRuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - 0x4002 E000--0x4002 EFFF"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1_rule(&mut self) -> Wwdt1RuleW<ApbGrp1MemRule1Spec> {
        Wwdt1RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 0x4002 F000--0x4002 FFFF"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_rule(&mut self) -> FreqmeRuleW<ApbGrp1MemRule1Spec> {
        FreqmeRuleW::new(self, 28)
    }
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_grp1_mem_rule1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_grp1_mem_rule1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbGrp1MemRule1Spec;
impl crate::RegisterSpec for ApbGrp1MemRule1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_grp1_mem_rule1::R`](R) reader structure"]
impl crate::Readable for ApbGrp1MemRule1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_grp1_mem_rule1::W`](W) writer structure"]
impl crate::Writable for ApbGrp1MemRule1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_GRP1_MEM_RULE1 to value 0"]
impl crate::Resettable for ApbGrp1MemRule1Spec {
    const RESET_VALUE: u32 = 0;
}
