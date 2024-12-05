#[doc = "Register `PDRUNCFG0_CLR` writer"]
pub type W = crate::W<Pdruncfg0ClrSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<PmicMode0> for bool {
    #[inline(always)]
    fn from(variant: PmicMode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE0` writer - no description available"]
pub type PmicMode0W<'a, REG> = crate::BitWriter<'a, REG, PmicMode0>;
impl<'a, REG> PmicMode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<PmicMode1> for bool {
    #[inline(always)]
    fn from(variant: PmicMode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE1` writer - no description available"]
pub type PmicMode1W<'a, REG> = crate::BitWriter<'a, REG, PmicMode1>;
impl<'a, REG> PmicMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VddcoreregLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<VddcoreregLp> for bool {
    #[inline(always)]
    fn from(variant: VddcoreregLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREREG_LP` writer - no description available"]
pub type VddcoreregLpW<'a, REG> = crate::BitWriter<'a, REG, VddcoreregLp>;
impl<'a, REG> VddcoreregLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmcrefLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<PmcrefLp> for bool {
    #[inline(always)]
    fn from(variant: PmcrefLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMCREF_LP` writer - no description available"]
pub type PmcrefLpW<'a, REG> = crate::BitWriter<'a, REG, PmcrefLp>;
impl<'a, REG> PmcrefLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8Pd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<Hvd1v8Pd> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8_PD` writer - no description available"]
pub type Hvd1v8PdW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8Pd>;
impl<'a, REG> Hvd1v8PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PorcoreLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<PorcoreLp> for bool {
    #[inline(always)]
    fn from(variant: PorcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORCORE_LP` writer - no description available"]
pub type PorcoreLpW<'a, REG> = crate::BitWriter<'a, REG, PorcoreLp>;
impl<'a, REG> PorcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvdcoreLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<LvdcoreLp> for bool {
    #[inline(always)]
    fn from(variant: LvdcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCORE_LP` writer - no description available"]
pub type LvdcoreLpW<'a, REG> = crate::BitWriter<'a, REG, LvdcoreLp>;
impl<'a, REG> LvdcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HvdcorePd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<HvdcorePd> for bool {
    #[inline(always)]
    fn from(variant: HvdcorePd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCORE_PD` writer - no description available"]
pub type HvdcorePdW<'a, REG> = crate::BitWriter<'a, REG, HvdcorePd>;
impl<'a, REG> HvdcorePdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RbbPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<RbbPd> for bool {
    #[inline(always)]
    fn from(variant: RbbPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBB_PD` writer - no description available"]
pub type RbbPdW<'a, REG> = crate::BitWriter<'a, REG, RbbPd>;
impl<'a, REG> RbbPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RbbPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(RbbPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbbPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<FbbPd> for bool {
    #[inline(always)]
    fn from(variant: FbbPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBB_PD` writer - no description available"]
pub type FbbPdW<'a, REG> = crate::BitWriter<'a, REG, FbbPd>;
impl<'a, REG> FbbPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FbbPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(FbbPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysxtalPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<SysxtalPd> for bool {
    #[inline(always)]
    fn from(variant: SysxtalPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSXTAL_PD` writer - no description available"]
pub type SysxtalPdW<'a, REG> = crate::BitWriter<'a, REG, SysxtalPd>;
impl<'a, REG> SysxtalPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LposcPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<LposcPd> for bool {
    #[inline(always)]
    fn from(variant: LposcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOSC_PD` writer - no description available"]
pub type LposcPdW<'a, REG> = crate::BitWriter<'a, REG, LposcPd>;
impl<'a, REG> LposcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfroPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<SfroPd> for bool {
    #[inline(always)]
    fn from(variant: SfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFRO_PD` writer - no description available"]
pub type SfroPdW<'a, REG> = crate::BitWriter<'a, REG, SfroPd>;
impl<'a, REG> SfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfroPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<FfroPd> for bool {
    #[inline(always)]
    fn from(variant: FfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFRO_PD` writer - no description available"]
pub type FfroPdW<'a, REG> = crate::BitWriter<'a, REG, FfroPd>;
impl<'a, REG> FfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllldoPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<SyspllldoPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLLDO_PD` writer - no description available"]
pub type SyspllldoPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllldoPd>;
impl<'a, REG> SyspllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllanaPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<SyspllanaPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLANA_PD` writer - no description available"]
pub type SyspllanaPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllanaPd>;
impl<'a, REG> SyspllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllldoPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AudpllldoPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLLDO_PD` writer - no description available"]
pub type AudpllldoPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllldoPd>;
impl<'a, REG> AudpllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllanaPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AudpllanaPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLANA_PD` writer - no description available"]
pub type AudpllanaPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllanaPd>;
impl<'a, REG> AudpllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AdcPd> for bool {
    #[inline(always)]
    fn from(variant: AdcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` writer - no description available"]
pub type AdcPdW<'a, REG> = crate::BitWriter<'a, REG, AdcPd>;
impl<'a, REG> AdcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AdcLp> for bool {
    #[inline(always)]
    fn from(variant: AdcLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_LP` writer - no description available"]
pub type AdcLpW<'a, REG> = crate::BitWriter<'a, REG, AdcLp>;
impl<'a, REG> AdcLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdctempsnsPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AdctempsnsPd> for bool {
    #[inline(always)]
    fn from(variant: AdctempsnsPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTEMPSNS_PD` writer - no description available"]
pub type AdctempsnsPdW<'a, REG> = crate::BitWriter<'a, REG, AdctempsnsPd>;
impl<'a, REG> AdctempsnsPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmpPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<AcmpPd> for bool {
    #[inline(always)]
    fn from(variant: AcmpPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_PD` writer - no description available"]
pub type AcmpPdW<'a, REG> = crate::BitWriter<'a, REG, AcmpPd>;
impl<'a, REG> AcmpPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0VdetLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<Hspad0VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad0VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_VDET_LP` writer - no description available"]
pub type Hspad0VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad0VdetLp>;
impl<'a, REG> Hspad0VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0RefPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<Hspad0RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad0RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_REF_PD` writer - no description available"]
pub type Hspad0RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad0RefPd>;
impl<'a, REG> Hspad0RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2VdetLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<Hspad2VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad2VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_VDET_LP` writer - no description available"]
pub type Hspad2VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad2VdetLp>;
impl<'a, REG> Hspad2VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::ClrPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2RefPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ClrPdruncfg0 = 1,
}
impl From<Hspad2RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad2RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_REF_PD` writer - no description available"]
pub type Hspad2RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad2RefPd>;
impl<'a, REG> Hspad2RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::NoEffect)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn clr_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::ClrPdruncfg0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pdruncfg0ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn pmic_mode0(&mut self) -> PmicMode0W<Pdruncfg0ClrSpec> {
        PmicMode0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn pmic_mode1(&mut self) -> PmicMode1W<Pdruncfg0ClrSpec> {
        PmicMode1W::new(self, 2)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn vddcorereg_lp(&mut self) -> VddcoreregLpW<Pdruncfg0ClrSpec> {
        VddcoreregLpW::new(self, 4)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn pmcref_lp(&mut self) -> PmcrefLpW<Pdruncfg0ClrSpec> {
        PmcrefLpW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn hvd1v8_pd(&mut self) -> Hvd1v8PdW<Pdruncfg0ClrSpec> {
        Hvd1v8PdW::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn porcore_lp(&mut self) -> PorcoreLpW<Pdruncfg0ClrSpec> {
        PorcoreLpW::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn lvdcore_lp(&mut self) -> LvdcoreLpW<Pdruncfg0ClrSpec> {
        LvdcoreLpW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn hvdcore_pd(&mut self) -> HvdcorePdW<Pdruncfg0ClrSpec> {
        HvdcorePdW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn rbb_pd(&mut self) -> RbbPdW<Pdruncfg0ClrSpec> {
        RbbPdW::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn fbb_pd(&mut self) -> FbbPdW<Pdruncfg0ClrSpec> {
        FbbPdW::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn sysxtal_pd(&mut self) -> SysxtalPdW<Pdruncfg0ClrSpec> {
        SysxtalPdW::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn lposc_pd(&mut self) -> LposcPdW<Pdruncfg0ClrSpec> {
        LposcPdW::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn sfro_pd(&mut self) -> SfroPdW<Pdruncfg0ClrSpec> {
        SfroPdW::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn ffro_pd(&mut self) -> FfroPdW<Pdruncfg0ClrSpec> {
        FfroPdW::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn syspllldo_pd(&mut self) -> SyspllldoPdW<Pdruncfg0ClrSpec> {
        SyspllldoPdW::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn syspllana_pd(&mut self) -> SyspllanaPdW<Pdruncfg0ClrSpec> {
        SyspllanaPdW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn audpllldo_pd(&mut self) -> AudpllldoPdW<Pdruncfg0ClrSpec> {
        AudpllldoPdW::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    pub fn audpllana_pd(&mut self) -> AudpllanaPdW<Pdruncfg0ClrSpec> {
        AudpllanaPdW::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> AdcPdW<Pdruncfg0ClrSpec> {
        AdcPdW::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn adc_lp(&mut self) -> AdcLpW<Pdruncfg0ClrSpec> {
        AdcLpW::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn adctempsns_pd(&mut self) -> AdctempsnsPdW<Pdruncfg0ClrSpec> {
        AdctempsnsPdW::new(self, 23)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn acmp_pd(&mut self) -> AcmpPdW<Pdruncfg0ClrSpec> {
        AcmpPdW::new(self, 25)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn hspad0_vdet_lp(&mut self) -> Hspad0VdetLpW<Pdruncfg0ClrSpec> {
        Hspad0VdetLpW::new(self, 26)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn hspad0_ref_pd(&mut self) -> Hspad0RefPdW<Pdruncfg0ClrSpec> {
        Hspad0RefPdW::new(self, 27)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn hspad2_vdet_lp(&mut self) -> Hspad2VdetLpW<Pdruncfg0ClrSpec> {
        Hspad2VdetLpW::new(self, 28)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    pub fn hspad2_ref_pd(&mut self) -> Hspad2RefPdW<Pdruncfg0ClrSpec> {
        Hspad2RefPdW::new(self, 29)
    }
}
#[doc = "Run configuration 0 clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg0ClrSpec;
impl crate::RegisterSpec for Pdruncfg0ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfg0_clr::W`](W) writer structure"]
impl crate::Writable for Pdruncfg0ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG0_CLR to value 0"]
impl crate::Resettable for Pdruncfg0ClrSpec {
    const RESET_VALUE: u32 = 0;
}
