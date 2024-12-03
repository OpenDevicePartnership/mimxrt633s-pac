#[doc = "Register `PRSTCTL2` reader"]
pub type R = crate::R<Prstctl2Spec>;
#[doc = "Register `PRSTCTL2` writer"]
pub type W = crate::W<Prstctl2Spec>;
#[doc = "CT32BIT0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Ct32bit0Rst> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_RST` reader - CT32BIT0 reset control"]
pub type Ct32bit0RstR = crate::BitReader<Ct32bit0Rst>;
impl Ct32bit0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit0Rst {
        match self.bits {
            false => Ct32bit0Rst::ClearReset,
            true => Ct32bit0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Ct32bit0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Ct32bit0Rst::SetReset
    }
}
#[doc = "Field `CT32BIT0_RST` writer - CT32BIT0 reset control"]
pub type Ct32bit0RstW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit0Rst>;
impl<'a, REG> Ct32bit0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit0Rst::SetReset)
    }
}
#[doc = "CT32BIT1 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit1Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Ct32bit1Rst> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_RST` reader - CT32BIT1 reset control"]
pub type Ct32bit1RstR = crate::BitReader<Ct32bit1Rst>;
impl Ct32bit1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit1Rst {
        match self.bits {
            false => Ct32bit1Rst::ClearReset,
            true => Ct32bit1Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Ct32bit1Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Ct32bit1Rst::SetReset
    }
}
#[doc = "Field `CT32BIT1_RST` writer - CT32BIT1 reset control"]
pub type Ct32bit1RstW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit1Rst>;
impl<'a, REG> Ct32bit1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit1Rst::SetReset)
    }
}
#[doc = "CT32BIT2 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit2Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Ct32bit2Rst> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit2Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_RST` reader - CT32BIT2 reset control"]
pub type Ct32bit2RstR = crate::BitReader<Ct32bit2Rst>;
impl Ct32bit2RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit2Rst {
        match self.bits {
            false => Ct32bit2Rst::ClearReset,
            true => Ct32bit2Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Ct32bit2Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Ct32bit2Rst::SetReset
    }
}
#[doc = "Field `CT32BIT2_RST` writer - CT32BIT2 reset control"]
pub type Ct32bit2RstW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit2Rst>;
impl<'a, REG> Ct32bit2RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit2Rst::SetReset)
    }
}
#[doc = "CT32BIT3 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit3Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Ct32bit3Rst> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit3Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_RST` reader - CT32BIT3 reset control"]
pub type Ct32bit3RstR = crate::BitReader<Ct32bit3Rst>;
impl Ct32bit3RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit3Rst {
        match self.bits {
            false => Ct32bit3Rst::ClearReset,
            true => Ct32bit3Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Ct32bit3Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Ct32bit3Rst::SetReset
    }
}
#[doc = "Field `CT32BIT3_RST` writer - CT32BIT3 reset control"]
pub type Ct32bit3RstW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit3Rst>;
impl<'a, REG> Ct32bit3RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit3Rst::SetReset)
    }
}
#[doc = "CT32BIT4 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ct32bit4Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Ct32bit4Rst> for bool {
    #[inline(always)]
    fn from(variant: Ct32bit4Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_RST` reader - CT32BIT4 reset control"]
pub type Ct32bit4RstR = crate::BitReader<Ct32bit4Rst>;
impl Ct32bit4RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ct32bit4Rst {
        match self.bits {
            false => Ct32bit4Rst::ClearReset,
            true => Ct32bit4Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Ct32bit4Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Ct32bit4Rst::SetReset
    }
}
#[doc = "Field `CT32BIT4_RST` writer - CT32BIT4 reset control"]
pub type Ct32bit4RstW<'a, REG> = crate::BitWriter<'a, REG, Ct32bit4Rst>;
impl<'a, REG> Ct32bit4RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ct32bit4Rst::SetReset)
    }
}
#[doc = "MRT0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrt0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Mrt0Rst> for bool {
    #[inline(always)]
    fn from(variant: Mrt0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0_RST` reader - MRT0 reset control"]
pub type Mrt0RstR = crate::BitReader<Mrt0Rst>;
impl Mrt0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrt0Rst {
        match self.bits {
            false => Mrt0Rst::ClearReset,
            true => Mrt0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Mrt0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Mrt0Rst::SetReset
    }
}
#[doc = "Field `MRT0_RST` writer - MRT0 reset control"]
pub type Mrt0RstW<'a, REG> = crate::BitWriter<'a, REG, Mrt0Rst>;
impl<'a, REG> Mrt0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mrt0Rst::SetReset)
    }
}
#[doc = "WWDT1 reset control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt1Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Wwdt1Rst> for bool {
    #[inline(always)]
    fn from(variant: Wwdt1Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1_RST` reader - WWDT1 reset control"]
pub type Wwdt1RstR = crate::BitReader<Wwdt1Rst>;
impl Wwdt1RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt1Rst {
        match self.bits {
            false => Wwdt1Rst::ClearReset,
            true => Wwdt1Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Wwdt1Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Wwdt1Rst::SetReset
    }
}
#[doc = "Field `WWDT1_RST` writer - WWDT1 reset control"]
pub type Wwdt1RstW<'a, REG> = crate::BitWriter<'a, REG, Wwdt1Rst>;
impl<'a, REG> Wwdt1RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt1Rst::SetReset)
    }
}
#[doc = "I3C0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3c0Rst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<I3c0Rst> for bool {
    #[inline(always)]
    fn from(variant: I3c0Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0_RST` reader - I3C0 reset control"]
pub type I3c0RstR = crate::BitReader<I3c0Rst>;
impl I3c0RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3c0Rst {
        match self.bits {
            false => I3c0Rst::ClearReset,
            true => I3c0Rst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == I3c0Rst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == I3c0Rst::SetReset
    }
}
#[doc = "Field `I3C0_RST` writer - I3C0 reset control"]
pub type I3c0RstW<'a, REG> = crate::BitWriter<'a, REG, I3c0Rst>;
impl<'a, REG> I3c0RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(I3c0Rst::SetReset)
    }
}
#[doc = "GPIOINTCTL reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpiointctlRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<GpiointctlRst> for bool {
    #[inline(always)]
    fn from(variant: GpiointctlRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL_RST` reader - GPIOINTCTL reset control"]
pub type GpiointctlRstR = crate::BitReader<GpiointctlRst>;
impl GpiointctlRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpiointctlRst {
        match self.bits {
            false => GpiointctlRst::ClearReset,
            true => GpiointctlRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == GpiointctlRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == GpiointctlRst::SetReset
    }
}
#[doc = "Field `GPIOINTCTL_RST` writer - GPIOINTCTL reset control"]
pub type GpiointctlRstW<'a, REG> = crate::BitWriter<'a, REG, GpiointctlRst>;
impl<'a, REG> GpiointctlRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(GpiointctlRst::SetReset)
    }
}
#[doc = "PMC reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PimctlRst {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<PimctlRst> for bool {
    #[inline(always)]
    fn from(variant: PimctlRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL_RST` reader - PMC reset control"]
pub type PimctlRstR = crate::BitReader<PimctlRst>;
impl PimctlRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PimctlRst {
        match self.bits {
            false => PimctlRst::ClearReset,
            true => PimctlRst::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == PimctlRst::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == PimctlRst::SetReset
    }
}
#[doc = "Field `PIMCTL_RST` writer - PMC reset control"]
pub type PimctlRstW<'a, REG> = crate::BitWriter<'a, REG, PimctlRst>;
impl<'a, REG> PimctlRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRst::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PimctlRst::SetReset)
    }
}
impl R {
    #[doc = "Bit 0 - CT32BIT0 reset control"]
    #[inline(always)]
    pub fn ct32bit0_rst(&self) -> Ct32bit0RstR {
        Ct32bit0RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CT32BIT1 reset control"]
    #[inline(always)]
    pub fn ct32bit1_rst(&self) -> Ct32bit1RstR {
        Ct32bit1RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CT32BIT2 reset control"]
    #[inline(always)]
    pub fn ct32bit2_rst(&self) -> Ct32bit2RstR {
        Ct32bit2RstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CT32BIT3 reset control"]
    #[inline(always)]
    pub fn ct32bit3_rst(&self) -> Ct32bit3RstR {
        Ct32bit3RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CT32BIT4 reset control"]
    #[inline(always)]
    pub fn ct32bit4_rst(&self) -> Ct32bit4RstR {
        Ct32bit4RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MRT0 reset control"]
    #[inline(always)]
    pub fn mrt0_rst(&self) -> Mrt0RstR {
        Mrt0RstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - WWDT1 reset control"]
    #[inline(always)]
    pub fn wwdt1_rst(&self) -> Wwdt1RstR {
        Wwdt1RstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - I3C0 reset control"]
    #[inline(always)]
    pub fn i3c0_rst(&self) -> I3c0RstR {
        I3c0RstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset control"]
    #[inline(always)]
    pub fn gpiointctl_rst(&self) -> GpiointctlRstR {
        GpiointctlRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PMC reset control"]
    #[inline(always)]
    pub fn pimctl_rst(&self) -> PimctlRstR {
        PimctlRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL2")
            .field("ct32bit0_rst", &self.ct32bit0_rst())
            .field("ct32bit1_rst", &self.ct32bit1_rst())
            .field("ct32bit2_rst", &self.ct32bit2_rst())
            .field("ct32bit3_rst", &self.ct32bit3_rst())
            .field("ct32bit4_rst", &self.ct32bit4_rst())
            .field("mrt0_rst", &self.mrt0_rst())
            .field("wwdt1_rst", &self.wwdt1_rst())
            .field("i3c0_rst", &self.i3c0_rst())
            .field("gpiointctl_rst", &self.gpiointctl_rst())
            .field("pimctl_rst", &self.pimctl_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT0 reset control"]
    #[inline(always)]
    pub fn ct32bit0_rst(&mut self) -> Ct32bit0RstW<Prstctl2Spec> {
        Ct32bit0RstW::new(self, 0)
    }
    #[doc = "Bit 1 - CT32BIT1 reset control"]
    #[inline(always)]
    pub fn ct32bit1_rst(&mut self) -> Ct32bit1RstW<Prstctl2Spec> {
        Ct32bit1RstW::new(self, 1)
    }
    #[doc = "Bit 2 - CT32BIT2 reset control"]
    #[inline(always)]
    pub fn ct32bit2_rst(&mut self) -> Ct32bit2RstW<Prstctl2Spec> {
        Ct32bit2RstW::new(self, 2)
    }
    #[doc = "Bit 3 - CT32BIT3 reset control"]
    #[inline(always)]
    pub fn ct32bit3_rst(&mut self) -> Ct32bit3RstW<Prstctl2Spec> {
        Ct32bit3RstW::new(self, 3)
    }
    #[doc = "Bit 4 - CT32BIT4 reset control"]
    #[inline(always)]
    pub fn ct32bit4_rst(&mut self) -> Ct32bit4RstW<Prstctl2Spec> {
        Ct32bit4RstW::new(self, 4)
    }
    #[doc = "Bit 8 - MRT0 reset control"]
    #[inline(always)]
    pub fn mrt0_rst(&mut self) -> Mrt0RstW<Prstctl2Spec> {
        Mrt0RstW::new(self, 8)
    }
    #[doc = "Bit 10 - WWDT1 reset control"]
    #[inline(always)]
    pub fn wwdt1_rst(&mut self) -> Wwdt1RstW<Prstctl2Spec> {
        Wwdt1RstW::new(self, 10)
    }
    #[doc = "Bit 16 - I3C0 reset control"]
    #[inline(always)]
    pub fn i3c0_rst(&mut self) -> I3c0RstW<Prstctl2Spec> {
        I3c0RstW::new(self, 16)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset control"]
    #[inline(always)]
    pub fn gpiointctl_rst(&mut self) -> GpiointctlRstW<Prstctl2Spec> {
        GpiointctlRstW::new(self, 30)
    }
    #[doc = "Bit 31 - PMC reset control"]
    #[inline(always)]
    pub fn pimctl_rst(&mut self) -> PimctlRstW<Prstctl2Spec> {
        PimctlRstW::new(self, 31)
    }
}
#[doc = "peripheral reset control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl2Spec;
impl crate::RegisterSpec for Prstctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstctl2::R`](R) reader structure"]
impl crate::Readable for Prstctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`prstctl2::W`](W) writer structure"]
impl crate::Writable for Prstctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL2 to value 0xc001_011f"]
impl crate::Resettable for Prstctl2Spec {
    const RESET_VALUE: u32 = 0xc001_011f;
}
