#[doc = "Register `AIPS_BRIDGE1_MEM_RULE1` reader"]
pub type R = crate::R<AipsBridge1MemRule1Spec>;
#[doc = "Register `AIPS_BRIDGE1_MEM_RULE1` writer"]
pub type W = crate::W<AipsBridge1MemRule1Spec>;
#[doc = "Field `RNG_RULE` reader - 0x4013 8000--0x4013 8FFF"]
pub type RngRuleR = crate::FieldReader;
#[doc = "Field `RNG_RULE` writer - 0x4013 8000--0x4013 8FFF"]
pub type RngRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACMP0_RULE` reader - 0x4013 9000--0x4013 9FFF"]
pub type Acmp0RuleR = crate::FieldReader;
#[doc = "Field `ACMP0_RULE` writer - 0x4013 9000--0x4013 9FFF"]
pub type Acmp0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC0_RULE` reader - 0x4013 A000--0x4013 AFFF"]
pub type Adc0RuleR = crate::FieldReader;
#[doc = "Field `ADC0_RULE` writer - 0x4013 A000--0x4013 AFFF"]
pub type Adc0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_HS_PHY_RULE` reader - 0x4013 B000--0x4013 BFFF"]
pub type UsbHsPhyRuleR = crate::FieldReader;
#[doc = "Field `USB_HS_PHY_RULE` writer - 0x4013 B000--0x4013 BFFF"]
pub type UsbHsPhyRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4013 8000--0x4013 8FFF"]
    #[inline(always)]
    pub fn rng_rule(&self) -> RngRuleR {
        RngRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4013 9000--0x4013 9FFF"]
    #[inline(always)]
    pub fn acmp0_rule(&self) -> Acmp0RuleR {
        Acmp0RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4013 A000--0x4013 AFFF"]
    #[inline(always)]
    pub fn adc0_rule(&self) -> Adc0RuleR {
        Adc0RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x4013 B000--0x4013 BFFF"]
    #[inline(always)]
    pub fn usb_hs_phy_rule(&self) -> UsbHsPhyRuleR {
        UsbHsPhyRuleR::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIPS_BRIDGE1_MEM_RULE1")
            .field("rng_rule", &self.rng_rule())
            .field("acmp0_rule", &self.acmp0_rule())
            .field("adc0_rule", &self.adc0_rule())
            .field("usb_hs_phy_rule", &self.usb_hs_phy_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4013 8000--0x4013 8FFF"]
    #[inline(always)]
    pub fn rng_rule(&mut self) -> RngRuleW<AipsBridge1MemRule1Spec> {
        RngRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4013 9000--0x4013 9FFF"]
    #[inline(always)]
    pub fn acmp0_rule(&mut self) -> Acmp0RuleW<AipsBridge1MemRule1Spec> {
        Acmp0RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4013 A000--0x4013 AFFF"]
    #[inline(always)]
    pub fn adc0_rule(&mut self) -> Adc0RuleW<AipsBridge1MemRule1Spec> {
        Adc0RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x4013 B000--0x4013 BFFF"]
    #[inline(always)]
    pub fn usb_hs_phy_rule(&mut self) -> UsbHsPhyRuleW<AipsBridge1MemRule1Spec> {
        UsbHsPhyRuleW::new(self, 12)
    }
}
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge1_mem_rule1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge1_mem_rule1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AipsBridge1MemRule1Spec;
impl crate::RegisterSpec for AipsBridge1MemRule1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aips_bridge1_mem_rule1::R`](R) reader structure"]
impl crate::Readable for AipsBridge1MemRule1Spec {}
#[doc = "`write(|w| ..)` method takes [`aips_bridge1_mem_rule1::W`](W) writer structure"]
impl crate::Writable for AipsBridge1MemRule1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE1_MEM_RULE1 to value 0"]
impl crate::Resettable for AipsBridge1MemRule1Spec {
    const RESET_VALUE: u32 = 0;
}
