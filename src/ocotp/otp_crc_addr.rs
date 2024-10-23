#[doc = "Register `OTP_CRC_ADDR` reader"]
pub type R = crate::R<OtpCrcAddrSpec>;
#[doc = "Register `OTP_CRC_ADDR` writer"]
pub type W = crate::W<OtpCrcAddrSpec>;
#[doc = "Field `CRC_START_ADDR` reader - CRC starting fuse word address"]
pub type CrcStartAddrR = crate::FieldReader<u16>;
#[doc = "Field `CRC_START_ADDR` writer - CRC starting fuse word address"]
pub type CrcStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CRC_END_ADDR` reader - CRC ending fuse word address"]
pub type CrcEndAddrR = crate::FieldReader<u16>;
#[doc = "Field `CRC_END_ADDR` writer - CRC ending fuse word address"]
pub type CrcEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CRC_REF_ADDR` reader - Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
pub type CrcRefAddrR = crate::FieldReader;
#[doc = "Field `CRC_REF_ADDR` writer - Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
pub type CrcRefAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:8 - CRC starting fuse word address"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CrcStartAddrR {
        CrcStartAddrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - CRC ending fuse word address"]
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CrcEndAddrR {
        CrcEndAddrR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:26 - Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
    #[inline(always)]
    pub fn crc_ref_addr(&self) -> CrcRefAddrR {
        CrcRefAddrR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CRC_ADDR")
            .field("crc_start_addr", &self.crc_start_addr())
            .field("crc_end_addr", &self.crc_end_addr())
            .field("crc_ref_addr", &self.crc_ref_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - CRC starting fuse word address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_start_addr(&mut self) -> CrcStartAddrW<OtpCrcAddrSpec> {
        CrcStartAddrW::new(self, 0)
    }
    #[doc = "Bits 12:20 - CRC ending fuse word address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_end_addr(&mut self) -> CrcEndAddrW<OtpCrcAddrSpec> {
        CrcEndAddrW::new(self, 12)
    }
    #[doc = "Bits 24:26 - Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ref_addr(&mut self) -> CrcRefAddrW<OtpCrcAddrSpec> {
        CrcRefAddrW::new(self, 24)
    }
}
#[doc = "CRC address range register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_crc_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_crc_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpCrcAddrSpec;
impl crate::RegisterSpec for OtpCrcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_crc_addr::R`](R) reader structure"]
impl crate::Readable for OtpCrcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_crc_addr::W`](W) writer structure"]
impl crate::Writable for OtpCrcAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_CRC_ADDR to value 0"]
impl crate::Resettable for OtpCrcAddrSpec {
    const RESET_VALUE: u32 = 0;
}
