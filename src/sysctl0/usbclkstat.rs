#[doc = "Register `USBCLKSTAT` reader"]
pub type R = crate::R<UsbclkstatSpec>;
#[doc = "Register `USBCLKSTAT` writer"]
pub type W = crate::W<UsbclkstatSpec>;
#[doc = "USB Device USB_NEEDCLK signal status:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevNeedClkst {
    #[doc = "0: low"]
    Low = 0,
    #[doc = "1: high"]
    High = 1,
}
impl From<DevNeedClkst> for bool {
    #[inline(always)]
    fn from(variant: DevNeedClkst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_NEED_CLKST` reader - USB Device USB_NEEDCLK signal status:"]
pub type DevNeedClkstR = crate::BitReader<DevNeedClkst>;
impl DevNeedClkstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevNeedClkst {
        match self.bits {
            false => DevNeedClkst::Low,
            true => DevNeedClkst::High,
        }
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DevNeedClkst::Low
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DevNeedClkst::High
    }
}
#[doc = "USB Device Host USB_NEEDCLK signal status:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostNeedClkst {
    #[doc = "0: low"]
    Low = 0,
    #[doc = "1: high"]
    High = 1,
}
impl From<HostNeedClkst> for bool {
    #[inline(always)]
    fn from(variant: HostNeedClkst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_NEED_CLKST` reader - USB Device Host USB_NEEDCLK signal status:"]
pub type HostNeedClkstR = crate::BitReader<HostNeedClkst>;
impl HostNeedClkstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostNeedClkst {
        match self.bits {
            false => HostNeedClkst::Low,
            true => HostNeedClkst::High,
        }
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HostNeedClkst::Low
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HostNeedClkst::High
    }
}
impl R {
    #[doc = "Bit 0 - USB Device USB_NEEDCLK signal status:"]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DevNeedClkstR {
        DevNeedClkstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Device Host USB_NEEDCLK signal status:"]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HostNeedClkstR {
        HostNeedClkstR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCLKSTAT")
            .field("dev_need_clkst", &self.dev_need_clkst())
            .field("host_need_clkst", &self.host_need_clkst())
            .finish()
    }
}
impl W {}
#[doc = "USB clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkstatSpec;
impl crate::RegisterSpec for UsbclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkstat::R`](R) reader structure"]
impl crate::Readable for UsbclkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclkstat::W`](W) writer structure"]
impl crate::Writable for UsbclkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCLKSTAT to value 0x03"]
impl crate::Resettable for UsbclkstatSpec {
    const RESET_VALUE: u32 = 0x03;
}
