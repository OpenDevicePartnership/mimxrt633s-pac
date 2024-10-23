#[doc = "Register `MASTER_SEC_LEVEL_ANTI_POL` reader"]
pub type R = crate::R<MasterSecLevelAntiPolSpec>;
#[doc = "Register `MASTER_SEC_LEVEL_ANTI_POL` writer"]
pub type W = crate::W<MasterSecLevelAntiPolSpec>;
#[doc = "POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerquadSec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<PowerquadSec> for u8 {
    #[inline(always)]
    fn from(variant: PowerquadSec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PowerquadSec {
    type Ux = u8;
}
impl crate::IsEnum for PowerquadSec {}
#[doc = "Field `POWERQUAD_SEC` reader - POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type PowerquadSecR = crate::FieldReader<PowerquadSec>;
impl PowerquadSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerquadSec {
        match self.bits {
            0 => PowerquadSec::EnumSP,
            1 => PowerquadSec::EnumSNp,
            2 => PowerquadSec::EnumNsP,
            3 => PowerquadSec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PowerquadSec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PowerquadSec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PowerquadSec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PowerquadSec::EnumNsNp
    }
}
#[doc = "Field `POWERQUAD_SEC` writer - POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type PowerquadSecW<'a, REG> = crate::FieldWriter<'a, REG, 2, PowerquadSec, crate::Safe>;
impl<'a, REG> PowerquadSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadSec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadSec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadSec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(PowerquadSec::EnumNsNp)
    }
}
#[doc = "DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DspSec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<DspSec> for u8 {
    #[inline(always)]
    fn from(variant: DspSec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DspSec {
    type Ux = u8;
}
impl crate::IsEnum for DspSec {}
#[doc = "Field `DSP_SEC` reader - DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type DspSecR = crate::FieldReader<DspSec>;
impl DspSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DspSec {
        match self.bits {
            0 => DspSec::EnumSP,
            1 => DspSec::EnumSNp,
            2 => DspSec::EnumNsP,
            3 => DspSec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DspSec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DspSec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DspSec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == DspSec::EnumNsNp
    }
}
#[doc = "Field `DSP_SEC` writer - DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type DspSecW<'a, REG> = crate::FieldWriter<'a, REG, 2, DspSec, crate::Safe>;
impl<'a, REG> DspSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(DspSec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(DspSec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(DspSec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(DspSec::EnumNsNp)
    }
}
#[doc = "DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0Sec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Dma0Sec> for u8 {
    #[inline(always)]
    fn from(variant: Dma0Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0Sec {
    type Ux = u8;
}
impl crate::IsEnum for Dma0Sec {}
#[doc = "Field `DMA0_SEC` reader - DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Dma0SecR = crate::FieldReader<Dma0Sec>;
impl Dma0SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0Sec {
        match self.bits {
            0 => Dma0Sec::EnumSP,
            1 => Dma0Sec::EnumSNp,
            2 => Dma0Sec::EnumNsP,
            3 => Dma0Sec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Dma0Sec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Dma0Sec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Dma0Sec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Dma0Sec::EnumNsNp
    }
}
#[doc = "Field `DMA0_SEC` writer - DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Dma0SecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0Sec, crate::Safe>;
impl<'a, REG> Dma0SecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Sec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Sec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Sec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0Sec::EnumNsNp)
    }
}
#[doc = "DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1Sec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Dma1Sec> for u8 {
    #[inline(always)]
    fn from(variant: Dma1Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1Sec {
    type Ux = u8;
}
impl crate::IsEnum for Dma1Sec {}
#[doc = "Field `DMA1_SEC` reader - DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Dma1SecR = crate::FieldReader<Dma1Sec>;
impl Dma1SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1Sec {
        match self.bits {
            0 => Dma1Sec::EnumSP,
            1 => Dma1Sec::EnumSNp,
            2 => Dma1Sec::EnumNsP,
            3 => Dma1Sec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Dma1Sec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Dma1Sec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Dma1Sec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Dma1Sec::EnumNsNp
    }
}
#[doc = "Field `DMA1_SEC` writer - DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Dma1SecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma1Sec, crate::Safe>;
impl<'a, REG> Dma1SecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Sec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Sec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Sec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1Sec::EnumNsNp)
    }
}
#[doc = "SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdio0Sec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Sdio0Sec> for u8 {
    #[inline(always)]
    fn from(variant: Sdio0Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdio0Sec {
    type Ux = u8;
}
impl crate::IsEnum for Sdio0Sec {}
#[doc = "Field `SDIO0_SEC` reader - SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Sdio0SecR = crate::FieldReader<Sdio0Sec>;
impl Sdio0SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio0Sec {
        match self.bits {
            0 => Sdio0Sec::EnumSP,
            1 => Sdio0Sec::EnumSNp,
            2 => Sdio0Sec::EnumNsP,
            3 => Sdio0Sec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdio0Sec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdio0Sec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdio0Sec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdio0Sec::EnumNsNp
    }
}
#[doc = "Field `SDIO0_SEC` writer - SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Sdio0SecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdio0Sec, crate::Safe>;
impl<'a, REG> Sdio0SecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Sec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Sec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Sec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Sec::EnumNsNp)
    }
}
#[doc = "SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg).\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdio1Sec {
    #[doc = "0: Secure and Priviledge user access allowed."]
    EnumSP = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    EnumSNp = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    EnumNsP = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 3,
}
impl From<Sdio1Sec> for u8 {
    #[inline(always)]
    fn from(variant: Sdio1Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdio1Sec {
    type Ux = u8;
}
impl crate::IsEnum for Sdio1Sec {}
#[doc = "Field `SDIO1_SEC` reader - SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Sdio1SecR = crate::FieldReader<Sdio1Sec>;
impl Sdio1SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio1Sec {
        match self.bits {
            0 => Sdio1Sec::EnumSP,
            1 => Sdio1Sec::EnumSNp,
            2 => Sdio1Sec::EnumNsP,
            3 => Sdio1Sec::EnumNsNp,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdio1Sec::EnumSP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdio1Sec::EnumSNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdio1Sec::EnumNsP
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdio1Sec::EnumNsNp
    }
}
#[doc = "Field `SDIO1_SEC` writer - SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
pub type Sdio1SecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdio1Sec, crate::Safe>;
impl<'a, REG> Sdio1SecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Sec::EnumSP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Sec::EnumSNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Sec::EnumNsP)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Sec::EnumNsNp)
    }
}
#[doc = "MASTER_SEC_LEVEL_ANTI_POL register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MasterSecLevelAntiPoleLock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<MasterSecLevelAntiPoleLock> for u8 {
    #[inline(always)]
    fn from(variant: MasterSecLevelAntiPoleLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MasterSecLevelAntiPoleLock {
    type Ux = u8;
}
impl crate::IsEnum for MasterSecLevelAntiPoleLock {}
#[doc = "Field `MASTER_SEC_LEVEL_ANTI_POLE_LOCK` reader - MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
pub type MasterSecLevelAntiPoleLockR = crate::FieldReader<MasterSecLevelAntiPoleLock>;
impl MasterSecLevelAntiPoleLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MasterSecLevelAntiPoleLock> {
        match self.bits {
            1 => Some(MasterSecLevelAntiPoleLock::Blocked),
            2 => Some(MasterSecLevelAntiPoleLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MasterSecLevelAntiPoleLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == MasterSecLevelAntiPoleLock::Writable
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTI_POLE_LOCK` writer - MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
pub type MasterSecLevelAntiPoleLockW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, MasterSecLevelAntiPoleLock>;
impl<'a, REG> MasterSecLevelAntiPoleLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelAntiPoleLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelAntiPoleLock::Writable)
    }
}
impl R {
    #[doc = "Bits 4:5 - POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn powerquad_sec(&self) -> PowerquadSecR {
        PowerquadSecR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn dsp_sec(&self) -> DspSecR {
        DspSecR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn dma0_sec(&self) -> Dma0SecR {
        Dma0SecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn dma1_sec(&self) -> Dma1SecR {
        Dma1SecR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn sdio0_sec(&self) -> Sdio0SecR {
        Sdio0SecR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    pub fn sdio1_sec(&self) -> Sdio1SecR {
        Sdio1SecR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_anti_pole_lock(&self) -> MasterSecLevelAntiPoleLockR {
        MasterSecLevelAntiPoleLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASTER_SEC_LEVEL_ANTI_POL")
            .field("powerquad_sec", &self.powerquad_sec())
            .field("dsp_sec", &self.dsp_sec())
            .field("dma0_sec", &self.dma0_sec())
            .field("dma1_sec", &self.dma1_sec())
            .field("sdio0_sec", &self.sdio0_sec())
            .field("sdio1_sec", &self.sdio1_sec())
            .field(
                "master_sec_level_anti_pole_lock",
                &self.master_sec_level_anti_pole_lock(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - POWERQUAD master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn powerquad_sec(&mut self) -> PowerquadSecW<MasterSecLevelAntiPolSpec> {
        PowerquadSecW::new(self, 4)
    }
    #[doc = "Bits 6:7 - DSP master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn dsp_sec(&mut self) -> DspSecW<MasterSecLevelAntiPolSpec> {
        DspSecW::new(self, 6)
    }
    #[doc = "Bits 8:9 - DMA0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn dma0_sec(&mut self) -> Dma0SecW<MasterSecLevelAntiPolSpec> {
        Dma0SecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DMA1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn dma1_sec(&mut self) -> Dma1SecW<MasterSecLevelAntiPolSpec> {
        Dma1SecW::new(self, 10)
    }
    #[doc = "Bits 12:13 - SDIO0 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_sec(&mut self) -> Sdio0SecW<MasterSecLevelAntiPolSpec> {
        Sdio0SecW::new(self, 12)
    }
    #[doc = "Bits 14:15 - SDIO1 master secure level control anti-pole value (i.e It must be written with the inverted value of the corresponding field in master_sec_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_sec(&mut self) -> Sdio1SecW<MasterSecLevelAntiPolSpec> {
        Sdio1SecW::new(self, 14)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_LEVEL_ANTI_POL register write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn master_sec_level_anti_pole_lock(
        &mut self,
    ) -> MasterSecLevelAntiPoleLockW<MasterSecLevelAntiPolSpec> {
        MasterSecLevelAntiPoleLockW::new(self, 30)
    }
}
#[doc = "master secure level anti-pole register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_level_anti_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_level_anti_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterSecLevelAntiPolSpec;
impl crate::RegisterSpec for MasterSecLevelAntiPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_sec_level_anti_pol::R`](R) reader structure"]
impl crate::Readable for MasterSecLevelAntiPolSpec {}
#[doc = "`write(|w| ..)` method takes [`master_sec_level_anti_pol::W`](W) writer structure"]
impl crate::Writable for MasterSecLevelAntiPolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASTER_SEC_LEVEL_ANTI_POL to value 0xbfff_ffff"]
impl crate::Resettable for MasterSecLevelAntiPolSpec {
    const RESET_VALUE: u32 = 0xbfff_ffff;
}
