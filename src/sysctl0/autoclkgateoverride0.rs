#[doc = "Register `AUTOCLKGATEOVERRIDE0` reader"]
pub type R = crate::R<Autoclkgateoverride0Spec>;
#[doc = "Register `AUTOCLKGATEOVERRIDE0` writer"]
pub type W = crate::W<Autoclkgateoverride0Spec>;
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahb2apb0 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<Ahb2apb0> for bool {
    #[inline(always)]
    fn from(variant: Ahb2apb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2APB0` reader - auto clock gating enable"]
pub type Ahb2apb0R = crate::BitReader<Ahb2apb0>;
impl Ahb2apb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahb2apb0 {
        match self.bits {
            false => Ahb2apb0::Enabled,
            true => Ahb2apb0::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ahb2apb0::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ahb2apb0::Disabled
    }
}
#[doc = "Field `AHB2APB0` writer - auto clock gating enable"]
pub type Ahb2apb0W<'a, REG> = crate::BitWriter<'a, REG, Ahb2apb0>;
impl<'a, REG> Ahb2apb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ahb2apb0::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ahb2apb0::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahb2apb1 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<Ahb2apb1> for bool {
    #[inline(always)]
    fn from(variant: Ahb2apb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2APB1` reader - auto clock gating enable"]
pub type Ahb2apb1R = crate::BitReader<Ahb2apb1>;
impl Ahb2apb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahb2apb1 {
        match self.bits {
            false => Ahb2apb1::Enabled,
            true => Ahb2apb1::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ahb2apb1::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ahb2apb1::Disabled
    }
}
#[doc = "Field `AHB2APB1` writer - auto clock gating enable"]
pub type Ahb2apb1W<'a, REG> = crate::BitWriter<'a, REG, Ahb2apb1>;
impl<'a, REG> Ahb2apb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ahb2apb1::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ahb2apb1::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcEngine {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<CrcEngine> for bool {
    #[inline(always)]
    fn from(variant: CrcEngine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_Engine` reader - auto clock gating enable"]
pub type CrcEngineR = crate::BitReader<CrcEngine>;
impl CrcEngineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcEngine {
        match self.bits {
            false => CrcEngine::Enabled,
            true => CrcEngine::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CrcEngine::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CrcEngine::Disabled
    }
}
#[doc = "Field `CRC_Engine` writer - auto clock gating enable"]
pub type CrcEngineW<'a, REG> = crate::BitWriter<'a, REG, CrcEngine>;
impl<'a, REG> CrcEngineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrcEngine::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrcEngine::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Casper {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<Casper> for bool {
    #[inline(always)]
    fn from(variant: Casper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Casper` reader - auto clock gating enable"]
pub type CasperR = crate::BitReader<Casper>;
impl CasperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Casper {
        match self.bits {
            false => Casper::Enabled,
            true => Casper::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Casper::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Casper::Disabled
    }
}
#[doc = "Field `Casper` writer - auto clock gating enable"]
pub type CasperW<'a, REG> = crate::BitWriter<'a, REG, Casper>;
impl<'a, REG> CasperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Casper::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac0 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<Dmac0> for bool {
    #[inline(always)]
    fn from(variant: Dmac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0` reader - auto clock gating enable"]
pub type Dmac0R = crate::BitReader<Dmac0>;
impl Dmac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac0 {
        match self.bits {
            false => Dmac0::Enabled,
            true => Dmac0::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac0::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac0::Disabled
    }
}
#[doc = "Field `DMAC0` writer - auto clock gating enable"]
pub type Dmac0W<'a, REG> = crate::BitWriter<'a, REG, Dmac0>;
impl<'a, REG> Dmac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0::Disabled)
    }
}
#[doc = "auto clock gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmac1 {
    #[doc = "0: Enable Auto-Clk"]
    Enabled = 0,
    #[doc = "1: Disable Auto-Clk"]
    Disabled = 1,
}
impl From<Dmac1> for bool {
    #[inline(always)]
    fn from(variant: Dmac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1` reader - auto clock gating enable"]
pub type Dmac1R = crate::BitReader<Dmac1>;
impl Dmac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmac1 {
        match self.bits {
            false => Dmac1::Enabled,
            true => Dmac1::Disabled,
        }
    }
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmac1::Enabled
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmac1::Disabled
    }
}
#[doc = "Field `DMAC1` writer - auto clock gating enable"]
pub type Dmac1W<'a, REG> = crate::BitWriter<'a, REG, Dmac1>;
impl<'a, REG> Dmac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Auto-Clk"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::Enabled)
    }
    #[doc = "Disable Auto-Clk"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - auto clock gating enable"]
    #[inline(always)]
    pub fn ahb2apb0(&self) -> Ahb2apb0R {
        Ahb2apb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - auto clock gating enable"]
    #[inline(always)]
    pub fn ahb2apb1(&self) -> Ahb2apb1R {
        Ahb2apb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - auto clock gating enable"]
    #[inline(always)]
    pub fn crc_engine(&self) -> CrcEngineR {
        CrcEngineR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - auto clock gating enable"]
    #[inline(always)]
    pub fn casper(&self) -> CasperR {
        CasperR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - auto clock gating enable"]
    #[inline(always)]
    pub fn dmac0(&self) -> Dmac0R {
        Dmac0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - auto clock gating enable"]
    #[inline(always)]
    pub fn dmac1(&self) -> Dmac1R {
        Dmac1R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCLKGATEOVERRIDE0")
            .field("ahb2apb0", &self.ahb2apb0())
            .field("ahb2apb1", &self.ahb2apb1())
            .field("crc_engine", &self.crc_engine())
            .field("casper", &self.casper())
            .field("dmac0", &self.dmac0())
            .field("dmac1", &self.dmac1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - auto clock gating enable"]
    #[inline(always)]
    pub fn ahb2apb0(&mut self) -> Ahb2apb0W<Autoclkgateoverride0Spec> {
        Ahb2apb0W::new(self, 0)
    }
    #[doc = "Bit 1 - auto clock gating enable"]
    #[inline(always)]
    pub fn ahb2apb1(&mut self) -> Ahb2apb1W<Autoclkgateoverride0Spec> {
        Ahb2apb1W::new(self, 1)
    }
    #[doc = "Bit 2 - auto clock gating enable"]
    #[inline(always)]
    pub fn crc_engine(&mut self) -> CrcEngineW<Autoclkgateoverride0Spec> {
        CrcEngineW::new(self, 2)
    }
    #[doc = "Bit 3 - auto clock gating enable"]
    #[inline(always)]
    pub fn casper(&mut self) -> CasperW<Autoclkgateoverride0Spec> {
        CasperW::new(self, 3)
    }
    #[doc = "Bit 4 - auto clock gating enable"]
    #[inline(always)]
    pub fn dmac0(&mut self) -> Dmac0W<Autoclkgateoverride0Spec> {
        Dmac0W::new(self, 4)
    }
    #[doc = "Bit 5 - auto clock gating enable"]
    #[inline(always)]
    pub fn dmac1(&mut self) -> Dmac1W<Autoclkgateoverride0Spec> {
        Dmac1W::new(self, 5)
    }
}
#[doc = "auto clock gating override 0\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autoclkgateoverride0Spec;
impl crate::RegisterSpec for Autoclkgateoverride0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autoclkgateoverride0::R`](R) reader structure"]
impl crate::Readable for Autoclkgateoverride0Spec {}
#[doc = "`write(|w| ..)` method takes [`autoclkgateoverride0::W`](W) writer structure"]
impl crate::Writable for Autoclkgateoverride0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE0 to value 0xffff_ffff"]
impl crate::Resettable for Autoclkgateoverride0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
