#[doc = "Register `HWVADST10` reader"]
pub type R = crate::R<Hwvadst10Spec>;
#[doc = "Register `HWVADST10` writer"]
pub type W = crate::W<Hwvadst10Spec>;
#[doc = "1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St10 {
    #[doc = "0: Normal operation, waiting for HWVAD trigger event (stage 0)."]
    Normal = 0,
    #[doc = "1: Reset internal interrupt flag by writing a '1' pulse."]
    Reset = 1,
}
impl From<St10> for bool {
    #[inline(always)]
    fn from(variant: St10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST10` reader - 1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
pub type St10R = crate::BitReader<St10>;
impl St10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St10 {
        match self.bits {
            false => St10::Normal,
            true => St10::Reset,
        }
    }
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == St10::Normal
    }
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == St10::Reset
    }
}
#[doc = "Field `ST10` writer - 1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
pub type St10W<'a, REG> = crate::BitWriter<'a, REG, St10>;
impl<'a, REG> St10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(St10::Normal)
    }
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(St10::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - 1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
    #[inline(always)]
    pub fn st10(&self) -> St10R {
        St10R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADST10")
            .field("st10", &self.st10())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
    #[inline(always)]
    pub fn st10(&mut self) -> St10W<Hwvadst10Spec> {
        St10W::new(self, 0)
    }
}
#[doc = "HWVAD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadst10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadst10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwvadst10Spec;
impl crate::RegisterSpec for Hwvadst10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadst10::R`](R) reader structure"]
impl crate::Readable for Hwvadst10Spec {}
#[doc = "`write(|w| ..)` method takes [`hwvadst10::W`](W) writer structure"]
impl crate::Writable for Hwvadst10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADST10 to value 0"]
impl crate::Resettable for Hwvadst10Spec {
    const RESET_VALUE: u32 = 0;
}
