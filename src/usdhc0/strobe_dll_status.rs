#[doc = "Register `STROBE_DLL_STATUS` reader"]
pub type R = crate::R<StrobeDllStatusSpec>;
#[doc = "Field `STROBE_DLL_STS_SLV_LOCK` reader - Strobe DLL Status Slave Lock"]
pub type StrobeDllStsSlvLockR = crate::BitReader;
#[doc = "Field `STROBE_DLL_STS_REF_LOCK` reader - Strobe DLL Status Reference Lock"]
pub type StrobeDllStsRefLockR = crate::BitReader;
#[doc = "Field `STROBE_DLL_STS_SLV_SEL` reader - Strobe DLL Status Slave Select"]
pub type StrobeDllStsSlvSelR = crate::FieldReader;
#[doc = "Field `STROBE_DLL_STS_REF_SEL` reader - Strobe DLL Status Reference Select"]
pub type StrobeDllStsRefSelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Strobe DLL Status Slave Lock"]
    #[inline(always)]
    pub fn strobe_dll_sts_slv_lock(&self) -> StrobeDllStsSlvLockR {
        StrobeDllStsSlvLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Strobe DLL Status Reference Lock"]
    #[inline(always)]
    pub fn strobe_dll_sts_ref_lock(&self) -> StrobeDllStsRefLockR {
        StrobeDllStsRefLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Strobe DLL Status Slave Select"]
    #[inline(always)]
    pub fn strobe_dll_sts_slv_sel(&self) -> StrobeDllStsSlvSelR {
        StrobeDllStsSlvSelR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Strobe DLL Status Reference Select"]
    #[inline(always)]
    pub fn strobe_dll_sts_ref_sel(&self) -> StrobeDllStsRefSelR {
        StrobeDllStsRefSelR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STROBE_DLL_STATUS")
            .field("strobe_dll_sts_slv_lock", &self.strobe_dll_sts_slv_lock())
            .field("strobe_dll_sts_ref_lock", &self.strobe_dll_sts_ref_lock())
            .field("strobe_dll_sts_slv_sel", &self.strobe_dll_sts_slv_sel())
            .field("strobe_dll_sts_ref_sel", &self.strobe_dll_sts_ref_sel())
            .finish()
    }
}
#[doc = "Strobe DLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`strobe_dll_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrobeDllStatusSpec;
impl crate::RegisterSpec for StrobeDllStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`strobe_dll_status::R`](R) reader structure"]
impl crate::Readable for StrobeDllStatusSpec {}
#[doc = "`reset()` method sets STROBE_DLL_STATUS to value 0"]
impl crate::Resettable for StrobeDllStatusSpec {
    const RESET_VALUE: u32 = 0;
}
