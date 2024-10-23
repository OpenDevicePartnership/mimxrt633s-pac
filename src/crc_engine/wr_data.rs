#[doc = "Register `WR_DATA` writer"]
pub type W = crate::W<WrDataSpec>;
#[doc = "Field `CRC_WR_DATA` writer - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
pub type CrcWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<WrDataSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    #[must_use]
    pub fn crc_wr_data(&mut self) -> CrcWrDataW<WrDataSpec> {
        CrcWrDataW::new(self, 0)
    }
}
#[doc = "CRC data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrDataSpec;
impl crate::RegisterSpec for WrDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wr_data::W`](W) writer structure"]
impl crate::Writable for WrDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_DATA to value 0"]
impl crate::Resettable for WrDataSpec {
    const RESET_VALUE: u32 = 0;
}
