#[doc = "Register `RNR` reader"]
pub type R = crate::R<RnrSpec>;
#[doc = "Register `RNR` writer"]
pub type W = crate::W<RnrSpec>;
#[doc = "Field `REGION` reader - Region number."]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - Region number."]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNR")
            .field("region", &self.region())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Region number."]
    #[inline(always)]
    pub fn region(&mut self) -> RegionW<RnrSpec> {
        RegionW::new(self, 0)
    }
}
#[doc = "Security Attribution Unit Region Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RnrSpec;
impl crate::RegisterSpec for RnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnr::R`](R) reader structure"]
impl crate::Readable for RnrSpec {}
#[doc = "`write(|w| ..)` method takes [`rnr::W`](W) writer structure"]
impl crate::Writable for RnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNR to value 0"]
impl crate::Resettable for RnrSpec {
    const RESET_VALUE: u32 = 0;
}
