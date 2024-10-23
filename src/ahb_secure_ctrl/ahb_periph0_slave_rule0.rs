#[doc = "Register `AHB_PERIPH0_SLAVE_RULE0` reader"]
pub type R = crate::R<AhbPeriph0SlaveRule0Spec>;
#[doc = "Register `AHB_PERIPH0_SLAVE_RULE0` writer"]
pub type W = crate::W<AhbPeriph0SlaveRule0Spec>;
#[doc = "Field `HSGPIO_RULE` reader - 0x40100000--0x40103FFF"]
pub type HsgpioRuleR = crate::FieldReader;
#[doc = "Field `HSGPIO_RULE` writer - 0x40100000--0x40103FFF"]
pub type HsgpioRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA0_RULE` reader - 0x40104000--0x40104FFF"]
pub type Dma0RuleR = crate::FieldReader;
#[doc = "Field `DMA0_RULE` writer - 0x40104000--0x40104FFF"]
pub type Dma0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA1_RULE` reader - 0x40105000--0x40105FFF"]
pub type Dma1RuleR = crate::FieldReader;
#[doc = "Field `DMA1_RULE` writer - 0x40105000--0x40105FFF"]
pub type Dma1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM0_RULE` reader - 0x40106000--0x40106FFF"]
pub type Flexcomm0RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM0_RULE` writer - 0x40106000--0x40106FFF"]
pub type Flexcomm0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM1_RULE` reader - 0x40107000--0x40107FFF"]
pub type Flexcomm1RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM1_RULE` writer - 0x40107000--0x40107FFF"]
pub type Flexcomm1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM2_RULE` reader - 0x40108000--0x40108FFF"]
pub type Flexcomm2RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM2_RULE` writer - 0x40108000--0x40108FFF"]
pub type Flexcomm2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXCOMM3_RULE` reader - 0x40109000--0x40109FFF"]
pub type Flexcomm3RuleR = crate::FieldReader;
#[doc = "Field `FLEXCOMM3_RULE` writer - 0x40109000--0x40109FFF"]
pub type Flexcomm3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEBUG_MAILBOX_RULE` reader - 0x4010F000--0x4010FFFF"]
pub type DebugMailboxRuleR = crate::FieldReader;
#[doc = "Field `DEBUG_MAILBOX_RULE` writer - 0x4010F000--0x4010FFFF"]
pub type DebugMailboxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x40100000--0x40103FFF"]
    #[inline(always)]
    pub fn hsgpio_rule(&self) -> HsgpioRuleR {
        HsgpioRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x40104000--0x40104FFF"]
    #[inline(always)]
    pub fn dma0_rule(&self) -> Dma0RuleR {
        Dma0RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x40105000--0x40105FFF"]
    #[inline(always)]
    pub fn dma1_rule(&self) -> Dma1RuleR {
        Dma1RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x40106000--0x40106FFF"]
    #[inline(always)]
    pub fn flexcomm0_rule(&self) -> Flexcomm0RuleR {
        Flexcomm0RuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x40107000--0x40107FFF"]
    #[inline(always)]
    pub fn flexcomm1_rule(&self) -> Flexcomm1RuleR {
        Flexcomm1RuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 0x40108000--0x40108FFF"]
    #[inline(always)]
    pub fn flexcomm2_rule(&self) -> Flexcomm2RuleR {
        Flexcomm2RuleR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x40109000--0x40109FFF"]
    #[inline(always)]
    pub fn flexcomm3_rule(&self) -> Flexcomm3RuleR {
        Flexcomm3RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 0x4010F000--0x4010FFFF"]
    #[inline(always)]
    pub fn debug_mailbox_rule(&self) -> DebugMailboxRuleR {
        DebugMailboxRuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_PERIPH0_SLAVE_RULE0")
            .field("hsgpio_rule", &self.hsgpio_rule())
            .field("dma0_rule", &self.dma0_rule())
            .field("dma1_rule", &self.dma1_rule())
            .field("flexcomm0_rule", &self.flexcomm0_rule())
            .field("flexcomm1_rule", &self.flexcomm1_rule())
            .field("flexcomm2_rule", &self.flexcomm2_rule())
            .field("flexcomm3_rule", &self.flexcomm3_rule())
            .field("debug_mailbox_rule", &self.debug_mailbox_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x40100000--0x40103FFF"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio_rule(&mut self) -> HsgpioRuleW<AhbPeriph0SlaveRule0Spec> {
        HsgpioRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x40104000--0x40104FFF"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_rule(&mut self) -> Dma0RuleW<AhbPeriph0SlaveRule0Spec> {
        Dma0RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x40105000--0x40105FFF"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_rule(&mut self) -> Dma1RuleW<AhbPeriph0SlaveRule0Spec> {
        Dma1RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x40106000--0x40106FFF"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rule(&mut self) -> Flexcomm0RuleW<AhbPeriph0SlaveRule0Spec> {
        Flexcomm0RuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x40107000--0x40107FFF"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rule(&mut self) -> Flexcomm1RuleW<AhbPeriph0SlaveRule0Spec> {
        Flexcomm1RuleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 0x40108000--0x40108FFF"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rule(&mut self) -> Flexcomm2RuleW<AhbPeriph0SlaveRule0Spec> {
        Flexcomm2RuleW::new(self, 20)
    }
    #[doc = "Bits 24:25 - 0x40109000--0x40109FFF"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rule(&mut self) -> Flexcomm3RuleW<AhbPeriph0SlaveRule0Spec> {
        Flexcomm3RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 0x4010F000--0x4010FFFF"]
    #[inline(always)]
    #[must_use]
    pub fn debug_mailbox_rule(&mut self) -> DebugMailboxRuleW<AhbPeriph0SlaveRule0Spec> {
        DebugMailboxRuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AHB peripheral slaves area 0x40100000--0x4010FFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph0_slave_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph0_slave_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbPeriph0SlaveRule0Spec;
impl crate::RegisterSpec for AhbPeriph0SlaveRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_periph0_slave_rule0::R`](R) reader structure"]
impl crate::Readable for AhbPeriph0SlaveRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_periph0_slave_rule0::W`](W) writer structure"]
impl crate::Writable for AhbPeriph0SlaveRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH0_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AhbPeriph0SlaveRule0Spec {
    const RESET_VALUE: u32 = 0;
}
