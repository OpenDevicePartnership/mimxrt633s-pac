#[doc = "Register `SETVALID0` writer"]
pub type W = crate::W<Setvalid0Spec>;
#[doc = "SetValid control for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Sv {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel 0."]
    Effect = 1,
}
impl From<Sv> for u32 {
    #[inline(always)]
    fn from(variant: Sv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sv {
    type Ux = u32;
}
impl crate::IsEnum for Sv {}
#[doc = "Field `SV` writer - SetValid control for DMA channel 0."]
pub type SvW<'a, REG> = crate::FieldWriter<'a, REG, 32, Sv>;
impl<'a, REG> SvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sv::NoEffect)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel 0."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sv::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Setvalid0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - SetValid control for DMA channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn sv(&mut self) -> SvW<Setvalid0Spec> {
        SvW::new(self, 0)
    }
}
#[doc = "Set ValidPending control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setvalid0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setvalid0Spec;
impl crate::RegisterSpec for Setvalid0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`setvalid0::W`](W) writer structure"]
impl crate::Writable for Setvalid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETVALID0 to value 0"]
impl crate::Resettable for Setvalid0Spec {
    const RESET_VALUE: u32 = 0;
}
