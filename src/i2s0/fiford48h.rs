#[doc = "Register `FIFORD48H` reader"]
pub type R = crate::R<Fiford48hSpec>;
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
        f.debug_struct("FIFORD48H")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford48h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiford48hSpec;
impl crate::RegisterSpec for Fiford48hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiford48h::R`](R) reader structure"]
impl crate::Readable for Fiford48hSpec {}
#[doc = "`reset()` method sets FIFORD48H to value 0"]
impl crate::Resettable for Fiford48hSpec {
    const RESET_VALUE: u32 = 0;
}
