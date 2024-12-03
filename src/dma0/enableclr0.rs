#[doc = "Register `ENABLECLR0` writer"]
pub type W = crate::W<Enableclr0Spec>;
#[doc = "Field `CLR` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Enableclr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<Enableclr0Spec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Channel Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enableclr0Spec;
impl crate::RegisterSpec for Enableclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enableclr0::W`](W) writer structure"]
impl crate::Writable for Enableclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLECLR0 to value 0"]
impl crate::Resettable for Enableclr0Spec {
    const RESET_VALUE: u32 = 0;
}
