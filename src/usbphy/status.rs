#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostdiscondetectStatus {
    #[doc = "0: USB cable disconnect has not been detected at the local host"]
    HostdiscondetectStatus0 = 0,
    #[doc = "1: USB cable disconnect has been detected at the local host"]
    HostdiscondetectStatus1 = 1,
}
impl From<HostdiscondetectStatus> for bool {
    #[inline(always)]
    fn from(variant: HostdiscondetectStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTDISCONDETECT_STATUS` reader - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
pub type HostdiscondetectStatusR = crate::BitReader<HostdiscondetectStatus>;
impl HostdiscondetectStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostdiscondetectStatus {
        match self.bits {
            false => HostdiscondetectStatus::HostdiscondetectStatus0,
            true => HostdiscondetectStatus::HostdiscondetectStatus1,
        }
    }
    #[doc = "USB cable disconnect has not been detected at the local host"]
    #[inline(always)]
    pub fn is_hostdiscondetect_status_0(&self) -> bool {
        *self == HostdiscondetectStatus::HostdiscondetectStatus0
    }
    #[doc = "USB cable disconnect has been detected at the local host"]
    #[inline(always)]
    pub fn is_hostdiscondetect_status_1(&self) -> bool {
        *self == HostdiscondetectStatus::HostdiscondetectStatus1
    }
}
#[doc = "Field `HOSTDISCONDETECT_STATUS` writer - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
pub type HostdiscondetectStatusW<'a, REG> = crate::BitWriter<'a, REG, HostdiscondetectStatus>;
impl<'a, REG> HostdiscondetectStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB cable disconnect has not been detected at the local host"]
    #[inline(always)]
    pub fn hostdiscondetect_status_0(self) -> &'a mut crate::W<REG> {
        self.variant(HostdiscondetectStatus::HostdiscondetectStatus0)
    }
    #[doc = "USB cable disconnect has been detected at the local host"]
    #[inline(always)]
    pub fn hostdiscondetect_status_1(self) -> &'a mut crate::W<REG> {
        self.variant(HostdiscondetectStatus::HostdiscondetectStatus1)
    }
}
#[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevpluginStatus {
    #[doc = "0: No attachment to a USB host is detected"]
    DevpluginStatus0 = 0,
    #[doc = "1: Cable attachment to a USB host is detected"]
    DevpluginStatus1 = 1,
}
impl From<DevpluginStatus> for bool {
    #[inline(always)]
    fn from(variant: DevpluginStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVPLUGIN_STATUS` reader - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
pub type DevpluginStatusR = crate::BitReader<DevpluginStatus>;
impl DevpluginStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevpluginStatus {
        match self.bits {
            false => DevpluginStatus::DevpluginStatus0,
            true => DevpluginStatus::DevpluginStatus1,
        }
    }
    #[doc = "No attachment to a USB host is detected"]
    #[inline(always)]
    pub fn is_devplugin_status_0(&self) -> bool {
        *self == DevpluginStatus::DevpluginStatus0
    }
    #[doc = "Cable attachment to a USB host is detected"]
    #[inline(always)]
    pub fn is_devplugin_status_1(&self) -> bool {
        *self == DevpluginStatus::DevpluginStatus1
    }
}
#[doc = "Field `DEVPLUGIN_STATUS` writer - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
pub type DevpluginStatusW<'a, REG> = crate::BitWriter<'a, REG, DevpluginStatus>;
impl<'a, REG> DevpluginStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No attachment to a USB host is detected"]
    #[inline(always)]
    pub fn devplugin_status_0(self) -> &'a mut crate::W<REG> {
        self.variant(DevpluginStatus::DevpluginStatus0)
    }
    #[doc = "Cable attachment to a USB host is detected"]
    #[inline(always)]
    pub fn devplugin_status_1(self) -> &'a mut crate::W<REG> {
        self.variant(DevpluginStatus::DevpluginStatus1)
    }
}
#[doc = "Field `RESUME_STATUS` reader - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
pub type ResumeStatusR = crate::BitReader;
#[doc = "Field `RESUME_STATUS` writer - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
pub type ResumeStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HostdiscondetectStatusR {
        HostdiscondetectStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DevpluginStatusR {
        DevpluginStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> ResumeStatusR {
        ResumeStatusR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("hostdiscondetect_status", &self.hostdiscondetect_status())
            .field("devplugin_status", &self.devplugin_status())
            .field("resume_status", &self.resume_status())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&mut self) -> HostdiscondetectStatusW<StatusSpec> {
        HostdiscondetectStatusW::new(self, 3)
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub fn devplugin_status(&mut self) -> DevpluginStatusW<StatusSpec> {
        DevpluginStatusW::new(self, 6)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&mut self) -> ResumeStatusW<StatusSpec> {
        ResumeStatusW::new(self, 10)
    }
}
#[doc = "USB PHY Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
