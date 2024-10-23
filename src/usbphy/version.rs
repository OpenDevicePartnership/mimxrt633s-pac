#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `STEP` reader - Fixed read-only value reflecting the stepping of the RTL version."]
pub type StepR = crate::FieldReader<u16>;
#[doc = "Field `MINOR` reader - Fixed read-only value reflecting the MINOR field of the RTL version"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Fixed read-only value reflecting the MAJOR field of the RTL versio"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fixed read-only value reflecting the MINOR field of the RTL version"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fixed read-only value reflecting the MAJOR field of the RTL versio"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[doc = "UTMI RTL Version\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0x0500_0000"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0x0500_0000;
}
