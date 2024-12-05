#[doc = "Register `INTENCLR1` writer"]
pub type W = crate::W<Intenclr1Spec>;
#[doc = "Field `CLR` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Intenclr1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<Intenclr1Spec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenclr1Spec;
impl crate::RegisterSpec for Intenclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr1::W`](W) writer structure"]
impl crate::Writable for Intenclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR1 to value 0"]
impl crate::Resettable for Intenclr1Spec {
    const RESET_VALUE: u32 = 0;
}
