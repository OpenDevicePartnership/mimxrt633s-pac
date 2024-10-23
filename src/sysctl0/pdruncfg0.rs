#[doc = "Register `PDRUNCFG0` reader"]
pub type R = crate::R<Pdruncfg0Spec>;
#[doc = "Register `PDRUNCFG0` writer"]
pub type W = crate::W<Pdruncfg0Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode0 {
    #[doc = "0: Set Mode0 to 0."]
    SetMode0_0 = 0,
    #[doc = "1: Set Mode0 to 1."]
    SetMode0_1 = 1,
}
impl From<PmicMode0> for bool {
    #[inline(always)]
    fn from(variant: PmicMode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE0` reader - no description available"]
pub type PmicMode0R = crate::BitReader<PmicMode0>;
impl PmicMode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmicMode0 {
        match self.bits {
            false => PmicMode0::SetMode0_0,
            true => PmicMode0::SetMode0_1,
        }
    }
    #[doc = "Set Mode0 to 0."]
    #[inline(always)]
    pub fn is_set_mode0_0(&self) -> bool {
        *self == PmicMode0::SetMode0_0
    }
    #[doc = "Set Mode0 to 1."]
    #[inline(always)]
    pub fn is_set_mode0_1(&self) -> bool {
        *self == PmicMode0::SetMode0_1
    }
}
#[doc = "Field `PMIC_MODE0` writer - no description available"]
pub type PmicMode0W<'a, REG> = crate::BitWriter<'a, REG, PmicMode0>;
impl<'a, REG> PmicMode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Mode0 to 0."]
    #[inline(always)]
    pub fn set_mode0_0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::SetMode0_0)
    }
    #[doc = "Set Mode0 to 1."]
    #[inline(always)]
    pub fn set_mode0_1(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::SetMode0_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode1 {
    #[doc = "0: Set Mode1 to 0."]
    SetMode1_0 = 0,
    #[doc = "1: Set Mode1 to 1."]
    SetMode1_1 = 1,
}
impl From<PmicMode1> for bool {
    #[inline(always)]
    fn from(variant: PmicMode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE1` reader - no description available"]
pub type PmicMode1R = crate::BitReader<PmicMode1>;
impl PmicMode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmicMode1 {
        match self.bits {
            false => PmicMode1::SetMode1_0,
            true => PmicMode1::SetMode1_1,
        }
    }
    #[doc = "Set Mode1 to 0."]
    #[inline(always)]
    pub fn is_set_mode1_0(&self) -> bool {
        *self == PmicMode1::SetMode1_0
    }
    #[doc = "Set Mode1 to 1."]
    #[inline(always)]
    pub fn is_set_mode1_1(&self) -> bool {
        *self == PmicMode1::SetMode1_1
    }
}
#[doc = "Field `PMIC_MODE1` writer - no description available"]
pub type PmicMode1W<'a, REG> = crate::BitWriter<'a, REG, PmicMode1>;
impl<'a, REG> PmicMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Mode1 to 0."]
    #[inline(always)]
    pub fn set_mode1_0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::SetMode1_0)
    }
    #[doc = "Set Mode1 to 1."]
    #[inline(always)]
    pub fn set_mode1_1(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::SetMode1_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VddcoreregLp {
    #[doc = "0: VDDCOREREG HP Mode"]
    HpMode = 0,
    #[doc = "1: LP Mode"]
    LpMode = 1,
}
impl From<VddcoreregLp> for bool {
    #[inline(always)]
    fn from(variant: VddcoreregLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREREG_LP` reader - no description available"]
pub type VddcoreregLpR = crate::BitReader<VddcoreregLp>;
impl VddcoreregLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VddcoreregLp {
        match self.bits {
            false => VddcoreregLp::HpMode,
            true => VddcoreregLp::LpMode,
        }
    }
    #[doc = "VDDCOREREG HP Mode"]
    #[inline(always)]
    pub fn is_hp_mode(&self) -> bool {
        *self == VddcoreregLp::HpMode
    }
    #[doc = "LP Mode"]
    #[inline(always)]
    pub fn is_lp_mode(&self) -> bool {
        *self == VddcoreregLp::LpMode
    }
}
#[doc = "Field `VDDCOREREG_LP` writer - no description available"]
pub type VddcoreregLpW<'a, REG> = crate::BitWriter<'a, REG, VddcoreregLp>;
impl<'a, REG> VddcoreregLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDCOREREG HP Mode"]
    #[inline(always)]
    pub fn hp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::HpMode)
    }
    #[doc = "LP Mode"]
    #[inline(always)]
    pub fn lp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::LpMode)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmcrefLp {
    #[doc = "0: PMCREF HP Mode"]
    HpMode = 0,
    #[doc = "1: PMCREF LP Mode"]
    LpMode = 1,
}
impl From<PmcrefLp> for bool {
    #[inline(always)]
    fn from(variant: PmcrefLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMCREF_LP` reader - no description available"]
pub type PmcrefLpR = crate::BitReader<PmcrefLp>;
impl PmcrefLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmcrefLp {
        match self.bits {
            false => PmcrefLp::HpMode,
            true => PmcrefLp::LpMode,
        }
    }
    #[doc = "PMCREF HP Mode"]
    #[inline(always)]
    pub fn is_hp_mode(&self) -> bool {
        *self == PmcrefLp::HpMode
    }
    #[doc = "PMCREF LP Mode"]
    #[inline(always)]
    pub fn is_lp_mode(&self) -> bool {
        *self == PmcrefLp::LpMode
    }
}
#[doc = "Field `PMCREF_LP` writer - no description available"]
pub type PmcrefLpW<'a, REG> = crate::BitWriter<'a, REG, PmcrefLp>;
impl<'a, REG> PmcrefLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMCREF HP Mode"]
    #[inline(always)]
    pub fn hp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::HpMode)
    }
    #[doc = "PMCREF LP Mode"]
    #[inline(always)]
    pub fn lp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::LpMode)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8Pd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<Hvd1v8Pd> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8_PD` reader - no description available"]
pub type Hvd1v8PdR = crate::BitReader<Hvd1v8Pd>;
impl Hvd1v8PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvd1v8Pd {
        match self.bits {
            false => Hvd1v8Pd::Enabled,
            true => Hvd1v8Pd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hvd1v8Pd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Hvd1v8Pd::PowerDown
    }
}
#[doc = "Field `HVD1V8_PD` writer - no description available"]
pub type Hvd1v8PdW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8Pd>;
impl<'a, REG> Hvd1v8PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PorcoreLp {
    #[doc = "0: LVD0V6 HP Mode"]
    HpMode = 0,
    #[doc = "1: LVD0V6 LP Mode"]
    LpMode = 1,
}
impl From<PorcoreLp> for bool {
    #[inline(always)]
    fn from(variant: PorcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORCORE_LP` reader - no description available"]
pub type PorcoreLpR = crate::BitReader<PorcoreLp>;
impl PorcoreLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PorcoreLp {
        match self.bits {
            false => PorcoreLp::HpMode,
            true => PorcoreLp::LpMode,
        }
    }
    #[doc = "LVD0V6 HP Mode"]
    #[inline(always)]
    pub fn is_hp_mode(&self) -> bool {
        *self == PorcoreLp::HpMode
    }
    #[doc = "LVD0V6 LP Mode"]
    #[inline(always)]
    pub fn is_lp_mode(&self) -> bool {
        *self == PorcoreLp::LpMode
    }
}
#[doc = "Field `PORCORE_LP` writer - no description available"]
pub type PorcoreLpW<'a, REG> = crate::BitWriter<'a, REG, PorcoreLp>;
impl<'a, REG> PorcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LVD0V6 HP Mode"]
    #[inline(always)]
    pub fn hp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::HpMode)
    }
    #[doc = "LVD0V6 LP Mode"]
    #[inline(always)]
    pub fn lp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::LpMode)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvdcoreLp {
    #[doc = "0: LVD0V85 HP Mode"]
    HpMode = 0,
    #[doc = "1: LVD0V85 LP Mode."]
    LpMode = 1,
}
impl From<LvdcoreLp> for bool {
    #[inline(always)]
    fn from(variant: LvdcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCORE_LP` reader - no description available"]
pub type LvdcoreLpR = crate::BitReader<LvdcoreLp>;
impl LvdcoreLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LvdcoreLp {
        match self.bits {
            false => LvdcoreLp::HpMode,
            true => LvdcoreLp::LpMode,
        }
    }
    #[doc = "LVD0V85 HP Mode"]
    #[inline(always)]
    pub fn is_hp_mode(&self) -> bool {
        *self == LvdcoreLp::HpMode
    }
    #[doc = "LVD0V85 LP Mode."]
    #[inline(always)]
    pub fn is_lp_mode(&self) -> bool {
        *self == LvdcoreLp::LpMode
    }
}
#[doc = "Field `LVDCORE_LP` writer - no description available"]
pub type LvdcoreLpW<'a, REG> = crate::BitWriter<'a, REG, LvdcoreLp>;
impl<'a, REG> LvdcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LVD0V85 HP Mode"]
    #[inline(always)]
    pub fn hp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::HpMode)
    }
    #[doc = "LVD0V85 LP Mode."]
    #[inline(always)]
    pub fn lp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::LpMode)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HvdcorePd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<HvdcorePd> for bool {
    #[inline(always)]
    fn from(variant: HvdcorePd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCORE_PD` reader - no description available"]
pub type HvdcorePdR = crate::BitReader<HvdcorePd>;
impl HvdcorePdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HvdcorePd {
        match self.bits {
            false => HvdcorePd::Enabled,
            true => HvdcorePd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HvdcorePd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == HvdcorePd::PowerDown
    }
}
#[doc = "Field `HVDCORE_PD` writer - no description available"]
pub type HvdcorePdW<'a, REG> = crate::BitWriter<'a, REG, HvdcorePd>;
impl<'a, REG> HvdcorePdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysxtalPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SysxtalPd> for bool {
    #[inline(always)]
    fn from(variant: SysxtalPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSXTAL_PD` reader - no description available"]
pub type SysxtalPdR = crate::BitReader<SysxtalPd>;
impl SysxtalPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysxtalPd {
        match self.bits {
            false => SysxtalPd::Enabled,
            true => SysxtalPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SysxtalPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SysxtalPd::PowerDown
    }
}
#[doc = "Field `SYSXTAL_PD` writer - no description available"]
pub type SysxtalPdW<'a, REG> = crate::BitWriter<'a, REG, SysxtalPd>;
impl<'a, REG> SysxtalPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LposcPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<LposcPd> for bool {
    #[inline(always)]
    fn from(variant: LposcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOSC_PD` reader - no description available"]
pub type LposcPdR = crate::BitReader<LposcPd>;
impl LposcPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LposcPd {
        match self.bits {
            false => LposcPd::Enabled,
            true => LposcPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LposcPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == LposcPd::PowerDown
    }
}
#[doc = "Field `LPOSC_PD` writer - no description available"]
pub type LposcPdW<'a, REG> = crate::BitWriter<'a, REG, LposcPd>;
impl<'a, REG> LposcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfroPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SfroPd> for bool {
    #[inline(always)]
    fn from(variant: SfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFRO_PD` reader - no description available"]
pub type SfroPdR = crate::BitReader<SfroPd>;
impl SfroPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfroPd {
        match self.bits {
            false => SfroPd::Enabled,
            true => SfroPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SfroPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SfroPd::PowerDown
    }
}
#[doc = "Field `SFRO_PD` writer - no description available"]
pub type SfroPdW<'a, REG> = crate::BitWriter<'a, REG, SfroPd>;
impl<'a, REG> SfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfroPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<FfroPd> for bool {
    #[inline(always)]
    fn from(variant: FfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFRO_PD` reader - no description available"]
pub type FfroPdR = crate::BitReader<FfroPd>;
impl FfroPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FfroPd {
        match self.bits {
            false => FfroPd::Enabled,
            true => FfroPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FfroPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FfroPd::PowerDown
    }
}
#[doc = "Field `FFRO_PD` writer - no description available"]
pub type FfroPdW<'a, REG> = crate::BitWriter<'a, REG, FfroPd>;
impl<'a, REG> FfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllldoPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SyspllldoPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLLDO_PD` reader - no description available"]
pub type SyspllldoPdR = crate::BitReader<SyspllldoPd>;
impl SyspllldoPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyspllldoPd {
        match self.bits {
            false => SyspllldoPd::Enabled,
            true => SyspllldoPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SyspllldoPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SyspllldoPd::PowerDown
    }
}
#[doc = "Field `SYSPLLLDO_PD` writer - no description available"]
pub type SyspllldoPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllldoPd>;
impl<'a, REG> SyspllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllanaPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<SyspllanaPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLANA_PD` reader - no description available"]
pub type SyspllanaPdR = crate::BitReader<SyspllanaPd>;
impl SyspllanaPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyspllanaPd {
        match self.bits {
            false => SyspllanaPd::Enabled,
            true => SyspllanaPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SyspllanaPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SyspllanaPd::PowerDown
    }
}
#[doc = "Field `SYSPLLANA_PD` writer - no description available"]
pub type SyspllanaPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllanaPd>;
impl<'a, REG> SyspllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllldoPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AudpllldoPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLLDO_PD` reader - no description available"]
pub type AudpllldoPdR = crate::BitReader<AudpllldoPd>;
impl AudpllldoPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AudpllldoPd {
        match self.bits {
            false => AudpllldoPd::Enabled,
            true => AudpllldoPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AudpllldoPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AudpllldoPd::PowerDown
    }
}
#[doc = "Field `AUDPLLLDO_PD` writer - no description available"]
pub type AudpllldoPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllldoPd>;
impl<'a, REG> AudpllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllanaPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AudpllanaPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLANA_PD` reader - no description available"]
pub type AudpllanaPdR = crate::BitReader<AudpllanaPd>;
impl AudpllanaPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AudpllanaPd {
        match self.bits {
            false => AudpllanaPd::Enabled,
            true => AudpllanaPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AudpllanaPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AudpllanaPd::PowerDown
    }
}
#[doc = "Field `AUDPLLANA_PD` writer - no description available"]
pub type AudpllanaPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllanaPd>;
impl<'a, REG> AudpllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AdcPd> for bool {
    #[inline(always)]
    fn from(variant: AdcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` reader - no description available"]
pub type AdcPdR = crate::BitReader<AdcPd>;
impl AdcPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcPd {
        match self.bits {
            false => AdcPd::Enabled,
            true => AdcPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AdcPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AdcPd::PowerDown
    }
}
#[doc = "Field `ADC_PD` writer - no description available"]
pub type AdcPdW<'a, REG> = crate::BitWriter<'a, REG, AdcPd>;
impl<'a, REG> AdcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcLp {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AdcLp> for bool {
    #[inline(always)]
    fn from(variant: AdcLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_LP` reader - no description available"]
pub type AdcLpR = crate::BitReader<AdcLp>;
impl AdcLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcLp {
        match self.bits {
            false => AdcLp::Enabled,
            true => AdcLp::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AdcLp::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AdcLp::PowerDown
    }
}
#[doc = "Field `ADC_LP` writer - no description available"]
pub type AdcLpW<'a, REG> = crate::BitWriter<'a, REG, AdcLp>;
impl<'a, REG> AdcLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdctempsnsPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AdctempsnsPd> for bool {
    #[inline(always)]
    fn from(variant: AdctempsnsPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTEMPSNS_PD` reader - no description available"]
pub type AdctempsnsPdR = crate::BitReader<AdctempsnsPd>;
impl AdctempsnsPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdctempsnsPd {
        match self.bits {
            false => AdctempsnsPd::Enabled,
            true => AdctempsnsPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AdctempsnsPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AdctempsnsPd::PowerDown
    }
}
#[doc = "Field `ADCTEMPSNS_PD` writer - no description available"]
pub type AdctempsnsPdW<'a, REG> = crate::BitWriter<'a, REG, AdctempsnsPd>;
impl<'a, REG> AdctempsnsPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::PowerDown)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmpPd {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: power down"]
    PowerDown = 1,
}
impl From<AcmpPd> for bool {
    #[inline(always)]
    fn from(variant: AcmpPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_PD` reader - no description available"]
pub type AcmpPdR = crate::BitReader<AcmpPd>;
impl AcmpPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmpPd {
        match self.bits {
            false => AcmpPd::Enabled,
            true => AcmpPd::PowerDown,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AcmpPd::Enabled
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == AcmpPd::PowerDown
    }
}
#[doc = "Field `ACMP_PD` writer - no description available"]
pub type AcmpPdW<'a, REG> = crate::BitWriter<'a, REG, AcmpPd>;
impl<'a, REG> AcmpPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::Enabled)
    }
    #[doc = "power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::PowerDown)
    }
}
#[doc = "High Speed Pad vdde0 voltage detect block\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0VdetLp {
    #[doc = "0: High Speed Pad VDET in Normal Mode"]
    NormalMode = 0,
    #[doc = "1: High Speed Pad VDET in Sleep Mode"]
    SleepMode = 1,
}
impl From<Hspad0VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad0VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_VDET_LP` reader - High Speed Pad vdde0 voltage detect block"]
pub type Hspad0VdetLpR = crate::BitReader<Hspad0VdetLp>;
impl Hspad0VdetLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspad0VdetLp {
        match self.bits {
            false => Hspad0VdetLp::NormalMode,
            true => Hspad0VdetLp::SleepMode,
        }
    }
    #[doc = "High Speed Pad VDET in Normal Mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Hspad0VdetLp::NormalMode
    }
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    #[inline(always)]
    pub fn is_sleep_mode(&self) -> bool {
        *self == Hspad0VdetLp::SleepMode
    }
}
#[doc = "Field `HSPAD0_VDET_LP` writer - High Speed Pad vdde0 voltage detect block"]
pub type Hspad0VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad0VdetLp>;
impl<'a, REG> Hspad0VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Pad VDET in Normal Mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::NormalMode)
    }
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    #[inline(always)]
    pub fn sleep_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::SleepMode)
    }
}
#[doc = "High speed Pad vdde0 reference blocks\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0RefPd {
    #[doc = "0: High Speed Pad VREF Enabled"]
    Enabled = 0,
    #[doc = "1: High Speed Pad VREF in Power Down"]
    PowerDown = 1,
}
impl From<Hspad0RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad0RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_REF_PD` reader - High speed Pad vdde0 reference blocks"]
pub type Hspad0RefPdR = crate::BitReader<Hspad0RefPd>;
impl Hspad0RefPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspad0RefPd {
        match self.bits {
            false => Hspad0RefPd::Enabled,
            true => Hspad0RefPd::PowerDown,
        }
    }
    #[doc = "High Speed Pad VREF Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hspad0RefPd::Enabled
    }
    #[doc = "High Speed Pad VREF in Power Down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Hspad0RefPd::PowerDown
    }
}
#[doc = "Field `HSPAD0_REF_PD` writer - High speed Pad vdde0 reference blocks"]
pub type Hspad0RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad0RefPd>;
impl<'a, REG> Hspad0RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Pad VREF Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::Enabled)
    }
    #[doc = "High Speed Pad VREF in Power Down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::PowerDown)
    }
}
#[doc = "High Speed Pad vdde2 voltage detect block\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2VdetLp {
    #[doc = "0: High Speed Pad VDET in Normal Mode"]
    NormalMode = 0,
    #[doc = "1: High Speed Pad VDET in Sleep Mode"]
    SleepMode = 1,
}
impl From<Hspad2VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad2VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_VDET_LP` reader - High Speed Pad vdde2 voltage detect block"]
pub type Hspad2VdetLpR = crate::BitReader<Hspad2VdetLp>;
impl Hspad2VdetLpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspad2VdetLp {
        match self.bits {
            false => Hspad2VdetLp::NormalMode,
            true => Hspad2VdetLp::SleepMode,
        }
    }
    #[doc = "High Speed Pad VDET in Normal Mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Hspad2VdetLp::NormalMode
    }
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    #[inline(always)]
    pub fn is_sleep_mode(&self) -> bool {
        *self == Hspad2VdetLp::SleepMode
    }
}
#[doc = "Field `HSPAD2_VDET_LP` writer - High Speed Pad vdde2 voltage detect block"]
pub type Hspad2VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad2VdetLp>;
impl<'a, REG> Hspad2VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Pad VDET in Normal Mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::NormalMode)
    }
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    #[inline(always)]
    pub fn sleep_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::SleepMode)
    }
}
#[doc = "High speed Pad vdde2 reference blocks\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2RefPd {
    #[doc = "0: High Speed Pad VREF Enabled"]
    Enabled = 0,
    #[doc = "1: High Speed Pad VREF in Power Down"]
    PowerDown = 1,
}
impl From<Hspad2RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad2RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_REF_PD` reader - High speed Pad vdde2 reference blocks"]
pub type Hspad2RefPdR = crate::BitReader<Hspad2RefPd>;
impl Hspad2RefPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspad2RefPd {
        match self.bits {
            false => Hspad2RefPd::Enabled,
            true => Hspad2RefPd::PowerDown,
        }
    }
    #[doc = "High Speed Pad VREF Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hspad2RefPd::Enabled
    }
    #[doc = "High Speed Pad VREF in Power Down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Hspad2RefPd::PowerDown
    }
}
#[doc = "Field `HSPAD2_REF_PD` writer - High speed Pad vdde2 reference blocks"]
pub type Hspad2RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad2RefPd>;
impl<'a, REG> Hspad2RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Pad VREF Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::Enabled)
    }
    #[doc = "High Speed Pad VREF in Power Down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::PowerDown)
    }
}
impl R {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn pmic_mode0(&self) -> PmicMode0R {
        PmicMode0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn pmic_mode1(&self) -> PmicMode1R {
        PmicMode1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn vddcorereg_lp(&self) -> VddcoreregLpR {
        VddcoreregLpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn pmcref_lp(&self) -> PmcrefLpR {
        PmcrefLpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn hvd1v8_pd(&self) -> Hvd1v8PdR {
        Hvd1v8PdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn porcore_lp(&self) -> PorcoreLpR {
        PorcoreLpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn lvdcore_lp(&self) -> LvdcoreLpR {
        LvdcoreLpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn hvdcore_pd(&self) -> HvdcorePdR {
        HvdcorePdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sysxtal_pd(&self) -> SysxtalPdR {
        SysxtalPdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn lposc_pd(&self) -> LposcPdR {
        LposcPdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn sfro_pd(&self) -> SfroPdR {
        SfroPdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn ffro_pd(&self) -> FfroPdR {
        FfroPdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn syspllldo_pd(&self) -> SyspllldoPdR {
        SyspllldoPdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn syspllana_pd(&self) -> SyspllanaPdR {
        SyspllanaPdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn audpllldo_pd(&self) -> AudpllldoPdR {
        AudpllldoPdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn audpllana_pd(&self) -> AudpllanaPdR {
        AudpllanaPdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn adc_pd(&self) -> AdcPdR {
        AdcPdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn adc_lp(&self) -> AdcLpR {
        AdcLpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn adctempsns_pd(&self) -> AdctempsnsPdR {
        AdctempsnsPdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn acmp_pd(&self) -> AcmpPdR {
        AcmpPdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - High Speed Pad vdde0 voltage detect block"]
    #[inline(always)]
    pub fn hspad0_vdet_lp(&self) -> Hspad0VdetLpR {
        Hspad0VdetLpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - High speed Pad vdde0 reference blocks"]
    #[inline(always)]
    pub fn hspad0_ref_pd(&self) -> Hspad0RefPdR {
        Hspad0RefPdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - High Speed Pad vdde2 voltage detect block"]
    #[inline(always)]
    pub fn hspad2_vdet_lp(&self) -> Hspad2VdetLpR {
        Hspad2VdetLpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - High speed Pad vdde2 reference blocks"]
    #[inline(always)]
    pub fn hspad2_ref_pd(&self) -> Hspad2RefPdR {
        Hspad2RefPdR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDRUNCFG0")
            .field("pmic_mode0", &self.pmic_mode0())
            .field("pmic_mode1", &self.pmic_mode1())
            .field("vddcorereg_lp", &self.vddcorereg_lp())
            .field("pmcref_lp", &self.pmcref_lp())
            .field("hvd1v8_pd", &self.hvd1v8_pd())
            .field("porcore_lp", &self.porcore_lp())
            .field("lvdcore_lp", &self.lvdcore_lp())
            .field("hvdcore_pd", &self.hvdcore_pd())
            .field("sysxtal_pd", &self.sysxtal_pd())
            .field("lposc_pd", &self.lposc_pd())
            .field("sfro_pd", &self.sfro_pd())
            .field("ffro_pd", &self.ffro_pd())
            .field("syspllldo_pd", &self.syspllldo_pd())
            .field("syspllana_pd", &self.syspllana_pd())
            .field("audpllldo_pd", &self.audpllldo_pd())
            .field("audpllana_pd", &self.audpllana_pd())
            .field("adc_pd", &self.adc_pd())
            .field("adc_lp", &self.adc_lp())
            .field("adctempsns_pd", &self.adctempsns_pd())
            .field("acmp_pd", &self.acmp_pd())
            .field("hspad0_vdet_lp", &self.hspad0_vdet_lp())
            .field("hspad0_ref_pd", &self.hspad0_ref_pd())
            .field("hspad2_vdet_lp", &self.hspad2_vdet_lp())
            .field("hspad2_ref_pd", &self.hspad2_ref_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode0(&mut self) -> PmicMode0W<Pdruncfg0Spec> {
        PmicMode0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode1(&mut self) -> PmicMode1W<Pdruncfg0Spec> {
        PmicMode1W::new(self, 2)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorereg_lp(&mut self) -> VddcoreregLpW<Pdruncfg0Spec> {
        VddcoreregLpW::new(self, 4)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmcref_lp(&mut self) -> PmcrefLpW<Pdruncfg0Spec> {
        PmcrefLpW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8_pd(&mut self) -> Hvd1v8PdW<Pdruncfg0Spec> {
        Hvd1v8PdW::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn porcore_lp(&mut self) -> PorcoreLpW<Pdruncfg0Spec> {
        PorcoreLpW::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcore_lp(&mut self) -> LvdcoreLpW<Pdruncfg0Spec> {
        LvdcoreLpW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcore_pd(&mut self) -> HvdcorePdW<Pdruncfg0Spec> {
        HvdcorePdW::new(self, 10)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sysxtal_pd(&mut self) -> SysxtalPdW<Pdruncfg0Spec> {
        SysxtalPdW::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lposc_pd(&mut self) -> LposcPdW<Pdruncfg0Spec> {
        LposcPdW::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sfro_pd(&mut self) -> SfroPdW<Pdruncfg0Spec> {
        SfroPdW::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ffro_pd(&mut self) -> FfroPdW<Pdruncfg0Spec> {
        FfroPdW::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn syspllldo_pd(&mut self) -> SyspllldoPdW<Pdruncfg0Spec> {
        SyspllldoPdW::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn syspllana_pd(&mut self) -> SyspllanaPdW<Pdruncfg0Spec> {
        SyspllanaPdW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn audpllldo_pd(&mut self) -> AudpllldoPdW<Pdruncfg0Spec> {
        AudpllldoPdW::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn audpllana_pd(&mut self) -> AudpllanaPdW<Pdruncfg0Spec> {
        AudpllanaPdW::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> AdcPdW<Pdruncfg0Spec> {
        AdcPdW::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adc_lp(&mut self) -> AdcLpW<Pdruncfg0Spec> {
        AdcLpW::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adctempsns_pd(&mut self) -> AdctempsnsPdW<Pdruncfg0Spec> {
        AdctempsnsPdW::new(self, 23)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn acmp_pd(&mut self) -> AcmpPdW<Pdruncfg0Spec> {
        AcmpPdW::new(self, 25)
    }
    #[doc = "Bit 26 - High Speed Pad vdde0 voltage detect block"]
    #[inline(always)]
    #[must_use]
    pub fn hspad0_vdet_lp(&mut self) -> Hspad0VdetLpW<Pdruncfg0Spec> {
        Hspad0VdetLpW::new(self, 26)
    }
    #[doc = "Bit 27 - High speed Pad vdde0 reference blocks"]
    #[inline(always)]
    #[must_use]
    pub fn hspad0_ref_pd(&mut self) -> Hspad0RefPdW<Pdruncfg0Spec> {
        Hspad0RefPdW::new(self, 27)
    }
    #[doc = "Bit 28 - High Speed Pad vdde2 voltage detect block"]
    #[inline(always)]
    #[must_use]
    pub fn hspad2_vdet_lp(&mut self) -> Hspad2VdetLpW<Pdruncfg0Spec> {
        Hspad2VdetLpW::new(self, 28)
    }
    #[doc = "Bit 29 - High speed Pad vdde2 reference blocks"]
    #[inline(always)]
    #[must_use]
    pub fn hspad2_ref_pd(&mut self) -> Hspad2RefPdW<Pdruncfg0Spec> {
        Hspad2RefPdW::new(self, 29)
    }
}
#[doc = "Run configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdruncfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg0Spec;
impl crate::RegisterSpec for Pdruncfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdruncfg0::R`](R) reader structure"]
impl crate::Readable for Pdruncfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdruncfg0::W`](W) writer structure"]
impl crate::Writable for Pdruncfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG0 to value 0x03fe_bc80"]
impl crate::Resettable for Pdruncfg0Spec {
    const RESET_VALUE: u32 = 0x03fe_bc80;
}
