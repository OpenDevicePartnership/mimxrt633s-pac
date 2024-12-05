#[doc = "Register `CMDH%s` reader"]
pub type R = crate::R<CmdhSpec>;
#[doc = "Register `CMDH%s` writer"]
pub type W = crate::W<CmdhSpec>;
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpen {
    #[doc = "0: Compare disabled."]
    Cmpen0 = 0,
    #[doc = "2: Compare enabled. Store on true."]
    Cmpen2 = 2,
    #[doc = "3: Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    Cmpen3 = 3,
}
impl From<Cmpen> for u8 {
    #[inline(always)]
    fn from(variant: Cmpen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpen {
    type Ux = u8;
}
impl crate::IsEnum for Cmpen {}
#[doc = "Field `CMPEN` reader - Compare Function Enable"]
pub type CmpenR = crate::FieldReader<Cmpen>;
impl CmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmpen> {
        match self.bits {
            0 => Some(Cmpen::Cmpen0),
            2 => Some(Cmpen::Cmpen2),
            3 => Some(Cmpen::Cmpen3),
            _ => None,
        }
    }
    #[doc = "Compare disabled."]
    #[inline(always)]
    pub fn is_cmpen_0(&self) -> bool {
        *self == Cmpen::Cmpen0
    }
    #[doc = "Compare enabled. Store on true."]
    #[inline(always)]
    pub fn is_cmpen_2(&self) -> bool {
        *self == Cmpen::Cmpen2
    }
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    #[inline(always)]
    pub fn is_cmpen_3(&self) -> bool {
        *self == Cmpen::Cmpen3
    }
}
#[doc = "Field `CMPEN` writer - Compare Function Enable"]
pub type CmpenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpen>;
impl<'a, REG> CmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare disabled."]
    #[inline(always)]
    pub fn cmpen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Cmpen0)
    }
    #[doc = "Compare enabled. Store on true."]
    #[inline(always)]
    pub fn cmpen_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Cmpen2)
    }
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    #[inline(always)]
    pub fn cmpen_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpen::Cmpen3)
    }
}
#[doc = "Loop with Increment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lwi {
    #[doc = "0: Auto channel increment disabled"]
    Lwi0 = 0,
    #[doc = "1: Auto channel increment enabled"]
    Lwi1 = 1,
}
impl From<Lwi> for bool {
    #[inline(always)]
    fn from(variant: Lwi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LWI` reader - Loop with Increment"]
pub type LwiR = crate::BitReader<Lwi>;
impl LwiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lwi {
        match self.bits {
            false => Lwi::Lwi0,
            true => Lwi::Lwi1,
        }
    }
    #[doc = "Auto channel increment disabled"]
    #[inline(always)]
    pub fn is_lwi_0(&self) -> bool {
        *self == Lwi::Lwi0
    }
    #[doc = "Auto channel increment enabled"]
    #[inline(always)]
    pub fn is_lwi_1(&self) -> bool {
        *self == Lwi::Lwi1
    }
}
#[doc = "Field `LWI` writer - Loop with Increment"]
pub type LwiW<'a, REG> = crate::BitWriter<'a, REG, Lwi>;
impl<'a, REG> LwiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto channel increment disabled"]
    #[inline(always)]
    pub fn lwi_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lwi::Lwi0)
    }
    #[doc = "Auto channel increment enabled"]
    #[inline(always)]
    pub fn lwi_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lwi::Lwi1)
    }
}
#[doc = "Sample Time Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sts {
    #[doc = "0: Minimum sample time of 3 ADCK cycles."]
    Sts0 = 0,
    #[doc = "1: 3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    Sts1 = 1,
    #[doc = "2: 3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    Sts2 = 2,
    #[doc = "3: 3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    Sts3 = 3,
    #[doc = "4: 3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    Sts4 = 4,
    #[doc = "5: 3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    Sts5 = 5,
    #[doc = "6: 3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    Sts6 = 6,
    #[doc = "7: 3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    Sts7 = 7,
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(variant: Sts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sts {
    type Ux = u8;
}
impl crate::IsEnum for Sts {}
#[doc = "Field `STS` reader - Sample Time Select"]
pub type StsR = crate::FieldReader<Sts>;
impl StsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sts {
        match self.bits {
            0 => Sts::Sts0,
            1 => Sts::Sts1,
            2 => Sts::Sts2,
            3 => Sts::Sts3,
            4 => Sts::Sts4,
            5 => Sts::Sts5,
            6 => Sts::Sts6,
            7 => Sts::Sts7,
            _ => unreachable!(),
        }
    }
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn is_sts_0(&self) -> bool {
        *self == Sts::Sts0
    }
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_1(&self) -> bool {
        *self == Sts::Sts1
    }
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_2(&self) -> bool {
        *self == Sts::Sts2
    }
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_3(&self) -> bool {
        *self == Sts::Sts3
    }
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_4(&self) -> bool {
        *self == Sts::Sts4
    }
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_5(&self) -> bool {
        *self == Sts::Sts5
    }
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_6(&self) -> bool {
        *self == Sts::Sts6
    }
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_sts_7(&self) -> bool {
        *self == Sts::Sts7
    }
}
#[doc = "Field `STS` writer - Sample Time Select"]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sts, crate::Safe>;
impl<'a, REG> StsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn sts_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts0)
    }
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts1)
    }
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts2)
    }
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts3)
    }
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts4)
    }
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts5)
    }
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts6)
    }
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sts::Sts7)
    }
}
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avgs {
    #[doc = "0: Single conversion."]
    Avgs0 = 0,
    #[doc = "1: 2 conversions averaged."]
    Avgs1 = 1,
    #[doc = "2: 4 conversions averaged."]
    Avgs2 = 2,
    #[doc = "3: 8 conversions averaged."]
    Avgs3 = 3,
    #[doc = "4: 16 conversions averaged."]
    Avgs4 = 4,
    #[doc = "5: 32 conversions averaged."]
    Avgs5 = 5,
    #[doc = "6: 64 conversions averaged."]
    Avgs6 = 6,
    #[doc = "7: 128 conversions averaged."]
    Avgs7 = 7,
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(variant: Avgs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avgs {
    type Ux = u8;
}
impl crate::IsEnum for Avgs {}
#[doc = "Field `AVGS` reader - Hardware Average Select"]
pub type AvgsR = crate::FieldReader<Avgs>;
impl AvgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avgs {
        match self.bits {
            0 => Avgs::Avgs0,
            1 => Avgs::Avgs1,
            2 => Avgs::Avgs2,
            3 => Avgs::Avgs3,
            4 => Avgs::Avgs4,
            5 => Avgs::Avgs5,
            6 => Avgs::Avgs6,
            7 => Avgs::Avgs7,
            _ => unreachable!(),
        }
    }
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn is_avgs_0(&self) -> bool {
        *self == Avgs::Avgs0
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_1(&self) -> bool {
        *self == Avgs::Avgs1
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_2(&self) -> bool {
        *self == Avgs::Avgs2
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_3(&self) -> bool {
        *self == Avgs::Avgs3
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_4(&self) -> bool {
        *self == Avgs::Avgs4
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_5(&self) -> bool {
        *self == Avgs::Avgs5
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_6(&self) -> bool {
        *self == Avgs::Avgs6
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn is_avgs_7(&self) -> bool {
        *self == Avgs::Avgs7
    }
}
#[doc = "Field `AVGS` writer - Hardware Average Select"]
pub type AvgsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Avgs, crate::Safe>;
impl<'a, REG> AvgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn avgs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs0)
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn avgs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs1)
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn avgs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs2)
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn avgs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs3)
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn avgs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs4)
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn avgs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs5)
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn avgs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs6)
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn avgs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::Avgs7)
    }
}
#[doc = "Loop Count Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loop {
    #[doc = "0: Looping not enabled. Command executes 1 time."]
    Loop0 = 0,
    #[doc = "1: Loop 1 time. Command executes 2 times."]
    Loop1 = 1,
    #[doc = "2: Loop 2 times. Command executes 3 times."]
    Loop2 = 2,
    #[doc = "3: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop3 = 3,
    #[doc = "4: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop4 = 4,
    #[doc = "5: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop5 = 5,
    #[doc = "6: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop6 = 6,
    #[doc = "7: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop7 = 7,
    #[doc = "8: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop8 = 8,
    #[doc = "9: Loop corresponding number of times. Command executes LOOP+1 times."]
    Loop9 = 9,
    #[doc = "15: Loop 15 times. Command executes 16 times."]
    Loop15 = 15,
}
impl From<Loop> for u8 {
    #[inline(always)]
    fn from(variant: Loop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loop {
    type Ux = u8;
}
impl crate::IsEnum for Loop {}
#[doc = "Field `LOOP` reader - Loop Count Select"]
pub type LoopR = crate::FieldReader<Loop>;
impl LoopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Loop> {
        match self.bits {
            0 => Some(Loop::Loop0),
            1 => Some(Loop::Loop1),
            2 => Some(Loop::Loop2),
            3 => Some(Loop::Loop3),
            4 => Some(Loop::Loop4),
            5 => Some(Loop::Loop5),
            6 => Some(Loop::Loop6),
            7 => Some(Loop::Loop7),
            8 => Some(Loop::Loop8),
            9 => Some(Loop::Loop9),
            15 => Some(Loop::Loop15),
            _ => None,
        }
    }
    #[doc = "Looping not enabled. Command executes 1 time."]
    #[inline(always)]
    pub fn is_loop_0(&self) -> bool {
        *self == Loop::Loop0
    }
    #[doc = "Loop 1 time. Command executes 2 times."]
    #[inline(always)]
    pub fn is_loop_1(&self) -> bool {
        *self == Loop::Loop1
    }
    #[doc = "Loop 2 times. Command executes 3 times."]
    #[inline(always)]
    pub fn is_loop_2(&self) -> bool {
        *self == Loop::Loop2
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_3(&self) -> bool {
        *self == Loop::Loop3
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_4(&self) -> bool {
        *self == Loop::Loop4
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_5(&self) -> bool {
        *self == Loop::Loop5
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_6(&self) -> bool {
        *self == Loop::Loop6
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_7(&self) -> bool {
        *self == Loop::Loop7
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_8(&self) -> bool {
        *self == Loop::Loop8
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn is_loop_9(&self) -> bool {
        *self == Loop::Loop9
    }
    #[doc = "Loop 15 times. Command executes 16 times."]
    #[inline(always)]
    pub fn is_loop_15(&self) -> bool {
        *self == Loop::Loop15
    }
}
#[doc = "Field `LOOP` writer - Loop Count Select"]
pub type LoopW<'a, REG> = crate::FieldWriter<'a, REG, 4, Loop>;
impl<'a, REG> LoopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Looping not enabled. Command executes 1 time."]
    #[inline(always)]
    pub fn loop_0(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop0)
    }
    #[doc = "Loop 1 time. Command executes 2 times."]
    #[inline(always)]
    pub fn loop_1(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop1)
    }
    #[doc = "Loop 2 times. Command executes 3 times."]
    #[inline(always)]
    pub fn loop_2(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop2)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_3(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop3)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_4(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop4)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_5(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop5)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_6(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop6)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_7(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop7)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_8(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop8)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_9(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop9)
    }
    #[doc = "Loop 15 times. Command executes 16 times."]
    #[inline(always)]
    pub fn loop_15(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loop15)
    }
}
#[doc = "Next Command Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Next {
    #[doc = "0: No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    Next0 = 0,
    #[doc = "1: Select CMD1 command buffer register as next command."]
    Next1 = 1,
    #[doc = "2: Select corresponding CMD command buffer register as next command"]
    Next2 = 2,
    #[doc = "3: Select corresponding CMD command buffer register as next command"]
    Next3 = 3,
    #[doc = "4: Select corresponding CMD command buffer register as next command"]
    Next4 = 4,
    #[doc = "5: Select corresponding CMD command buffer register as next command"]
    Next5 = 5,
    #[doc = "6: Select corresponding CMD command buffer register as next command"]
    Next6 = 6,
    #[doc = "7: Select corresponding CMD command buffer register as next command"]
    Next7 = 7,
    #[doc = "8: Select corresponding CMD command buffer register as next command"]
    Next8 = 8,
    #[doc = "9: Select corresponding CMD command buffer register as next command"]
    Next9 = 9,
    #[doc = "15: Select CMD15 command buffer register as next command."]
    Next15 = 15,
}
impl From<Next> for u8 {
    #[inline(always)]
    fn from(variant: Next) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Next {
    type Ux = u8;
}
impl crate::IsEnum for Next {}
#[doc = "Field `NEXT` reader - Next Command Select"]
pub type NextR = crate::FieldReader<Next>;
impl NextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Next> {
        match self.bits {
            0 => Some(Next::Next0),
            1 => Some(Next::Next1),
            2 => Some(Next::Next2),
            3 => Some(Next::Next3),
            4 => Some(Next::Next4),
            5 => Some(Next::Next5),
            6 => Some(Next::Next6),
            7 => Some(Next::Next7),
            8 => Some(Next::Next8),
            9 => Some(Next::Next9),
            15 => Some(Next::Next15),
            _ => None,
        }
    }
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    #[inline(always)]
    pub fn is_next_0(&self) -> bool {
        *self == Next::Next0
    }
    #[doc = "Select CMD1 command buffer register as next command."]
    #[inline(always)]
    pub fn is_next_1(&self) -> bool {
        *self == Next::Next1
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_2(&self) -> bool {
        *self == Next::Next2
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_3(&self) -> bool {
        *self == Next::Next3
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_4(&self) -> bool {
        *self == Next::Next4
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_5(&self) -> bool {
        *self == Next::Next5
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_6(&self) -> bool {
        *self == Next::Next6
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_7(&self) -> bool {
        *self == Next::Next7
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_8(&self) -> bool {
        *self == Next::Next8
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn is_next_9(&self) -> bool {
        *self == Next::Next9
    }
    #[doc = "Select CMD15 command buffer register as next command."]
    #[inline(always)]
    pub fn is_next_15(&self) -> bool {
        *self == Next::Next15
    }
}
#[doc = "Field `NEXT` writer - Next Command Select"]
pub type NextW<'a, REG> = crate::FieldWriter<'a, REG, 4, Next>;
impl<'a, REG> NextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    #[inline(always)]
    pub fn next_0(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next0)
    }
    #[doc = "Select CMD1 command buffer register as next command."]
    #[inline(always)]
    pub fn next_1(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next1)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_2(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next2)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_3(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next3)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_4(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next4)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_5(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next5)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_6(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next6)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_7(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next7)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_8(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next8)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_9(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next9)
    }
    #[doc = "Select CMD15 command buffer register as next command."]
    #[inline(always)]
    pub fn next_15(self) -> &'a mut crate::W<REG> {
        self.variant(Next::Next15)
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Function Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    pub fn lwi(&self) -> LwiR {
        LwiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AvgsR {
        AvgsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    pub fn next(&self) -> NextR {
        NextR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH")
            .field("cmpen", &self.cmpen())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Compare Function Enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<CmdhSpec> {
        CmpenW::new(self, 0)
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    pub fn lwi(&mut self) -> LwiW<CmdhSpec> {
        LwiW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    pub fn sts(&mut self) -> StsW<CmdhSpec> {
        StsW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&mut self) -> AvgsW<CmdhSpec> {
        AvgsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<CmdhSpec> {
        LoopW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    pub fn next(&mut self) -> NextW<CmdhSpec> {
        NextW::new(self, 24)
    }
}
#[doc = "ADC Command High Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdhSpec;
impl crate::RegisterSpec for CmdhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdh::R`](R) reader structure"]
impl crate::Readable for CmdhSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdh::W`](W) writer structure"]
impl crate::Writable for CmdhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDH%s to value 0"]
impl crate::Resettable for CmdhSpec {
    const RESET_VALUE: u32 = 0;
}
