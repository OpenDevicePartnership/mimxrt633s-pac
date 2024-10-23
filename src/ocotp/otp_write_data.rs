#[doc = "Register `OTP_WRITE_DATA` reader"]
pub type R = crate::R<OtpWriteDataSpec>;
#[doc = "Field `WRITE_DATA` reader - Fuse word programming data. After the write operation is unlocked in OTP_CTRL register, writing data to this register automatically start the programming procedure."]
pub type WriteDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Fuse word programming data. After the write operation is unlocked in OTP_CTRL register, writing data to this register automatically start the programming procedure."]
    #[inline(always)]
    pub fn write_data(&self) -> WriteDataR {
        WriteDataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_WRITE_DATA")
            .field("write_data", &self.write_data())
            .finish()
    }
}
#[doc = "OTP programming data register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_write_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpWriteDataSpec;
impl crate::RegisterSpec for OtpWriteDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_write_data::R`](R) reader structure"]
impl crate::Readable for OtpWriteDataSpec {}
#[doc = "`reset()` method sets OTP_WRITE_DATA to value 0"]
impl crate::Resettable for OtpWriteDataSpec {
    const RESET_VALUE: u32 = 0;
}
