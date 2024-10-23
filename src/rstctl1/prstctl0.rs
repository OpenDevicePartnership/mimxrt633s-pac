#[doc = "Register `PRSTCTL0` reader"]
pub type R = crate::R<Prstctl0Spec>;
#[doc = "Register `PRSTCTL0` writer"]
pub type W = crate::W<Prstctl0Spec>;
#[doc = "FLEXCOMM0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm0Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_RST` reader - FLEXCOMM0 reset control"]
pub type Flexcomm0RstR = crate::BitReader<Flexcomm0Rst>;
impl Flexcomm0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0Rst {
        match self.bits {
            false => Flexcomm0Rst::ClearReset,
            true => Flexcomm0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm0Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM0_RST` writer - FLEXCOMM0 reset control"]
pub type Flexcomm0RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Rst>;
impl<'a, REG> Flexcomm0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Rst::SetReset)
    }
}
#[doc = "FLEXCOMM1 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm1Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_RST` reader - FLEXCOMM1 reset control"]
pub type Flexcomm1RstR = crate::BitReader<Flexcomm1Rst>;
impl Flexcomm1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1Rst {
        match self.bits {
            false => Flexcomm1Rst::ClearReset,
            true => Flexcomm1Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm1Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm1Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM1_RST` writer - FLEXCOMM1 reset control"]
pub type Flexcomm1RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Rst>;
impl<'a, REG> Flexcomm1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Rst::SetReset)
    }
}
#[doc = "FLEXCOMM2 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm2Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_RST` reader - FLEXCOMM2 reset control"]
pub type Flexcomm2RstR = crate::BitReader<Flexcomm2Rst>;
impl Flexcomm2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2Rst {
        match self.bits {
            false => Flexcomm2Rst::ClearReset,
            true => Flexcomm2Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm2Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm2Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM2_RST` writer - FLEXCOMM2 reset control"]
pub type Flexcomm2RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Rst>;
impl<'a, REG> Flexcomm2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rst::SetReset)
    }
}
#[doc = "FLEXCOMM3 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm3Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_RST` reader - FLEXCOMM3 reset control"]
pub type Flexcomm3RstR = crate::BitReader<Flexcomm3Rst>;
impl Flexcomm3RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3Rst {
        match self.bits {
            false => Flexcomm3Rst::ClearReset,
            true => Flexcomm3Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm3Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm3Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM3_RST` writer - FLEXCOMM3 reset control"]
pub type Flexcomm3RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Rst>;
impl<'a, REG> Flexcomm3RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rst::SetReset)
    }
}
#[doc = "FLEXCOMM4 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm4Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_RST` reader - FLEXCOMM4 reset control"]
pub type Flexcomm4RstR = crate::BitReader<Flexcomm4Rst>;
impl Flexcomm4RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4Rst {
        match self.bits {
            false => Flexcomm4Rst::ClearReset,
            true => Flexcomm4Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm4Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm4Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM4_RST` writer - FLEXCOMM4 reset control"]
pub type Flexcomm4RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Rst>;
impl<'a, REG> Flexcomm4RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rst::SetReset)
    }
}
#[doc = "FLEXCOMM5 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm5Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_RST` reader - FLEXCOMM5 reset control"]
pub type Flexcomm5RstR = crate::BitReader<Flexcomm5Rst>;
impl Flexcomm5RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5Rst {
        match self.bits {
            false => Flexcomm5Rst::ClearReset,
            true => Flexcomm5Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm5Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm5Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM5_RST` writer - FLEXCOMM5 reset control"]
pub type Flexcomm5RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Rst>;
impl<'a, REG> Flexcomm5RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Rst::SetReset)
    }
}
#[doc = "FLEXCOMM6 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm6Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_RST` reader - FLEXCOMM6 reset control"]
pub type Flexcomm6RstR = crate::BitReader<Flexcomm6Rst>;
impl Flexcomm6RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6Rst {
        match self.bits {
            false => Flexcomm6Rst::ClearReset,
            true => Flexcomm6Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm6Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm6Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM6_RST` writer - FLEXCOMM6 reset control"]
pub type Flexcomm6RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Rst>;
impl<'a, REG> Flexcomm6RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Rst::SetReset)
    }
}
#[doc = "FLEXCOMM7 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm7Rst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_RST` reader - FLEXCOMM7 reset control"]
pub type Flexcomm7RstR = crate::BitReader<Flexcomm7Rst>;
impl Flexcomm7RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7Rst {
        match self.bits {
            false => Flexcomm7Rst::ClearReset,
            true => Flexcomm7Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm7Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm7Rst::SetReset
    }
}
#[doc = "Field `FLEXCOMM7_RST` writer - FLEXCOMM7 reset control"]
pub type Flexcomm7RstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Rst>;
impl<'a, REG> Flexcomm7RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Rst::SetReset)
    }
}
#[doc = "FLEXCOMM14 SPI reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm14SpiRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm14SpiRst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm14SpiRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14_SPI_RST` reader - FLEXCOMM14 SPI reset control"]
pub type Flexcomm14SpiRstR = crate::BitReader<Flexcomm14SpiRst>;
impl Flexcomm14SpiRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm14SpiRst {
        match self.bits {
            false => Flexcomm14SpiRst::ClearReset,
            true => Flexcomm14SpiRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm14SpiRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm14SpiRst::SetReset
    }
}
#[doc = "Field `FLEXCOMM14_SPI_RST` writer - FLEXCOMM14 SPI reset control"]
pub type Flexcomm14SpiRstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm14SpiRst>;
impl<'a, REG> Flexcomm14SpiRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm14SpiRst::SetReset)
    }
}
#[doc = "FLEXCOMM15 I2C reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm15I2cRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Flexcomm15I2cRst> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm15I2cRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15_I2C_RST` reader - FLEXCOMM15 I2C reset control"]
pub type Flexcomm15I2cRstR = crate::BitReader<Flexcomm15I2cRst>;
impl Flexcomm15I2cRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm15I2cRst {
        match self.bits {
            false => Flexcomm15I2cRst::ClearReset,
            true => Flexcomm15I2cRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Flexcomm15I2cRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Flexcomm15I2cRst::SetReset
    }
}
#[doc = "Field `FLEXCOMM15_I2C_RST` writer - FLEXCOMM15 I2C reset control"]
pub type Flexcomm15I2cRstW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm15I2cRst>;
impl<'a, REG> Flexcomm15I2cRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm15I2cRst::SetReset)
    }
}
#[doc = "DMIC0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmic0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Dmic0Rst> for bool {
    #[inline(always)]
    fn from(variant: Dmic0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0_RST` reader - DMIC0 reset control"]
pub type Dmic0RstR = crate::BitReader<Dmic0Rst>;
impl Dmic0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmic0Rst {
        match self.bits {
            false => Dmic0Rst::ClearReset,
            true => Dmic0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Dmic0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Dmic0Rst::SetReset
    }
}
#[doc = "Field `DMIC0_RST` writer - DMIC0 reset control"]
pub type Dmic0RstW<'a, REG> = crate::BitWriter<'a, REG, Dmic0Rst>;
impl<'a, REG> Dmic0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dmic0Rst::SetReset)
    }
}
#[doc = "osevent timer reset control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsevtTimerRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<OsevtTimerRst> for bool {
    #[inline(always)]
    fn from(variant: OsevtTimerRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVT_TIMER_RST` reader - osevent timer reset control"]
pub type OsevtTimerRstR = crate::BitReader<OsevtTimerRst>;
impl OsevtTimerRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OsevtTimerRst {
        match self.bits {
            false => OsevtTimerRst::ClearReset,
            true => OsevtTimerRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == OsevtTimerRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == OsevtTimerRst::SetReset
    }
}
#[doc = "Field `OSEVT_TIMER_RST` writer - osevent timer reset control"]
pub type OsevtTimerRstW<'a, REG> = crate::BitWriter<'a, REG, OsevtTimerRst>;
impl<'a, REG> OsevtTimerRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OsevtTimerRst::SetReset)
    }
}
impl R {
    #[doc = "Bit 8 - FLEXCOMM0 reset control"]
    #[inline(always)]
    pub fn flexcomm0_rst(&self) -> Flexcomm0RstR {
        Flexcomm0RstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLEXCOMM1 reset control"]
    #[inline(always)]
    pub fn flexcomm1_rst(&self) -> Flexcomm1RstR {
        Flexcomm1RstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLEXCOMM2 reset control"]
    #[inline(always)]
    pub fn flexcomm2_rst(&self) -> Flexcomm2RstR {
        Flexcomm2RstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FLEXCOMM3 reset control"]
    #[inline(always)]
    pub fn flexcomm3_rst(&self) -> Flexcomm3RstR {
        Flexcomm3RstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FLEXCOMM4 reset control"]
    #[inline(always)]
    pub fn flexcomm4_rst(&self) -> Flexcomm4RstR {
        Flexcomm4RstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FLEXCOMM5 reset control"]
    #[inline(always)]
    pub fn flexcomm5_rst(&self) -> Flexcomm5RstR {
        Flexcomm5RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FLEXCOMM6 reset control"]
    #[inline(always)]
    pub fn flexcomm6_rst(&self) -> Flexcomm6RstR {
        Flexcomm6RstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLEXCOMM7 reset control"]
    #[inline(always)]
    pub fn flexcomm7_rst(&self) -> Flexcomm7RstR {
        Flexcomm7RstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - FLEXCOMM14 SPI reset control"]
    #[inline(always)]
    pub fn flexcomm14_spi_rst(&self) -> Flexcomm14SpiRstR {
        Flexcomm14SpiRstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FLEXCOMM15 I2C reset control"]
    #[inline(always)]
    pub fn flexcomm15_i2c_rst(&self) -> Flexcomm15I2cRstR {
        Flexcomm15I2cRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMIC0 reset control"]
    #[inline(always)]
    pub fn dmic0_rst(&self) -> Dmic0RstR {
        Dmic0RstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - osevent timer reset control"]
    #[inline(always)]
    pub fn osevt_timer_rst(&self) -> OsevtTimerRstR {
        OsevtTimerRstR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL0")
            .field("flexcomm0_rst", &self.flexcomm0_rst())
            .field("flexcomm1_rst", &self.flexcomm1_rst())
            .field("flexcomm2_rst", &self.flexcomm2_rst())
            .field("flexcomm3_rst", &self.flexcomm3_rst())
            .field("flexcomm4_rst", &self.flexcomm4_rst())
            .field("flexcomm5_rst", &self.flexcomm5_rst())
            .field("flexcomm6_rst", &self.flexcomm6_rst())
            .field("flexcomm7_rst", &self.flexcomm7_rst())
            .field("flexcomm14_spi_rst", &self.flexcomm14_spi_rst())
            .field("flexcomm15_i2c_rst", &self.flexcomm15_i2c_rst())
            .field("dmic0_rst", &self.dmic0_rst())
            .field("osevt_timer_rst", &self.osevt_timer_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - FLEXCOMM0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rst(&mut self) -> Flexcomm0RstW<Prstctl0Spec> {
        Flexcomm0RstW::new(self, 8)
    }
    #[doc = "Bit 9 - FLEXCOMM1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rst(&mut self) -> Flexcomm1RstW<Prstctl0Spec> {
        Flexcomm1RstW::new(self, 9)
    }
    #[doc = "Bit 10 - FLEXCOMM2 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rst(&mut self) -> Flexcomm2RstW<Prstctl0Spec> {
        Flexcomm2RstW::new(self, 10)
    }
    #[doc = "Bit 11 - FLEXCOMM3 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rst(&mut self) -> Flexcomm3RstW<Prstctl0Spec> {
        Flexcomm3RstW::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCOMM4 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_rst(&mut self) -> Flexcomm4RstW<Prstctl0Spec> {
        Flexcomm4RstW::new(self, 12)
    }
    #[doc = "Bit 13 - FLEXCOMM5 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_rst(&mut self) -> Flexcomm5RstW<Prstctl0Spec> {
        Flexcomm5RstW::new(self, 13)
    }
    #[doc = "Bit 14 - FLEXCOMM6 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_rst(&mut self) -> Flexcomm6RstW<Prstctl0Spec> {
        Flexcomm6RstW::new(self, 14)
    }
    #[doc = "Bit 15 - FLEXCOMM7 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_rst(&mut self) -> Flexcomm7RstW<Prstctl0Spec> {
        Flexcomm7RstW::new(self, 15)
    }
    #[doc = "Bit 22 - FLEXCOMM14 SPI reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_spi_rst(&mut self) -> Flexcomm14SpiRstW<Prstctl0Spec> {
        Flexcomm14SpiRstW::new(self, 22)
    }
    #[doc = "Bit 23 - FLEXCOMM15 I2C reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15_i2c_rst(&mut self) -> Flexcomm15I2cRstW<Prstctl0Spec> {
        Flexcomm15I2cRstW::new(self, 23)
    }
    #[doc = "Bit 24 - DMIC0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_rst(&mut self) -> Dmic0RstW<Prstctl0Spec> {
        Dmic0RstW::new(self, 24)
    }
    #[doc = "Bit 27 - osevent timer reset control"]
    #[inline(always)]
    #[must_use]
    pub fn osevt_timer_rst(&mut self) -> OsevtTimerRstW<Prstctl0Spec> {
        OsevtTimerRstW::new(self, 27)
    }
}
#[doc = "peripheral reset control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl0Spec;
impl crate::RegisterSpec for Prstctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstctl0::R`](R) reader structure"]
impl crate::Readable for Prstctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`prstctl0::W`](W) writer structure"]
impl crate::Writable for Prstctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL0 to value 0x01c0_ff00"]
impl crate::Resettable for Prstctl0Spec {
    const RESET_VALUE: u32 = 0x01c0_ff00;
}
