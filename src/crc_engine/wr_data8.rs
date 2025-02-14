#[doc = "Register `WR_DATA8` writer"]
pub type W = crate::W<WrData8Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<WrData8Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "CRC data register, 8-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrData8Spec;
impl crate::RegisterSpec for WrData8Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`wr_data8::W`](W) writer structure"]
impl crate::Writable for WrData8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WR_DATA8 to value 0"]
impl crate::Resettable for WrData8Spec {
    const RESET_VALUE: u8 = 0;
}
