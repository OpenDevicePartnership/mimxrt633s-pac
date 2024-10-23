#[doc = "Register `EPTOGGLE` reader"]
pub type R = crate::R<EptoggleSpec>;
#[doc = "Field `TOGGLE` reader - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub type ToggleR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> ToggleR {
        ToggleR::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPTOGGLE")
            .field("toggle", &self.toggle())
            .finish()
    }
}
#[doc = "USB Endpoint toggle register\n\nYou can [`read`](crate::Reg::read) this register and get [`eptoggle::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EptoggleSpec;
impl crate::RegisterSpec for EptoggleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptoggle::R`](R) reader structure"]
impl crate::Readable for EptoggleSpec {}
#[doc = "`reset()` method sets EPTOGGLE to value 0"]
impl crate::Resettable for EptoggleSpec {
    const RESET_VALUE: u32 = 0;
}
