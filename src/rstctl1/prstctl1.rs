#[doc = "Register `PRSTCTL1` reader"]
pub type R = crate::R<Prstctl1Spec>;
#[doc = "Register `PRSTCTL1` writer"]
pub type W = crate::W<Prstctl1Spec>;
#[doc = "HSGPIO0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio0Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0_RST` reader - HSGPIO0 reset control"]
pub type Hsgpio0RstR = crate::BitReader<Hsgpio0Rst>;
impl Hsgpio0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio0Rst {
        match self.bits {
            false => Hsgpio0Rst::ClearReset,
            true => Hsgpio0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio0Rst::SetReset
    }
}
#[doc = "Field `HSGPIO0_RST` writer - HSGPIO0 reset control"]
pub type Hsgpio0RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio0Rst>;
impl<'a, REG> Hsgpio0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio0Rst::SetReset)
    }
}
#[doc = "HSGPIO1 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio1Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio1Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1_RST` reader - HSGPIO1 reset control"]
pub type Hsgpio1RstR = crate::BitReader<Hsgpio1Rst>;
impl Hsgpio1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio1Rst {
        match self.bits {
            false => Hsgpio1Rst::ClearReset,
            true => Hsgpio1Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio1Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio1Rst::SetReset
    }
}
#[doc = "Field `HSGPIO1_RST` writer - HSGPIO1 reset control"]
pub type Hsgpio1RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio1Rst>;
impl<'a, REG> Hsgpio1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio1Rst::SetReset)
    }
}
#[doc = "HSGPIO2 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio2Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio2Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2_RST` reader - HSGPIO2 reset control"]
pub type Hsgpio2RstR = crate::BitReader<Hsgpio2Rst>;
impl Hsgpio2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio2Rst {
        match self.bits {
            false => Hsgpio2Rst::ClearReset,
            true => Hsgpio2Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio2Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio2Rst::SetReset
    }
}
#[doc = "Field `HSGPIO2_RST` writer - HSGPIO2 reset control"]
pub type Hsgpio2RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio2Rst>;
impl<'a, REG> Hsgpio2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio2Rst::SetReset)
    }
}
#[doc = "HSGPIO3 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio3Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio3Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio3Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3_RST` reader - HSGPIO3 reset control"]
pub type Hsgpio3RstR = crate::BitReader<Hsgpio3Rst>;
impl Hsgpio3RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio3Rst {
        match self.bits {
            false => Hsgpio3Rst::ClearReset,
            true => Hsgpio3Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio3Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio3Rst::SetReset
    }
}
#[doc = "Field `HSGPIO3_RST` writer - HSGPIO3 reset control"]
pub type Hsgpio3RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio3Rst>;
impl<'a, REG> Hsgpio3RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio3Rst::SetReset)
    }
}
#[doc = "HSGPIO4 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio4Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio4Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio4Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4_RST` reader - HSGPIO4 reset control"]
pub type Hsgpio4RstR = crate::BitReader<Hsgpio4Rst>;
impl Hsgpio4RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio4Rst {
        match self.bits {
            false => Hsgpio4Rst::ClearReset,
            true => Hsgpio4Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio4Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio4Rst::SetReset
    }
}
#[doc = "Field `HSGPIO4_RST` writer - HSGPIO4 reset control"]
pub type Hsgpio4RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio4Rst>;
impl<'a, REG> Hsgpio4RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio4Rst::SetReset)
    }
}
#[doc = "HSGPIO5 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio5Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio5Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio5Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5_RST` reader - HSGPIO5 reset control"]
pub type Hsgpio5RstR = crate::BitReader<Hsgpio5Rst>;
impl Hsgpio5RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio5Rst {
        match self.bits {
            false => Hsgpio5Rst::ClearReset,
            true => Hsgpio5Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio5Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio5Rst::SetReset
    }
}
#[doc = "Field `HSGPIO5_RST` writer - HSGPIO5 reset control"]
pub type Hsgpio5RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio5Rst>;
impl<'a, REG> Hsgpio5RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio5Rst::SetReset)
    }
}
#[doc = "HSGPIO6 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio6Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio6Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio6Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6_RST` reader - HSGPIO6 reset control"]
pub type Hsgpio6RstR = crate::BitReader<Hsgpio6Rst>;
impl Hsgpio6RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio6Rst {
        match self.bits {
            false => Hsgpio6Rst::ClearReset,
            true => Hsgpio6Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio6Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio6Rst::SetReset
    }
}
#[doc = "Field `HSGPIO6_RST` writer - HSGPIO6 reset control"]
pub type Hsgpio6RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio6Rst>;
impl<'a, REG> Hsgpio6RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio6Rst::SetReset)
    }
}
#[doc = "HSGPIO7 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsgpio7Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Hsgpio7Rst> for bool {
    #[inline(always)]
    fn from(variant: Hsgpio7Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7_RST` reader - HSGPIO7 reset control"]
pub type Hsgpio7RstR = crate::BitReader<Hsgpio7Rst>;
impl Hsgpio7RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsgpio7Rst {
        match self.bits {
            false => Hsgpio7Rst::ClearReset,
            true => Hsgpio7Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Hsgpio7Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Hsgpio7Rst::SetReset
    }
}
#[doc = "Field `HSGPIO7_RST` writer - HSGPIO7 reset control"]
pub type Hsgpio7RstW<'a, REG> = crate::BitWriter<'a, REG, Hsgpio7Rst>;
impl<'a, REG> Hsgpio7RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Hsgpio7Rst::SetReset)
    }
}
#[doc = "CRC reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<CrcRst> for bool {
    #[inline(always)]
    fn from(variant: CrcRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_RST` reader - CRC reset control"]
pub type CrcRstR = crate::BitReader<CrcRst>;
impl CrcRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcRst {
        match self.bits {
            false => CrcRst::ClearReset,
            true => CrcRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == CrcRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == CrcRst::SetReset
    }
}
#[doc = "Field `CRC_RST` writer - CRC reset control"]
pub type CrcRstW<'a, REG> = crate::BitWriter<'a, REG, CrcRst>;
impl<'a, REG> CrcRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(CrcRst::SetReset)
    }
}
#[doc = "DMAC0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Dmac0Rst> for bool {
    #[inline(always)]
    fn from(variant: Dmac0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0_RST` reader - DMAC0 reset control"]
pub type Dmac0RstR = crate::BitReader<Dmac0Rst>;
impl Dmac0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0Rst {
        match self.bits {
            false => Dmac0Rst::ClearReset,
            true => Dmac0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Dmac0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Dmac0Rst::SetReset
    }
}
#[doc = "Field `DMAC0_RST` writer - DMAC0 reset control"]
pub type Dmac0RstW<'a, REG> = crate::BitWriter<'a, REG, Dmac0Rst>;
impl<'a, REG> Dmac0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0Rst::SetReset)
    }
}
#[doc = "DMAC1 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Dmac1Rst> for bool {
    #[inline(always)]
    fn from(variant: Dmac1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1_RST` reader - DMAC1 reset control"]
pub type Dmac1RstR = crate::BitReader<Dmac1Rst>;
impl Dmac1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1Rst {
        match self.bits {
            false => Dmac1Rst::ClearReset,
            true => Dmac1Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Dmac1Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Dmac1Rst::SetReset
    }
}
#[doc = "Field `DMAC1_RST` writer - DMAC1 reset control"]
pub type Dmac1RstW<'a, REG> = crate::BitWriter<'a, REG, Dmac1Rst>;
impl<'a, REG> Dmac1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1Rst::SetReset)
    }
}
#[doc = "MU reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<MuRst> for bool {
    #[inline(always)]
    fn from(variant: MuRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU_RST` reader - MU reset control"]
pub type MuRstR = crate::BitReader<MuRst>;
impl MuRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuRst {
        match self.bits {
            false => MuRst::ClearReset,
            true => MuRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == MuRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == MuRst::SetReset
    }
}
#[doc = "Field `MU_RST` writer - MU reset control"]
pub type MuRstW<'a, REG> = crate::BitWriter<'a, REG, MuRst>;
impl<'a, REG> MuRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(MuRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(MuRst::SetReset)
    }
}
#[doc = "SEMA reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemaRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<SemaRst> for bool {
    #[inline(always)]
    fn from(variant: SemaRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA_RST` reader - SEMA reset control"]
pub type SemaRstR = crate::BitReader<SemaRst>;
impl SemaRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SemaRst {
        match self.bits {
            false => SemaRst::ClearReset,
            true => SemaRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == SemaRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == SemaRst::SetReset
    }
}
#[doc = "Field `SEMA_RST` writer - SEMA reset control"]
pub type SemaRstW<'a, REG> = crate::BitWriter<'a, REG, SemaRst>;
impl<'a, REG> SemaRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SemaRst::SetReset)
    }
}
#[doc = "FREQME reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreqmeRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<FreqmeRst> for bool {
    #[inline(always)]
    fn from(variant: FreqmeRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_RST` reader - FREQME reset control"]
pub type FreqmeRstR = crate::BitReader<FreqmeRst>;
impl FreqmeRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FreqmeRst {
        match self.bits {
            false => FreqmeRst::ClearReset,
            true => FreqmeRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == FreqmeRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == FreqmeRst::SetReset
    }
}
#[doc = "Field `FREQME_RST` writer - FREQME reset control"]
pub type FreqmeRstW<'a, REG> = crate::BitWriter<'a, REG, FreqmeRst>;
impl<'a, REG> FreqmeRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FreqmeRst::SetReset)
    }
}
impl R {
    #[doc = "Bit 0 - HSGPIO0 reset control"]
    #[inline(always)]
    pub fn hsgpio0_rst(&self) -> Hsgpio0RstR {
        Hsgpio0RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSGPIO1 reset control"]
    #[inline(always)]
    pub fn hsgpio1_rst(&self) -> Hsgpio1RstR {
        Hsgpio1RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSGPIO2 reset control"]
    #[inline(always)]
    pub fn hsgpio2_rst(&self) -> Hsgpio2RstR {
        Hsgpio2RstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSGPIO3 reset control"]
    #[inline(always)]
    pub fn hsgpio3_rst(&self) -> Hsgpio3RstR {
        Hsgpio3RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSGPIO4 reset control"]
    #[inline(always)]
    pub fn hsgpio4_rst(&self) -> Hsgpio4RstR {
        Hsgpio4RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSGPIO5 reset control"]
    #[inline(always)]
    pub fn hsgpio5_rst(&self) -> Hsgpio5RstR {
        Hsgpio5RstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSGPIO6 reset control"]
    #[inline(always)]
    pub fn hsgpio6_rst(&self) -> Hsgpio6RstR {
        Hsgpio6RstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSGPIO7 reset control"]
    #[inline(always)]
    pub fn hsgpio7_rst(&self) -> Hsgpio7RstR {
        Hsgpio7RstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC reset control"]
    #[inline(always)]
    pub fn crc_rst(&self) -> CrcRstR {
        CrcRstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC0 reset control"]
    #[inline(always)]
    pub fn dmac0_rst(&self) -> Dmac0RstR {
        Dmac0RstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC1 reset control"]
    #[inline(always)]
    pub fn dmac1_rst(&self) -> Dmac1RstR {
        Dmac1RstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - MU reset control"]
    #[inline(always)]
    pub fn mu_rst(&self) -> MuRstR {
        MuRstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SEMA reset control"]
    #[inline(always)]
    pub fn sema_rst(&self) -> SemaRstR {
        SemaRstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - FREQME reset control"]
    #[inline(always)]
    pub fn freqme_rst(&self) -> FreqmeRstR {
        FreqmeRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL1")
            .field("hsgpio0_rst", &self.hsgpio0_rst())
            .field("hsgpio1_rst", &self.hsgpio1_rst())
            .field("hsgpio2_rst", &self.hsgpio2_rst())
            .field("hsgpio3_rst", &self.hsgpio3_rst())
            .field("hsgpio4_rst", &self.hsgpio4_rst())
            .field("hsgpio5_rst", &self.hsgpio5_rst())
            .field("hsgpio6_rst", &self.hsgpio6_rst())
            .field("hsgpio7_rst", &self.hsgpio7_rst())
            .field("crc_rst", &self.crc_rst())
            .field("dmac0_rst", &self.dmac0_rst())
            .field("dmac1_rst", &self.dmac1_rst())
            .field("mu_rst", &self.mu_rst())
            .field("sema_rst", &self.sema_rst())
            .field("freqme_rst", &self.freqme_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 reset control"]
    #[inline(always)]
    pub fn hsgpio0_rst(&mut self) -> Hsgpio0RstW<Prstctl1Spec> {
        Hsgpio0RstW::new(self, 0)
    }
    #[doc = "Bit 1 - HSGPIO1 reset control"]
    #[inline(always)]
    pub fn hsgpio1_rst(&mut self) -> Hsgpio1RstW<Prstctl1Spec> {
        Hsgpio1RstW::new(self, 1)
    }
    #[doc = "Bit 2 - HSGPIO2 reset control"]
    #[inline(always)]
    pub fn hsgpio2_rst(&mut self) -> Hsgpio2RstW<Prstctl1Spec> {
        Hsgpio2RstW::new(self, 2)
    }
    #[doc = "Bit 3 - HSGPIO3 reset control"]
    #[inline(always)]
    pub fn hsgpio3_rst(&mut self) -> Hsgpio3RstW<Prstctl1Spec> {
        Hsgpio3RstW::new(self, 3)
    }
    #[doc = "Bit 4 - HSGPIO4 reset control"]
    #[inline(always)]
    pub fn hsgpio4_rst(&mut self) -> Hsgpio4RstW<Prstctl1Spec> {
        Hsgpio4RstW::new(self, 4)
    }
    #[doc = "Bit 5 - HSGPIO5 reset control"]
    #[inline(always)]
    pub fn hsgpio5_rst(&mut self) -> Hsgpio5RstW<Prstctl1Spec> {
        Hsgpio5RstW::new(self, 5)
    }
    #[doc = "Bit 6 - HSGPIO6 reset control"]
    #[inline(always)]
    pub fn hsgpio6_rst(&mut self) -> Hsgpio6RstW<Prstctl1Spec> {
        Hsgpio6RstW::new(self, 6)
    }
    #[doc = "Bit 7 - HSGPIO7 reset control"]
    #[inline(always)]
    pub fn hsgpio7_rst(&mut self) -> Hsgpio7RstW<Prstctl1Spec> {
        Hsgpio7RstW::new(self, 7)
    }
    #[doc = "Bit 16 - CRC reset control"]
    #[inline(always)]
    pub fn crc_rst(&mut self) -> CrcRstW<Prstctl1Spec> {
        CrcRstW::new(self, 16)
    }
    #[doc = "Bit 23 - DMAC0 reset control"]
    #[inline(always)]
    pub fn dmac0_rst(&mut self) -> Dmac0RstW<Prstctl1Spec> {
        Dmac0RstW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC1 reset control"]
    #[inline(always)]
    pub fn dmac1_rst(&mut self) -> Dmac1RstW<Prstctl1Spec> {
        Dmac1RstW::new(self, 24)
    }
    #[doc = "Bit 28 - MU reset control"]
    #[inline(always)]
    pub fn mu_rst(&mut self) -> MuRstW<Prstctl1Spec> {
        MuRstW::new(self, 28)
    }
    #[doc = "Bit 29 - SEMA reset control"]
    #[inline(always)]
    pub fn sema_rst(&mut self) -> SemaRstW<Prstctl1Spec> {
        SemaRstW::new(self, 29)
    }
    #[doc = "Bit 31 - FREQME reset control"]
    #[inline(always)]
    pub fn freqme_rst(&mut self) -> FreqmeRstW<Prstctl1Spec> {
        FreqmeRstW::new(self, 31)
    }
}
#[doc = "peripheral reset control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl1Spec;
impl crate::RegisterSpec for Prstctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstctl1::R`](R) reader structure"]
impl crate::Readable for Prstctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`prstctl1::W`](W) writer structure"]
impl crate::Writable for Prstctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL1 to value 0xb181_00ff"]
impl crate::Resettable for Prstctl1Spec {
    const RESET_VALUE: u32 = 0xb181_00ff;
}
