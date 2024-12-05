#[doc = "Register `USBSTS` reader"]
pub type R = crate::R<UsbstsSpec>;
#[doc = "Register `USBSTS` writer"]
pub type W = crate::W<UsbstsSpec>;
#[doc = "Field `PCD` reader - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port. or - the ID pin value changes or - an LPM token has been transmitted to enter LPM L1 suspend state.. Software must write a one to clear the bit"]
pub type PcdR = crate::BitReader;
#[doc = "Field `PCD` writer - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port. or - the ID pin value changes or - an LPM token has been transmitted to enter LPM L1 suspend state.. Software must write a one to clear the bit"]
pub type PcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLR` reader - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATL_IRQ` reader - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub type AtlIrqR = crate::BitReader;
#[doc = "Field `ATL_IRQ` writer - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub type AtlIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_IRQ` reader - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub type IsoIrqR = crate::BitReader;
#[doc = "Field `ISO_IRQ` writer - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub type IsoIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_IRQ` reader - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub type IntIrqR = crate::BitReader;
#[doc = "Field `INT_IRQ` writer - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub type IntIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_IRQ` reader - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub type SofIrqR = crate::BitReader;
#[doc = "Field `SOF_IRQ` writer - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub type SofIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port. or - the ID pin value changes or - an LPM token has been transmitted to enter LPM L1 suspend state.. Software must write a one to clear the bit"]
    #[inline(always)]
    pub fn pcd(&self) -> PcdR {
        PcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&self) -> AtlIrqR {
        AtlIrqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&self) -> IsoIrqR {
        IsoIrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&self) -> IntIrqR {
        IntIrqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&self) -> SofIrqR {
        SofIrqR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBSTS")
            .field("pcd", &self.pcd())
            .field("flr", &self.flr())
            .field("atl_irq", &self.atl_irq())
            .field("iso_irq", &self.iso_irq())
            .field("int_irq", &self.int_irq())
            .field("sof_irq", &self.sof_irq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port. or - the ID pin value changes or - an LPM token has been transmitted to enter LPM L1 suspend state.. Software must write a one to clear the bit"]
    #[inline(always)]
    pub fn pcd(&mut self) -> PcdW<UsbstsSpec> {
        PcdW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&mut self) -> FlrW<UsbstsSpec> {
        FlrW::new(self, 3)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&mut self) -> AtlIrqW<UsbstsSpec> {
        AtlIrqW::new(self, 16)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&mut self) -> IsoIrqW<UsbstsSpec> {
        IsoIrqW::new(self, 17)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&mut self) -> IntIrqW<UsbstsSpec> {
        IntIrqW::new(self, 18)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&mut self) -> SofIrqW<UsbstsSpec> {
        SofIrqW::new(self, 19)
    }
}
#[doc = "USB Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbstsSpec;
impl crate::RegisterSpec for UsbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbsts::R`](R) reader structure"]
impl crate::Readable for UsbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`usbsts::W`](W) writer structure"]
impl crate::Writable for UsbstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for UsbstsSpec {
    const RESET_VALUE: u32 = 0;
}
