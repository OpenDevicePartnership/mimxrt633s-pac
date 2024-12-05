#[doc = "Register `AHB_PERIPH1_SLAVE_RULE0` reader"]
pub type R = crate::R<AhbPeriph1SlaveRule0Spec>;
#[doc = "Register `AHB_PERIPH1_SLAVE_RULE0` writer"]
pub type W = crate::W<AhbPeriph1SlaveRule0Spec>;
#[doc = "Field `CRC_RULE` reader - Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
pub type CrcRuleR = crate::FieldReader;
#[doc = "Field `CRC_RULE` writer - Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
pub type CrcRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMIC_RULE` reader - 0x40121000--0x40121FFF"]
pub type DmicRuleR = crate::FieldReader;
#[doc = "Field `DMIC_RULE` writer - 0x40121000--0x40121FFF"]
pub type DmicRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM4_RULE` reader - 0x40122000--0x40122FFF"]
pub type Flexcomm4RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM4_RULE` writer - 0x40122000--0x40122FFF"]
pub type Flexcomm4RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM5_RULE` reader - 0x40123000--0x40123FFF"]
pub type Flexcomm5RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM5_RULE` writer - 0x40123000--0x40123FFF"]
pub type Flexcomm5RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM6_RULE` reader - 0x40124000--0x40124FFF"]
pub type Flexcomm6RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM6_RULE` writer - 0x40124000--0x40124FFF"]
pub type Flexcomm6RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM7_RULE` reader - 0x40125000--0x40125FFF"]
pub type Flexcomm7RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM7_RULE` writer - 0x40125000--0x40125FFF"]
pub type Flexcomm7RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM14_RULE` reader - 0x40126000--0x40126FFF"]
pub type Flexcomm14RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM14_RULE` writer - 0x40126000--0x40126FFF"]
pub type Flexcomm14RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM15_RULE` reader - 0x40127000--0x40127FFF"]
pub type Flexcomm15RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM15_RULE` writer - 0x40127000--0x40127FFF"]
pub type Flexcomm15RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
    #[inline(always)]
    pub fn crc_rule(&self) -> CrcRuleR {
        CrcRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x40121000--0x40121FFF"]
    #[inline(always)]
    pub fn dmic_rule(&self) -> DmicRuleR {
        DmicRuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x40122000--0x40122FFF"]
    #[inline(always)]
    pub fn flexcomm4_rule(&self) -> Flexcomm4RuleR {
        Flexcomm4RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x40123000--0x40123FFF"]
    #[inline(always)]
    pub fn flexcomm5_rule(&self) -> Flexcomm5RuleR {
        Flexcomm5RuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x40124000--0x40124FFF"]
    #[inline(always)]
    pub fn flexcomm6_rule(&self) -> Flexcomm6RuleR {
        Flexcomm6RuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 0x40125000--0x40125FFF"]
    #[inline(always)]
    pub fn flexcomm7_rule(&self) -> Flexcomm7RuleR {
        Flexcomm7RuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x40126000--0x40126FFF"]
    #[inline(always)]
    pub fn flexcomm14_rule(&self) -> Flexcomm14RuleR {
        Flexcomm14RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 0x40127000--0x40127FFF"]
    #[inline(always)]
    pub fn flexcomm15_rule(&self) -> Flexcomm15RuleR {
        Flexcomm15RuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_PERIPH1_SLAVE_RULE0")
            .field("crc_rule", &self.crc_rule())
            .field("dmic_rule", &self.dmic_rule())
            .field("flexcomm4_rule", &self.flexcomm4_rule())
            .field("flexcomm5_rule", &self.flexcomm5_rule())
            .field("flexcomm6_rule", &self.flexcomm6_rule())
            .field("flexcomm7_rule", &self.flexcomm7_rule())
            .field("flexcomm14_rule", &self.flexcomm14_rule())
            .field("flexcomm15_rule", &self.flexcomm15_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for AHB peripheral slaves area 0x40120000--0x40120FFF"]
    #[inline(always)]
    pub fn crc_rule(&mut self) -> CrcRuleW<AhbPeriph1SlaveRule0Spec> {
        CrcRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x40121000--0x40121FFF"]
    #[inline(always)]
    pub fn dmic_rule(&mut self) -> DmicRuleW<AhbPeriph1SlaveRule0Spec> {
        DmicRuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x40122000--0x40122FFF"]
    #[inline(always)]
    pub fn flexcomm4_rule(&mut self) -> Flexcomm4RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm4RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x40123000--0x40123FFF"]
    #[inline(always)]
    pub fn flexcomm5_rule(&mut self) -> Flexcomm5RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm5RuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x40124000--0x40124FFF"]
    #[inline(always)]
    pub fn flexcomm6_rule(&mut self) -> Flexcomm6RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm6RuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 0x40125000--0x40125FFF"]
    #[inline(always)]
    pub fn flexcomm7_rule(&mut self) -> Flexcomm7RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm7RuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - 0x40126000--0x40126FFF"]
    #[inline(always)]
    pub fn flexcomm14_rule(&mut self) -> Flexcomm14RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm14RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 0x40127000--0x40127FFF"]
    #[inline(always)]
    pub fn flexcomm15_rule(&mut self) -> Flexcomm15RuleW<AhbPeriph1SlaveRule0Spec> {
        Flexcomm15RuleW::new(self, 28)
    }
}
#[doc = "the memory map is 0x40120000--0x40127FFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph1_slave_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph1_slave_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbPeriph1SlaveRule0Spec;
impl crate::RegisterSpec for AhbPeriph1SlaveRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_periph1_slave_rule0::R`](R) reader structure"]
impl crate::Readable for AhbPeriph1SlaveRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_periph1_slave_rule0::W`](W) writer structure"]
impl crate::Writable for AhbPeriph1SlaveRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH1_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AhbPeriph1SlaveRule0Spec {
    const RESET_VALUE: u32 = 0;
}
