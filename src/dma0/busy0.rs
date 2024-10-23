#[doc = "Register `BUSY0` reader"]
pub type R = crate::R<Busy0Spec>;
#[doc = "Busy flag for DMA channel 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Bsy {
    #[doc = "0: DMAchannel 0 is not busy."]
    NotBusy = 0,
    #[doc = "1: DMAchannel 0 is busy."]
    Busy = 1,
}
impl From<Bsy> for u32 {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bsy {
    type Ux = u32;
}
impl crate::IsEnum for Bsy {}
#[doc = "Field `BSY` reader - Busy flag for DMA channel 0."]
pub type BsyR = crate::FieldReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bsy> {
        match self.bits {
            0 => Some(Bsy::NotBusy),
            1 => Some(Bsy::Busy),
            _ => None,
        }
    }
    #[doc = "DMAchannel 0 is not busy."]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Bsy::NotBusy
    }
    #[doc = "DMAchannel 0 is busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Bsy::Busy
    }
}
impl R {
    #[doc = "Bits 0:31 - Busy flag for DMA channel 0."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSY0").field("bsy", &self.bsy()).finish()
    }
}
#[doc = "Channel Busy status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busy0Spec;
impl crate::RegisterSpec for Busy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy0::R`](R) reader structure"]
impl crate::Readable for Busy0Spec {}
#[doc = "`reset()` method sets BUSY0 to value 0"]
impl crate::Resettable for Busy0Spec {
    const RESET_VALUE: u32 = 0;
}
