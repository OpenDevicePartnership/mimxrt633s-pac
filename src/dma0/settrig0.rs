#[doc = "Register `SETTRIG0` writer"]
pub type W = crate::W<Settrig0Spec>;
#[doc = "Set Trigger control bit for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Trig {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the Trig bit for DMA channel 0."]
    Effect = 1,
}
impl From<Trig> for u32 {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trig {
    type Ux = u32;
}
impl crate::IsEnum for Trig {}
#[doc = "Field `TRIG` writer - Set Trigger control bit for DMA channel 0."]
pub type TrigW<'a, REG> = crate::FieldWriter<'a, REG, 32, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::NoEffect)
    }
    #[doc = "Sets the Trig bit for DMA channel 0."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Settrig0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Set Trigger control bit for DMA channel 0."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<Settrig0Spec> {
        TrigW::new(self, 0)
    }
}
#[doc = "Set Trigger control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`settrig0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Settrig0Spec;
impl crate::RegisterSpec for Settrig0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`settrig0::W`](W) writer structure"]
impl crate::Writable for Settrig0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETTRIG0 to value 0"]
impl crate::Resettable for Settrig0Spec {
    const RESET_VALUE: u32 = 0;
}
