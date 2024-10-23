#[doc = "Register `CORDIC_Z` reader"]
pub type R = crate::R<CordicZSpec>;
#[doc = "Register `CORDIC_Z` writer"]
pub type W = crate::W<CordicZSpec>;
#[doc = "Field `cordic_z` reader - Cordic input z"]
pub type CordicZR = crate::FieldReader<u32>;
#[doc = "Field `cordic_z` writer - Cordic input z"]
pub type CordicZW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cordic input z"]
    #[inline(always)]
    pub fn cordic_z(&self) -> CordicZR {
        CordicZR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_Z")
            .field("cordic_z", &self.cordic_z())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input z"]
    #[inline(always)]
    #[must_use]
    pub fn cordic_z(&mut self) -> CordicZW<CordicZSpec> {
        CordicZW::new(self, 0)
    }
}
#[doc = "Cordic input Z register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CordicZSpec;
impl crate::RegisterSpec for CordicZSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordic_z::R`](R) reader structure"]
impl crate::Readable for CordicZSpec {}
#[doc = "`write(|w| ..)` method takes [`cordic_z::W`](W) writer structure"]
impl crate::Writable for CordicZSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORDIC_Z to value 0"]
impl crate::Resettable for CordicZSpec {
    const RESET_VALUE: u32 = 0;
}
