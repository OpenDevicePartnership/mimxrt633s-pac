#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activeint {
    #[doc = "0: Not pending. No enabled interrupts are pending."]
    NotPending = 0,
    #[doc = "1: Pending. At least one enabled interrupt is pending."]
    Pending = 1,
}
impl From<Activeint> for bool {
    #[inline(always)]
    fn from(variant: Activeint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVEINT` reader - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
pub type ActiveintR = crate::BitReader<Activeint>;
impl ActiveintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Activeint {
        match self.bits {
            false => Activeint::NotPending,
            true => Activeint::Pending,
        }
    }
    #[doc = "Not pending. No enabled interrupts are pending."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Activeint::NotPending
    }
    #[doc = "Pending. At least one enabled interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Activeint::Pending
    }
}
#[doc = "Summarizes whether any error interrupts are pending.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activeerrint {
    #[doc = "0: Not pending. No error interrupts are pending."]
    NotPending = 0,
    #[doc = "1: Pending. At least one error interrupt is pending."]
    Pending = 1,
}
impl From<Activeerrint> for bool {
    #[inline(always)]
    fn from(variant: Activeerrint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVEERRINT` reader - Summarizes whether any error interrupts are pending."]
pub type ActiveerrintR = crate::BitReader<Activeerrint>;
impl ActiveerrintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Activeerrint {
        match self.bits {
            false => Activeerrint::NotPending,
            true => Activeerrint::Pending,
        }
    }
    #[doc = "Not pending. No error interrupts are pending."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Activeerrint::NotPending
    }
    #[doc = "Pending. At least one error interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Activeerrint::Pending
    }
}
impl R {
    #[doc = "Bit 1 - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub fn activeint(&self) -> ActiveintR {
        ActiveintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub fn activeerrint(&self) -> ActiveerrintR {
        ActiveerrintR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("activeint", &self.activeint())
            .field("activeerrint", &self.activeerrint())
            .finish()
    }
}
#[doc = "Interrupt status.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
