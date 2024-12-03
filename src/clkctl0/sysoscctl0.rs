#[doc = "Register `SYSOSCCTL0` reader"]
pub type R = crate::R<Sysoscctl0Spec>;
#[doc = "Register `SYSOSCCTL0` writer"]
pub type W = crate::W<Sysoscctl0Spec>;
#[doc = "Enable signal for low power mode. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpEnable {
    #[doc = "0: High Gain Mode(HP)."]
    Hp = 0,
    #[doc = "1: Low Power mode (LP)."]
    Lp = 1,
}
impl From<LpEnable> for bool {
    #[inline(always)]
    fn from(variant: LpEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_ENABLE` reader - Enable signal for low power mode. . ."]
pub type LpEnableR = crate::BitReader<LpEnable>;
impl LpEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpEnable {
        match self.bits {
            false => LpEnable::Hp,
            true => LpEnable::Lp,
        }
    }
    #[doc = "High Gain Mode(HP)."]
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        *self == LpEnable::Hp
    }
    #[doc = "Low Power mode (LP)."]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == LpEnable::Lp
    }
}
#[doc = "Field `LP_ENABLE` writer - Enable signal for low power mode. . ."]
pub type LpEnableW<'a, REG> = crate::BitWriter<'a, REG, LpEnable>;
impl<'a, REG> LpEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Gain Mode(HP)."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(LpEnable::Hp)
    }
    #[doc = "Low Power mode (LP)."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(LpEnable::Lp)
    }
}
#[doc = "Enable signal for external bypass clock. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassEnable {
    #[doc = "0: Normal Mode."]
    NormalMode = 0,
    #[doc = "1: Bypass Mode."]
    BypassMode = 1,
}
impl From<BypassEnable> for bool {
    #[inline(always)]
    fn from(variant: BypassEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS_ENABLE` reader - Enable signal for external bypass clock. . ."]
pub type BypassEnableR = crate::BitReader<BypassEnable>;
impl BypassEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassEnable {
        match self.bits {
            false => BypassEnable::NormalMode,
            true => BypassEnable::BypassMode,
        }
    }
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == BypassEnable::NormalMode
    }
    #[doc = "Bypass Mode."]
    #[inline(always)]
    pub fn is_bypass_mode(&self) -> bool {
        *self == BypassEnable::BypassMode
    }
}
#[doc = "Field `BYPASS_ENABLE` writer - Enable signal for external bypass clock. . ."]
pub type BypassEnableW<'a, REG> = crate::BitWriter<'a, REG, BypassEnable>;
impl<'a, REG> BypassEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(BypassEnable::NormalMode)
    }
    #[doc = "Bypass Mode."]
    #[inline(always)]
    pub fn bypass_mode(self) -> &'a mut crate::W<REG> {
        self.variant(BypassEnable::BypassMode)
    }
}
impl R {
    #[doc = "Bit 0 - Enable signal for low power mode. . ."]
    #[inline(always)]
    pub fn lp_enable(&self) -> LpEnableR {
        LpEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable signal for external bypass clock. . ."]
    #[inline(always)]
    pub fn bypass_enable(&self) -> BypassEnableR {
        BypassEnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSOSCCTL0")
            .field("lp_enable", &self.lp_enable())
            .field("bypass_enable", &self.bypass_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable signal for low power mode. . ."]
    #[inline(always)]
    pub fn lp_enable(&mut self) -> LpEnableW<Sysoscctl0Spec> {
        LpEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable signal for external bypass clock. . ."]
    #[inline(always)]
    pub fn bypass_enable(&mut self) -> BypassEnableW<Sysoscctl0Spec> {
        BypassEnableW::new(self, 1)
    }
}
#[doc = "system oscillator control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysoscctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysoscctl0Spec;
impl crate::RegisterSpec for Sysoscctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysoscctl0::R`](R) reader structure"]
impl crate::Readable for Sysoscctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sysoscctl0::W`](W) writer structure"]
impl crate::Writable for Sysoscctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCCTL0 to value 0"]
impl crate::Resettable for Sysoscctl0Spec {
    const RESET_VALUE: u32 = 0;
}
