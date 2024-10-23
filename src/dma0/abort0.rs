#[doc = "Register `ABORT0` writer"]
pub type W = crate::W<Abort0Spec>;
#[doc = "Abort control for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Abortctrl {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Aborts DMA operations on channel 0."]
    Effect = 1,
}
impl From<Abortctrl> for u32 {
    #[inline(always)]
    fn from(variant: Abortctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abortctrl {
    type Ux = u32;
}
impl crate::IsEnum for Abortctrl {}
#[doc = "Field `ABORTCTRL` writer - Abort control for DMA channel 0."]
pub type AbortctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, Abortctrl>;
impl<'a, REG> AbortctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abortctrl::NoEffect)
    }
    #[doc = "Aborts DMA operations on channel 0."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abortctrl::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Abort0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Abort control for DMA channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn abortctrl(&mut self) -> AbortctrlW<Abort0Spec> {
        AbortctrlW::new(self, 0)
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Abort0Spec;
impl crate::RegisterSpec for Abort0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abort0::W`](W) writer structure"]
impl crate::Writable for Abort0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for Abort0Spec {
    const RESET_VALUE: u32 = 0;
}
