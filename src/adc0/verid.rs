#[doc = "Register `VERID` reader"]
pub type R = crate::R<VeridSpec>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Res {
    #[doc = "0: Up to 13-bit differential/12-bit single ended resolution supported."]
    Res0 = 0,
    #[doc = "1: Up to 16-bit differential/15-bit single ended resolution supported."]
    Res1 = 1,
}
impl From<Res> for bool {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RES` reader - Resolution"]
pub type ResR = crate::BitReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Res {
        match self.bits {
            false => Res::Res0,
            true => Res::Res1,
        }
    }
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    #[inline(always)]
    pub fn is_res_0(&self) -> bool {
        *self == Res::Res0
    }
    #[doc = "Up to 16-bit differential/15-bit single ended resolution supported."]
    #[inline(always)]
    pub fn is_res_1(&self) -> bool {
        *self == Res::Res1
    }
}
#[doc = "Differential Supported\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diffen {
    #[doc = "0: Differential operation not supported."]
    Diffen0 = 0,
    #[doc = "1: Differential operation supported. CMDLa\\[DIFF\\]
and CMDLa\\[ABSEL\\]
control fields implemented."]
    Diffen1 = 1,
}
impl From<Diffen> for bool {
    #[inline(always)]
    fn from(variant: Diffen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFEN` reader - Differential Supported"]
pub type DiffenR = crate::BitReader<Diffen>;
impl DiffenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diffen {
        match self.bits {
            false => Diffen::Diffen0,
            true => Diffen::Diffen1,
        }
    }
    #[doc = "Differential operation not supported."]
    #[inline(always)]
    pub fn is_diffen_0(&self) -> bool {
        *self == Diffen::Diffen0
    }
    #[doc = "Differential operation supported. CMDLa\\[DIFF\\]
and CMDLa\\[ABSEL\\]
control fields implemented."]
    #[inline(always)]
    pub fn is_diffen_1(&self) -> bool {
        *self == Diffen::Diffen1
    }
}
#[doc = "Multi Vref Implemented\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mvi {
    #[doc = "0: Single voltage reference high (VREFH) input supported."]
    Mvi0 = 0,
    #[doc = "1: Multiple voltage reference high (VREFH) inputs supported."]
    Mvi1 = 1,
}
impl From<Mvi> for bool {
    #[inline(always)]
    fn from(variant: Mvi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MVI` reader - Multi Vref Implemented"]
pub type MviR = crate::BitReader<Mvi>;
impl MviR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mvi {
        match self.bits {
            false => Mvi::Mvi0,
            true => Mvi::Mvi1,
        }
    }
    #[doc = "Single voltage reference high (VREFH) input supported."]
    #[inline(always)]
    pub fn is_mvi_0(&self) -> bool {
        *self == Mvi::Mvi0
    }
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    #[inline(always)]
    pub fn is_mvi_1(&self) -> bool {
        *self == Mvi::Mvi1
    }
}
#[doc = "Channel Scale Width\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csw {
    #[doc = "0: Channel scaling not supported."]
    Csw0 = 0,
    #[doc = "1: Channel scaling supported. 1-bit CSCALE control field."]
    Csw1 = 1,
    #[doc = "6: Channel scaling supported. 6-bit CSCALE control field."]
    Csw6 = 6,
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(variant: Csw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csw {
    type Ux = u8;
}
impl crate::IsEnum for Csw {}
#[doc = "Field `CSW` reader - Channel Scale Width"]
pub type CswR = crate::FieldReader<Csw>;
impl CswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csw> {
        match self.bits {
            0 => Some(Csw::Csw0),
            1 => Some(Csw::Csw1),
            6 => Some(Csw::Csw6),
            _ => None,
        }
    }
    #[doc = "Channel scaling not supported."]
    #[inline(always)]
    pub fn is_csw_0(&self) -> bool {
        *self == Csw::Csw0
    }
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    #[inline(always)]
    pub fn is_csw_1(&self) -> bool {
        *self == Csw::Csw1
    }
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    #[inline(always)]
    pub fn is_csw_6(&self) -> bool {
        *self == Csw::Csw6
    }
}
#[doc = "Voltage Reference 1 Range Control Bit Implemented\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vr1rngi {
    #[doc = "0: Range control not required. CFG\\[VREF1RNG\\]
is not implemented."]
    Vr1rngi0 = 0,
    #[doc = "1: Range control required. CFG\\[VREF1RNG\\]
is implemented."]
    Vr1rngi1 = 1,
}
impl From<Vr1rngi> for bool {
    #[inline(always)]
    fn from(variant: Vr1rngi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VR1RNGI` reader - Voltage Reference 1 Range Control Bit Implemented"]
pub type Vr1rngiR = crate::BitReader<Vr1rngi>;
impl Vr1rngiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vr1rngi {
        match self.bits {
            false => Vr1rngi::Vr1rngi0,
            true => Vr1rngi::Vr1rngi1,
        }
    }
    #[doc = "Range control not required. CFG\\[VREF1RNG\\]
is not implemented."]
    #[inline(always)]
    pub fn is_vr1rngi_0(&self) -> bool {
        *self == Vr1rngi::Vr1rngi0
    }
    #[doc = "Range control required. CFG\\[VREF1RNG\\]
is implemented."]
    #[inline(always)]
    pub fn is_vr1rngi_1(&self) -> bool {
        *self == Vr1rngi::Vr1rngi1
    }
}
#[doc = "Internal ADC Clock implemented\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iadcki {
    #[doc = "0: Internal clock source not implemented."]
    Iadcki0 = 0,
    #[doc = "1: Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    Iadcki1 = 1,
}
impl From<Iadcki> for bool {
    #[inline(always)]
    fn from(variant: Iadcki) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IADCKI` reader - Internal ADC Clock implemented"]
pub type IadckiR = crate::BitReader<Iadcki>;
impl IadckiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iadcki {
        match self.bits {
            false => Iadcki::Iadcki0,
            true => Iadcki::Iadcki1,
        }
    }
    #[doc = "Internal clock source not implemented."]
    #[inline(always)]
    pub fn is_iadcki_0(&self) -> bool {
        *self == Iadcki::Iadcki0
    }
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    #[inline(always)]
    pub fn is_iadcki_1(&self) -> bool {
        *self == Iadcki::Iadcki1
    }
}
#[doc = "Calibration Offset Function Implemented\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calofsi {
    #[doc = "0: Offset calibration and offset trimming not implemented."]
    Calofsi0 = 0,
    #[doc = "1: Offset calibration and offset trimming implemented."]
    Calofsi1 = 1,
}
impl From<Calofsi> for bool {
    #[inline(always)]
    fn from(variant: Calofsi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOFSI` reader - Calibration Offset Function Implemented"]
pub type CalofsiR = crate::BitReader<Calofsi>;
impl CalofsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calofsi {
        match self.bits {
            false => Calofsi::Calofsi0,
            true => Calofsi::Calofsi1,
        }
    }
    #[doc = "Offset calibration and offset trimming not implemented."]
    #[inline(always)]
    pub fn is_calofsi_0(&self) -> bool {
        *self == Calofsi::Calofsi0
    }
    #[doc = "Offset calibration and offset trimming implemented."]
    #[inline(always)]
    pub fn is_calofsi_1(&self) -> bool {
        *self == Calofsi::Calofsi1
    }
}
#[doc = "Field `MINOR` reader - Minor Version Number"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Major Version Number"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential Supported"]
    #[inline(always)]
    pub fn diffen(&self) -> DiffenR {
        DiffenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi Vref Implemented"]
    #[inline(always)]
    pub fn mvi(&self) -> MviR {
        MviR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel Scale Width"]
    #[inline(always)]
    pub fn csw(&self) -> CswR {
        CswR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub fn vr1rngi(&self) -> Vr1rngiR {
        Vr1rngiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal ADC Clock implemented"]
    #[inline(always)]
    pub fn iadcki(&self) -> IadckiR {
        IadckiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Calibration Offset Function Implemented"]
    #[inline(always)]
    pub fn calofsi(&self) -> CalofsiR {
        CalofsiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID")
            .field("res", &self.res())
            .field("diffen", &self.diffen())
            .field("mvi", &self.mvi())
            .field("csw", &self.csw())
            .field("vr1rngi", &self.vr1rngi())
            .field("iadcki", &self.iadcki())
            .field("calofsi", &self.calofsi())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[doc = "Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VeridSpec;
impl crate::RegisterSpec for VeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid::R`](R) reader structure"]
impl crate::Readable for VeridSpec {}
#[doc = "`reset()` method sets VERID to value 0x0100_001a"]
impl crate::Resettable for VeridSpec {
    const RESET_VALUE: u32 = 0x0100_001a;
}
