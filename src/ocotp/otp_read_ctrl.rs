#[doc = "Register `OTP_READ_CTRL` reader"]
pub type R = crate::R<OtpReadCtrlSpec>;
#[doc = "Register `OTP_READ_CTRL` writer"]
pub type W = crate::W<OtpReadCtrlSpec>;
#[doc = "Field `READ` reader - no description available"]
pub type ReadR = crate::BitReader;
#[doc = "Field `READ` writer - no description available"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_READ_CTRL")
            .field("read", &self.read())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<OtpReadCtrlSpec> {
        ReadW::new(self, 0)
    }
}
#[doc = "OTP read start register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_read_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_read_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReadCtrlSpec;
impl crate::RegisterSpec for OtpReadCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_read_ctrl::R`](R) reader structure"]
impl crate::Readable for OtpReadCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_read_ctrl::W`](W) writer structure"]
impl crate::Writable for OtpReadCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_READ_CTRL to value 0"]
impl crate::Resettable for OtpReadCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
