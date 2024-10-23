#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Force Logically Disabled Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fldm {
    #[doc = "0: No effect on the operating mode."]
    Fldm0 = 0,
    #[doc = "1: Force entry into LDM after a write with this data bit set. SR\\[MODE\\]
signals the operating mode."]
    Fldm1 = 1,
}
impl From<Fldm> for bool {
    #[inline(always)]
    fn from(variant: Fldm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLDM` reader - Force Logically Disabled Mode"]
pub type FldmR = crate::BitReader<Fldm>;
impl FldmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fldm {
        match self.bits {
            false => Fldm::Fldm0,
            true => Fldm::Fldm1,
        }
    }
    #[doc = "No effect on the operating mode."]
    #[inline(always)]
    pub fn is_fldm_0(&self) -> bool {
        *self == Fldm::Fldm0
    }
    #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\]
signals the operating mode."]
    #[inline(always)]
    pub fn is_fldm_1(&self) -> bool {
        *self == Fldm::Fldm1
    }
}
#[doc = "Field `FLDM` writer - Force Logically Disabled Mode"]
pub type FldmW<'a, REG> = crate::BitWriter<'a, REG, Fldm>;
impl<'a, REG> FldmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on the operating mode."]
    #[inline(always)]
    pub fn fldm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fldm::Fldm0)
    }
    #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\]
signals the operating mode."]
    #[inline(always)]
    pub fn fldm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fldm::Fldm1)
    }
}
#[doc = "Restricted Register Access Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrae {
    #[doc = "0: Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    Rrae0 = 0,
    #[doc = "1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Rrae1 = 1,
}
impl From<Rrae> for bool {
    #[inline(always)]
    fn from(variant: Rrae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRAE` reader - Restricted Register Access Enable"]
pub type RraeR = crate::BitReader<Rrae>;
impl RraeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrae {
        match self.bits {
            false => Rrae::Rrae0,
            true => Rrae::Rrae1,
        }
    }
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    #[inline(always)]
    pub fn is_rrae_0(&self) -> bool {
        *self == Rrae::Rrae0
    }
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    #[inline(always)]
    pub fn is_rrae_1(&self) -> bool {
        *self == Rrae::Rrae1
    }
}
#[doc = "Field `RRAE` writer - Restricted Register Access Enable"]
pub type RraeW<'a, REG> = crate::BitWriter<'a, REG, Rrae>;
impl<'a, REG> RraeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    #[inline(always)]
    pub fn rrae_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrae::Rrae0)
    }
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    #[inline(always)]
    pub fn rrae_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrae::Rrae1)
    }
}
#[doc = "Global OTFAD Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ge {
    #[doc = "0: OTFAD has decryption disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    Ge0 = 0,
    #[doc = "1: OTFAD has decryption enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    Ge1 = 1,
}
impl From<Ge> for bool {
    #[inline(always)]
    fn from(variant: Ge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - Global OTFAD Enable"]
pub type GeR = crate::BitReader<Ge>;
impl GeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ge {
        match self.bits {
            false => Ge::Ge0,
            true => Ge::Ge1,
        }
    }
    #[doc = "OTFAD has decryption disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    #[inline(always)]
    pub fn is_ge_0(&self) -> bool {
        *self == Ge::Ge0
    }
    #[doc = "OTFAD has decryption enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    #[inline(always)]
    pub fn is_ge_1(&self) -> bool {
        *self == Ge::Ge1
    }
}
#[doc = "Field `GE` writer - Global OTFAD Enable"]
pub type GeW<'a, REG> = crate::BitWriter<'a, REG, Ge>;
impl<'a, REG> GeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OTFAD has decryption disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    #[inline(always)]
    pub fn ge_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ge::Ge0)
    }
    #[doc = "OTFAD has decryption enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    #[inline(always)]
    pub fn ge_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ge::Ge1)
    }
}
impl R {
    #[doc = "Bit 3 - Force Logically Disabled Mode"]
    #[inline(always)]
    pub fn fldm(&self) -> FldmR {
        FldmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Restricted Register Access Enable"]
    #[inline(always)]
    pub fn rrae(&self) -> RraeR {
        RraeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Global OTFAD Enable"]
    #[inline(always)]
    pub fn ge(&self) -> GeR {
        GeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("fldm", &self.fldm())
            .field("rrae", &self.rrae())
            .field("ge", &self.ge())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Force Logically Disabled Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fldm(&mut self) -> FldmW<CrSpec> {
        FldmW::new(self, 3)
    }
    #[doc = "Bit 7 - Restricted Register Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrae(&mut self) -> RraeW<CrSpec> {
        RraeW::new(self, 7)
    }
    #[doc = "Bit 31 - Global OTFAD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GeW<CrSpec> {
        GeW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
