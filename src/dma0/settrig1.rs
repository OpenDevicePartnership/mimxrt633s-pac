#[doc = "Register `SETTRIG1` writer"]
pub type W = crate::W<Settrig1Spec>;
#[doc = "Set Trigger control bit for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settrig32 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the Trig bit for DMA channel 32."]
    Effect = 1,
}
impl From<Settrig32> for bool {
    #[inline(always)]
    fn from(variant: Settrig32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG32` writer - Set Trigger control bit for DMA channel 32."]
pub type Settrig32W<'a, REG> = crate::BitWriter<'a, REG, Settrig32>;
impl<'a, REG> Settrig32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Settrig32::NoEffect)
    }
    #[doc = "Sets the Trig bit for DMA channel 32."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Settrig32::Effect)
    }
}
#[doc = "Additional Set Trigger control bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Settrig63_33 {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Sets the Trig bit for DMA channel for the relevant DMA channel."]
    Effect = 1,
}
impl From<Settrig63_33> for u32 {
    #[inline(always)]
    fn from(variant: Settrig63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Settrig63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Settrig63_33 {}
#[doc = "Field `SETTRIG63_33` writer - Additional Set Trigger control bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Settrig63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Settrig63_33>;
impl<'a, REG> Settrig63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Settrig63_33::NoEffect)
    }
    #[doc = "Sets the Trig bit for DMA channel for the relevant DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut crate::W<REG> {
        self.variant(Settrig63_33::Effect)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Settrig1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set Trigger control bit for DMA channel 32."]
    #[inline(always)]
    pub fn settrig32(&mut self) -> Settrig32W<Settrig1Spec> {
        Settrig32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional Set Trigger control bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn settrig63_33(&mut self) -> Settrig63_33W<Settrig1Spec> {
        Settrig63_33W::new(self, 1)
    }
}
#[doc = "Set Trigger control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`settrig1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Settrig1Spec;
impl crate::RegisterSpec for Settrig1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`settrig1::W`](W) writer structure"]
impl crate::Writable for Settrig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETTRIG1 to value 0"]
impl crate::Resettable for Settrig1Spec {
    const RESET_VALUE: u32 = 0;
}
