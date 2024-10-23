#[doc = "Register `CAP[%s]` reader"]
pub type R = crate::R<CapSpec>;
#[doc = "Field `CAP_VALUE` reader - Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
pub type CapValueR = crate::FieldReader<u32>;
#[doc = "Field `VALID` reader - Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub fn cap_value(&self) -> CapValueR {
        CapValueR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP")
            .field("cap_value", &self.cap_value())
            .field("valid", &self.valid())
            .finish()
    }
}
#[doc = "Capture register .\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSpec;
impl crate::RegisterSpec for CapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CapSpec {}
#[doc = "`reset()` method sets CAP[%s]
to value 0"]
impl crate::Resettable for CapSpec {
    const RESET_VALUE: u32 = 0;
}
