#[doc = "Register `SCMC` reader"]
pub type R = crate::R<ScmcSpec>;
#[doc = "Field `MONO_CT` reader - Monobit Count"]
pub type MonoCtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Monobit Count"]
    #[inline(always)]
    pub fn mono_ct(&self) -> MonoCtR {
        MonoCtR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMC")
            .field("mono_ct", &self.mono_ct())
            .finish()
    }
}
#[doc = "Statistical Check Monobit Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scmc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmcSpec;
impl crate::RegisterSpec for ScmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmc::R`](R) reader structure"]
impl crate::Readable for ScmcSpec {}
#[doc = "`reset()` method sets SCMC to value 0"]
impl crate::Resettable for ScmcSpec {
    const RESET_VALUE: u32 = 0;
}
