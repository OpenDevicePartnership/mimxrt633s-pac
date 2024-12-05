#[doc = "Register `SETVALID1` writer"]
pub type W = crate::W<Setvalid1Spec>;
#[doc = "SetValid control for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setvalid32 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel 32."]
    Effect = 1,
}
impl From<Setvalid32> for bool {
    #[inline(always)]
    fn from(variant: Setvalid32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID32` writer - SetValid control for DMA channel 32."]
pub type Setvalid32W<'a, REG> = crate::BitWriter<'a, REG, Setvalid32>;
impl<'a, REG> Setvalid32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setvalid32::NoEffect)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel 32."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setvalid32::Effect)
    }
}
#[doc = "Additional SetValid controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Setvalid63_33 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the ValidPending control bit for the relevant DMA channel."]
    Effect = 1,
}
impl From<Setvalid63_33> for u32 {
    #[inline(always)]
    fn from(variant: Setvalid63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setvalid63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Setvalid63_33 {}
#[doc = "Field `SETVALID63_33` writer - Additional SetValid controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Setvalid63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Setvalid63_33>;
impl<'a, REG> Setvalid63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setvalid63_33::NoEffect)
    }
    #[doc = "Sets the ValidPending control bit for the relevant DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setvalid63_33::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Setvalid1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - SetValid control for DMA channel 32."]
    #[inline(always)]
    pub fn setvalid32(&mut self) -> Setvalid32W<Setvalid1Spec> {
        Setvalid32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional SetValid controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn setvalid63_33(&mut self) -> Setvalid63_33W<Setvalid1Spec> {
        Setvalid63_33W::new(self, 1)
    }
}
#[doc = "Set ValidPending control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setvalid1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setvalid1Spec;
impl crate::RegisterSpec for Setvalid1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`setvalid1::W`](W) writer structure"]
impl crate::Writable for Setvalid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETVALID1 to value 0"]
impl crate::Resettable for Setvalid1Spec {
    const RESET_VALUE: u32 = 0;
}
