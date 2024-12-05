#[doc = "Register `FIFOWR48H` writer"]
pub type W = crate::W<Fifowr48hSpec>;
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Fifowr48hSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:23 - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<Fifowr48hSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr48h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifowr48hSpec;
impl crate::RegisterSpec for Fifowr48hSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifowr48h::W`](W) writer structure"]
impl crate::Writable for Fifowr48hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOWR48H to value 0"]
impl crate::Resettable for Fifowr48hSpec {
    const RESET_VALUE: u32 = 0;
}
