#[doc = "Register `HCCPARAMS` reader"]
pub type R = crate::R<HccparamsSpec>;
#[doc = "Field `LPMC` reader - Link Power Management Capability."]
pub type LpmcR = crate::BitReader;
impl R {
    #[doc = "Bit 17 - Link Power Management Capability."]
    #[inline(always)]
    pub fn lpmc(&self) -> LpmcR {
        LpmcR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCPARAMS")
            .field("lpmc", &self.lpmc())
            .finish()
    }
}
#[doc = "Host Controller Capability Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccparamsSpec;
impl crate::RegisterSpec for HccparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccparams::R`](R) reader structure"]
impl crate::Readable for HccparamsSpec {}
#[doc = "`reset()` method sets HCCPARAMS to value 0x0002_0006"]
impl crate::Resettable for HccparamsSpec {
    const RESET_VALUE: u32 = 0x0002_0006;
}
