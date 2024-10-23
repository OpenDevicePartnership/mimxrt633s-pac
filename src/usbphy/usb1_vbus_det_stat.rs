#[doc = "Register `USB1_VBUS_DET_STAT` reader"]
pub type R = crate::R<Usb1VbusDetStatSpec>;
#[doc = "Session End indicator Session End status, value inverted from Session Valid comparator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sessend {
    #[doc = "0: The VBUS voltage is above the Session Valid threshold"]
    Sessend0 = 0,
    #[doc = "1: The VBUS voltage is below the Session Valid threshold"]
    Sessend1 = 1,
}
impl From<Sessend> for bool {
    #[inline(always)]
    fn from(variant: Sessend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SESSEND` reader - Session End indicator Session End status, value inverted from Session Valid comparator"]
pub type SessendR = crate::BitReader<Sessend>;
impl SessendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sessend {
        match self.bits {
            false => Sessend::Sessend0,
            true => Sessend::Sessend1,
        }
    }
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    #[inline(always)]
    pub fn is_sessend_0(&self) -> bool {
        *self == Sessend::Sessend0
    }
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    #[inline(always)]
    pub fn is_sessend_1(&self) -> bool {
        *self == Sessend::Sessend1
    }
}
#[doc = "B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bvalid {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    Bvalid0 = 0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    Bvalid1 = 1,
}
impl From<Bvalid> for bool {
    #[inline(always)]
    fn from(variant: Bvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BVALID` reader - B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator"]
pub type BvalidR = crate::BitReader<Bvalid>;
impl BvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bvalid {
        match self.bits {
            false => Bvalid::Bvalid0,
            true => Bvalid::Bvalid1,
        }
    }
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    #[inline(always)]
    pub fn is_bvalid_0(&self) -> bool {
        *self == Bvalid::Bvalid0
    }
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    #[inline(always)]
    pub fn is_bvalid_1(&self) -> bool {
        *self == Bvalid::Bvalid1
    }
}
#[doc = "A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avalid {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    Avalid0 = 0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    Avalid1 = 1,
}
impl From<Avalid> for bool {
    #[inline(always)]
    fn from(variant: Avalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVALID` reader - A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator"]
pub type AvalidR = crate::BitReader<Avalid>;
impl AvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avalid {
        match self.bits {
            false => Avalid::Avalid0,
            true => Avalid::Avalid1,
        }
    }
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    #[inline(always)]
    pub fn is_avalid_0(&self) -> bool {
        *self == Avalid::Avalid0
    }
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    #[inline(always)]
    pub fn is_avalid_1(&self) -> bool {
        *self == Avalid::Avalid1
    }
}
#[doc = "VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusValid {
    #[doc = "0: VBUS is below the comparator threshold"]
    VbusValid0 = 0,
    #[doc = "1: VBUS is above the comparator threshold"]
    VbusValid1 = 1,
}
impl From<VbusValid> for bool {
    #[inline(always)]
    fn from(variant: VbusValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_VALID` reader - VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin"]
pub type VbusValidR = crate::BitReader<VbusValid>;
impl VbusValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusValid {
        match self.bits {
            false => VbusValid::VbusValid0,
            true => VbusValid::VbusValid1,
        }
    }
    #[doc = "VBUS is below the comparator threshold"]
    #[inline(always)]
    pub fn is_vbus_valid_0(&self) -> bool {
        *self == VbusValid::VbusValid0
    }
    #[doc = "VBUS is above the comparator threshold"]
    #[inline(always)]
    pub fn is_vbus_valid_1(&self) -> bool {
        *self == VbusValid::VbusValid1
    }
}
#[doc = "VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusValid3v {
    #[doc = "0: VBUS voltage is below VBUS_VALID_3V threshold"]
    VbusValid3v0 = 0,
    #[doc = "1: VBUS voltage is above VBUS_VALID_3V threshold"]
    VbusValid3v1 = 1,
}
impl From<VbusValid3v> for bool {
    #[inline(always)]
    fn from(variant: VbusValid3v) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_VALID_3V` reader - VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators"]
pub type VbusValid3vR = crate::BitReader<VbusValid3v>;
impl VbusValid3vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusValid3v {
        match self.bits {
            false => VbusValid3v::VbusValid3v0,
            true => VbusValid3v::VbusValid3v1,
        }
    }
    #[doc = "VBUS voltage is below VBUS_VALID_3V threshold"]
    #[inline(always)]
    pub fn is_vbus_valid_3v_0(&self) -> bool {
        *self == VbusValid3v::VbusValid3v0
    }
    #[doc = "VBUS voltage is above VBUS_VALID_3V threshold"]
    #[inline(always)]
    pub fn is_vbus_valid_3v_1(&self) -> bool {
        *self == VbusValid3v::VbusValid3v1
    }
}
impl R {
    #[doc = "Bit 0 - Session End indicator Session End status, value inverted from Session Valid comparator"]
    #[inline(always)]
    pub fn sessend(&self) -> SessendR {
        SessendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline(always)]
    pub fn bvalid(&self) -> BvalidR {
        BvalidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline(always)]
    pub fn avalid(&self) -> AvalidR {
        AvalidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VbusValidR {
        VbusValidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators"]
    #[inline(always)]
    pub fn vbus_valid_3v(&self) -> VbusValid3vR {
        VbusValid3vR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .finish()
    }
}
#[doc = "USB PHY VBUS Detector Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_det_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1VbusDetStatSpec;
impl crate::RegisterSpec for Usb1VbusDetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_vbus_det_stat::R`](R) reader structure"]
impl crate::Readable for Usb1VbusDetStatSpec {}
#[doc = "`reset()` method sets USB1_VBUS_DET_STAT to value 0"]
impl crate::Resettable for Usb1VbusDetStatSpec {
    const RESET_VALUE: u32 = 0;
}
