#[doc = "Register `ENABLESET0` reader"]
pub type R = crate::R<Enableset0Spec>;
#[doc = "Register `ENABLESET0` writer"]
pub type W = crate::W<Enableset0Spec>;
#[doc = "Enable for DMA channel 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ena {
    #[doc = "0: DMAchannel 0 is disabled."]
    Disabled = 0,
    #[doc = "1: DMAchannel 0 is enabled."]
    Enabled = 1,
}
impl From<Ena> for u32 {
    #[inline(always)]
    fn from(variant: Ena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ena {
    type Ux = u32;
}
impl crate::IsEnum for Ena {}
#[doc = "Field `ENA` reader - Enable for DMA channel 0"]
pub type EnaR = crate::FieldReader<Ena>;
impl EnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ena> {
        match self.bits {
            0 => Some(Ena::Disabled),
            1 => Some(Ena::Enabled),
            _ => None,
        }
    }
    #[doc = "DMAchannel 0 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena::Disabled
    }
    #[doc = "DMAchannel 0 is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena::Enabled
    }
}
#[doc = "Field `ENA` writer - Enable for DMA channel 0"]
pub type EnaW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ena>;
impl<'a, REG> EnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "DMAchannel 0 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena::Disabled)
    }
    #[doc = "DMAchannel 0 is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:31 - Enable for DMA channel 0"]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLESET0")
            .field("ena", &self.ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable for DMA channel 0"]
    #[inline(always)]
    pub fn ena(&mut self) -> EnaW<Enableset0Spec> {
        EnaW::new(self, 0)
    }
}
#[doc = "Channel Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`enableset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enableset0Spec;
impl crate::RegisterSpec for Enableset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enableset0::R`](R) reader structure"]
impl crate::Readable for Enableset0Spec {}
#[doc = "`write(|w| ..)` method takes [`enableset0::W`](W) writer structure"]
impl crate::Writable for Enableset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLESET0 to value 0"]
impl crate::Resettable for Enableset0Spec {
    const RESET_VALUE: u32 = 0;
}
