#[doc = "Register `BUSY1` reader"]
pub type R = crate::R<Busy1Spec>;
#[doc = "Busy flag for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy32 {
    #[doc = "0: DMAchannel 32 is not busy."]
    NotBusy = 0,
    #[doc = "1: DMAchannel 0 is busy."]
    Busy = 1,
}
impl From<Busy32> for bool {
    #[inline(always)]
    fn from(variant: Busy32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY32` reader - Busy flag for DMA channel 32."]
pub type Busy32R = crate::BitReader<Busy32>;
impl Busy32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy32 {
        match self.bits {
            false => Busy32::NotBusy,
            true => Busy32::Busy,
        }
    }
    #[doc = "DMAchannel 32 is not busy."]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Busy32::NotBusy
    }
    #[doc = "DMAchannel 0 is busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy32::Busy
    }
}
#[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Busy63_33 {
    #[doc = "0: The relevant DMA channel is not busy."]
    NotBusy = 0,
    #[doc = "1: The relevant DMA channel is busy."]
    Busy = 1,
}
impl From<Busy63_33> for u32 {
    #[inline(always)]
    fn from(variant: Busy63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Busy63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Busy63_33 {}
#[doc = "Field `BUSY63_33` reader - Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Busy63_33R = crate::FieldReader<Busy63_33>;
impl Busy63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Busy63_33> {
        match self.bits {
            0 => Some(Busy63_33::NotBusy),
            1 => Some(Busy63_33::Busy),
            _ => None,
        }
    }
    #[doc = "The relevant DMA channel is not busy."]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Busy63_33::NotBusy
    }
    #[doc = "The relevant DMA channel is busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy63_33::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Busy flag for DMA channel 32."]
    #[inline(always)]
    pub fn busy32(&self) -> Busy32R {
        Busy32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn busy63_33(&self) -> Busy63_33R {
        Busy63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSY1")
            .field("busy32", &self.busy32())
            .field("busy63_33", &self.busy63_33())
            .finish()
    }
}
#[doc = "Channel Busy status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busy1Spec;
impl crate::RegisterSpec for Busy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy1::R`](R) reader structure"]
impl crate::Readable for Busy1Spec {}
#[doc = "`reset()` method sets BUSY1 to value 0"]
impl crate::Resettable for Busy1Spec {
    const RESET_VALUE: u32 = 0;
}
