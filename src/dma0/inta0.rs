#[doc = "Register `INTA0` reader"]
pub type R = crate::R<Inta0Spec>;
#[doc = "Register `INTA0` writer"]
pub type W = crate::W<Inta0Spec>;
#[doc = "Interrupt A status for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ia {
    #[doc = "0: The DMAchannel 0 interrupt A is not active."]
    NotActive = 0,
    #[doc = "1: The DMAchannel 0 interrupt A is active."]
    Active = 1,
}
impl From<Ia> for u32 {
    #[inline(always)]
    fn from(variant: Ia) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ia {
    type Ux = u32;
}
impl crate::IsEnum for Ia {}
#[doc = "Field `IA` reader - Interrupt A status for DMA channel 0."]
pub type IaR = crate::FieldReader<Ia>;
impl IaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ia> {
        match self.bits {
            0 => Some(Ia::NotActive),
            1 => Some(Ia::Active),
            _ => None,
        }
    }
    #[doc = "The DMAchannel 0 interrupt A is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Ia::NotActive
    }
    #[doc = "The DMAchannel 0 interrupt A is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ia::Active
    }
}
#[doc = "Field `IA` writer - Interrupt A status for DMA channel 0."]
pub type IaW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ia>;
impl<'a, REG> IaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The DMAchannel 0 interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ia::NotActive)
    }
    #[doc = "The DMAchannel 0 interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ia::Active)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel 0."]
    #[inline(always)]
    pub fn ia(&self) -> IaR {
        IaR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTA0").field("ia", &self.ia()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel 0."]
    #[inline(always)]
    pub fn ia(&mut self) -> IaW<Inta0Spec> {
        IaW::new(self, 0)
    }
}
#[doc = "Interrupt A status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`inta0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inta0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inta0Spec;
impl crate::RegisterSpec for Inta0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inta0::R`](R) reader structure"]
impl crate::Readable for Inta0Spec {}
#[doc = "`write(|w| ..)` method takes [`inta0::W`](W) writer structure"]
impl crate::Writable for Inta0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTA0 to value 0"]
impl crate::Resettable for Inta0Spec {
    const RESET_VALUE: u32 = 0;
}
