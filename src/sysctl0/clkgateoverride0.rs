#[doc = "Register `CLKGATEOVERRIDE0` reader"]
pub type R = crate::R<Clkgateoverride0Spec>;
#[doc = "Register `CLKGATEOVERRIDE0` writer"]
pub type W = crate::W<Clkgateoverride0Spec>;
#[doc = "sdio 0 clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0 {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Sdio0> for bool {
    #[inline(always)]
    fn from(variant: Sdio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_0` reader - sdio 0 clock override"]
pub type Sdio0R = crate::BitReader<Sdio0>;
impl Sdio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio0 {
        match self.bits {
            false => Sdio0::NoEffect,
            true => Sdio0::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Sdio0::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Sdio0::Override
    }
}
#[doc = "Field `SDIO_0` writer - sdio 0 clock override"]
pub type Sdio0W<'a, REG> = crate::BitWriter<'a, REG, Sdio0>;
impl<'a, REG> Sdio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::Override)
    }
}
#[doc = "sdio 1 clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1 {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Sdio1> for bool {
    #[inline(always)]
    fn from(variant: Sdio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_1` reader - sdio 1 clock override"]
pub type Sdio1R = crate::BitReader<Sdio1>;
impl Sdio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio1 {
        match self.bits {
            false => Sdio1::NoEffect,
            true => Sdio1::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Sdio1::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Sdio1::Override
    }
}
#[doc = "Field `SDIO_1` writer - sdio 1 clock override"]
pub type Sdio1W<'a, REG> = crate::BitWriter<'a, REG, Sdio1>;
impl<'a, REG> Sdio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::Override)
    }
}
#[doc = "usbhsphy clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbhsphy {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Usbhsphy> for bool {
    #[inline(always)]
    fn from(variant: Usbhsphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHSPHY` reader - usbhsphy clock override"]
pub type UsbhsphyR = crate::BitReader<Usbhsphy>;
impl UsbhsphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbhsphy {
        match self.bits {
            false => Usbhsphy::NoEffect,
            true => Usbhsphy::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Usbhsphy::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Usbhsphy::Override
    }
}
#[doc = "Field `USBHSPHY` writer - usbhsphy clock override"]
pub type UsbhsphyW<'a, REG> = crate::BitWriter<'a, REG, Usbhsphy>;
impl<'a, REG> UsbhsphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhsphy::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhsphy::Override)
    }
}
#[doc = "adc clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Adc> for bool {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - adc clock override"]
pub type AdcR = crate::BitReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            false => Adc::NoEffect,
            true => Adc::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Adc::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Adc::Override
    }
}
#[doc = "Field `ADC` writer - adc clock override"]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG, Adc>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Override)
    }
}
#[doc = "mu clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mu {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Mu> for bool {
    #[inline(always)]
    fn from(variant: Mu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU` reader - mu clock override"]
pub type MuR = crate::BitReader<Mu>;
impl MuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mu {
        match self.bits {
            false => Mu::NoEffect,
            true => Mu::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Mu::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Mu::Override
    }
}
#[doc = "Field `MU` writer - mu clock override"]
pub type MuW<'a, REG> = crate::BitWriter<'a, REG, Mu>;
impl<'a, REG> MuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Mu::Override)
    }
}
#[doc = "acomparator clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Acmp> for bool {
    #[inline(always)]
    fn from(variant: Acmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` reader - acomparator clock override"]
pub type AcmpR = crate::BitReader<Acmp>;
impl AcmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp {
        match self.bits {
            false => Acmp::NoEffect,
            true => Acmp::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Acmp::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Acmp::Override
    }
}
#[doc = "Field `ACMP` writer - acomparator clock override"]
pub type AcmpW<'a, REG> = crate::BitWriter<'a, REG, Acmp>;
impl<'a, REG> AcmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp::Override)
    }
}
#[doc = "pmc clock override\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmc {
    #[doc = "0: no effect"]
    NoEffect = 0,
    #[doc = "1: override"]
    Override = 1,
}
impl From<Pmc> for bool {
    #[inline(always)]
    fn from(variant: Pmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMC` reader - pmc clock override"]
pub type PmcR = crate::BitReader<Pmc>;
impl PmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmc {
        match self.bits {
            false => Pmc::NoEffect,
            true => Pmc::Override,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Pmc::NoEffect
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Pmc::Override
    }
}
#[doc = "Field `PMC` writer - pmc clock override"]
pub type PmcW<'a, REG> = crate::BitWriter<'a, REG, Pmc>;
impl<'a, REG> PmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Pmc::NoEffect)
    }
    #[doc = "override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Pmc::Override)
    }
}
impl R {
    #[doc = "Bit 0 - sdio 0 clock override"]
    #[inline(always)]
    pub fn sdio_0(&self) -> Sdio0R {
        Sdio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sdio 1 clock override"]
    #[inline(always)]
    pub fn sdio_1(&self) -> Sdio1R {
        Sdio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usbhsphy clock override"]
    #[inline(always)]
    pub fn usbhsphy(&self) -> UsbhsphyR {
        UsbhsphyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - adc clock override"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - mu clock override"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - acomparator clock override"]
    #[inline(always)]
    pub fn acmp(&self) -> AcmpR {
        AcmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pmc clock override"]
    #[inline(always)]
    pub fn pmc(&self) -> PmcR {
        PmcR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKGATEOVERRIDE0")
            .field("sdio_0", &self.sdio_0())
            .field("sdio_1", &self.sdio_1())
            .field("usbhsphy", &self.usbhsphy())
            .field("adc", &self.adc())
            .field("mu", &self.mu())
            .field("acmp", &self.acmp())
            .field("pmc", &self.pmc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - sdio 0 clock override"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_0(&mut self) -> Sdio0W<Clkgateoverride0Spec> {
        Sdio0W::new(self, 0)
    }
    #[doc = "Bit 1 - sdio 1 clock override"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_1(&mut self) -> Sdio1W<Clkgateoverride0Spec> {
        Sdio1W::new(self, 1)
    }
    #[doc = "Bit 2 - usbhsphy clock override"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsphy(&mut self) -> UsbhsphyW<Clkgateoverride0Spec> {
        UsbhsphyW::new(self, 2)
    }
    #[doc = "Bit 3 - adc clock override"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> AdcW<Clkgateoverride0Spec> {
        AdcW::new(self, 3)
    }
    #[doc = "Bit 4 - mu clock override"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<Clkgateoverride0Spec> {
        MuW::new(self, 4)
    }
    #[doc = "Bit 5 - acomparator clock override"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> AcmpW<Clkgateoverride0Spec> {
        AcmpW::new(self, 5)
    }
    #[doc = "Bit 6 - pmc clock override"]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PmcW<Clkgateoverride0Spec> {
        PmcW::new(self, 6)
    }
}
#[doc = "Clock gate override 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clkgateoverride0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkgateoverride0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkgateoverride0Spec;
impl crate::RegisterSpec for Clkgateoverride0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgateoverride0::R`](R) reader structure"]
impl crate::Readable for Clkgateoverride0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgateoverride0::W`](W) writer structure"]
impl crate::Writable for Clkgateoverride0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATEOVERRIDE0 to value 0xffff_ffff"]
impl crate::Resettable for Clkgateoverride0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
