#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENHOSTDISCONDETECT` reader - For host mode, enables high-speed disconnect detector"]
pub type EnhostdiscondetectR = crate::BitReader;
#[doc = "Field `ENHOSTDISCONDETECT` writer - For host mode, enables high-speed disconnect detector"]
pub type EnhostdiscondetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Indicates that the device has disconnected in High-Speed mode"]
pub type HostdiscondetectIrqR = crate::BitReader;
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Indicates that the device has disconnected in High-Speed mode"]
pub type HostdiscondetectIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endevplugindet {
    #[doc = "0: Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    Endevplugindet0 = 0,
    #[doc = "1: Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    Endevplugindet1 = 1,
}
impl From<Endevplugindet> for bool {
    #[inline(always)]
    fn from(variant: Endevplugindet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEVPLUGINDET` reader - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type EndevplugindetR = crate::BitReader<Endevplugindet>;
impl EndevplugindetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endevplugindet {
        match self.bits {
            false => Endevplugindet::Endevplugindet0,
            true => Endevplugindet::Endevplugindet1,
        }
    }
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn is_endevplugindet_0(&self) -> bool {
        *self == Endevplugindet::Endevplugindet0
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn is_endevplugindet_1(&self) -> bool {
        *self == Endevplugindet::Endevplugindet1
    }
}
#[doc = "Field `ENDEVPLUGINDET` writer - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type EndevplugindetW<'a, REG> = crate::BitWriter<'a, REG, Endevplugindet>;
impl<'a, REG> EndevplugindetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn endevplugindet_0(self) -> &'a mut crate::W<REG> {
        self.variant(Endevplugindet::Endevplugindet0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn endevplugindet_1(self) -> &'a mut crate::W<REG> {
        self.variant(Endevplugindet::Endevplugindet1)
    }
}
#[doc = "Field `DEVPLUGIN_IRQ` reader - Indicates that the device is connected"]
pub type DevpluginIrqR = crate::BitReader;
#[doc = "Field `DEVPLUGIN_IRQ` writer - Indicates that the device is connected"]
pub type DevpluginIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUTMILEVEL2` reader - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type Enutmilevel2R = crate::BitReader;
#[doc = "Field `ENUTMILEVEL2` writer - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type Enutmilevel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUTMILEVEL3` reader - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type Enutmilevel3R = crate::BitReader;
#[doc = "Field `ENUTMILEVEL3` writer - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type Enutmilevel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORESUME_EN` reader - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AutoresumeEnR = crate::BitReader;
#[doc = "Field `AUTORESUME_EN` writer - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AutoresumeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type EnautoclrClkgateR = crate::BitReader;
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type EnautoclrClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type EnautoclrPhyPwdR = crate::BitReader;
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type EnautoclrPhyPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSDLL_RST_EN` reader - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
pub type FsdllRstEnR = crate::BitReader;
#[doc = "Field `FSDLL_RST_EN` writer - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
pub type FsdllRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_FORCE_LS_SE0` reader - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HostForceLsSe0R = crate::BitReader;
#[doc = "Field `HOST_FORCE_LS_SE0` writer - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HostForceLsSe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_SUSPENDM` reader - Used by the PHY to indicate a powered-down state"]
pub type UtmiSuspendmR = crate::BitReader;
#[doc = "Field `UTMI_SUSPENDM` writer - Used by the PHY to indicate a powered-down state"]
pub type UtmiSuspendmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGATE` reader - Gate UTMI Clocks"]
pub type ClkgateR = crate::BitReader;
#[doc = "Field `CLKGATE` writer - Gate UTMI Clocks"]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRST` reader - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SftrstR = crate::BitReader;
#[doc = "Field `SFTRST` writer - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SftrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> EnhostdiscondetectR {
        EnhostdiscondetectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HostdiscondetectIrqR {
        HostdiscondetectIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&self) -> EndevplugindetR {
        EndevplugindetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DevpluginIrqR {
        DevpluginIrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> Enutmilevel2R {
        Enutmilevel2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> Enutmilevel3R {
        Enutmilevel3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AutoresumeEnR {
        AutoresumeEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> EnautoclrClkgateR {
        EnautoclrClkgateR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> EnautoclrPhyPwdR {
        EnautoclrPhyPwdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&self) -> FsdllRstEnR {
        FsdllRstEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HostForceLsSe0R {
        HostForceLsSe0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UtmiSuspendmR {
        UtmiSuspendmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SftrstR {
        SftrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
            .field("host_force_ls_se0", &self.host_force_ls_se0())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    #[must_use]
    pub fn enhostdiscondetect(&mut self) -> EnhostdiscondetectW<CtrlSpec> {
        EnhostdiscondetectW::new(self, 1)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn hostdiscondetect_irq(&mut self) -> HostdiscondetectIrqW<CtrlSpec> {
        HostdiscondetectIrqW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    #[must_use]
    pub fn endevplugindet(&mut self) -> EndevplugindetW<CtrlSpec> {
        EndevplugindetW::new(self, 4)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_irq(&mut self) -> DevpluginIrqW<CtrlSpec> {
        DevpluginIrqW::new(self, 12)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel2(&mut self) -> Enutmilevel2W<CtrlSpec> {
        Enutmilevel2W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel3(&mut self) -> Enutmilevel3W<CtrlSpec> {
        Enutmilevel3W::new(self, 15)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn autoresume_en(&mut self) -> AutoresumeEnW<CtrlSpec> {
        AutoresumeEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_clkgate(&mut self) -> EnautoclrClkgateW<CtrlSpec> {
        EnautoclrClkgateW::new(self, 19)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_phy_pwd(&mut self) -> EnautoclrPhyPwdW<CtrlSpec> {
        EnautoclrPhyPwdW::new(self, 20)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    #[must_use]
    pub fn fsdll_rst_en(&mut self) -> FsdllRstEnW<CtrlSpec> {
        FsdllRstEnW::new(self, 24)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    #[must_use]
    pub fn host_force_ls_se0(&mut self) -> HostForceLsSe0W<CtrlSpec> {
        HostForceLsSe0W::new(self, 28)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_suspendm(&mut self) -> UtmiSuspendmW<CtrlSpec> {
        UtmiSuspendmW::new(self, 29)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> ClkgateW<CtrlSpec> {
        ClkgateW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SftrstW<CtrlSpec> {
        SftrstW::new(self, 31)
    }
}
#[doc = "USB PHY General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0xc000_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
