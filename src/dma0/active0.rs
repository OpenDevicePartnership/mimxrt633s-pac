#[doc = "Register `ACTIVE0` reader"]
pub type R = crate::R<Active0Spec>;
#[doc = "Active flag for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Act {
    #[doc = "0: DMAchannel 0 is not active."]
    NotActive = 0,
    #[doc = "1: DMAchannel 0 is active."]
    Active = 1,
}
impl From<Act> for u32 {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Act {
    type Ux = u32;
}
impl crate::IsEnum for Act {}
#[doc = "Field `ACT` reader - Active flag for DMA channel 0."]
pub type ActR = crate::FieldReader<Act>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Act> {
        match self.bits {
            0 => Some(Act::NotActive),
            1 => Some(Act::Active),
            _ => None,
        }
    }
    #[doc = "DMAchannel 0 is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Act::NotActive
    }
    #[doc = "DMAchannel 0 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Act::Active
    }
}
impl R {
    #[doc = "Bits 0:31 - Active flag for DMA channel 0."]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVE0").field("act", &self.act()).finish()
    }
}
#[doc = "Channel Active status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`active0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Active0Spec;
impl crate::RegisterSpec for Active0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active0::R`](R) reader structure"]
impl crate::Readable for Active0Spec {}
#[doc = "`reset()` method sets ACTIVE0 to value 0"]
impl crate::Resettable for Active0Spec {
    const RESET_VALUE: u32 = 0;
}
