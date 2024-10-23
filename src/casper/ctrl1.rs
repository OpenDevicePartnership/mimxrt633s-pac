#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `ITER` reader - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
pub type IterR = crate::FieldReader;
#[doc = "Field `ITER` writer - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
pub type IterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODE` reader - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resbpair {
    #[doc = "0: Bank-pair 0 (1st)"]
    Pair0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    Pair1 = 1,
}
impl From<Resbpair> for bool {
    #[inline(always)]
    fn from(variant: Resbpair) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESBPAIR` reader - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
pub type ResbpairR = crate::BitReader<Resbpair>;
impl ResbpairR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resbpair {
        match self.bits {
            false => Resbpair::Pair0,
            true => Resbpair::Pair1,
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == Resbpair::Pair0
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == Resbpair::Pair1
    }
}
#[doc = "Field `RESBPAIR` writer - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
pub type ResbpairW<'a, REG> = crate::BitWriter<'a, REG, Resbpair>;
impl<'a, REG> ResbpairW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut crate::W<REG> {
        self.variant(Resbpair::Pair0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut crate::W<REG> {
        self.variant(Resbpair::Pair1)
    }
}
#[doc = "Field `RESOFF` reader - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
pub type ResoffR = crate::FieldReader<u16>;
#[doc = "Field `RESOFF` writer - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
pub type ResoffW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cskip {
    #[doc = "0: No Skip"]
    NoSkip = 0,
    #[doc = "1: Skip if Carry is 1"]
    SkipIf1 = 1,
    #[doc = "2: Skip if Carry is 0"]
    SkipIf0 = 2,
    #[doc = "3: Set CTRLOFF to CDOFF and Skip"]
    SetAndSkip = 3,
}
impl From<Cskip> for u8 {
    #[inline(always)]
    fn from(variant: Cskip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cskip {
    type Ux = u8;
}
impl crate::IsEnum for Cskip {}
#[doc = "Field `CSKIP` reader - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
pub type CskipR = crate::FieldReader<Cskip>;
impl CskipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cskip {
        match self.bits {
            0 => Cskip::NoSkip,
            1 => Cskip::SkipIf1,
            2 => Cskip::SkipIf0,
            3 => Cskip::SetAndSkip,
            _ => unreachable!(),
        }
    }
    #[doc = "No Skip"]
    #[inline(always)]
    pub fn is_no_skip(&self) -> bool {
        *self == Cskip::NoSkip
    }
    #[doc = "Skip if Carry is 1"]
    #[inline(always)]
    pub fn is_skip_if_1(&self) -> bool {
        *self == Cskip::SkipIf1
    }
    #[doc = "Skip if Carry is 0"]
    #[inline(always)]
    pub fn is_skip_if_0(&self) -> bool {
        *self == Cskip::SkipIf0
    }
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    #[inline(always)]
    pub fn is_set_and_skip(&self) -> bool {
        *self == Cskip::SetAndSkip
    }
}
#[doc = "Field `CSKIP` writer - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
pub type CskipW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cskip, crate::Safe>;
impl<'a, REG> CskipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Skip"]
    #[inline(always)]
    pub fn no_skip(self) -> &'a mut crate::W<REG> {
        self.variant(Cskip::NoSkip)
    }
    #[doc = "Skip if Carry is 1"]
    #[inline(always)]
    pub fn skip_if_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cskip::SkipIf1)
    }
    #[doc = "Skip if Carry is 0"]
    #[inline(always)]
    pub fn skip_if_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cskip::SkipIf0)
    }
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    #[inline(always)]
    pub fn set_and_skip(self) -> &'a mut crate::W<REG> {
        self.variant(Cskip::SetAndSkip)
    }
}
impl R {
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline(always)]
    pub fn resbpair(&self) -> ResbpairR {
        ResbpairR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline(always)]
    pub fn resoff(&self) -> ResoffR {
        ResoffR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline(always)]
    pub fn cskip(&self) -> CskipR {
        CskipR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("iter", &self.iter())
            .field("mode", &self.mode())
            .field("resbpair", &self.resbpair())
            .field("resoff", &self.resoff())
            .field("cskip", &self.cskip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> IterW<Ctrl1Spec> {
        IterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Ctrl1Spec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline(always)]
    #[must_use]
    pub fn resbpair(&mut self) -> ResbpairW<Ctrl1Spec> {
        ResbpairW::new(self, 16)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline(always)]
    #[must_use]
    pub fn resoff(&mut self) -> ResoffW<Ctrl1Spec> {
        ResoffW::new(self, 18)
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline(always)]
    #[must_use]
    pub fn cskip(&mut self) -> CskipW<Ctrl1Spec> {
        CskipW::new(self, 30)
    }
}
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
