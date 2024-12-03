#[doc = "Register `ERRINT0` reader"]
pub type R = crate::R<Errint0Spec>;
#[doc = "Register `ERRINT0` writer"]
pub type W = crate::W<Errint0Spec>;
#[doc = "Error Interrupt flag for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Err {
    #[doc = "0: The Error Interrupt is not active for DMA channel 0."]
    NotActive = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel 0."]
    Pending = 1,
}
impl From<Err> for u32 {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Err {
    type Ux = u32;
}
impl crate::IsEnum for Err {}
#[doc = "Field `ERR` reader - Error Interrupt flag for DMA channel 0."]
pub type ErrR = crate::FieldReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Err> {
        match self.bits {
            0 => Some(Err::NotActive),
            1 => Some(Err::Pending),
            _ => None,
        }
    }
    #[doc = "The Error Interrupt is not active for DMA channel 0."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Err::NotActive
    }
    #[doc = "The Error Interrupt is pending for DMA channel 0."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Err::Pending
    }
}
#[doc = "Field `ERR` writer - Error Interrupt flag for DMA channel 0."]
pub type ErrW<'a, REG> = crate::FieldWriter<'a, REG, 32, Err>;
impl<'a, REG> ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The Error Interrupt is not active for DMA channel 0."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Err::NotActive)
    }
    #[doc = "The Error Interrupt is pending for DMA channel 0."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Err::Pending)
    }
}
impl R {
    #[doc = "Bits 0:31 - Error Interrupt flag for DMA channel 0."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRINT0").field("err", &self.err()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Error Interrupt flag for DMA channel 0."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<Errint0Spec> {
        ErrW::new(self, 0)
    }
}
#[doc = "Error Interrupt status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`errint0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Errint0Spec;
impl crate::RegisterSpec for Errint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errint0::R`](R) reader structure"]
impl crate::Readable for Errint0Spec {}
#[doc = "`write(|w| ..)` method takes [`errint0::W`](W) writer structure"]
impl crate::Writable for Errint0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRINT0 to value 0"]
impl crate::Resettable for Errint0Spec {
    const RESET_VALUE: u32 = 0;
}
