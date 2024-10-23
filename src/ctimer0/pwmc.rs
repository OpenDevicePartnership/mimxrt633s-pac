#[doc = "Register `PWMC` reader"]
pub type R = crate::R<PwmcSpec>;
#[doc = "Register `PWMC` writer"]
pub type W = crate::W<PwmcSpec>;
#[doc = "PWM mode enable for channel0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen0 {
    #[doc = "0: Match. CTIMERn_MAT0 is controlled by EM0."]
    Match = 0,
    #[doc = "1: PWM. PWM mode is enabled for CTIMERn_MAT0."]
    Pwm = 1,
}
impl From<Pwmen0> for bool {
    #[inline(always)]
    fn from(variant: Pwmen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN0` reader - PWM mode enable for channel0."]
pub type Pwmen0R = crate::BitReader<Pwmen0>;
impl Pwmen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen0 {
        match self.bits {
            false => Pwmen0::Match,
            true => Pwmen0::Pwm,
        }
    }
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Pwmen0::Match
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Pwmen0::Pwm
    }
}
#[doc = "Field `PWMEN0` writer - PWM mode enable for channel0."]
pub type Pwmen0W<'a, REG> = crate::BitWriter<'a, REG, Pwmen0>;
impl<'a, REG> Pwmen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen0::Match)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen0::Pwm)
    }
}
#[doc = "PWM mode enable for channel1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen1 {
    #[doc = "0: Match. CTIMERn_MAT01 is controlled by EM1."]
    Match = 0,
    #[doc = "1: PWM. PWM mode is enabled for CTIMERn_MAT1."]
    Pwm = 1,
}
impl From<Pwmen1> for bool {
    #[inline(always)]
    fn from(variant: Pwmen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN1` reader - PWM mode enable for channel1."]
pub type Pwmen1R = crate::BitReader<Pwmen1>;
impl Pwmen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen1 {
        match self.bits {
            false => Pwmen1::Match,
            true => Pwmen1::Pwm,
        }
    }
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Pwmen1::Match
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Pwmen1::Pwm
    }
}
#[doc = "Field `PWMEN1` writer - PWM mode enable for channel1."]
pub type Pwmen1W<'a, REG> = crate::BitWriter<'a, REG, Pwmen1>;
impl<'a, REG> Pwmen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen1::Match)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen1::Pwm)
    }
}
#[doc = "PWM mode enable for channel2.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen2 {
    #[doc = "0: Match. CTIMERn_MAT2 is controlled by EM2."]
    Match = 0,
    #[doc = "1: PWM. PWM mode is enabled for CTIMERn_MAT2."]
    Pwm = 1,
}
impl From<Pwmen2> for bool {
    #[inline(always)]
    fn from(variant: Pwmen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN2` reader - PWM mode enable for channel2."]
pub type Pwmen2R = crate::BitReader<Pwmen2>;
impl Pwmen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen2 {
        match self.bits {
            false => Pwmen2::Match,
            true => Pwmen2::Pwm,
        }
    }
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Pwmen2::Match
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Pwmen2::Pwm
    }
}
#[doc = "Field `PWMEN2` writer - PWM mode enable for channel2."]
pub type Pwmen2W<'a, REG> = crate::BitWriter<'a, REG, Pwmen2>;
impl<'a, REG> Pwmen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen2::Match)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen2::Pwm)
    }
}
#[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmen3 {
    #[doc = "0: Match. CTIMERn_MAT3 is controlled by EM3."]
    Match = 0,
    #[doc = "1: PWM. PWM mode is enabled for CT132Bn_MAT3."]
    Pwm = 1,
}
impl From<Pwmen3> for bool {
    #[inline(always)]
    fn from(variant: Pwmen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN3` reader - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
pub type Pwmen3R = crate::BitReader<Pwmen3>;
impl Pwmen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmen3 {
        match self.bits {
            false => Pwmen3::Match,
            true => Pwmen3::Pwm,
        }
    }
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Pwmen3::Match
    }
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Pwmen3::Pwm
    }
}
#[doc = "Field `PWMEN3` writer - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
pub type Pwmen3W<'a, REG> = crate::BitWriter<'a, REG, Pwmen3>;
impl<'a, REG> Pwmen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen3::Match)
    }
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmen3::Pwm)
    }
}
impl R {
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    pub fn pwmen0(&self) -> Pwmen0R {
        Pwmen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    pub fn pwmen1(&self) -> Pwmen1R {
        Pwmen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    pub fn pwmen2(&self) -> Pwmen2R {
        Pwmen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub fn pwmen3(&self) -> Pwmen3R {
        Pwmen3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMC")
            .field("pwmen0", &self.pwmen0())
            .field("pwmen1", &self.pwmen1())
            .field("pwmen2", &self.pwmen2())
            .field("pwmen3", &self.pwmen3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    #[must_use]
    pub fn pwmen0(&mut self) -> Pwmen0W<PwmcSpec> {
        Pwmen0W::new(self, 0)
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    #[must_use]
    pub fn pwmen1(&mut self) -> Pwmen1W<PwmcSpec> {
        Pwmen1W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    #[must_use]
    pub fn pwmen2(&mut self) -> Pwmen2W<PwmcSpec> {
        Pwmen2W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pwmen3(&mut self) -> Pwmen3W<PwmcSpec> {
        Pwmen3W::new(self, 3)
    }
}
#[doc = "PWM Control Register. The PWMCON enables PWM mode for the external match pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmcSpec;
impl crate::RegisterSpec for PwmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmc::R`](R) reader structure"]
impl crate::Readable for PwmcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmc::W`](W) writer structure"]
impl crate::Writable for PwmcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMC to value 0"]
impl crate::Resettable for PwmcSpec {
    const RESET_VALUE: u32 = 0;
}
