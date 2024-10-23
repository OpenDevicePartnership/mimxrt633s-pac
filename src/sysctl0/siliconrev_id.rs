#[doc = "Register `SILICONREV_ID` reader"]
pub type R = crate::R<SiliconrevIdSpec>;
#[doc = "Field `MINOR` reader - Silicon revision minor tag. (IE, 0, 2, 3, etc)"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Silicon revision major tag. (IE, A, B, C, etc)"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Silicon revision minor tag. (IE, 0, 2, 3, etc)"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Silicon revision major tag. (IE, A, B, C, etc)"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SILICONREV_ID")
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[doc = "SILICONREV ID\n\nYou can [`read`](crate::Reg::read) this register and get [`siliconrev_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SiliconrevIdSpec;
impl crate::RegisterSpec for SiliconrevIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`siliconrev_id::R`](R) reader structure"]
impl crate::Readable for SiliconrevIdSpec {}
#[doc = "`reset()` method sets SILICONREV_ID to value 0x000b_0000"]
impl crate::Resettable for SiliconrevIdSpec {
    const RESET_VALUE: u32 = 0x000b_0000;
}
