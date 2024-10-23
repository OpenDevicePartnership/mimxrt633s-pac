#[doc = "Register `ACTIVE1` reader"]
pub type R = crate::R<Active1Spec>;
#[doc = "Active flag for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active32 {
    #[doc = "0: DMAchannel 32 is not active."]
    NotActive = 0,
    #[doc = "1: DMAchannel 32 is active."]
    Active = 1,
}
impl From<Active32> for bool {
    #[inline(always)]
    fn from(variant: Active32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE32` reader - Active flag for DMA channel 32."]
pub type Active32R = crate::BitReader<Active32>;
impl Active32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active32 {
        match self.bits {
            false => Active32::NotActive,
            true => Active32::Active,
        }
    }
    #[doc = "DMAchannel 32 is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Active32::NotActive
    }
    #[doc = "DMAchannel 32 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Active32::Active
    }
}
#[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Active63_33 {
    #[doc = "0: The relevant DMA channel is not active."]
    NotActive = 0,
    #[doc = "1: The relevant DMA channel is active."]
    Active = 1,
}
impl From<Active63_33> for u32 {
    #[inline(always)]
    fn from(variant: Active63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Active63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Active63_33 {}
#[doc = "Field `ACTIVE63_33` reader - Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Active63_33R = crate::FieldReader<Active63_33>;
impl Active63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Active63_33> {
        match self.bits {
            0 => Some(Active63_33::NotActive),
            1 => Some(Active63_33::Active),
            _ => None,
        }
    }
    #[doc = "The relevant DMA channel is not active."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Active63_33::NotActive
    }
    #[doc = "The relevant DMA channel is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Active63_33::Active
    }
}
impl R {
    #[doc = "Bit 0 - Active flag for DMA channel 32."]
    #[inline(always)]
    pub fn active32(&self) -> Active32R {
        Active32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn active63_33(&self) -> Active63_33R {
        Active63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVE1")
            .field("active32", &self.active32())
            .field("active63_33", &self.active63_33())
            .finish()
    }
}
#[doc = "Channel Active status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`active1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Active1Spec;
impl crate::RegisterSpec for Active1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active1::R`](R) reader structure"]
impl crate::Readable for Active1Spec {}
#[doc = "`reset()` method sets ACTIVE1 to value 0"]
impl crate::Resettable for Active1Spec {
    const RESET_VALUE: u32 = 0;
}
