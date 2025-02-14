#[doc = "Register `WR_DATA16` writer"]
pub type W = crate::W<WrData16Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<WrData16Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CRC data register, 16-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrData16Spec;
impl crate::RegisterSpec for WrData16Spec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`wr_data16::W`](W) writer structure"]
impl crate::Writable for WrData16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WR_DATA16 to value 0"]
impl crate::Resettable for WrData16Spec {
    const RESET_VALUE: u16 = 0;
}
