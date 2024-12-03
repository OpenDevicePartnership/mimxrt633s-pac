#[doc = "Register `C3` reader"]
pub type R = crate::R<C3Spec>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3Spec>;
#[doc = "Analog Comparator Phase2 Timing Control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acph2tc {
    #[doc = "0: Phase2 active time in one sampling period equals to T"]
    Acph2tc0 = 0,
    #[doc = "1: Phase2 active time in one sampling period equals to 2*T"]
    Acph2tc1 = 1,
    #[doc = "2: Phase2 active time in one sampling period equals to 4*T"]
    Acph2tc2 = 2,
    #[doc = "3: Phase2 active time in one sampling period equals to 8*T"]
    Acph2tc3 = 3,
    #[doc = "4: Phase2 active time in one sampling period equals to 16*T"]
    Acph2tc4 = 4,
    #[doc = "5: Phase2 active time in one sampling period equals to 32*T"]
    Acph2tc5 = 5,
    #[doc = "6: Phase2 active time in one sampling period equals to 64*T"]
    Acph2tc6 = 6,
    #[doc = "7: Phase2 active time in one sampling period equals to 16*T"]
    Acph2tc7 = 7,
}
impl From<Acph2tc> for u8 {
    #[inline(always)]
    fn from(variant: Acph2tc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acph2tc {
    type Ux = u8;
}
impl crate::IsEnum for Acph2tc {}
#[doc = "Field `ACPH2TC` reader - Analog Comparator Phase2 Timing Control."]
pub type Acph2tcR = crate::FieldReader<Acph2tc>;
impl Acph2tcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acph2tc {
        match self.bits {
            0 => Acph2tc::Acph2tc0,
            1 => Acph2tc::Acph2tc1,
            2 => Acph2tc::Acph2tc2,
            3 => Acph2tc::Acph2tc3,
            4 => Acph2tc::Acph2tc4,
            5 => Acph2tc::Acph2tc5,
            6 => Acph2tc::Acph2tc6,
            7 => Acph2tc::Acph2tc7,
            _ => unreachable!(),
        }
    }
    #[doc = "Phase2 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn is_acph2tc_0(&self) -> bool {
        *self == Acph2tc::Acph2tc0
    }
    #[doc = "Phase2 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn is_acph2tc_1(&self) -> bool {
        *self == Acph2tc::Acph2tc1
    }
    #[doc = "Phase2 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn is_acph2tc_2(&self) -> bool {
        *self == Acph2tc::Acph2tc2
    }
    #[doc = "Phase2 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn is_acph2tc_3(&self) -> bool {
        *self == Acph2tc::Acph2tc3
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn is_acph2tc_4(&self) -> bool {
        *self == Acph2tc::Acph2tc4
    }
    #[doc = "Phase2 active time in one sampling period equals to 32*T"]
    #[inline(always)]
    pub fn is_acph2tc_5(&self) -> bool {
        *self == Acph2tc::Acph2tc5
    }
    #[doc = "Phase2 active time in one sampling period equals to 64*T"]
    #[inline(always)]
    pub fn is_acph2tc_6(&self) -> bool {
        *self == Acph2tc::Acph2tc6
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn is_acph2tc_7(&self) -> bool {
        *self == Acph2tc::Acph2tc7
    }
}
#[doc = "Field `ACPH2TC` writer - Analog Comparator Phase2 Timing Control."]
pub type Acph2tcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Acph2tc, crate::Safe>;
impl<'a, REG> Acph2tcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Phase2 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph2tc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc0)
    }
    #[doc = "Phase2 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn acph2tc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc1)
    }
    #[doc = "Phase2 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn acph2tc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc2)
    }
    #[doc = "Phase2 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn acph2tc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc3)
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn acph2tc_4(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc4)
    }
    #[doc = "Phase2 active time in one sampling period equals to 32*T"]
    #[inline(always)]
    pub fn acph2tc_5(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc5)
    }
    #[doc = "Phase2 active time in one sampling period equals to 64*T"]
    #[inline(always)]
    pub fn acph2tc_6(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc6)
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn acph2tc_7(self) -> &'a mut crate::W<REG> {
        self.variant(Acph2tc::Acph2tc7)
    }
}
#[doc = "Analog Comparator Phase1 Timing Control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acph1tc {
    #[doc = "0: Phase1 active time in one sampling period equals to T"]
    Acph1tc0 = 0,
    #[doc = "1: Phase1 active time in one sampling period equals to 2*T"]
    Acph1tc1 = 1,
    #[doc = "2: Phase1 active time in one sampling period equals to 4*T"]
    Acph1tc2 = 2,
    #[doc = "3: Phase1 active time in one sampling period equals to 8*T"]
    Acph1tc3 = 3,
    #[doc = "4: Phase1 active time in one sampling period equals to T"]
    Acph1tc4 = 4,
    #[doc = "5: Phase1 active time in one sampling period equals to T"]
    Acph1tc5 = 5,
    #[doc = "6: Phase1 active time in one sampling period equals to T"]
    Acph1tc6 = 6,
    #[doc = "7: Phase1 active time in one sampling period equals to 0"]
    Acph1tc7 = 7,
}
impl From<Acph1tc> for u8 {
    #[inline(always)]
    fn from(variant: Acph1tc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acph1tc {
    type Ux = u8;
}
impl crate::IsEnum for Acph1tc {}
#[doc = "Field `ACPH1TC` reader - Analog Comparator Phase1 Timing Control."]
pub type Acph1tcR = crate::FieldReader<Acph1tc>;
impl Acph1tcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acph1tc {
        match self.bits {
            0 => Acph1tc::Acph1tc0,
            1 => Acph1tc::Acph1tc1,
            2 => Acph1tc::Acph1tc2,
            3 => Acph1tc::Acph1tc3,
            4 => Acph1tc::Acph1tc4,
            5 => Acph1tc::Acph1tc5,
            6 => Acph1tc::Acph1tc6,
            7 => Acph1tc::Acph1tc7,
            _ => unreachable!(),
        }
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn is_acph1tc_0(&self) -> bool {
        *self == Acph1tc::Acph1tc0
    }
    #[doc = "Phase1 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn is_acph1tc_1(&self) -> bool {
        *self == Acph1tc::Acph1tc1
    }
    #[doc = "Phase1 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn is_acph1tc_2(&self) -> bool {
        *self == Acph1tc::Acph1tc2
    }
    #[doc = "Phase1 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn is_acph1tc_3(&self) -> bool {
        *self == Acph1tc::Acph1tc3
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn is_acph1tc_4(&self) -> bool {
        *self == Acph1tc::Acph1tc4
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn is_acph1tc_5(&self) -> bool {
        *self == Acph1tc::Acph1tc5
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn is_acph1tc_6(&self) -> bool {
        *self == Acph1tc::Acph1tc6
    }
    #[doc = "Phase1 active time in one sampling period equals to 0"]
    #[inline(always)]
    pub fn is_acph1tc_7(&self) -> bool {
        *self == Acph1tc::Acph1tc7
    }
}
#[doc = "Field `ACPH1TC` writer - Analog Comparator Phase1 Timing Control."]
pub type Acph1tcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Acph1tc, crate::Safe>;
impl<'a, REG> Acph1tcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc0)
    }
    #[doc = "Phase1 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn acph1tc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc1)
    }
    #[doc = "Phase1 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn acph1tc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc2)
    }
    #[doc = "Phase1 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn acph1tc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc3)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_4(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc4)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_5(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc5)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_6(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc6)
    }
    #[doc = "Phase1 active time in one sampling period equals to 0"]
    #[inline(always)]
    pub fn acph1tc_7(self) -> &'a mut crate::W<REG> {
        self.variant(Acph1tc::Acph1tc7)
    }
}
#[doc = "Analog Comparator Sampling Time control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acsat {
    #[doc = "0: The sampling time equals to T"]
    Acsat0 = 0,
    #[doc = "1: The sampling time equasl to 2*T"]
    Acsat1 = 1,
    #[doc = "2: The sampling time equasl to 4*T"]
    Acsat2 = 2,
    #[doc = "3: The sampling time equasl to 8*T"]
    Acsat3 = 3,
    #[doc = "4: The sampling time equasl to 16*T"]
    Acsat4 = 4,
    #[doc = "5: The sampling time equasl to 32*T"]
    Acsat5 = 5,
    #[doc = "6: The sampling time equasl to 64*T"]
    Acsat6 = 6,
    #[doc = "7: The sampling time equasl to 256*T"]
    Acsat7 = 7,
}
impl From<Acsat> for u8 {
    #[inline(always)]
    fn from(variant: Acsat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acsat {
    type Ux = u8;
}
impl crate::IsEnum for Acsat {}
#[doc = "Field `ACSAT` reader - Analog Comparator Sampling Time control."]
pub type AcsatR = crate::FieldReader<Acsat>;
impl AcsatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acsat {
        match self.bits {
            0 => Acsat::Acsat0,
            1 => Acsat::Acsat1,
            2 => Acsat::Acsat2,
            3 => Acsat::Acsat3,
            4 => Acsat::Acsat4,
            5 => Acsat::Acsat5,
            6 => Acsat::Acsat6,
            7 => Acsat::Acsat7,
            _ => unreachable!(),
        }
    }
    #[doc = "The sampling time equals to T"]
    #[inline(always)]
    pub fn is_acsat_0(&self) -> bool {
        *self == Acsat::Acsat0
    }
    #[doc = "The sampling time equasl to 2*T"]
    #[inline(always)]
    pub fn is_acsat_1(&self) -> bool {
        *self == Acsat::Acsat1
    }
    #[doc = "The sampling time equasl to 4*T"]
    #[inline(always)]
    pub fn is_acsat_2(&self) -> bool {
        *self == Acsat::Acsat2
    }
    #[doc = "The sampling time equasl to 8*T"]
    #[inline(always)]
    pub fn is_acsat_3(&self) -> bool {
        *self == Acsat::Acsat3
    }
    #[doc = "The sampling time equasl to 16*T"]
    #[inline(always)]
    pub fn is_acsat_4(&self) -> bool {
        *self == Acsat::Acsat4
    }
    #[doc = "The sampling time equasl to 32*T"]
    #[inline(always)]
    pub fn is_acsat_5(&self) -> bool {
        *self == Acsat::Acsat5
    }
    #[doc = "The sampling time equasl to 64*T"]
    #[inline(always)]
    pub fn is_acsat_6(&self) -> bool {
        *self == Acsat::Acsat6
    }
    #[doc = "The sampling time equasl to 256*T"]
    #[inline(always)]
    pub fn is_acsat_7(&self) -> bool {
        *self == Acsat::Acsat7
    }
}
#[doc = "Field `ACSAT` writer - Analog Comparator Sampling Time control."]
pub type AcsatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Acsat, crate::Safe>;
impl<'a, REG> AcsatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The sampling time equals to T"]
    #[inline(always)]
    pub fn acsat_0(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat0)
    }
    #[doc = "The sampling time equasl to 2*T"]
    #[inline(always)]
    pub fn acsat_1(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat1)
    }
    #[doc = "The sampling time equasl to 4*T"]
    #[inline(always)]
    pub fn acsat_2(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat2)
    }
    #[doc = "The sampling time equasl to 8*T"]
    #[inline(always)]
    pub fn acsat_3(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat3)
    }
    #[doc = "The sampling time equasl to 16*T"]
    #[inline(always)]
    pub fn acsat_4(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat4)
    }
    #[doc = "The sampling time equasl to 32*T"]
    #[inline(always)]
    pub fn acsat_5(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat5)
    }
    #[doc = "The sampling time equasl to 64*T"]
    #[inline(always)]
    pub fn acsat_6(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat6)
    }
    #[doc = "The sampling time equasl to 256*T"]
    #[inline(always)]
    pub fn acsat_7(self) -> &'a mut crate::W<REG> {
        self.variant(Acsat::Acsat7)
    }
}
#[doc = "Discrete Mode Clock Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmcs {
    #[doc = "0: Slow clock is selected for the timing generation."]
    Dmcs0 = 0,
    #[doc = "1: Fast clock is selected for the timing generation."]
    Dmcs1 = 1,
}
impl From<Dmcs> for bool {
    #[inline(always)]
    fn from(variant: Dmcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMCS` reader - Discrete Mode Clock Selection"]
pub type DmcsR = crate::BitReader<Dmcs>;
impl DmcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmcs {
        match self.bits {
            false => Dmcs::Dmcs0,
            true => Dmcs::Dmcs1,
        }
    }
    #[doc = "Slow clock is selected for the timing generation."]
    #[inline(always)]
    pub fn is_dmcs_0(&self) -> bool {
        *self == Dmcs::Dmcs0
    }
    #[doc = "Fast clock is selected for the timing generation."]
    #[inline(always)]
    pub fn is_dmcs_1(&self) -> bool {
        *self == Dmcs::Dmcs1
    }
}
#[doc = "Field `DMCS` writer - Discrete Mode Clock Selection"]
pub type DmcsW<'a, REG> = crate::BitWriter<'a, REG, Dmcs>;
impl<'a, REG> DmcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slow clock is selected for the timing generation."]
    #[inline(always)]
    pub fn dmcs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmcs::Dmcs0)
    }
    #[doc = "Fast clock is selected for the timing generation."]
    #[inline(always)]
    pub fn dmcs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmcs::Dmcs1)
    }
}
#[doc = "Resistor Divider Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdive {
    #[doc = "0: The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    Rdive0 = 0,
    #[doc = "1: The resistor is enabled because the inputs are above 1.8v."]
    Rdive1 = 1,
}
impl From<Rdive> for bool {
    #[inline(always)]
    fn from(variant: Rdive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIVE` reader - Resistor Divider Enable"]
pub type RdiveR = crate::BitReader<Rdive>;
impl RdiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdive {
        match self.bits {
            false => Rdive::Rdive0,
            true => Rdive::Rdive1,
        }
    }
    #[doc = "The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    #[inline(always)]
    pub fn is_rdive_0(&self) -> bool {
        *self == Rdive::Rdive0
    }
    #[doc = "The resistor is enabled because the inputs are above 1.8v."]
    #[inline(always)]
    pub fn is_rdive_1(&self) -> bool {
        *self == Rdive::Rdive1
    }
}
#[doc = "Field `RDIVE` writer - Resistor Divider Enable"]
pub type RdiveW<'a, REG> = crate::BitWriter<'a, REG, Rdive>;
impl<'a, REG> RdiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    #[inline(always)]
    pub fn rdive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdive::Rdive0)
    }
    #[doc = "The resistor is enabled because the inputs are above 1.8v."]
    #[inline(always)]
    pub fn rdive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdive::Rdive1)
    }
}
#[doc = "Negative Channel Continuous Mode Enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nchcten {
    #[doc = "0: Negative channel is in Discrete Mode and special timing needs to be configured."]
    Nchcten0 = 0,
    #[doc = "1: Negative channel is in Continuous Mode and no special timing is requried."]
    Nchcten1 = 1,
}
impl From<Nchcten> for bool {
    #[inline(always)]
    fn from(variant: Nchcten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCHCTEN` reader - Negative Channel Continuous Mode Enable."]
pub type NchctenR = crate::BitReader<Nchcten>;
impl NchctenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nchcten {
        match self.bits {
            false => Nchcten::Nchcten0,
            true => Nchcten::Nchcten1,
        }
    }
    #[doc = "Negative channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn is_nchcten_0(&self) -> bool {
        *self == Nchcten::Nchcten0
    }
    #[doc = "Negative channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn is_nchcten_1(&self) -> bool {
        *self == Nchcten::Nchcten1
    }
}
#[doc = "Field `NCHCTEN` writer - Negative Channel Continuous Mode Enable."]
pub type NchctenW<'a, REG> = crate::BitWriter<'a, REG, Nchcten>;
impl<'a, REG> NchctenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Negative channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn nchcten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nchcten::Nchcten0)
    }
    #[doc = "Negative channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn nchcten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nchcten::Nchcten1)
    }
}
#[doc = "Positive Channel Continuous Mode Enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pchcten {
    #[doc = "0: Positive channel is in Discrete Mode and special timing needs to be configured."]
    Pchcten0 = 0,
    #[doc = "1: Positive channel is in Continuous Mode and no special timing is requried."]
    Pchcten1 = 1,
}
impl From<Pchcten> for bool {
    #[inline(always)]
    fn from(variant: Pchcten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCHCTEN` reader - Positive Channel Continuous Mode Enable."]
pub type PchctenR = crate::BitReader<Pchcten>;
impl PchctenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pchcten {
        match self.bits {
            false => Pchcten::Pchcten0,
            true => Pchcten::Pchcten1,
        }
    }
    #[doc = "Positive channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn is_pchcten_0(&self) -> bool {
        *self == Pchcten::Pchcten0
    }
    #[doc = "Positive channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn is_pchcten_1(&self) -> bool {
        *self == Pchcten::Pchcten1
    }
}
#[doc = "Field `PCHCTEN` writer - Positive Channel Continuous Mode Enable."]
pub type PchctenW<'a, REG> = crate::BitWriter<'a, REG, Pchcten>;
impl<'a, REG> PchctenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn pchcten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pchcten::Pchcten0)
    }
    #[doc = "Positive channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn pchcten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pchcten::Pchcten1)
    }
}
impl R {
    #[doc = "Bits 4:6 - Analog Comparator Phase2 Timing Control."]
    #[inline(always)]
    pub fn acph2tc(&self) -> Acph2tcR {
        Acph2tcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Analog Comparator Phase1 Timing Control."]
    #[inline(always)]
    pub fn acph1tc(&self) -> Acph1tcR {
        Acph1tcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Analog Comparator Sampling Time control."]
    #[inline(always)]
    pub fn acsat(&self) -> AcsatR {
        AcsatR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Discrete Mode Clock Selection"]
    #[inline(always)]
    pub fn dmcs(&self) -> DmcsR {
        DmcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Resistor Divider Enable"]
    #[inline(always)]
    pub fn rdive(&self) -> RdiveR {
        RdiveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Negative Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn nchcten(&self) -> NchctenR {
        NchctenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Positive Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn pchcten(&self) -> PchctenR {
        PchctenR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3")
            .field("acph2tc", &self.acph2tc())
            .field("acph1tc", &self.acph1tc())
            .field("acsat", &self.acsat())
            .field("dmcs", &self.dmcs())
            .field("rdive", &self.rdive())
            .field("nchcten", &self.nchcten())
            .field("pchcten", &self.pchcten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:6 - Analog Comparator Phase2 Timing Control."]
    #[inline(always)]
    pub fn acph2tc(&mut self) -> Acph2tcW<C3Spec> {
        Acph2tcW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Analog Comparator Phase1 Timing Control."]
    #[inline(always)]
    pub fn acph1tc(&mut self) -> Acph1tcW<C3Spec> {
        Acph1tcW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Analog Comparator Sampling Time control."]
    #[inline(always)]
    pub fn acsat(&mut self) -> AcsatW<C3Spec> {
        AcsatW::new(self, 12)
    }
    #[doc = "Bit 16 - Discrete Mode Clock Selection"]
    #[inline(always)]
    pub fn dmcs(&mut self) -> DmcsW<C3Spec> {
        DmcsW::new(self, 16)
    }
    #[doc = "Bit 20 - Resistor Divider Enable"]
    #[inline(always)]
    pub fn rdive(&mut self) -> RdiveW<C3Spec> {
        RdiveW::new(self, 20)
    }
    #[doc = "Bit 24 - Negative Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn nchcten(&mut self) -> NchctenW<C3Spec> {
        NchctenW::new(self, 24)
    }
    #[doc = "Bit 28 - Positive Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn pchcten(&mut self) -> PchctenW<C3Spec> {
        PchctenW::new(self, 28)
    }
}
#[doc = "CMP Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3Spec;
impl crate::RegisterSpec for C3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3Spec {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C3 to value 0x1100_0000"]
impl crate::Resettable for C3Spec {
    const RESET_VALUE: u32 = 0x1100_0000;
}
