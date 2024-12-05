#[doc = "Register `AHB_PERIPH2_SLAVE_RULE0` reader"]
pub type R = crate::R<AhbPeriph2SlaveRule0Spec>;
#[doc = "Register `AHB_PERIPH2_SLAVE_RULE0` writer"]
pub type W = crate::W<AhbPeriph2SlaveRule0Spec>;
#[doc = "Field `USB_HS_RAM_RULE` reader - 0x40140000--0x40143FFF"]
pub type UsbHsRamRuleR = crate::FieldReader;
#[doc = "Field `USB_HS_RAM_RULE` writer - 0x40140000--0x40143FFF"]
pub type UsbHsRamRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_HS_DEV_RULE` reader - 0x40144000--0x40144FFF"]
pub type UsbHsDevRuleR = crate::FieldReader;
#[doc = "Field `USB_HS_DEV_RULE` writer - 0x40144000--0x40144FFF"]
pub type UsbHsDevRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_HS_HOST_RULE` reader - 0x40145000--0x40145FFF"]
pub type UsbHsHostRuleR = crate::FieldReader;
#[doc = "Field `USB_HS_HOST_RULE` writer - 0x40145000--0x40145FFF"]
pub type UsbHsHostRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCT_RULE` reader - 0x40146000--0x40146FFF"]
pub type SctRuleR = crate::FieldReader;
#[doc = "Field `SCT_RULE` writer - 0x40146000--0x40146FFF"]
pub type SctRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0x40140000--0x40143FFF"]
    #[inline(always)]
    pub fn usb_hs_ram_rule(&self) -> UsbHsRamRuleR {
        UsbHsRamRuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 0x40144000--0x40144FFF"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&self) -> UsbHsDevRuleR {
        UsbHsDevRuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 0x40145000--0x40145FFF"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&self) -> UsbHsHostRuleR {
        UsbHsHostRuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 0x40146000--0x40146FFF"]
    #[inline(always)]
    pub fn sct_rule(&self) -> SctRuleR {
        SctRuleR::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_PERIPH2_SLAVE_RULE0")
            .field("usb_hs_ram_rule", &self.usb_hs_ram_rule())
            .field("usb_hs_dev_rule", &self.usb_hs_dev_rule())
            .field("usb_hs_host_rule", &self.usb_hs_host_rule())
            .field("sct_rule", &self.sct_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x40140000--0x40143FFF"]
    #[inline(always)]
    pub fn usb_hs_ram_rule(&mut self) -> UsbHsRamRuleW<AhbPeriph2SlaveRule0Spec> {
        UsbHsRamRuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 0x40144000--0x40144FFF"]
    #[inline(always)]
    pub fn usb_hs_dev_rule(&mut self) -> UsbHsDevRuleW<AhbPeriph2SlaveRule0Spec> {
        UsbHsDevRuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 0x40145000--0x40145FFF"]
    #[inline(always)]
    pub fn usb_hs_host_rule(&mut self) -> UsbHsHostRuleW<AhbPeriph2SlaveRule0Spec> {
        UsbHsHostRuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 0x40146000--0x40146FFF"]
    #[inline(always)]
    pub fn sct_rule(&mut self) -> SctRuleW<AhbPeriph2SlaveRule0Spec> {
        SctRuleW::new(self, 12)
    }
}
#[doc = "Security access rules for AHB peripheral slaves area 0x40140000--0x4014BFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_periph2_slave_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_periph2_slave_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbPeriph2SlaveRule0Spec;
impl crate::RegisterSpec for AhbPeriph2SlaveRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_periph2_slave_rule0::R`](R) reader structure"]
impl crate::Readable for AhbPeriph2SlaveRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_periph2_slave_rule0::W`](W) writer structure"]
impl crate::Writable for AhbPeriph2SlaveRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH2_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AhbPeriph2SlaveRule0Spec {
    const RESET_VALUE: u32 = 0;
}
