#[doc = "Register `MWDATABE` writer"]
pub type W = crate::W<MwdatabeSpec>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MwdatabeSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MwdatabeSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Master Write Data Byte End Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatabe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdatabeSpec;
impl crate::RegisterSpec for MwdatabeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatabe::W`](W) writer structure"]
impl crate::Writable for MwdatabeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATABE to value 0"]
impl crate::Resettable for MwdatabeSpec {
    const RESET_VALUE: u32 = 0;
}
