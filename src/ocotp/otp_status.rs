#[doc = "Register `OTP_STATUS` reader"]
pub type R = crate::R<OtpStatusSpec>;
#[doc = "Register `OTP_STATUS` writer"]
pub type W = crate::W<OtpStatusSpec>;
#[doc = "Field `SEC` reader - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
pub type SecR = crate::BitReader;
#[doc = "Field `SEC` writer - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
pub type SecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DED` reader - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
pub type DedR = crate::BitReader;
#[doc = "Field `DED` writer - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
pub type DedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKED` reader - OTP LOCKED status during read/write operation. Write 1 to clear."]
pub type LockedR = crate::BitReader;
#[doc = "Field `LOCKED` writer - OTP LOCKED status during read/write operation. Write 1 to clear."]
pub type LockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGFAIL` reader - OTP PROGFAIL status. Write 1 to clear."]
pub type ProgfailR = crate::BitReader;
#[doc = "Field `PROGFAIL` writer - OTP PROGFAIL status. Write 1 to clear."]
pub type ProgfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - OTP ACK value"]
pub type AckR = crate::BitReader;
#[doc = "Field `PWOK` reader - OTP Power OK status. Indicate that power VDD are in the operating range."]
pub type PwokR = crate::BitReader;
#[doc = "Field `SEC_RELOAD` reader - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
pub type SecReloadR = crate::BitReader;
#[doc = "Field `SEC_RELOAD` writer - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
pub type SecReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DED_RELOAD` reader - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
pub type DedReloadR = crate::BitReader;
#[doc = "Field `DED_RELOAD` writer - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
pub type DedReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - OTP controller status bit. When active, no new write or read access to OTP (including RELOAD_SHADOWS) can be performed. Cleared by the controller when the access completes. After reset (or after setting RELOAD_SHADOWS), this bit is set by the controller and cleared after all the shadow registers are successfully loaded."]
pub type BusyR = crate::BitReader;
#[doc = "Field `ERROR` reader - Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_FAIL` reader - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
pub type CrcFailR = crate::BitReader;
#[doc = "Field `CRC_FAIL` writer - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
pub type CrcFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_LATCHED` reader - Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
pub type FuseLatchedR = crate::BitReader;
impl R {
    #[doc = "Bit 9 - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn ded(&self) -> DedR {
        DedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTP PROGFAIL status. Write 1 to clear."]
    #[inline(always)]
    pub fn progfail(&self) -> ProgfailR {
        ProgfailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OTP ACK value"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OTP Power OK status. Indicate that power VDD are in the operating range."]
    #[inline(always)]
    pub fn pwok(&self) -> PwokR {
        PwokR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn sec_reload(&self) -> SecReloadR {
        SecReloadR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn ded_reload(&self) -> DedReloadR {
        DedReloadR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTP controller status bit. When active, no new write or read access to OTP (including RELOAD_SHADOWS) can be performed. Cleared by the controller when the access completes. After reset (or after setting RELOAD_SHADOWS), this bit is set by the controller and cleared after all the shadow registers are successfully loaded."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[inline(always)]
    pub fn crc_fail(&self) -> CrcFailR {
        CrcFailR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
    #[inline(always)]
    pub fn fuse_latched(&self) -> FuseLatchedR {
        FuseLatchedR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_STATUS")
            .field("sec", &self.sec())
            .field("ded", &self.ded())
            .field("locked", &self.locked())
            .field("progfail", &self.progfail())
            .field("ack", &self.ack())
            .field("pwok", &self.pwok())
            .field("sec_reload", &self.sec_reload())
            .field("ded_reload", &self.ded_reload())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("crc_fail", &self.crc_fail())
            .field("fuse_latched", &self.fuse_latched())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<OtpStatusSpec> {
        SecW::new(self, 9)
    }
    #[doc = "Bit 10 - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn ded(&mut self) -> DedW<OtpStatusSpec> {
        DedW::new(self, 10)
    }
    #[doc = "Bit 11 - OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[inline(always)]
    pub fn locked(&mut self) -> LockedW<OtpStatusSpec> {
        LockedW::new(self, 11)
    }
    #[doc = "Bit 12 - OTP PROGFAIL status. Write 1 to clear."]
    #[inline(always)]
    pub fn progfail(&mut self) -> ProgfailW<OtpStatusSpec> {
        ProgfailW::new(self, 12)
    }
    #[doc = "Bit 20 - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn sec_reload(&mut self) -> SecReloadW<OtpStatusSpec> {
        SecReloadW::new(self, 20)
    }
    #[doc = "Bit 21 - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn ded_reload(&mut self) -> DedReloadW<OtpStatusSpec> {
        DedReloadW::new(self, 21)
    }
    #[doc = "Bit 23 - Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<OtpStatusSpec> {
        ErrorW::new(self, 23)
    }
    #[doc = "Bit 24 - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[inline(always)]
    pub fn crc_fail(&mut self) -> CrcFailW<OtpStatusSpec> {
        CrcFailW::new(self, 24)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpStatusSpec;
impl crate::RegisterSpec for OtpStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_status::R`](R) reader structure"]
impl crate::Readable for OtpStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_status::W`](W) writer structure"]
impl crate::Writable for OtpStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_STATUS to value 0x0200_6000"]
impl crate::Resettable for OtpStatusSpec {
    const RESET_VALUE: u32 = 0x0200_6000;
}
