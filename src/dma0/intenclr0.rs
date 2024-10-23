#[doc = "Register `INTENCLR0` writer"]
pub type W = crate::W<Intenclr0Spec>;
#[doc = "Field `CLR` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet0."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Intenclr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears corresponding bits in the DMAIntEnSet0."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<Intenclr0Spec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenclr0Spec;
impl crate::RegisterSpec for Intenclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr0::W`](W) writer structure"]
impl crate::Writable for Intenclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR0 to value 0"]
impl crate::Resettable for Intenclr0Spec {
    const RESET_VALUE: u32 = 0;
}
