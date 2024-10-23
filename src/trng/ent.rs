#[doc = "Register `ENT[%s]` reader"]
pub type R = crate::R<EntSpec>;
#[doc = "Field `ENT` reader - Entropy Value"]
pub type EntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Entropy Value"]
    #[inline(always)]
    pub fn ent(&self) -> EntR {
        EntR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENT").field("ent", &self.ent()).finish()
    }
}
#[doc = "Entropy Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ent::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntSpec;
impl crate::RegisterSpec for EntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ent::R`](R) reader structure"]
impl crate::Readable for EntSpec {}
#[doc = "`reset()` method sets ENT[%s]
to value 0"]
impl crate::Resettable for EntSpec {
    const RESET_VALUE: u32 = 0;
}
