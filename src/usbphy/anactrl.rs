#[doc = "Register `ANACTRL` reader"]
pub type R = crate::R<AnactrlSpec>;
#[doc = "Register `ANACTRL` writer"]
pub type W = crate::W<AnactrlSpec>;
#[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevPulldown {
    #[doc = "0: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    DevPulldown0 = 0,
    #[doc = "1: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    DevPulldown1 = 1,
}
impl From<DevPulldown> for bool {
    #[inline(always)]
    fn from(variant: DevPulldown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_PULLDOWN` reader - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DevPulldownR = crate::BitReader<DevPulldown>;
impl DevPulldownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevPulldown {
        match self.bits {
            false => DevPulldown::DevPulldown0,
            true => DevPulldown::DevPulldown1,
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn is_dev_pulldown_0(&self) -> bool {
        *self == DevPulldown::DevPulldown0
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn is_dev_pulldown_1(&self) -> bool {
        *self == DevPulldown::DevPulldown1
    }
}
#[doc = "Field `DEV_PULLDOWN` writer - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DevPulldownW<'a, REG> = crate::BitWriter<'a, REG, DevPulldown>;
impl<'a, REG> DevPulldownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn dev_pulldown_0(self) -> &'a mut crate::W<REG> {
        self.variant(DevPulldown::DevPulldown0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn dev_pulldown_1(self) -> &'a mut crate::W<REG> {
        self.variant(DevPulldown::DevPulldown1)
    }
}
impl R {
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DevPulldownR {
        DevPulldownR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL")
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DevPulldownW<AnactrlSpec> {
        DevPulldownW::new(self, 10)
    }
}
#[doc = "USB PHY Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnactrlSpec;
impl crate::RegisterSpec for AnactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anactrl::R`](R) reader structure"]
impl crate::Readable for AnactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`anactrl::W`](W) writer structure"]
impl crate::Writable for AnactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANACTRL to value 0x0a00_0402"]
impl crate::Resettable for AnactrlSpec {
    const RESET_VALUE: u32 = 0x0a00_0402;
}
