#[doc = "Register `DLL_STATUS` reader"]
pub type R = crate::R<DllStatusSpec>;
#[doc = "Field `DLL_STS_SLV_LOCK` reader - DLL_STS_SLV_LOCK"]
pub type DllStsSlvLockR = crate::BitReader;
#[doc = "Field `DLL_STS_REF_LOCK` reader - DLL_STS_REF_LOCK"]
pub type DllStsRefLockR = crate::BitReader;
#[doc = "Field `DLL_STS_SLV_SEL` reader - DLL_STS_SLV_SEL"]
pub type DllStsSlvSelR = crate::FieldReader;
#[doc = "Field `DLL_STS_REF_SEL` reader - DLL_STS_REF_SEL"]
pub type DllStsRefSelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DLL_STS_SLV_LOCK"]
    #[inline(always)]
    pub fn dll_sts_slv_lock(&self) -> DllStsSlvLockR {
        DllStsSlvLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL_STS_REF_LOCK"]
    #[inline(always)]
    pub fn dll_sts_ref_lock(&self) -> DllStsRefLockR {
        DllStsRefLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - DLL_STS_SLV_SEL"]
    #[inline(always)]
    pub fn dll_sts_slv_sel(&self) -> DllStsSlvSelR {
        DllStsSlvSelR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - DLL_STS_REF_SEL"]
    #[inline(always)]
    pub fn dll_sts_ref_sel(&self) -> DllStsRefSelR {
        DllStsRefSelR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_STATUS")
            .field("dll_sts_slv_lock", &self.dll_sts_slv_lock())
            .field("dll_sts_ref_lock", &self.dll_sts_ref_lock())
            .field("dll_sts_slv_sel", &self.dll_sts_slv_sel())
            .field("dll_sts_ref_sel", &self.dll_sts_ref_sel())
            .finish()
    }
}
#[doc = "DLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllStatusSpec;
impl crate::RegisterSpec for DllStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_status::R`](R) reader structure"]
impl crate::Readable for DllStatusSpec {}
#[doc = "`reset()` method sets DLL_STATUS to value 0x0200"]
impl crate::Resettable for DllStatusSpec {
    const RESET_VALUE: u32 = 0x0200;
}
