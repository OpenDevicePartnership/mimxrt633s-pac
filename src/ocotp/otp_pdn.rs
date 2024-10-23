#[doc = "Register `OTP_PDN` reader"]
pub type R = crate::R<OtpPdnSpec>;
#[doc = "Register `OTP_PDN` writer"]
pub type W = crate::W<OtpPdnSpec>;
#[doc = "Field `PDN` reader - This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
pub type PdnR = crate::BitReader;
#[doc = "Field `PDN` writer - This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
pub type PdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
    #[inline(always)]
    pub fn pdn(&self) -> PdnR {
        PdnR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_PDN").field("pdn", &self.pdn()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
    #[inline(always)]
    #[must_use]
    pub fn pdn(&mut self) -> PdnW<OtpPdnSpec> {
        PdnW::new(self, 0)
    }
}
#[doc = "Power-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_pdn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_pdn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpPdnSpec;
impl crate::RegisterSpec for OtpPdnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_pdn::R`](R) reader structure"]
impl crate::Readable for OtpPdnSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_pdn::W`](W) writer structure"]
impl crate::Writable for OtpPdnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_PDN to value 0x01"]
impl crate::Resettable for OtpPdnSpec {
    const RESET_VALUE: u32 = 0x01;
}
