#[doc = "Register `OTP_CTRL` reader"]
pub type R = crate::R<OtpCtrlSpec>;
#[doc = "Register `OTP_CTRL` writer"]
pub type W = crate::W<OtpCtrlSpec>;
#[doc = "Field `ADDR` reader - OTP word address for read/programming"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - OTP word address for read/programming"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RELOAD_SHADOWS` reader - Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
pub type ReloadShadowsR = crate::BitReader;
#[doc = "Field `RELOAD_SHADOWS` writer - Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
pub type ReloadShadowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_TEST` reader - Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
pub type CrcTestR = crate::BitReader;
#[doc = "Field `CRC_TEST` writer - Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
pub type CrcTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORDLOCK` reader - Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
pub type WordlockR = crate::BitReader;
#[doc = "Field `WORDLOCK` writer - Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
pub type WordlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_UNLOCK` reader - Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
pub type WrUnlockR = crate::FieldReader<u16>;
#[doc = "Field `WR_UNLOCK` writer - Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
pub type WrUnlockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:8 - OTP word address for read/programming"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
    #[inline(always)]
    pub fn reload_shadows(&self) -> ReloadShadowsR {
        ReloadShadowsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
    #[inline(always)]
    pub fn crc_test(&self) -> CrcTestR {
        CrcTestR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
    #[inline(always)]
    pub fn wordlock(&self) -> WordlockR {
        WordlockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
    #[inline(always)]
    pub fn wr_unlock(&self) -> WrUnlockR {
        WrUnlockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CTRL")
            .field("addr", &self.addr())
            .field("reload_shadows", &self.reload_shadows())
            .field("crc_test", &self.crc_test())
            .field("wordlock", &self.wordlock())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - OTP word address for read/programming"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<OtpCtrlSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 11 - Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
    #[inline(always)]
    #[must_use]
    pub fn reload_shadows(&mut self) -> ReloadShadowsW<OtpCtrlSpec> {
        ReloadShadowsW::new(self, 11)
    }
    #[doc = "Bit 12 - Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
    #[inline(always)]
    #[must_use]
    pub fn crc_test(&mut self) -> CrcTestW<OtpCtrlSpec> {
        CrcTestW::new(self, 12)
    }
    #[doc = "Bit 15 - Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wordlock(&mut self) -> WordlockW<OtpCtrlSpec> {
        WordlockW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
    #[inline(always)]
    #[must_use]
    pub fn wr_unlock(&mut self) -> WrUnlockW<OtpCtrlSpec> {
        WrUnlockW::new(self, 16)
    }
}
#[doc = "Control/address register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpCtrlSpec;
impl crate::RegisterSpec for OtpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_ctrl::R`](R) reader structure"]
impl crate::Readable for OtpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_ctrl::W`](W) writer structure"]
impl crate::Writable for OtpCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_CTRL to value 0"]
impl crate::Resettable for OtpCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
