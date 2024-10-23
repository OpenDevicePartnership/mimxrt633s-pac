#[doc = "Register `FIFORD48HNOPOP` reader"]
pub type R = crate::R<Fiford48hnopopSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
pub type RxdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORD48HNOPOP")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford48hnopop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiford48hnopopSpec;
impl crate::RegisterSpec for Fiford48hnopopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiford48hnopop::R`](R) reader structure"]
impl crate::Readable for Fiford48hnopopSpec {}
#[doc = "`reset()` method sets FIFORD48HNOPOP to value 0"]
impl crate::Resettable for Fiford48hnopopSpec {
    const RESET_VALUE: u32 = 0;
}
