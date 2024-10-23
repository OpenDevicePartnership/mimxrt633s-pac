#[doc = "Register `SWDATABE` writer"]
pub type W = crate::W<SwdatabeSpec>;
#[doc = "Field `DATA` writer - The data byte to send to the master"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SwdatabeSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The data byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<SwdatabeSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Slave Write Data Byte End\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatabe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdatabeSpec;
impl crate::RegisterSpec for SwdatabeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdatabe::W`](W) writer structure"]
impl crate::Writable for SwdatabeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWDATABE to value 0"]
impl crate::Resettable for SwdatabeSpec {
    const RESET_VALUE: u32 = 0;
}
