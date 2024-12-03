#[doc = "Register `OTP_SHADOW[%s]` reader"]
pub type R = crate::R<OtpShadowSpec>;
#[doc = "Register `OTP_SHADOW[%s]` writer"]
pub type W = crate::W<OtpShadowSpec>;
#[doc = "Field `shadow` reader - OTP shadow register"]
pub type ShadowR = crate::FieldReader<u32>;
#[doc = "Field `shadow` writer - OTP shadow register"]
pub type ShadowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP shadow register"]
    #[inline(always)]
    pub fn shadow(&self) -> ShadowR {
        ShadowR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_SHADOW")
            .field("shadow", &self.shadow())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP shadow register"]
    #[inline(always)]
    pub fn shadow(&mut self) -> ShadowW<OtpShadowSpec> {
        ShadowW::new(self, 0)
    }
}
#[doc = "OTP shadow register N\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_shadow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_shadow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpShadowSpec;
impl crate::RegisterSpec for OtpShadowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_shadow::R`](R) reader structure"]
impl crate::Readable for OtpShadowSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_shadow::W`](W) writer structure"]
impl crate::Writable for OtpShadowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_SHADOW[%s]
to value 0"]
impl crate::Resettable for OtpShadowSpec {
    const RESET_VALUE: u32 = 0;
}
