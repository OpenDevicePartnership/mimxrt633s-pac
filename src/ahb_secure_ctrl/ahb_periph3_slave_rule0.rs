#[doc = "Register `AHB_PERIPH3_SLAVE_RULE0` reader"]
pub type R = crate::R<AhbPeriph3SlaveRule0Spec>;
#[doc = "Register `AHB_PERIPH3_SLAVE_RULE0` writer"]
pub type W = crate::W<AhbPeriph3SlaveRule0Spec>;
#[doc = "Field `PQ_COPRO_RULE` reader - 0x40150000--0x40150FFF"]
pub type PqCoproRuleR = crate::FieldReader;
#[doc = "Field `PQ_COPRO_RULE` writer - 0x40150000--0x40150FFF"]
pub type PqCoproRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CASPER_COPRO_RULE` reader - 0x40151000--0x40151FFF"]
pub type CasperCoproRuleR = crate::FieldReader;
#[doc = "Field `CASPER_COPRO_RULE` writer - 0x40151000--0x40151FFF"]
pub type CasperCoproRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CASPER_RAM_RULE` reader - 0x40152000--0x40152FFF"]
pub type CasperRamRuleR = crate::FieldReader;
#[doc = "Field `CASPER_RAM_RULE` writer - 0x40152000--0x40152FFF"]
pub type CasperRamRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECURE_GPIO_RULE` reader - 0x40154000--0x40157FFF"]
pub type SecureGpioRuleR = crate::FieldReader;
#[doc = "Field `SECURE_GPIO_RULE` writer - 0x40154000--0x40157FFF"]
pub type SecureGpioRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HASH_RULE` reader - 0x40158000--0x40158FFF"]
pub type HashRuleR = crate::FieldReader;
#[doc = "Field `HASH_RULE` writer - 0x40158000--0x40158FFF"]
pub type HashRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x40150000--0x40150FFF"]
    #[inline(always)]
    pub fn pq_copro_rule(&self) -> PqCoproRuleR {
        PqCoproRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x40151000--0x40151FFF"]
    #[inline(always)]
    pub fn casper_copro_rule(&self) -> CasperCoproRuleR {
        CasperCoproRuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x40152000--0x40152FFF"]
    #[inline(always)]
    pub fn casper_ram_rule(&self) -> CasperRamRuleR {
        CasperRamRuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x40154000--0x40157FFF"]
    #[inline(always)]
    pub fn secure_gpio_rule(&self) -> SecureGpioRuleR {
        SecureGpioRuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x40158000--0x40158FFF"]
    #[inline(always)]
    pub fn hash_rule(&self) -> HashRuleR {
        HashRuleR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_PERIPH3_SLAVE_RULE0")
            .field("pq_copro_rule", &self.pq_copro_rule())
            .field("casper_copro_rule", &self.casper_copro_rule())
            .field("casper_ram_rule", &self.casper_ram_rule())
            .field("secure_gpio_rule", &self.secure_gpio_rule())
            .field("hash_rule", &self.hash_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x40150000--0x40150FFF"]
    #[inline(always)]
    pub fn pq_copro_rule(&mut self) -> PqCoproRuleW<AhbPeriph3SlaveRule0Spec> {
        PqCoproRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x40151000--0x40151FFF"]
    #[inline(always)]
    pub fn casper_copro_rule(&mut self) -> CasperCoproRuleW<AhbPeriph3SlaveRule0Spec> {
        CasperCoproRuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x40152000--0x40152FFF"]
    #[inline(always)]
    pub fn casper_ram_rule(&mut self) -> CasperRamRuleW<AhbPeriph3SlaveRule0Spec> {
        CasperRamRuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x40154000--0x40157FFF"]
    #[inline(always)]
    pub fn secure_gpio_rule(&mut self) -> SecureGpioRuleW<AhbPeriph3SlaveRule0Spec> {
        SecureGpioRuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x40158000--0x40158FFF"]
    #[inline(always)]
    pub fn hash_rule(&mut self) -> HashRuleW<AhbPeriph3SlaveRule0Spec> {
        HashRuleW::new(self, 16)
    }
}
#[doc = "Security access rules for AHB peripheral slaves area 0x40150000--0x40158FFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph3_slave_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph3_slave_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbPeriph3SlaveRule0Spec;
impl crate::RegisterSpec for AhbPeriph3SlaveRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_periph3_slave_rule0::R`](R) reader structure"]
impl crate::Readable for AhbPeriph3SlaveRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_periph3_slave_rule0::W`](W) writer structure"]
impl crate::Writable for AhbPeriph3SlaveRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH3_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AhbPeriph3SlaveRule0Spec {
    const RESET_VALUE: u32 = 0;
}
