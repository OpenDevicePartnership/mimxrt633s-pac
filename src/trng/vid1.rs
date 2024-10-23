#[doc = "Register `VID1` reader"]
pub type R = crate::R<Vid1Spec>;
#[doc = "Shows the IP's Minor revision of the TRNG.\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MinRev {
    #[doc = "0: Minor revision number for TRNG."]
    MinRev0 = 0,
}
impl From<MinRev> for u8 {
    #[inline(always)]
    fn from(variant: MinRev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MinRev {
    type Ux = u8;
}
impl crate::IsEnum for MinRev {}
#[doc = "Field `MIN_REV` reader - Shows the IP's Minor revision of the TRNG."]
pub type MinRevR = crate::FieldReader<MinRev>;
impl MinRevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MinRev> {
        match self.bits {
            0 => Some(MinRev::MinRev0),
            _ => None,
        }
    }
    #[doc = "Minor revision number for TRNG."]
    #[inline(always)]
    pub fn is_min_rev_0(&self) -> bool {
        *self == MinRev::MinRev0
    }
}
#[doc = "Shows the IP's Major revision of the TRNG.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MajRev {
    #[doc = "1: Major revision number for TRNG."]
    MajRev1 = 1,
}
impl From<MajRev> for u8 {
    #[inline(always)]
    fn from(variant: MajRev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MajRev {
    type Ux = u8;
}
impl crate::IsEnum for MajRev {}
#[doc = "Field `MAJ_REV` reader - Shows the IP's Major revision of the TRNG."]
pub type MajRevR = crate::FieldReader<MajRev>;
impl MajRevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MajRev> {
        match self.bits {
            1 => Some(MajRev::MajRev1),
            _ => None,
        }
    }
    #[doc = "Major revision number for TRNG."]
    #[inline(always)]
    pub fn is_maj_rev_1(&self) -> bool {
        *self == MajRev::MajRev1
    }
}
#[doc = "Shows the IP ID.\n\nValue on reset: 48"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IpId {
    #[doc = "48: ID for TRNG."]
    IpId48 = 48,
}
impl From<IpId> for u16 {
    #[inline(always)]
    fn from(variant: IpId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IpId {
    type Ux = u16;
}
impl crate::IsEnum for IpId {}
#[doc = "Field `IP_ID` reader - Shows the IP ID."]
pub type IpIdR = crate::FieldReader<IpId>;
impl IpIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IpId> {
        match self.bits {
            48 => Some(IpId::IpId48),
            _ => None,
        }
    }
    #[doc = "ID for TRNG."]
    #[inline(always)]
    pub fn is_ip_id_48(&self) -> bool {
        *self == IpId::IpId48
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub fn min_rev(&self) -> MinRevR {
        MinRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's Major revision of the TRNG."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MajRevR {
        MajRevR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Shows the IP ID."]
    #[inline(always)]
    pub fn ip_id(&self) -> IpIdR {
        IpIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID1")
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[doc = "Version ID Register (MS)\n\nYou can [`read`](crate::Reg::read) this register and get [`vid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vid1Spec;
impl crate::RegisterSpec for Vid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid1::R`](R) reader structure"]
impl crate::Readable for Vid1Spec {}
#[doc = "`reset()` method sets VID1 to value 0x0030_0104"]
impl crate::Resettable for Vid1Spec {
    const RESET_VALUE: u32 = 0x0030_0104;
}
