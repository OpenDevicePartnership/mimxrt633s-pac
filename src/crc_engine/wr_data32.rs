#[doc = "Register `WR_DATA32` writer"]
pub type W = crate::W<WrData32Spec>;
#[doc = "Field `CRC_WR_DATA` writer - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
pub type CrcWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<WrData32Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub fn crc_wr_data(&mut self) -> CrcWrDataW<WrData32Spec> {
        CrcWrDataW::new(self, 0)
    }
}
#[doc = "CRC data register, 32-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrData32Spec;
impl crate::RegisterSpec for WrData32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wr_data32::W`](W) writer structure"]
impl crate::Writable for WrData32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_DATA32 to value 0"]
impl crate::Resettable for WrData32Spec {
    const RESET_VALUE: u32 = 0;
}
