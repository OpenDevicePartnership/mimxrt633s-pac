#[doc = "Register `DBG_FEATURES_DP` reader"]
pub type R = crate::R<DbgFeaturesDpSpec>;
#[doc = "Register `DBG_FEATURES_DP` writer"]
pub type W = crate::W<DbgFeaturesDpSpec>;
#[doc = "CM33 Debug Enable Control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbgen {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: disabled"]
    Disbaled1 = 1,
    #[doc = "2: disabled"]
    Disabled2 = 2,
    #[doc = "3: disabled"]
    Disabled3 = 3,
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbgen {
    type Ux = u8;
}
impl crate::IsEnum for Dbgen {}
#[doc = "Field `DBGEN` reader - CM33 Debug Enable Control"]
pub type DbgenR = crate::FieldReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            0 => Dbgen::Enabled,
            1 => Dbgen::Disbaled1,
            2 => Dbgen::Disabled2,
            3 => Dbgen::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dbgen::Enabled
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disbaled1(&self) -> bool {
        *self == Dbgen::Disbaled1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled2(&self) -> bool {
        *self == Dbgen::Disabled2
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled3(&self) -> bool {
        *self == Dbgen::Disabled3
    }
}
#[doc = "Field `DBGEN` writer - CM33 Debug Enable Control"]
pub type DbgenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dbgen, crate::Safe>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Enabled)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disbaled1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disbaled1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled2(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disabled2)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled3(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disabled3)
    }
}
#[doc = "CM33 NID Enable Control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Niden {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: disabled"]
    Disbaled1 = 1,
    #[doc = "2: disabled"]
    Disabled2 = 2,
    #[doc = "3: disabled"]
    Disabled3 = 3,
}
impl From<Niden> for u8 {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Niden {
    type Ux = u8;
}
impl crate::IsEnum for Niden {}
#[doc = "Field `NIDEN` reader - CM33 NID Enable Control"]
pub type NidenR = crate::FieldReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            0 => Niden::Enabled,
            1 => Niden::Disbaled1,
            2 => Niden::Disabled2,
            3 => Niden::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Niden::Enabled
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disbaled1(&self) -> bool {
        *self == Niden::Disbaled1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled2(&self) -> bool {
        *self == Niden::Disabled2
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled3(&self) -> bool {
        *self == Niden::Disabled3
    }
}
#[doc = "Field `NIDEN` writer - CM33 NID Enable Control"]
pub type NidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Niden, crate::Safe>;
impl<'a, REG> NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Enabled)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disbaled1(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disbaled1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled2(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disabled2)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled3(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disabled3)
    }
}
#[doc = "CM33 SPID Enable Control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spiden {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: disabled"]
    Disbaled1 = 1,
    #[doc = "2: disabled"]
    Disabled2 = 2,
    #[doc = "3: disabled"]
    Disabled3 = 3,
}
impl From<Spiden> for u8 {
    #[inline(always)]
    fn from(variant: Spiden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spiden {
    type Ux = u8;
}
impl crate::IsEnum for Spiden {}
#[doc = "Field `SPIDEN` reader - CM33 SPID Enable Control"]
pub type SpidenR = crate::FieldReader<Spiden>;
impl SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiden {
        match self.bits {
            0 => Spiden::Enabled,
            1 => Spiden::Disbaled1,
            2 => Spiden::Disabled2,
            3 => Spiden::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Spiden::Enabled
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disbaled1(&self) -> bool {
        *self == Spiden::Disbaled1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled2(&self) -> bool {
        *self == Spiden::Disabled2
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled3(&self) -> bool {
        *self == Spiden::Disabled3
    }
}
#[doc = "Field `SPIDEN` writer - CM33 SPID Enable Control"]
pub type SpidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spiden, crate::Safe>;
impl<'a, REG> SpidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Enabled)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disbaled1(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disbaled1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled2(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disabled2)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled3(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disabled3)
    }
}
#[doc = "CM33 SPNIDEN Enable Control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spniden {
    #[doc = "0: enabled"]
    Enabled = 0,
    #[doc = "1: disabled"]
    Disbaled1 = 1,
    #[doc = "2: disabled"]
    Disabled2 = 2,
    #[doc = "3: disabled"]
    Disabled3 = 3,
}
impl From<Spniden> for u8 {
    #[inline(always)]
    fn from(variant: Spniden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spniden {
    type Ux = u8;
}
impl crate::IsEnum for Spniden {}
#[doc = "Field `SPNIDEN` reader - CM33 SPNIDEN Enable Control"]
pub type SpnidenR = crate::FieldReader<Spniden>;
impl SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spniden {
        match self.bits {
            0 => Spniden::Enabled,
            1 => Spniden::Disbaled1,
            2 => Spniden::Disabled2,
            3 => Spniden::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Spniden::Enabled
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disbaled1(&self) -> bool {
        *self == Spniden::Disbaled1
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled2(&self) -> bool {
        *self == Spniden::Disabled2
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disabled3(&self) -> bool {
        *self == Spniden::Disabled3
    }
}
#[doc = "Field `SPNIDEN` writer - CM33 SPNIDEN Enable Control"]
pub type SpnidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spniden, crate::Safe>;
impl<'a, REG> SpnidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Enabled)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disbaled1(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disbaled1)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled2(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disabled2)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disabled3(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disabled3)
    }
}
impl R {
    #[doc = "Bits 0:1 - CM33 Debug Enable Control"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CM33 NID Enable Control"]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CM33 SPID Enable Control"]
    #[inline(always)]
    pub fn spiden(&self) -> SpidenR {
        SpidenR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    pub fn spniden(&self) -> SpnidenR {
        SpnidenR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_FEATURES_DP")
            .field("dbgen", &self.dbgen())
            .field("niden", &self.niden())
            .field("spiden", &self.spiden())
            .field("spniden", &self.spniden())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - CM33 Debug Enable Control"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<DbgFeaturesDpSpec> {
        DbgenW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CM33 NID Enable Control"]
    #[inline(always)]
    pub fn niden(&mut self) -> NidenW<DbgFeaturesDpSpec> {
        NidenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CM33 SPID Enable Control"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SpidenW<DbgFeaturesDpSpec> {
        SpidenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SpnidenW<DbgFeaturesDpSpec> {
        SpnidenW::new(self, 6)
    }
}
#[doc = "Debug features duplicate\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_features_dp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_features_dp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgFeaturesDpSpec;
impl crate::RegisterSpec for DbgFeaturesDpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_features_dp::R`](R) reader structure"]
impl crate::Readable for DbgFeaturesDpSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_features_dp::W`](W) writer structure"]
impl crate::Writable for DbgFeaturesDpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_FEATURES_DP to value 0x55"]
impl crate::Resettable for DbgFeaturesDpSpec {
    const RESET_VALUE: u32 = 0x55;
}
