#[doc = "Register `PRSTCTL1` reader"]
pub type R = crate::R<Prstctl1Spec>;
#[doc = "Register `PRSTCTL1` writer"]
pub type W = crate::W<Prstctl1Spec>;
#[doc = "SDIO0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Sdio0> for bool {
    #[inline(always)]
    fn from(variant: Sdio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0` reader - SDIO0 reset control"]
pub type Sdio0R = crate::BitReader<Sdio0>;
impl Sdio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio0 {
        match self.bits {
            false => Sdio0::ClearReset,
            true => Sdio0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Sdio0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Sdio0::SetReset
    }
}
#[doc = "Field `SDIO0` writer - SDIO0 reset control"]
pub type Sdio0W<'a, REG> = crate::BitWriter<'a, REG, Sdio0>;
impl<'a, REG> Sdio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0::SetReset)
    }
}
#[doc = "SDIO1 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Sdio1> for bool {
    #[inline(always)]
    fn from(variant: Sdio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1` reader - SDIO1 reset control"]
pub type Sdio1R = crate::BitReader<Sdio1>;
impl Sdio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio1 {
        match self.bits {
            false => Sdio1::ClearReset,
            true => Sdio1::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Sdio1::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Sdio1::SetReset
    }
}
#[doc = "Field `SDIO1` writer - SDIO1 reset control"]
pub type Sdio1W<'a, REG> = crate::BitWriter<'a, REG, Sdio1>;
impl<'a, REG> Sdio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1::SetReset)
    }
}
#[doc = "Analog comparator reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Acmp0> for bool {
    #[inline(always)]
    fn from(variant: Acmp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0` reader - Analog comparator reset control"]
pub type Acmp0R = crate::BitReader<Acmp0>;
impl Acmp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp0 {
        match self.bits {
            false => Acmp0::ClearReset,
            true => Acmp0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Acmp0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Acmp0::SetReset
    }
}
#[doc = "Field `ACMP0` writer - Analog comparator reset control"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG, Acmp0>;
impl<'a, REG> Acmp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0::SetReset)
    }
}
#[doc = "ADC reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - ADC reset control"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            false => Adc0::ClearReset,
            true => Adc0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Adc0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Adc0::SetReset
    }
}
#[doc = "Field `ADC0` writer - ADC reset control"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::SetReset)
    }
}
#[doc = "SHSGPIO0 reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shsgpio0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Shsgpio0> for bool {
    #[inline(always)]
    fn from(variant: Shsgpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0` reader - SHSGPIO0 reset control"]
pub type Shsgpio0R = crate::BitReader<Shsgpio0>;
impl Shsgpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shsgpio0 {
        match self.bits {
            false => Shsgpio0::ClearReset,
            true => Shsgpio0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Shsgpio0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Shsgpio0::SetReset
    }
}
#[doc = "Field `SHSGPIO0` writer - SHSGPIO0 reset control"]
pub type Shsgpio0W<'a, REG> = crate::BitWriter<'a, REG, Shsgpio0>;
impl<'a, REG> Shsgpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0::SetReset)
    }
}
impl R {
    #[doc = "Bit 2 - SDIO0 reset control"]
    #[inline(always)]
    pub fn sdio0(&self) -> Sdio0R {
        Sdio0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO1 reset control"]
    #[inline(always)]
    pub fn sdio1(&self) -> Sdio1R {
        Sdio1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog comparator reset control"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC reset control"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - SHSGPIO0 reset control"]
    #[inline(always)]
    pub fn shsgpio0(&self) -> Shsgpio0R {
        Shsgpio0R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL1")
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("acmp0", &self.acmp0())
            .field("adc0", &self.adc0())
            .field("shsgpio0", &self.shsgpio0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 reset control"]
    #[inline(always)]
    pub fn sdio0(&mut self) -> Sdio0W<Prstctl1Spec> {
        Sdio0W::new(self, 2)
    }
    #[doc = "Bit 3 - SDIO1 reset control"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> Sdio1W<Prstctl1Spec> {
        Sdio1W::new(self, 3)
    }
    #[doc = "Bit 15 - Analog comparator reset control"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<Prstctl1Spec> {
        Acmp0W::new(self, 15)
    }
    #[doc = "Bit 16 - ADC reset control"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<Prstctl1Spec> {
        Adc0W::new(self, 16)
    }
    #[doc = "Bit 24 - SHSGPIO0 reset control"]
    #[inline(always)]
    pub fn shsgpio0(&mut self) -> Shsgpio0W<Prstctl1Spec> {
        Shsgpio0W::new(self, 24)
    }
}
#[doc = "peripheral reset control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prstctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstctl1Spec;
impl crate::RegisterSpec for Prstctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstctl1::R`](R) reader structure"]
impl crate::Readable for Prstctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`prstctl1::W`](W) writer structure"]
impl crate::Writable for Prstctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCTL1 to value 0x0101_810c"]
impl crate::Resettable for Prstctl1Spec {
    const RESET_VALUE: u32 = 0x0101_810c;
}
