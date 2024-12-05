#[doc = "Register `PRSTCTL2` reader"]
pub type R = crate::R<Prstctl2Spec>;
#[doc = "Register `PRSTCTL2` writer"]
pub type W = crate::W<Prstctl2Spec>;
#[doc = "utick reset control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Utick0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Utick0> for bool {
    #[inline(always)]
    fn from(variant: Utick0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0` reader - utick reset control"]
pub type Utick0R = crate::BitReader<Utick0>;
impl Utick0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Utick0 {
        match self.bits {
            false => Utick0::ClearReset,
            true => Utick0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Utick0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Utick0::SetReset
    }
}
#[doc = "Field `UTICK0` writer - utick reset control"]
pub type Utick0W<'a, REG> = crate::BitWriter<'a, REG, Utick0>;
impl<'a, REG> Utick0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Utick0::SetReset)
    }
}
#[doc = "wdt reset control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdt0 {
    #[doc = "0: clear reset"]
    ClearReset = 0,
    #[doc = "1: set reset"]
    SetReset = 1,
}
impl From<Wwdt0> for bool {
    #[inline(always)]
    fn from(variant: Wwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0` reader - wdt reset control"]
pub type Wwdt0R = crate::BitReader<Wwdt0>;
impl Wwdt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdt0 {
        match self.bits {
            false => Wwdt0::ClearReset,
            true => Wwdt0::SetReset,
        }
    }
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == Wwdt0::ClearReset
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn is_set_reset(&self) -> bool {
        *self == Wwdt0::SetReset
    }
}
#[doc = "Field `WWDT0` writer - wdt reset control"]
pub type Wwdt0W<'a, REG> = crate::BitWriter<'a, REG, Wwdt0>;
impl<'a, REG> Wwdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear reset"]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::ClearReset)
    }
    #[doc = "set reset"]
    #[inline(always)]
    pub fn set_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdt0::SetReset)
    }
}
impl R {
    #[doc = "Bit 0 - utick reset control"]
    #[inline(always)]
    pub fn utick0(&self) -> Utick0R {
        Utick0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt reset control"]
    #[inline(always)]
    pub fn wwdt0(&self) -> Wwdt0R {
        Wwdt0R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCTL2")
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - utick reset control"]
    #[inline(always)]
    pub fn utick0(&mut self) -> Utick0W<Prstctl2Spec> {
        Utick0W::new(self, 0)
    }
    #[doc = "Bit 1 - wdt reset control"]
    #[inline(always)]
    pub fn wwdt0(&mut self) -> Wwdt0W<Prstctl2Spec> {
        Wwdt0W::new(self, 1)
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
#[doc = "`reset()` method sets PRSTCTL2 to value 0x1c00_0001"]
impl crate::Resettable for Prstctl2Spec {
    const RESET_VALUE: u32 = 0x1c00_0001;
}
