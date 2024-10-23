#[doc = "Register `TFDR[%s]` writer"]
pub type W = crate::W<TfdrSpec>;
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TfdrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TfdrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "IP TX FIFO Data Register x\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfdrSpec;
impl crate::RegisterSpec for TfdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tfdr::W`](W) writer structure"]
impl crate::Writable for TfdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFDR[%s]
to value 0"]
impl crate::Resettable for TfdrSpec {
    const RESET_VALUE: u32 = 0;
}
