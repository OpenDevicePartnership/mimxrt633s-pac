#[doc = "Register `HWVADLOWZ` reader"]
pub type R = crate::R<HwvadlowzSpec>;
#[doc = "Field `LOWZ` reader - Average noise-floor value"]
pub type LowzR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Average noise-floor value"]
    #[inline(always)]
    pub fn lowz(&self) -> LowzR {
        LowzR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADLOWZ")
            .field("lowz", &self.lowz())
            .finish()
    }
}
#[doc = "HWVAD noise envelope estimator register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadlowz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadlowzSpec;
impl crate::RegisterSpec for HwvadlowzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadlowz::R`](R) reader structure"]
impl crate::Readable for HwvadlowzSpec {}
#[doc = "`reset()` method sets HWVADLOWZ to value 0"]
impl crate::Resettable for HwvadlowzSpec {
    const RESET_VALUE: u32 = 0;
}
