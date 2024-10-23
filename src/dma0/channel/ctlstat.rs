#[doc = "Register `CTLSTAT` reader"]
pub type R = crate::R<CtlstatSpec>;
#[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Validpending {
    #[doc = "0: No effect. No effect on DMA operation."]
    NoEffect = 0,
    #[doc = "1: Valid pending."]
    ValidPending = 1,
}
impl From<Validpending> for bool {
    #[inline(always)]
    fn from(variant: Validpending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALIDPENDING` reader - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
pub type ValidpendingR = crate::BitReader<Validpending>;
impl ValidpendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Validpending {
        match self.bits {
            false => Validpending::NoEffect,
            true => Validpending::ValidPending,
        }
    }
    #[doc = "No effect. No effect on DMA operation."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Validpending::NoEffect
    }
    #[doc = "Valid pending."]
    #[inline(always)]
    pub fn is_valid_pending(&self) -> bool {
        *self == Validpending::ValidPending
    }
}
#[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NotTriggered = 0,
    #[doc = "1: Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    Triggered = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            false => Trig::NotTriggered,
            true => Trig::Triggered,
        }
    }
    #[doc = "Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == Trig::NotTriggered
    }
    #[doc = "Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == Trig::Triggered
    }
}
impl R {
    #[doc = "Bit 0 - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub fn validpending(&self) -> ValidpendingR {
        ValidpendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTLSTAT")
            .field("validpending", &self.validpending())
            .field("trig", &self.trig())
            .finish()
    }
}
#[doc = "Control and status register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlstatSpec;
impl crate::RegisterSpec for CtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlstat::R`](R) reader structure"]
impl crate::Readable for CtlstatSpec {}
#[doc = "`reset()` method sets CTLSTAT to value 0"]
impl crate::Resettable for CtlstatSpec {
    const RESET_VALUE: u32 = 0;
}
