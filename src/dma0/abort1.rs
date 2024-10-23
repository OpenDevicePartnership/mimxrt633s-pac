#[doc = "Register `ABORT1` writer"]
pub type W = crate::W<Abort1Spec>;
#[doc = "Abort control for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abort32 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Aborts DMA operations on channel 32."]
    Effect = 1,
}
impl From<Abort32> for bool {
    #[inline(always)]
    fn from(variant: Abort32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT32` writer - Abort control for DMA channel 32."]
pub type Abort32W<'a, REG> = crate::BitWriter<'a, REG, Abort32>;
impl<'a, REG> Abort32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abort32::NoEffect)
    }
    #[doc = "Aborts DMA operations on channel 32."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abort32::Effect)
    }
}
#[doc = "Additional Abort controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Abort63_33 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Aborts DMA operations on the relevant channel."]
    Effect = 1,
}
impl From<Abort63_33> for u32 {
    #[inline(always)]
    fn from(variant: Abort63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abort63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Abort63_33 {}
#[doc = "Field `ABORT63_33` writer - Additional Abort controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Abort63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Abort63_33>;
impl<'a, REG> Abort63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abort63_33::NoEffect)
    }
    #[doc = "Aborts DMA operations on the relevant channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Abort63_33::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Abort1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Abort control for DMA channel 32."]
    #[inline(always)]
    #[must_use]
    pub fn abort32(&mut self) -> Abort32W<Abort1Spec> {
        Abort32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional Abort controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn abort63_33(&mut self) -> Abort63_33W<Abort1Spec> {
        Abort63_33W::new(self, 1)
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Abort1Spec;
impl crate::RegisterSpec for Abort1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abort1::W`](W) writer structure"]
impl crate::Writable for Abort1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT1 to value 0"]
impl crate::Resettable for Abort1Spec {
    const RESET_VALUE: u32 = 0;
}
