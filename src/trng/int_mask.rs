#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwErr {
    #[doc = "0: Corresponding interrupt of INT_STATUS is masked."]
    HwErr0 = 0,
    #[doc = "1: Corresponding bit of INT_STATUS is active."]
    HwErr1 = 1,
}
impl From<HwErr> for bool {
    #[inline(always)]
    fn from(variant: HwErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_ERR` reader - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
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
    #[doc = "Corresponding interrupt of INT_STATUS is masked."]
    #[inline(always)]
    pub fn is_hw_err_0(&self) -> bool {
        *self == HwErr::HwErr0
    }
    #[doc = "Corresponding bit of INT_STATUS is active."]
    #[inline(always)]
    pub fn is_hw_err_1(&self) -> bool {
        *self == HwErr::HwErr1
    }
}
#[doc = "Field `HW_ERR` writer - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
pub type HwErrW<'a, REG> = crate::BitWriter<'a, REG, HwErr>;
impl<'a, REG> HwErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding interrupt of INT_STATUS is masked."]
    #[inline(always)]
    pub fn hw_err_0(self) -> &'a mut crate::W<REG> {
        self.variant(HwErr::HwErr0)
    }
    #[doc = "Corresponding bit of INT_STATUS is active."]
    #[inline(always)]
    pub fn hw_err_1(self) -> &'a mut crate::W<REG> {
        self.variant(HwErr::HwErr1)
    }
}
#[doc = "Same behavior as bit 0 above.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntVal {
    #[doc = "0: Same behavior as bit 0 above."]
    EntVal0 = 0,
    #[doc = "1: Same behavior as bit 0 above."]
    EntVal1 = 1,
}
impl From<EntVal> for bool {
    #[inline(always)]
    fn from(variant: EntVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENT_VAL` reader - Same behavior as bit 0 above."]
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
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn is_ent_val_0(&self) -> bool {
        *self == EntVal::EntVal0
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn is_ent_val_1(&self) -> bool {
        *self == EntVal::EntVal1
    }
}
#[doc = "Field `ENT_VAL` writer - Same behavior as bit 0 above."]
pub type EntValW<'a, REG> = crate::BitWriter<'a, REG, EntVal>;
impl<'a, REG> EntValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn ent_val_0(self) -> &'a mut crate::W<REG> {
        self.variant(EntVal::EntVal0)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn ent_val_1(self) -> &'a mut crate::W<REG> {
        self.variant(EntVal::EntVal1)
    }
}
#[doc = "Same behavior as bit 0 above.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrqCtFail {
    #[doc = "0: Same behavior as bit 0 above."]
    FrqCtFail0 = 0,
    #[doc = "1: Same behavior as bit 0 above."]
    FrqCtFail1 = 1,
}
impl From<FrqCtFail> for bool {
    #[inline(always)]
    fn from(variant: FrqCtFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Same behavior as bit 0 above."]
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
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn is_frq_ct_fail_0(&self) -> bool {
        *self == FrqCtFail::FrqCtFail0
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn is_frq_ct_fail_1(&self) -> bool {
        *self == FrqCtFail::FrqCtFail1
    }
}
#[doc = "Field `FRQ_CT_FAIL` writer - Same behavior as bit 0 above."]
pub type FrqCtFailW<'a, REG> = crate::BitWriter<'a, REG, FrqCtFail>;
impl<'a, REG> FrqCtFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn frq_ct_fail_0(self) -> &'a mut crate::W<REG> {
        self.variant(FrqCtFail::FrqCtFail0)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn frq_ct_fail_1(self) -> &'a mut crate::W<REG> {
        self.variant(FrqCtFail::FrqCtFail1)
    }
}
impl R {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    pub fn hw_err(&self) -> HwErrR {
        HwErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn ent_val(&self) -> EntValR {
        EntValR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FrqCtFailR {
        FrqCtFailR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MASK")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    #[must_use]
    pub fn hw_err(&mut self) -> HwErrW<IntMaskSpec> {
        HwErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 above."]
    #[inline(always)]
    #[must_use]
    pub fn ent_val(&mut self) -> EntValW<IntMaskSpec> {
        EntValW::new(self, 1)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 above."]
    #[inline(always)]
    #[must_use]
    pub fn frq_ct_fail(&mut self) -> FrqCtFailW<IntMaskSpec> {
        FrqCtFailW::new(self, 2)
    }
}
#[doc = "Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
