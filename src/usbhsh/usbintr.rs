#[doc = "Register `USBINTR` reader"]
pub type R = crate::R<UsbintrSpec>;
#[doc = "Register `USBINTR` writer"]
pub type W = crate::W<UsbintrSpec>;
#[doc = "Field `PCDE` reader - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
pub type PcdeR = crate::BitReader;
#[doc = "Field `PCDE` writer - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
pub type PcdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLRE` reader - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
pub type FlreR = crate::BitReader;
#[doc = "Field `FLRE` writer - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
pub type FlreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATL_IRQ_E` reader - ATL IRQ Enable bit: 1: enable 0: disable."]
pub type AtlIrqER = crate::BitReader;
#[doc = "Field `ATL_IRQ_E` writer - ATL IRQ Enable bit: 1: enable 0: disable."]
pub type AtlIrqEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_IRQ_E` reader - ISO IRQ Enable bit: 1: enable 0: disable."]
pub type IsoIrqER = crate::BitReader;
#[doc = "Field `ISO_IRQ_E` writer - ISO IRQ Enable bit: 1: enable 0: disable."]
pub type IsoIrqEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_IRQ_E` reader - INT IRQ Enable bit: 1: enable 0: disable."]
pub type IntIrqER = crate::BitReader;
#[doc = "Field `INT_IRQ_E` writer - INT IRQ Enable bit: 1: enable 0: disable."]
pub type IntIrqEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_E` reader - SOF Interrupt Enable bit: 1: enable 0: disable."]
pub type SofER = crate::BitReader;
#[doc = "Field `SOF_E` writer - SOF Interrupt Enable bit: 1: enable 0: disable."]
pub type SofEW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&self) -> PcdeR {
        PcdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&self) -> FlreR {
        FlreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&self) -> AtlIrqER {
        AtlIrqER::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&self) -> IsoIrqER {
        IsoIrqER::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&self) -> IntIrqER {
        IntIrqER::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&self) -> SofER {
        SofER::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBINTR")
            .field("pcde", &self.pcde())
            .field("flre", &self.flre())
            .field("atl_irq_e", &self.atl_irq_e())
            .field("iso_irq_e", &self.iso_irq_e())
            .field("int_irq_e", &self.int_irq_e())
            .field("sof_e", &self.sof_e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn pcde(&mut self) -> PcdeW<UsbintrSpec> {
        PcdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn flre(&mut self) -> FlreW<UsbintrSpec> {
        FlreW::new(self, 3)
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn atl_irq_e(&mut self) -> AtlIrqEW<UsbintrSpec> {
        AtlIrqEW::new(self, 16)
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn iso_irq_e(&mut self) -> IsoIrqEW<UsbintrSpec> {
        IsoIrqEW::new(self, 17)
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn int_irq_e(&mut self) -> IntIrqEW<UsbintrSpec> {
        IntIrqEW::new(self, 18)
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn sof_e(&mut self) -> SofEW<UsbintrSpec> {
        SofEW::new(self, 19)
    }
}
#[doc = "USB Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbintrSpec;
impl crate::RegisterSpec for UsbintrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintr::R`](R) reader structure"]
impl crate::Readable for UsbintrSpec {}
#[doc = "`write(|w| ..)` method takes [`usbintr::W`](W) writer structure"]
impl crate::Writable for UsbintrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for UsbintrSpec {
    const RESET_VALUE: u32 = 0;
}
