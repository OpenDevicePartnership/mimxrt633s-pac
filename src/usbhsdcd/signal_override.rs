#[doc = "Register `SIGNAL_OVERRIDE` reader"]
pub type R = crate::R<SignalOverrideSpec>;
#[doc = "Register `SIGNAL_OVERRIDE` writer"]
pub type W = crate::W<SignalOverrideSpec>;
#[doc = "Phase Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    NoOverride = 0,
    #[doc = "2: Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PriDetOverride = 2,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Phase Selection"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ps> {
        match self.bits {
            0 => Some(Ps::NoOverride),
            2 => Some(Ps::PriDetOverride),
            _ => None,
        }
    }
    #[doc = "No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    #[inline(always)]
    pub fn is_no_override(&self) -> bool {
        *self == Ps::NoOverride
    }
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    #[inline(always)]
    pub fn is_pri_det_override(&self) -> bool {
        *self == Ps::PriDetOverride
    }
}
#[doc = "Field `PS` writer - Phase Selection"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    #[inline(always)]
    pub fn no_override(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::NoOverride)
    }
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    #[inline(always)]
    pub fn pri_det_override(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::PriDetOverride)
    }
}
impl R {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGNAL_OVERRIDE")
            .field("ps", &self.ps())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<SignalOverrideSpec> {
        PsW::new(self, 0)
    }
}
#[doc = "Signal Override Register\n\nYou can [`read`](crate::Reg::read) this register and get [`signal_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`signal_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SignalOverrideSpec;
impl crate::RegisterSpec for SignalOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`signal_override::R`](R) reader structure"]
impl crate::Readable for SignalOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`signal_override::W`](W) writer structure"]
impl crate::Writable for SignalOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGNAL_OVERRIDE to value 0"]
impl crate::Resettable for SignalOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
