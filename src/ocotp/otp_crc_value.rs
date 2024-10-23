#[doc = "Register `OTP_CRC_VALUE` reader"]
pub type R = crate::R<OtpCrcValueSpec>;
#[doc = "Register `OTP_CRC_VALUE` writer"]
pub type W = crate::W<OtpCrcValueSpec>;
#[doc = "Field `CRC_VALUE` reader - The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
pub type CrcValueR = crate::FieldReader<u32>;
#[doc = "Field `CRC_VALUE` writer - The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
pub type CrcValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
    #[inline(always)]
    pub fn crc_value(&self) -> CrcValueR {
        CrcValueR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CRC_VALUE")
            .field("crc_value", &self.crc_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
    #[inline(always)]
    #[must_use]
    pub fn crc_value(&mut self) -> CrcValueW<OtpCrcValueSpec> {
        CrcValueW::new(self, 0)
    }
}
#[doc = "CRC result register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_crc_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_crc_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpCrcValueSpec;
impl crate::RegisterSpec for OtpCrcValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_crc_value::R`](R) reader structure"]
impl crate::Readable for OtpCrcValueSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_crc_value::W`](W) writer structure"]
impl crate::Writable for OtpCrcValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_CRC_VALUE to value 0"]
impl crate::Resettable for OtpCrcValueSpec {
    const RESET_VALUE: u32 = 0;
}
