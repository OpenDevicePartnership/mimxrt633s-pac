#[doc = "Register `USB1_CHRG_DET_STAT` reader"]
pub type R = crate::R<Usb1ChrgDetStatSpec>;
#[doc = "Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlugContact {
    #[doc = "0: No USB cable attachment has been detected"]
    PlugContact0 = 0,
    #[doc = "1: A USB cable attachment between the device and host has been detected"]
    PlugContact1 = 1,
}
impl From<PlugContact> for bool {
    #[inline(always)]
    fn from(variant: PlugContact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLUG_CONTACT` reader - Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
pub type PlugContactR = crate::BitReader<PlugContact>;
impl PlugContactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlugContact {
        match self.bits {
            false => PlugContact::PlugContact0,
            true => PlugContact::PlugContact1,
        }
    }
    #[doc = "No USB cable attachment has been detected"]
    #[inline(always)]
    pub fn is_plug_contact_0(&self) -> bool {
        *self == PlugContact::PlugContact0
    }
    #[doc = "A USB cable attachment between the device and host has been detected"]
    #[inline(always)]
    pub fn is_plug_contact_1(&self) -> bool {
        *self == PlugContact::PlugContact1
    }
}
#[doc = "Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChrgDetected {
    #[doc = "0: Standard Downstream Port (SDP) has been detected"]
    ChrgDetected0 = 0,
    #[doc = "1: Charging Port has been detected"]
    ChrgDetected1 = 1,
}
impl From<ChrgDetected> for bool {
    #[inline(always)]
    fn from(variant: ChrgDetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHRG_DETECTED` reader - Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
pub type ChrgDetectedR = crate::BitReader<ChrgDetected>;
impl ChrgDetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChrgDetected {
        match self.bits {
            false => ChrgDetected::ChrgDetected0,
            true => ChrgDetected::ChrgDetected1,
        }
    }
    #[doc = "Standard Downstream Port (SDP) has been detected"]
    #[inline(always)]
    pub fn is_chrg_detected_0(&self) -> bool {
        *self == ChrgDetected::ChrgDetected0
    }
    #[doc = "Charging Port has been detected"]
    #[inline(always)]
    pub fn is_chrg_detected_1(&self) -> bool {
        *self == ChrgDetected::ChrgDetected1
    }
}
#[doc = "Single ended receiver output for the USB_DM pin, from charger detection circuits.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmState {
    #[doc = "0: USB_DM pin voltage is < 0.8V"]
    DmState0 = 0,
    #[doc = "1: USB_DM pin voltage is > 2.0V"]
    DmState1 = 1,
}
impl From<DmState> for bool {
    #[inline(always)]
    fn from(variant: DmState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DM_STATE` reader - Single ended receiver output for the USB_DM pin, from charger detection circuits."]
pub type DmStateR = crate::BitReader<DmState>;
impl DmStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmState {
        match self.bits {
            false => DmState::DmState0,
            true => DmState::DmState1,
        }
    }
    #[doc = "USB_DM pin voltage is < 0.8V"]
    #[inline(always)]
    pub fn is_dm_state_0(&self) -> bool {
        *self == DmState::DmState0
    }
    #[doc = "USB_DM pin voltage is > 2.0V"]
    #[inline(always)]
    pub fn is_dm_state_1(&self) -> bool {
        *self == DmState::DmState1
    }
}
#[doc = "Single ended receiver output for the USB_DP pin, from charger detection circuits.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpState {
    #[doc = "0: USB_DP pin voltage is < 0.8V"]
    DpState0 = 0,
    #[doc = "1: USB_DP pin voltage is > 2.0V"]
    DpState1 = 1,
}
impl From<DpState> for bool {
    #[inline(always)]
    fn from(variant: DpState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_STATE` reader - Single ended receiver output for the USB_DP pin, from charger detection circuits."]
pub type DpStateR = crate::BitReader<DpState>;
impl DpStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpState {
        match self.bits {
            false => DpState::DpState0,
            true => DpState::DpState1,
        }
    }
    #[doc = "USB_DP pin voltage is < 0.8V"]
    #[inline(always)]
    pub fn is_dp_state_0(&self) -> bool {
        *self == DpState::DpState0
    }
    #[doc = "USB_DP pin voltage is > 2.0V"]
    #[inline(always)]
    pub fn is_dp_state_1(&self) -> bool {
        *self == DpState::DpState1
    }
}
#[doc = "Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecdetDcp {
    #[doc = "0: Charging Downstream Port (CDP) has been detected"]
    SecdetDcp0 = 0,
    #[doc = "1: Downstream Charging Port (DCP) has been detected"]
    SecdetDcp1 = 1,
}
impl From<SecdetDcp> for bool {
    #[inline(always)]
    fn from(variant: SecdetDcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECDET_DCP` reader - Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
pub type SecdetDcpR = crate::BitReader<SecdetDcp>;
impl SecdetDcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecdetDcp {
        match self.bits {
            false => SecdetDcp::SecdetDcp0,
            true => SecdetDcp::SecdetDcp1,
        }
    }
    #[doc = "Charging Downstream Port (CDP) has been detected"]
    #[inline(always)]
    pub fn is_secdet_dcp_0(&self) -> bool {
        *self == SecdetDcp::SecdetDcp0
    }
    #[doc = "Downstream Charging Port (DCP) has been detected"]
    #[inline(always)]
    pub fn is_secdet_dcp_1(&self) -> bool {
        *self == SecdetDcp::SecdetDcp1
    }
}
impl R {
    #[doc = "Bit 0 - Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
    #[inline(always)]
    pub fn plug_contact(&self) -> PlugContactR {
        PlugContactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
    #[inline(always)]
    pub fn chrg_detected(&self) -> ChrgDetectedR {
        ChrgDetectedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dm_state(&self) -> DmStateR {
        DmStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dp_state(&self) -> DpStateR {
        DpStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
    #[inline(always)]
    pub fn secdet_dcp(&self) -> SecdetDcpR {
        SecdetDcpR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DET_STAT")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[doc = "USB PHY Charger Detect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_chrg_det_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1ChrgDetStatSpec;
impl crate::RegisterSpec for Usb1ChrgDetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_chrg_det_stat::R`](R) reader structure"]
impl crate::Readable for Usb1ChrgDetStatSpec {}
#[doc = "`reset()` method sets USB1_CHRG_DET_STAT to value 0"]
impl crate::Resettable for Usb1ChrgDetStatSpec {
    const RESET_VALUE: u32 = 0;
}
