#[doc = "Register `OTP_VERSION` reader"]
pub type R = crate::R<OtpVersionSpec>;
#[doc = "Field `STEP_VER` reader - OTP controller step version"]
pub type StepVerR = crate::FieldReader<u16>;
#[doc = "Field `MINOR_VER` reader - OTP controller minor version"]
pub type MinorVerR = crate::FieldReader;
#[doc = "Field `MAJOR_VER` reader - OTP controller major version"]
pub type MajorVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - OTP controller step version"]
    #[inline(always)]
    pub fn step_ver(&self) -> StepVerR {
        StepVerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - OTP controller minor version"]
    #[inline(always)]
    pub fn minor_ver(&self) -> MinorVerR {
        MinorVerR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - OTP controller major version"]
    #[inline(always)]
    pub fn major_ver(&self) -> MajorVerR {
        MajorVerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_VERSION")
            .field("step_ver", &self.step_ver())
            .field("minor_ver", &self.minor_ver())
            .field("major_ver", &self.major_ver())
            .finish()
    }
}
#[doc = "VERSION ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpVersionSpec;
impl crate::RegisterSpec for OtpVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_version::R`](R) reader structure"]
impl crate::Readable for OtpVersionSpec {}
#[doc = "`reset()` method sets OTP_VERSION to value 0x0800_0000"]
impl crate::Resettable for OtpVersionSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
