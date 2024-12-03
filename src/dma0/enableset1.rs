#[doc = "Register `ENABLESET1` reader"]
pub type R = crate::R<Enableset1Spec>;
#[doc = "Register `ENABLESET1` writer"]
pub type W = crate::W<Enableset1Spec>;
#[doc = "Enable for DMA channel 32\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable32 {
    #[doc = "0: DMAchannel 32 is disabled."]
    Disabled = 0,
    #[doc = "1: DMAchannel 32 is enabled."]
    Enabled = 1,
}
impl From<Enable32> for bool {
    #[inline(always)]
    fn from(variant: Enable32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE32` reader - Enable for DMA channel 32"]
pub type Enable32R = crate::BitReader<Enable32>;
impl Enable32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable32 {
        match self.bits {
            false => Enable32::Disabled,
            true => Enable32::Enabled,
        }
    }
    #[doc = "DMAchannel 32 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable32::Disabled
    }
    #[doc = "DMAchannel 32 is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable32::Enabled
    }
}
#[doc = "Field `ENABLE32` writer - Enable for DMA channel 32"]
pub type Enable32W<'a, REG> = crate::BitWriter<'a, REG, Enable32>;
impl<'a, REG> Enable32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAchannel 32 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable32::Disabled)
    }
    #[doc = "DMAchannel 32 is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable32::Enabled)
    }
}
#[doc = "Additional enables for remaining DMA channels in the range 63 to 33.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Enable63_33 {
    #[doc = "0: The relevant DMA channel is disabled."]
    Disabled = 0,
    #[doc = "1: The relevant DMA channel is enabled."]
    Enabled = 1,
}
impl From<Enable63_33> for u32 {
    #[inline(always)]
    fn from(variant: Enable63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enable63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Enable63_33 {}
#[doc = "Field `ENABLE63_33` reader - Additional enables for remaining DMA channels in the range 63 to 33."]
pub type Enable63_33R = crate::FieldReader<Enable63_33>;
impl Enable63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enable63_33> {
        match self.bits {
            0 => Some(Enable63_33::Disabled),
            1 => Some(Enable63_33::Enabled),
            _ => None,
        }
    }
    #[doc = "The relevant DMA channel is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable63_33::Disabled
    }
    #[doc = "The relevant DMA channel is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable63_33::Enabled
    }
}
#[doc = "Field `ENABLE63_33` writer - Additional enables for remaining DMA channels in the range 63 to 33."]
pub type Enable63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Enable63_33>;
impl<'a, REG> Enable63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The relevant DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable63_33::Disabled)
    }
    #[doc = "The relevant DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable63_33::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable for DMA channel 32"]
    #[inline(always)]
    pub fn enable32(&self) -> Enable32R {
        Enable32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional enables for remaining DMA channels in the range 63 to 33."]
    #[inline(always)]
    pub fn enable63_33(&self) -> Enable63_33R {
        Enable63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLESET1")
            .field("enable32", &self.enable32())
            .field("enable63_33", &self.enable63_33())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable for DMA channel 32"]
    #[inline(always)]
    pub fn enable32(&mut self) -> Enable32W<Enableset1Spec> {
        Enable32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional enables for remaining DMA channels in the range 63 to 33."]
    #[inline(always)]
    pub fn enable63_33(&mut self) -> Enable63_33W<Enableset1Spec> {
        Enable63_33W::new(self, 1)
    }
}
#[doc = "Channel Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`enableset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Enableset1Spec;
impl crate::RegisterSpec for Enableset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enableset1::R`](R) reader structure"]
impl crate::Readable for Enableset1Spec {}
#[doc = "`write(|w| ..)` method takes [`enableset1::W`](W) writer structure"]
impl crate::Writable for Enableset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLESET1 to value 0"]
impl crate::Resettable for Enableset1Spec {
    const RESET_VALUE: u32 = 0;
}
