#[doc = "Register `SID` reader"]
pub type R = crate::R<SidSpec>;
#[doc = "Field `ID` reader - ID"]
pub type IdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SID").field("id", &self.id()).finish()
    }
}
#[doc = "Slave Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidSpec;
impl crate::RegisterSpec for SidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sid::R`](R) reader structure"]
impl crate::Readable for SidSpec {}
#[doc = "`reset()` method sets SID to value 0xedcb_0000"]
impl crate::Resettable for SidSpec {
    const RESET_VALUE: u32 = 0xedcb_0000;
}
