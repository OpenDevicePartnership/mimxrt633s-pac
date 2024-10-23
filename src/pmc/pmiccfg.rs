#[doc = "Register `PMICCFG` reader"]
pub type R = crate::R<PmiccfgSpec>;
#[doc = "Register `PMICCFG` writer"]
pub type W = crate::W<PmiccfgSpec>;
#[doc = "vddcore state in PMIC mode 0\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddcorem0 {
    #[doc = "0: off"]
    Vddcorem0_0 = 0,
    #[doc = "1: powered"]
    Vddcorem0_1 = 1,
}
impl From<Vddcorem0> for bool {
    #[inline(always)]
    fn from(variant: Vddcorem0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREM0` reader - vddcore state in PMIC mode 0"]
pub type Vddcorem0R = crate::BitReader<Vddcorem0>;
impl Vddcorem0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddcorem0 {
        match self.bits {
            false => Vddcorem0::Vddcorem0_0,
            true => Vddcorem0::Vddcorem0_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vddcorem0_0(&self) -> bool {
        *self == Vddcorem0::Vddcorem0_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vddcorem0_1(&self) -> bool {
        *self == Vddcorem0::Vddcorem0_1
    }
}
#[doc = "Field `VDDCOREM0` writer - vddcore state in PMIC mode 0"]
pub type Vddcorem0W<'a, REG> = crate::BitWriter<'a, REG, Vddcorem0>;
impl<'a, REG> Vddcorem0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vddcorem0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem0::Vddcorem0_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vddcorem0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem0::Vddcorem0_1)
    }
}
#[doc = "vddcore state in PMIC mode 1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddcorem1 {
    #[doc = "0: off"]
    Vddcorem1_0 = 0,
    #[doc = "1: powered"]
    Vddcorem1_1 = 1,
}
impl From<Vddcorem1> for bool {
    #[inline(always)]
    fn from(variant: Vddcorem1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREM1` reader - vddcore state in PMIC mode 1"]
pub type Vddcorem1R = crate::BitReader<Vddcorem1>;
impl Vddcorem1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddcorem1 {
        match self.bits {
            false => Vddcorem1::Vddcorem1_0,
            true => Vddcorem1::Vddcorem1_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vddcorem1_0(&self) -> bool {
        *self == Vddcorem1::Vddcorem1_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vddcorem1_1(&self) -> bool {
        *self == Vddcorem1::Vddcorem1_1
    }
}
#[doc = "Field `VDDCOREM1` writer - vddcore state in PMIC mode 1"]
pub type Vddcorem1W<'a, REG> = crate::BitWriter<'a, REG, Vddcorem1>;
impl<'a, REG> Vddcorem1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vddcorem1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem1::Vddcorem1_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vddcorem1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem1::Vddcorem1_1)
    }
}
#[doc = "vddcore state in PMIC mode 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddcorem2 {
    #[doc = "0: off"]
    Vddcorem2_0 = 0,
    #[doc = "1: powered"]
    Vddcorem2_1 = 1,
}
impl From<Vddcorem2> for bool {
    #[inline(always)]
    fn from(variant: Vddcorem2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREM2` reader - vddcore state in PMIC mode 2"]
pub type Vddcorem2R = crate::BitReader<Vddcorem2>;
impl Vddcorem2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddcorem2 {
        match self.bits {
            false => Vddcorem2::Vddcorem2_0,
            true => Vddcorem2::Vddcorem2_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vddcorem2_0(&self) -> bool {
        *self == Vddcorem2::Vddcorem2_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vddcorem2_1(&self) -> bool {
        *self == Vddcorem2::Vddcorem2_1
    }
}
#[doc = "Field `VDDCOREM2` writer - vddcore state in PMIC mode 2"]
pub type Vddcorem2W<'a, REG> = crate::BitWriter<'a, REG, Vddcorem2>;
impl<'a, REG> Vddcorem2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vddcorem2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem2::Vddcorem2_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vddcorem2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem2::Vddcorem2_1)
    }
}
#[doc = "vddcore state in PMIC mode 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddcorem3 {
    #[doc = "0: off"]
    Vddcorem3_0 = 0,
    #[doc = "1: powered"]
    Vddcorem3_1 = 1,
}
impl From<Vddcorem3> for bool {
    #[inline(always)]
    fn from(variant: Vddcorem3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREM3` reader - vddcore state in PMIC mode 3"]
pub type Vddcorem3R = crate::BitReader<Vddcorem3>;
impl Vddcorem3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddcorem3 {
        match self.bits {
            false => Vddcorem3::Vddcorem3_0,
            true => Vddcorem3::Vddcorem3_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vddcorem3_0(&self) -> bool {
        *self == Vddcorem3::Vddcorem3_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vddcorem3_1(&self) -> bool {
        *self == Vddcorem3::Vddcorem3_1
    }
}
#[doc = "Field `VDDCOREM3` writer - vddcore state in PMIC mode 3"]
pub type Vddcorem3W<'a, REG> = crate::BitWriter<'a, REG, Vddcorem3>;
impl<'a, REG> Vddcorem3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vddcorem3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem3::Vddcorem3_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vddcorem3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vddcorem3::Vddcorem3_1)
    }
}
#[doc = "vdd1v8 state in PMIC mode 0\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdd1v8m0 {
    #[doc = "0: off"]
    Vdd1v8m0_0 = 0,
    #[doc = "1: powered"]
    Vdd1v8m0_1 = 1,
}
impl From<Vdd1v8m0> for bool {
    #[inline(always)]
    fn from(variant: Vdd1v8m0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDD1V8M0` reader - vdd1v8 state in PMIC mode 0"]
pub type Vdd1v8m0R = crate::BitReader<Vdd1v8m0>;
impl Vdd1v8m0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdd1v8m0 {
        match self.bits {
            false => Vdd1v8m0::Vdd1v8m0_0,
            true => Vdd1v8m0::Vdd1v8m0_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vdd1v8m0_0(&self) -> bool {
        *self == Vdd1v8m0::Vdd1v8m0_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vdd1v8m0_1(&self) -> bool {
        *self == Vdd1v8m0::Vdd1v8m0_1
    }
}
#[doc = "Field `VDD1V8M0` writer - vdd1v8 state in PMIC mode 0"]
pub type Vdd1v8m0W<'a, REG> = crate::BitWriter<'a, REG, Vdd1v8m0>;
impl<'a, REG> Vdd1v8m0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vdd1v8m0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m0::Vdd1v8m0_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vdd1v8m0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m0::Vdd1v8m0_1)
    }
}
#[doc = "vdd1v8 state in PMIC mode 1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdd1v8m1 {
    #[doc = "0: off"]
    Vdd1v8m1_0 = 0,
    #[doc = "1: powered"]
    Vdd1v8m1_1 = 1,
}
impl From<Vdd1v8m1> for bool {
    #[inline(always)]
    fn from(variant: Vdd1v8m1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDD1V8M1` reader - vdd1v8 state in PMIC mode 1"]
pub type Vdd1v8m1R = crate::BitReader<Vdd1v8m1>;
impl Vdd1v8m1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdd1v8m1 {
        match self.bits {
            false => Vdd1v8m1::Vdd1v8m1_0,
            true => Vdd1v8m1::Vdd1v8m1_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vdd1v8m1_0(&self) -> bool {
        *self == Vdd1v8m1::Vdd1v8m1_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vdd1v8m1_1(&self) -> bool {
        *self == Vdd1v8m1::Vdd1v8m1_1
    }
}
#[doc = "Field `VDD1V8M1` writer - vdd1v8 state in PMIC mode 1"]
pub type Vdd1v8m1W<'a, REG> = crate::BitWriter<'a, REG, Vdd1v8m1>;
impl<'a, REG> Vdd1v8m1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vdd1v8m1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m1::Vdd1v8m1_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vdd1v8m1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m1::Vdd1v8m1_1)
    }
}
#[doc = "vdd1v8 state in PMIC mode 2\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdd1v8m2 {
    #[doc = "0: off"]
    Vdd1v8m2_0 = 0,
    #[doc = "1: powered"]
    Vdd1v8m2_1 = 1,
}
impl From<Vdd1v8m2> for bool {
    #[inline(always)]
    fn from(variant: Vdd1v8m2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDD1V8M2` reader - vdd1v8 state in PMIC mode 2"]
pub type Vdd1v8m2R = crate::BitReader<Vdd1v8m2>;
impl Vdd1v8m2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdd1v8m2 {
        match self.bits {
            false => Vdd1v8m2::Vdd1v8m2_0,
            true => Vdd1v8m2::Vdd1v8m2_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vdd1v8m2_0(&self) -> bool {
        *self == Vdd1v8m2::Vdd1v8m2_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vdd1v8m2_1(&self) -> bool {
        *self == Vdd1v8m2::Vdd1v8m2_1
    }
}
#[doc = "Field `VDD1V8M2` writer - vdd1v8 state in PMIC mode 2"]
pub type Vdd1v8m2W<'a, REG> = crate::BitWriter<'a, REG, Vdd1v8m2>;
impl<'a, REG> Vdd1v8m2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vdd1v8m2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m2::Vdd1v8m2_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vdd1v8m2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m2::Vdd1v8m2_1)
    }
}
#[doc = "vdd1v8 state in PMIC mode 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdd1v8m3 {
    #[doc = "0: off"]
    Vdd1v8m3_0 = 0,
    #[doc = "1: powered"]
    Vdd1v8m3_1 = 1,
}
impl From<Vdd1v8m3> for bool {
    #[inline(always)]
    fn from(variant: Vdd1v8m3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDD1V8M3` reader - vdd1v8 state in PMIC mode 3"]
pub type Vdd1v8m3R = crate::BitReader<Vdd1v8m3>;
impl Vdd1v8m3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdd1v8m3 {
        match self.bits {
            false => Vdd1v8m3::Vdd1v8m3_0,
            true => Vdd1v8m3::Vdd1v8m3_1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_vdd1v8m3_0(&self) -> bool {
        *self == Vdd1v8m3::Vdd1v8m3_0
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn is_vdd1v8m3_1(&self) -> bool {
        *self == Vdd1v8m3::Vdd1v8m3_1
    }
}
#[doc = "Field `VDD1V8M3` writer - vdd1v8 state in PMIC mode 3"]
pub type Vdd1v8m3W<'a, REG> = crate::BitWriter<'a, REG, Vdd1v8m3>;
impl<'a, REG> Vdd1v8m3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "off"]
    #[inline(always)]
    pub fn vdd1v8m3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m3::Vdd1v8m3_0)
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn vdd1v8m3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdd1v8m3::Vdd1v8m3_1)
    }
}
impl R {
    #[doc = "Bit 0 - vddcore state in PMIC mode 0"]
    #[inline(always)]
    pub fn vddcorem0(&self) -> Vddcorem0R {
        Vddcorem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - vddcore state in PMIC mode 1"]
    #[inline(always)]
    pub fn vddcorem1(&self) -> Vddcorem1R {
        Vddcorem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vddcore state in PMIC mode 2"]
    #[inline(always)]
    pub fn vddcorem2(&self) -> Vddcorem2R {
        Vddcorem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - vddcore state in PMIC mode 3"]
    #[inline(always)]
    pub fn vddcorem3(&self) -> Vddcorem3R {
        Vddcorem3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - vdd1v8 state in PMIC mode 0"]
    #[inline(always)]
    pub fn vdd1v8m0(&self) -> Vdd1v8m0R {
        Vdd1v8m0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - vdd1v8 state in PMIC mode 1"]
    #[inline(always)]
    pub fn vdd1v8m1(&self) -> Vdd1v8m1R {
        Vdd1v8m1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - vdd1v8 state in PMIC mode 2"]
    #[inline(always)]
    pub fn vdd1v8m2(&self) -> Vdd1v8m2R {
        Vdd1v8m2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vdd1v8 state in PMIC mode 3"]
    #[inline(always)]
    pub fn vdd1v8m3(&self) -> Vdd1v8m3R {
        Vdd1v8m3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMICCFG")
            .field("vddcorem0", &self.vddcorem0())
            .field("vddcorem1", &self.vddcorem1())
            .field("vddcorem2", &self.vddcorem2())
            .field("vddcorem3", &self.vddcorem3())
            .field("vdd1v8m0", &self.vdd1v8m0())
            .field("vdd1v8m1", &self.vdd1v8m1())
            .field("vdd1v8m2", &self.vdd1v8m2())
            .field("vdd1v8m3", &self.vdd1v8m3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - vddcore state in PMIC mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem0(&mut self) -> Vddcorem0W<PmiccfgSpec> {
        Vddcorem0W::new(self, 0)
    }
    #[doc = "Bit 1 - vddcore state in PMIC mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem1(&mut self) -> Vddcorem1W<PmiccfgSpec> {
        Vddcorem1W::new(self, 1)
    }
    #[doc = "Bit 2 - vddcore state in PMIC mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem2(&mut self) -> Vddcorem2W<PmiccfgSpec> {
        Vddcorem2W::new(self, 2)
    }
    #[doc = "Bit 3 - vddcore state in PMIC mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem3(&mut self) -> Vddcorem3W<PmiccfgSpec> {
        Vddcorem3W::new(self, 3)
    }
    #[doc = "Bit 4 - vdd1v8 state in PMIC mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m0(&mut self) -> Vdd1v8m0W<PmiccfgSpec> {
        Vdd1v8m0W::new(self, 4)
    }
    #[doc = "Bit 5 - vdd1v8 state in PMIC mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m1(&mut self) -> Vdd1v8m1W<PmiccfgSpec> {
        Vdd1v8m1W::new(self, 5)
    }
    #[doc = "Bit 6 - vdd1v8 state in PMIC mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m2(&mut self) -> Vdd1v8m2W<PmiccfgSpec> {
        Vdd1v8m2W::new(self, 6)
    }
    #[doc = "Bit 7 - vdd1v8 state in PMIC mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m3(&mut self) -> Vdd1v8m3W<PmiccfgSpec> {
        Vdd1v8m3W::new(self, 7)
    }
}
#[doc = "PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on\n\nYou can [`read`](crate::Reg::read) this register and get [`pmiccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmiccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmiccfgSpec;
impl crate::RegisterSpec for PmiccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmiccfg::R`](R) reader structure"]
impl crate::Readable for PmiccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmiccfg::W`](W) writer structure"]
impl crate::Writable for PmiccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMICCFG to value 0x73"]
impl crate::Resettable for PmiccfgSpec {
    const RESET_VALUE: u32 = 0x73;
}
