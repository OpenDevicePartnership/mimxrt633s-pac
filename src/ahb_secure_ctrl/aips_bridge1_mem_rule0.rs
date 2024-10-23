#[doc = "Register `AIPS_BRIDGE1_MEM_RULE0` reader"]
pub type R = crate::R<AipsBridge1MemRule0Spec>;
#[doc = "Register `AIPS_BRIDGE1_MEM_RULE0` writer"]
pub type W = crate::W<AipsBridge1MemRule0Spec>;
#[doc = "Field `OTP_RULE0` reader - 0x4013 0000--0x4013 0FFF"]
pub type OtpRule0R = crate::FieldReader;
#[doc = "Field `OTP_RULE0` writer - 0x4013 0000--0x4013 0FFF"]
pub type OtpRule0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTP_RULE1` reader - 0x4013 1000--0x4013 1FFF"]
pub type OtpRule1R = crate::FieldReader;
#[doc = "Field `OTP_RULE1` writer - 0x4013 1000--0x4013 1FFF"]
pub type OtpRule1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTP_RULE2` reader - 0x4013 2000--0x4013 2FFF"]
pub type OtpRule2R = crate::FieldReader;
#[doc = "Field `OTP_RULE2` writer - 0x4013 2000--0x4013 2FFF"]
pub type OtpRule2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTP_RULE3` reader - 0x4013 3000--0x4013 3FFF"]
pub type OtpRule3R = crate::FieldReader;
#[doc = "Field `OTP_RULE3` writer - 0x4013 3000--0x4013 3FFF"]
pub type OtpRule3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLEXSPI_AND_OTFAD_RULE` reader - 0x4013 4000--0x4013 4FFF"]
pub type FlexspiAndOtfadRuleR = crate::FieldReader;
#[doc = "Field `FLEXSPI_AND_OTFAD_RULE` writer - 0x4013 4000--0x4013 4FFF"]
pub type FlexspiAndOtfadRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO0_RULE` reader - 0x4013 6000--0x4013 6FFF"]
pub type Sdio0RuleR = crate::FieldReader;
#[doc = "Field `SDIO0_RULE` writer - 0x4013 6000--0x4013 6FFF"]
pub type Sdio0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO1_RULE` reader - 0x4013 7000--0x4013 7FFF"]
pub type Sdio1RuleR = crate::FieldReader;
#[doc = "Field `SDIO1_RULE` writer - 0x4013 7000--0x4013 7FFF"]
pub type Sdio1RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x4013 0000--0x4013 0FFF"]
    #[inline(always)]
    pub fn otp_rule0(&self) -> OtpRule0R {
        OtpRule0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x4013 1000--0x4013 1FFF"]
    #[inline(always)]
    pub fn otp_rule1(&self) -> OtpRule1R {
        OtpRule1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x4013 2000--0x4013 2FFF"]
    #[inline(always)]
    pub fn otp_rule2(&self) -> OtpRule2R {
        OtpRule2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x4013 3000--0x4013 3FFF"]
    #[inline(always)]
    pub fn otp_rule3(&self) -> OtpRule3R {
        OtpRule3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 0x4013 4000--0x4013 4FFF"]
    #[inline(always)]
    pub fn flexspi_and_otfad_rule(&self) -> FlexspiAndOtfadRuleR {
        FlexspiAndOtfadRuleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0x4013 6000--0x4013 6FFF"]
    #[inline(always)]
    pub fn sdio0_rule(&self) -> Sdio0RuleR {
        Sdio0RuleR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 0x4013 7000--0x4013 7FFF"]
    #[inline(always)]
    pub fn sdio1_rule(&self) -> Sdio1RuleR {
        Sdio1RuleR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIPS_BRIDGE1_MEM_RULE0")
            .field("otp_rule0", &self.otp_rule0())
            .field("otp_rule1", &self.otp_rule1())
            .field("otp_rule2", &self.otp_rule2())
            .field("otp_rule3", &self.otp_rule3())
            .field("flexspi_and_otfad_rule", &self.flexspi_and_otfad_rule())
            .field("sdio0_rule", &self.sdio0_rule())
            .field("sdio1_rule", &self.sdio1_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x4013 0000--0x4013 0FFF"]
    #[inline(always)]
    #[must_use]
    pub fn otp_rule0(&mut self) -> OtpRule0W<AipsBridge1MemRule0Spec> {
        OtpRule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x4013 1000--0x4013 1FFF"]
    #[inline(always)]
    #[must_use]
    pub fn otp_rule1(&mut self) -> OtpRule1W<AipsBridge1MemRule0Spec> {
        OtpRule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x4013 2000--0x4013 2FFF"]
    #[inline(always)]
    #[must_use]
    pub fn otp_rule2(&mut self) -> OtpRule2W<AipsBridge1MemRule0Spec> {
        OtpRule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x4013 3000--0x4013 3FFF"]
    #[inline(always)]
    #[must_use]
    pub fn otp_rule3(&mut self) -> OtpRule3W<AipsBridge1MemRule0Spec> {
        OtpRule3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - 0x4013 4000--0x4013 4FFF"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_and_otfad_rule(&mut self) -> FlexspiAndOtfadRuleW<AipsBridge1MemRule0Spec> {
        FlexspiAndOtfadRuleW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 0x4013 6000--0x4013 6FFF"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_rule(&mut self) -> Sdio0RuleW<AipsBridge1MemRule0Spec> {
        Sdio0RuleW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 0x4013 7000--0x4013 7FFF"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_rule(&mut self) -> Sdio1RuleW<AipsBridge1MemRule0Spec> {
        Sdio1RuleW::new(self, 28)
    }
}
#[doc = "Security access rules for AIPS Bridge peripherals. Each AIPS bridge sector is 4 Kbytes, there're 16 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`aips_bridge1_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aips_bridge1_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AipsBridge1MemRule0Spec;
impl crate::RegisterSpec for AipsBridge1MemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aips_bridge1_mem_rule0::R`](R) reader structure"]
impl crate::Readable for AipsBridge1MemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`aips_bridge1_mem_rule0::W`](W) writer structure"]
impl crate::Writable for AipsBridge1MemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE1_MEM_RULE0 to value 0"]
impl crate::Resettable for AipsBridge1MemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
