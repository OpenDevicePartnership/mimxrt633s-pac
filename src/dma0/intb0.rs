#[doc = "Register `INTB0` reader"]
pub type R = crate::R<Intb0Spec>;
#[doc = "Register `INTB0` writer"]
pub type W = crate::W<Intb0Spec>;
#[doc = "Interrupt B status for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ib {
    #[doc = "0: The DMAchannel 0 interrupt B is not active."]
    NotActive = 0,
    #[doc = "1: The DMAchannel 0 interrupt B is active."]
    Active = 1,
}
impl From<Ib> for u32 {
    #[inline(always)]
    fn from(variant: Ib) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ib {
    type Ux = u32;
}
impl crate::IsEnum for Ib {}
#[doc = "Field `IB` reader - Interrupt B status for DMA channel 0."]
pub type IbR = crate::FieldReader<Ib>;
impl IbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ib> {
        match self.bits {
            0 => Some(Ib::NotActive),
            1 => Some(Ib::Active),
            _ => None,
        }
    }
    #[doc = "The DMAchannel 0 interrupt B is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Ib::NotActive
    }
    #[doc = "The DMAchannel 0 interrupt B is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ib::Active
    }
}
#[doc = "Field `IB` writer - Interrupt B status for DMA channel 0."]
pub type IbW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ib>;
impl<'a, REG> IbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The DMAchannel 0 interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ib::NotActive)
    }
    #[doc = "The DMAchannel 0 interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ib::Active)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel 0."]
    #[inline(always)]
    pub fn ib(&self) -> IbR {
        IbR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTB0").field("ib", &self.ib()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt B status for DMA channel 0."]
    #[inline(always)]
    pub fn ib(&mut self) -> IbW<Intb0Spec> {
        IbW::new(self, 0)
    }
}
#[doc = "Interrupt B status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intb0Spec;
impl crate::RegisterSpec for Intb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intb0::R`](R) reader structure"]
impl crate::Readable for Intb0Spec {}
#[doc = "`write(|w| ..)` method takes [`intb0::W`](W) writer structure"]
impl crate::Writable for Intb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTB0 to value 0"]
impl crate::Resettable for Intb0Spec {
    const RESET_VALUE: u32 = 0;
}
