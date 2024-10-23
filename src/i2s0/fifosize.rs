#[doc = "Register `FIFOSIZE` reader"]
pub type R = crate::R<FifosizeSpec>;
#[doc = "Register `FIFOSIZE` writer"]
pub type W = crate::W<FifosizeSpec>;
#[doc = "Field `FIFOSIZE` reader - the fifo size is equal to the template parameter \"fifo\"/2 ."]
pub type FifosizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - the fifo size is equal to the template parameter \"fifo\"/2 ."]
    #[inline(always)]
    pub fn fifosize(&self) -> FifosizeR {
        FifosizeR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSIZE")
            .field("fifosize", &self.fifosize())
            .finish()
    }
}
impl W {}
#[doc = "FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifosize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifosize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifosizeSpec;
impl crate::RegisterSpec for FifosizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifosize::R`](R) reader structure"]
impl crate::Readable for FifosizeSpec {}
#[doc = "`write(|w| ..)` method takes [`fifosize::W`](W) writer structure"]
impl crate::Writable for FifosizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOSIZE to value 0"]
impl crate::Resettable for FifosizeSpec {
    const RESET_VALUE: u32 = 0;
}
