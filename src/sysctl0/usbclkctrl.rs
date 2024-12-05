#[doc = "Register `USBCLKCTRL` reader"]
pub type R = crate::R<UsbclkctrlSpec>;
#[doc = "Register `USBCLKCTRL` writer"]
pub type W = crate::W<UsbclkctrlSpec>;
#[doc = "USB0 Device need clock signal control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApDevClk {
    #[doc = "0: Under hardware control."]
    UnderHwCtrl = 0,
    #[doc = "1: Forced high."]
    ForcedHigh = 1,
}
impl From<ApDevClk> for bool {
    #[inline(always)]
    fn from(variant: ApDevClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_DEV_CLK` reader - USB0 Device need clock signal control"]
pub type ApDevClkR = crate::BitReader<ApDevClk>;
impl ApDevClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApDevClk {
        match self.bits {
            false => ApDevClk::UnderHwCtrl,
            true => ApDevClk::ForcedHigh,
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn is_under_hw_ctrl(&self) -> bool {
        *self == ApDevClk::UnderHwCtrl
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn is_forced_high(&self) -> bool {
        *self == ApDevClk::ForcedHigh
    }
}
#[doc = "Field `AP_DEV_CLK` writer - USB0 Device need clock signal control"]
pub type ApDevClkW<'a, REG> = crate::BitWriter<'a, REG, ApDevClk>;
impl<'a, REG> ApDevClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn under_hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApDevClk::UnderHwCtrl)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced_high(self) -> &'a mut crate::W<REG> {
        self.variant(ApDevClk::ForcedHigh)
    }
}
#[doc = "USB0 Device need clock polarity for triggering the USB1 wake-up interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolDevClk {
    #[doc = "0: Falling edge of device need_clock triggers wake-up."]
    FallingEdge = 0,
    #[doc = "1: Rising edge of device need_clock triggers wake-up."]
    RisingEdge = 1,
}
impl From<PolDevClk> for bool {
    #[inline(always)]
    fn from(variant: PolDevClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_DEV_CLK` reader - USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
pub type PolDevClkR = crate::BitReader<PolDevClk>;
impl PolDevClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolDevClk {
        match self.bits {
            false => PolDevClk::FallingEdge,
            true => PolDevClk::RisingEdge,
        }
    }
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PolDevClk::FallingEdge
    }
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PolDevClk::RisingEdge
    }
}
#[doc = "Field `POL_DEV_CLK` writer - USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
pub type PolDevClkW<'a, REG> = crate::BitWriter<'a, REG, PolDevClk>;
impl<'a, REG> PolDevClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PolDevClk::FallingEdge)
    }
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PolDevClk::RisingEdge)
    }
}
#[doc = "USB0 Host need clock signal control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApHostClk {
    #[doc = "0: Under hardware control."]
    UnderHwCtrl = 0,
    #[doc = "1: Forced high."]
    ForcedHigh = 1,
}
impl From<ApHostClk> for bool {
    #[inline(always)]
    fn from(variant: ApHostClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP_HOST_CLK` reader - USB0 Host need clock signal control"]
pub type ApHostClkR = crate::BitReader<ApHostClk>;
impl ApHostClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ApHostClk {
        match self.bits {
            false => ApHostClk::UnderHwCtrl,
            true => ApHostClk::ForcedHigh,
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn is_under_hw_ctrl(&self) -> bool {
        *self == ApHostClk::UnderHwCtrl
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn is_forced_high(&self) -> bool {
        *self == ApHostClk::ForcedHigh
    }
}
#[doc = "Field `AP_HOST_CLK` writer - USB0 Host need clock signal control"]
pub type ApHostClkW<'a, REG> = crate::BitWriter<'a, REG, ApHostClk>;
impl<'a, REG> ApHostClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn under_hw_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(ApHostClk::UnderHwCtrl)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced_high(self) -> &'a mut crate::W<REG> {
        self.variant(ApHostClk::ForcedHigh)
    }
}
#[doc = "USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolHostClk {
    #[doc = "0: Falling edge of host need_clock triggers wake-up."]
    FallingEdge = 0,
    #[doc = "1: Rising edge of host need_clock triggers wake-up."]
    RisingEdge = 1,
}
impl From<PolHostClk> for bool {
    #[inline(always)]
    fn from(variant: PolHostClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_HOST_CLK` reader - USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
pub type PolHostClkR = crate::BitReader<PolHostClk>;
impl PolHostClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolHostClk {
        match self.bits {
            false => PolHostClk::FallingEdge,
            true => PolHostClk::RisingEdge,
        }
    }
    #[doc = "Falling edge of host need_clock triggers wake-up."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PolHostClk::FallingEdge
    }
    #[doc = "Rising edge of host need_clock triggers wake-up."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PolHostClk::RisingEdge
    }
}
#[doc = "Field `POL_HOST_CLK` writer - USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
pub type PolHostClkW<'a, REG> = crate::BitWriter<'a, REG, PolHostClk>;
impl<'a, REG> PolHostClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge of host need_clock triggers wake-up."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PolHostClk::FallingEdge)
    }
    #[doc = "Rising edge of host need_clock triggers wake-up."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PolHostClk::RisingEdge)
    }
}
#[doc = "External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsDevWakeupN {
    #[doc = "0: Forces USB0 PHY to wake-up."]
    ForceToWakeup = 0,
    #[doc = "1: Normal USB0 PHY behavior."]
    Normal = 1,
}
impl From<HsDevWakeupN> for bool {
    #[inline(always)]
    fn from(variant: HsDevWakeupN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` reader - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
pub type HsDevWakeupNR = crate::BitReader<HsDevWakeupN>;
impl HsDevWakeupNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsDevWakeupN {
        match self.bits {
            false => HsDevWakeupN::ForceToWakeup,
            true => HsDevWakeupN::Normal,
        }
    }
    #[doc = "Forces USB0 PHY to wake-up."]
    #[inline(always)]
    pub fn is_force_to_wakeup(&self) -> bool {
        *self == HsDevWakeupN::ForceToWakeup
    }
    #[doc = "Normal USB0 PHY behavior."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == HsDevWakeupN::Normal
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` writer - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
pub type HsDevWakeupNW<'a, REG> = crate::BitWriter<'a, REG, HsDevWakeupN>;
impl<'a, REG> HsDevWakeupNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Forces USB0 PHY to wake-up."]
    #[inline(always)]
    pub fn force_to_wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(HsDevWakeupN::ForceToWakeup)
    }
    #[doc = "Normal USB0 PHY behavior."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(HsDevWakeupN::Normal)
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device need clock signal control"]
    #[inline(always)]
    pub fn ap_dev_clk(&self) -> ApDevClkR {
        ApDevClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_dev_clk(&self) -> PolDevClkR {
        PolDevClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB0 Host need clock signal control"]
    #[inline(always)]
    pub fn ap_host_clk(&self) -> ApHostClkR {
        ApHostClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_host_clk(&self) -> PolHostClkR {
        PolHostClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HsDevWakeupNR {
        HsDevWakeupNR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCLKCTRL")
            .field("ap_dev_clk", &self.ap_dev_clk())
            .field("pol_dev_clk", &self.pol_dev_clk())
            .field("ap_host_clk", &self.ap_host_clk())
            .field("pol_host_clk", &self.pol_host_clk())
            .field("hs_dev_wakeup_n", &self.hs_dev_wakeup_n())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device need clock signal control"]
    #[inline(always)]
    pub fn ap_dev_clk(&mut self) -> ApDevClkW<UsbclkctrlSpec> {
        ApDevClkW::new(self, 0)
    }
    #[doc = "Bit 1 - USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_dev_clk(&mut self) -> PolDevClkW<UsbclkctrlSpec> {
        PolDevClkW::new(self, 1)
    }
    #[doc = "Bit 2 - USB0 Host need clock signal control"]
    #[inline(always)]
    pub fn ap_host_clk(&mut self) -> ApHostClkW<UsbclkctrlSpec> {
        ApHostClkW::new(self, 2)
    }
    #[doc = "Bit 3 - USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_host_clk(&mut self) -> PolHostClkW<UsbclkctrlSpec> {
        PolHostClkW::new(self, 3)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&mut self) -> HsDevWakeupNW<UsbclkctrlSpec> {
        HsDevWakeupNW::new(self, 4)
    }
}
#[doc = "USB clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkctrlSpec;
impl crate::RegisterSpec for UsbclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkctrl::R`](R) reader structure"]
impl crate::Readable for UsbclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclkctrl::W`](W) writer structure"]
impl crate::Writable for UsbclkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCLKCTRL to value 0x10"]
impl crate::Resettable for UsbclkctrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
