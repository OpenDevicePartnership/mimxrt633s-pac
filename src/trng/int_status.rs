#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Read: Error status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwErr {
    #[doc = "0: no error"]
    HwErr0 = 0,
    #[doc = "1: error detected."]
    HwErr1 = 1,
}
impl From<HwErr> for bool {
    #[inline(always)]
    fn from(variant: HwErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_ERR` reader - Read: Error status"]
pub type HwErrR = crate::BitReader<HwErr>;
impl HwErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwErr {
        match self.bits {
            false => HwErr::HwErr0,
            true => HwErr::HwErr1,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_hw_err_0(&self) -> bool {
        *self == HwErr::HwErr0
    }
    #[doc = "error detected."]
    #[inline(always)]
    pub fn is_hw_err_1(&self) -> bool {
        *self == HwErr::HwErr1
    }
}
#[doc = "Read only: Entropy Valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntVal {
    #[doc = "0: Busy generation entropy. Any value read is invalid."]
    EntVal0 = 0,
    #[doc = "1: TRNG can be stopped and entropy is valid if read."]
    EntVal1 = 1,
}
impl From<EntVal> for bool {
    #[inline(always)]
    fn from(variant: EntVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub type EntValR = crate::BitReader<EntVal>;
impl EntValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EntVal {
        match self.bits {
            false => EntVal::EntVal0,
            true => EntVal::EntVal1,
        }
    }
    #[doc = "Busy generation entropy. Any value read is invalid."]
    #[inline(always)]
    pub fn is_ent_val_0(&self) -> bool {
        *self == EntVal::EntVal0
    }
    #[doc = "TRNG can be stopped and entropy is valid if read."]
    #[inline(always)]
    pub fn is_ent_val_1(&self) -> bool {
        *self == EntVal::EntVal1
    }
}
#[doc = "Read only: Frequency Count Fail\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrqCtFail {
    #[doc = "0: No hardware nor self test frequency errors."]
    FrqCtFail0 = 0,
    #[doc = "1: The frequency counter has detected a failure."]
    FrqCtFail1 = 1,
}
impl From<FrqCtFail> for bool {
    #[inline(always)]
    fn from(variant: FrqCtFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Read only: Frequency Count Fail"]
pub type FrqCtFailR = crate::BitReader<FrqCtFail>;
impl FrqCtFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrqCtFail {
        match self.bits {
            false => FrqCtFail::FrqCtFail0,
            true => FrqCtFail::FrqCtFail1,
        }
    }
    #[doc = "No hardware nor self test frequency errors."]
    #[inline(always)]
    pub fn is_frq_ct_fail_0(&self) -> bool {
        *self == FrqCtFail::FrqCtFail0
    }
    #[doc = "The frequency counter has detected a failure."]
    #[inline(always)]
    pub fn is_frq_ct_fail_1(&self) -> bool {
        *self == FrqCtFail::FrqCtFail1
    }
}
impl R {
    #[doc = "Bit 0 - Read: Error status"]
    #[inline(always)]
    pub fn hw_err(&self) -> HwErrR {
        HwErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> EntValR {
        EntValR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FrqCtFailR {
        FrqCtFailR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .finish()
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
