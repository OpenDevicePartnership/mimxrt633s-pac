#[doc = "Register `OSC32KHZCTL0` reader"]
pub type R = crate::R<Osc32khzctl0Spec>;
#[doc = "Register `OSC32KHZCTL0` writer"]
pub type W = crate::W<Osc32khzctl0Spec>;
#[doc = "32KHz Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ena32khz {
    #[doc = "0: disable"]
    Disabled = 0,
    #[doc = "1: enable"]
    Enabled = 1,
}
impl From<Ena32khz> for bool {
    #[inline(always)]
    fn from(variant: Ena32khz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA32KHZ` reader - 32KHz Enable."]
pub type Ena32khzR = crate::BitReader<Ena32khz>;
impl Ena32khzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ena32khz {
        match self.bits {
            false => Ena32khz::Disabled,
            true => Ena32khz::Enabled,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ena32khz::Disabled
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ena32khz::Enabled
    }
}
#[doc = "Field `ENA32KHZ` writer - 32KHz Enable."]
pub type Ena32khzW<'a, REG> = crate::BitWriter<'a, REG, Ena32khz>;
impl<'a, REG> Ena32khzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena32khz::Disabled)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ena32khz::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - 32KHz Enable."]
    #[inline(always)]
    pub fn ena32khz(&self) -> Ena32khzR {
        Ena32khzR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSC32KHZCTL0")
            .field("ena32khz", &self.ena32khz())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 32KHz Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ena32khz(&mut self) -> Ena32khzW<Osc32khzctl0Spec> {
        Ena32khzW::new(self, 0)
    }
}
#[doc = "32k oscillator control0\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32khzctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32khzctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc32khzctl0Spec;
impl crate::RegisterSpec for Osc32khzctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc32khzctl0::R`](R) reader structure"]
impl crate::Readable for Osc32khzctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`osc32khzctl0::W`](W) writer structure"]
impl crate::Writable for Osc32khzctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSC32KHZCTL0 to value 0"]
impl crate::Resettable for Osc32khzctl0Spec {
    const RESET_VALUE: u32 = 0;
}
