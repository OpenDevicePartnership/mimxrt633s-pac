#[doc = "Register `OTP_READ_DATA` reader"]
pub type R = crate::R<OtpReadDataSpec>;
#[doc = "Field `READ_DATA` reader - Fuse word read data from read operation"]
pub type ReadDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Fuse word read data from read operation"]
    #[inline(always)]
    pub fn read_data(&self) -> ReadDataR {
        ReadDataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_READ_DATA")
            .field("read_data", &self.read_data())
            .finish()
    }
}
#[doc = "OTP read data register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_read_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReadDataSpec;
impl crate::RegisterSpec for OtpReadDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_read_data::R`](R) reader structure"]
impl crate::Readable for OtpReadDataSpec {}
#[doc = "`reset()` method sets OTP_READ_DATA to value 0"]
impl crate::Resettable for OtpReadDataSpec {
    const RESET_VALUE: u32 = 0;
}
