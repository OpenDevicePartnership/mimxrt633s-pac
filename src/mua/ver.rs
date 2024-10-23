#[doc = "Register `VER` reader"]
pub type R = crate::R<VerSpec>;
#[doc = "Feature Specification Number\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Feature {
    #[doc = "0: Standard features implemented"]
    Feature0 = 0,
    #[doc = "1: RAIP and RAIE register bits implemented on MUA side"]
    Feature1 = 1,
    #[doc = "2: MUA and MUB implemented with the same function. some bits in CR register are moved to CCR register."]
    Feature2 = 2,
    #[doc = "4: some sync logic are deleted for synchronized MUA and MUB. RAIP and RDIP monitor Core reset instead of MU reset. Add HRIP and MURIP and their interrupt enable bits on both sides. Delete RS bit. Add COO mode in PM state."]
    Feature4 = 4,
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(variant: Feature) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Feature {
    type Ux = u16;
}
impl crate::IsEnum for Feature {}
#[doc = "Field `FEATURE` reader - Feature Specification Number"]
pub type FeatureR = crate::FieldReader<Feature>;
impl FeatureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Feature> {
        match self.bits {
            0 => Some(Feature::Feature0),
            1 => Some(Feature::Feature1),
            2 => Some(Feature::Feature2),
            4 => Some(Feature::Feature4),
            _ => None,
        }
    }
    #[doc = "Standard features implemented"]
    #[inline(always)]
    pub fn is_feature_0(&self) -> bool {
        *self == Feature::Feature0
    }
    #[doc = "RAIP and RAIE register bits implemented on MUA side"]
    #[inline(always)]
    pub fn is_feature_1(&self) -> bool {
        *self == Feature::Feature1
    }
    #[doc = "MUA and MUB implemented with the same function. some bits in CR register are moved to CCR register."]
    #[inline(always)]
    pub fn is_feature_2(&self) -> bool {
        *self == Feature::Feature2
    }
    #[doc = "some sync logic are deleted for synchronized MUA and MUB. RAIP and RDIP monitor Core reset instead of MU reset. Add HRIP and MURIP and their interrupt enable bits on both sides. Delete RS bit. Add COO mode in PM state."]
    #[inline(always)]
    pub fn is_feature_4(&self) -> bool {
        *self == Feature::Feature4
    }
}
#[doc = "Field `MINOR` reader - Minor Version Number"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Major Version Number"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Feature Specification Number"]
    #[inline(always)]
    pub fn feature(&self) -> FeatureR {
        FeatureR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[doc = "Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerSpec;
impl crate::RegisterSpec for VerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VerSpec {}
#[doc = "`reset()` method sets VER to value 0x0100_0001"]
impl crate::Resettable for VerSpec {
    const RESET_VALUE: u32 = 0x0100_0001;
}
