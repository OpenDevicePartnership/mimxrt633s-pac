#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `MDPCP` reader - MDPC Present"]
pub type MdpcpR = crate::BitReader;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Operating in Normal mode (NRM)"]
    Mode0 = 0,
    #[doc = "1: Unused (reserved)"]
    Mode1 = 1,
    #[doc = "2: Unused (reserved)"]
    Mode2 = 2,
    #[doc = "3: Operating in Logically Disabled Mode (LDM)"]
    Mode3 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Mode0,
            1 => Mode::Mode1,
            2 => Mode::Mode2,
            3 => Mode::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "Operating in Normal mode (NRM)"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "Unused (reserved)"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
    }
    #[doc = "Unused (reserved)"]
    #[inline(always)]
    pub fn is_mode_2(&self) -> bool {
        *self == Mode::Mode2
    }
    #[doc = "Operating in Logically Disabled Mode (LDM)"]
    #[inline(always)]
    pub fn is_mode_3(&self) -> bool {
        *self == Mode::Mode3
    }
}
#[doc = "Field `NCTX` reader - Number of Contexts"]
pub type NctxR = crate::FieldReader;
#[doc = "Field `HRL` reader - Hardware Revision Level"]
pub type HrlR = crate::FieldReader;
#[doc = "Restricted Register Access Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rram {
    #[doc = "0: Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    Rram0 = 0,
    #[doc = "1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Rram1 = 1,
}
impl From<Rram> for bool {
    #[inline(always)]
    fn from(variant: Rram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRAM` reader - Restricted Register Access Mode"]
pub type RramR = crate::BitReader<Rram>;
impl RramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rram {
        match self.bits {
            false => Rram::Rram0,
            true => Rram::Rram1,
        }
    }
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    #[inline(always)]
    pub fn is_rram_0(&self) -> bool {
        *self == Rram::Rram0
    }
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    #[inline(always)]
    pub fn is_rram_1(&self) -> bool {
        *self == Rram::Rram1
    }
}
#[doc = "Global Enable Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gem {
    #[doc = "0: OTFAD is disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    Gem0 = 0,
    #[doc = "1: OTFAD is enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    Gem1 = 1,
}
impl From<Gem> for bool {
    #[inline(always)]
    fn from(variant: Gem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEM` reader - Global Enable Mode"]
pub type GemR = crate::BitReader<Gem>;
impl GemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gem {
        match self.bits {
            false => Gem::Gem0,
            true => Gem::Gem1,
        }
    }
    #[doc = "OTFAD is disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    #[inline(always)]
    pub fn is_gem_0(&self) -> bool {
        *self == Gem::Gem0
    }
    #[doc = "OTFAD is enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    #[inline(always)]
    pub fn is_gem_1(&self) -> bool {
        *self == Gem::Gem1
    }
}
impl R {
    #[doc = "Bit 1 - MDPC Present"]
    #[inline(always)]
    pub fn mdpcp(&self) -> MdpcpR {
        MdpcpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Number of Contexts"]
    #[inline(always)]
    pub fn nctx(&self) -> NctxR {
        NctxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Hardware Revision Level"]
    #[inline(always)]
    pub fn hrl(&self) -> HrlR {
        HrlR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Restricted Register Access Mode"]
    #[inline(always)]
    pub fn rram(&self) -> RramR {
        RramR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Global Enable Mode"]
    #[inline(always)]
    pub fn gem(&self) -> GemR {
        GemR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("mdpcp", &self.mdpcp())
            .field("mode", &self.mode())
            .field("nctx", &self.nctx())
            .field("hrl", &self.hrl())
            .field("rram", &self.rram())
            .field("gem", &self.gem())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x40;
}
