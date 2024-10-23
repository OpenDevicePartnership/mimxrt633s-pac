#[doc = "Register `SYSTEM_NSTICK_CALIB` reader"]
pub type R = crate::R<SystemNstickCalibSpec>;
#[doc = "Register `SYSTEM_NSTICK_CALIB` writer"]
pub type W = crate::W<SystemNstickCalibSpec>;
#[doc = "Field `SYSTEM_NSTICK_CALIB` reader - Selects the system non-secure tick calibration value of the M33."]
pub type SystemNstickCalibR = crate::FieldReader<u32>;
#[doc = "Field `SYSTEM_NSTICK_CALIB` writer - Selects the system non-secure tick calibration value of the M33."]
pub type SystemNstickCalibW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Selects the system non-secure tick calibration value of the M33."]
    #[inline(always)]
    pub fn system_nstick_calib(&self) -> SystemNstickCalibR {
        SystemNstickCalibR::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_NSTICK_CALIB")
            .field("system_nstick_calib", &self.system_nstick_calib())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25 - Selects the system non-secure tick calibration value of the M33."]
    #[inline(always)]
    #[must_use]
    pub fn system_nstick_calib(&mut self) -> SystemNstickCalibW<SystemNstickCalibSpec> {
        SystemNstickCalibW::new(self, 0)
    }
}
#[doc = "system nstick calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`system_nstick_calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_nstick_calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemNstickCalibSpec;
impl crate::RegisterSpec for SystemNstickCalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_nstick_calib::R`](R) reader structure"]
impl crate::Readable for SystemNstickCalibSpec {}
#[doc = "`write(|w| ..)` method takes [`system_nstick_calib::W`](W) writer structure"]
impl crate::Writable for SystemNstickCalibSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_NSTICK_CALIB to value 0"]
impl crate::Resettable for SystemNstickCalibSpec {
    const RESET_VALUE: u32 = 0;
}
