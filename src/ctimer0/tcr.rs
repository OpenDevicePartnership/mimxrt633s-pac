#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Counter enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen {
    #[doc = "0: Disabled.The counters are disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The Timer Counter and Prescale Counter are enabled."]
    Enabled = 1,
}
impl From<Cen> for bool {
    #[inline(always)]
    fn from(variant: Cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable."]
pub type CenR = crate::BitReader<Cen>;
impl CenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen {
        match self.bits {
            false => Cen::Disabled,
            true => Cen::Enabled,
        }
    }
    #[doc = "Disabled.The counters are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cen::Disabled
    }
    #[doc = "Enabled. The Timer Counter and Prescale Counter are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cen::Enabled
    }
}
#[doc = "Field `CEN` writer - Counter enable."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG, Cen>;
impl<'a, REG> CenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled.The counters are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Disabled)
    }
    #[doc = "Enabled. The Timer Counter and Prescale Counter are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Enabled)
    }
}
#[doc = "Counter reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crst {
    #[doc = "0: Disabled. Do nothing."]
    Disabled = 0,
    #[doc = "1: Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    Enabled = 1,
}
impl From<Crst> for bool {
    #[inline(always)]
    fn from(variant: Crst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRST` reader - Counter reset."]
pub type CrstR = crate::BitReader<Crst>;
impl CrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crst {
        match self.bits {
            false => Crst::Disabled,
            true => Crst::Enabled,
        }
    }
    #[doc = "Disabled. Do nothing."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Crst::Disabled
    }
    #[doc = "Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Crst::Enabled
    }
}
#[doc = "Field `CRST` writer - Counter reset."]
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG, Crst>;
impl<'a, REG> CrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Do nothing."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crst::Disabled)
    }
    #[doc = "Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crst::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("cen", &self.cen())
            .field("crst", &self.crst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<TcrSpec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<TcrSpec> {
        CrstW::new(self, 1)
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {
    const RESET_VALUE: u32 = 0;
}
