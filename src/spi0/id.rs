#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `APERTURE` reader - Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
pub type ApertureR = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor revision of module implementation."]
pub type MinorRevR = crate::FieldReader;
#[doc = "Field `MAJOR_REV` reader - Major revision of module implementation."]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `ID` reader - Module identifier for the selected function."]
pub type IdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub fn aperture(&self) -> ApertureR {
        ApertureR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation."]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("aperture", &self.aperture())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[doc = "Peripheral identification register.\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0;
}
