#[doc = "Register `INTENSET0` reader"]
pub type R = crate::R<Intenset0Spec>;
#[doc = "Register `INTENSET0` writer"]
pub type W = crate::W<Intenset0Spec>;
#[doc = "Interrupt Enable read and set for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Inten {
    #[doc = "0: The Interrupt for DMA channel 0 is disabled."]
    Disabled = 0,
    #[doc = "1: The Interrupt for DMA channel 0 is enabled."]
    Enabled = 1,
}
impl From<Inten> for u32 {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inten {
    type Ux = u32;
}
impl crate::IsEnum for Inten {}
#[doc = "Field `INTEN` reader - Interrupt Enable read and set for DMA channel 0."]
pub type IntenR = crate::FieldReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inten> {
        match self.bits {
            0 => Some(Inten::Disabled),
            1 => Some(Inten::Enabled),
            _ => None,
        }
    }
    #[doc = "The Interrupt for DMA channel 0 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten::Disabled
    }
    #[doc = "The Interrupt for DMA channel 0 is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten::Enabled
    }
}
#[doc = "Field `INTEN` writer - Interrupt Enable read and set for DMA channel 0."]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 32, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The Interrupt for DMA channel 0 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disabled)
    }
    #[doc = "The Interrupt for DMA channel 0 is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel 0."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET0")
            .field("inten", &self.inten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<Intenset0Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenset0Spec;
impl crate::RegisterSpec for Intenset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset0::R`](R) reader structure"]
impl crate::Readable for Intenset0Spec {}
#[doc = "`write(|w| ..)` method takes [`intenset0::W`](W) writer structure"]
impl crate::Writable for Intenset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET0 to value 0"]
impl crate::Resettable for Intenset0Spec {
    const RESET_VALUE: u32 = 0;
}
