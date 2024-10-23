#[doc = "Register `SUM` reader"]
pub type R = crate::R<SumSpec>;
#[doc = "Field `CRC_SUM` reader - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
pub type CrcSumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub fn crc_sum(&self) -> CrcSumR {
        CrcSumR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUM")
            .field("crc_sum", &self.crc_sum())
            .finish()
    }
}
#[doc = "CRC checksum register\n\nYou can [`read`](crate::Reg::read) this register and get [`sum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SumSpec;
impl crate::RegisterSpec for SumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum::R`](R) reader structure"]
impl crate::Readable for SumSpec {}
#[doc = "`reset()` method sets SUM to value 0xffff"]
impl crate::Resettable for SumSpec {
    const RESET_VALUE: u32 = 0xffff;
}
