#[doc = "Register `RXEVPULSEGEN` reader"]
pub type R = crate::R<RxevpulsegenSpec>;
#[doc = "Register `RXEVPULSEGEN` writer"]
pub type W = crate::W<RxevpulsegenSpec>;
#[doc = "RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxevpulsegen {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Pulse RXEV High for one PSCLK cycle."]
    PulseRxevHigh = 1,
}
impl From<Rxevpulsegen> for bool {
    #[inline(always)]
    fn from(variant: Rxevpulsegen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEVPULSEGEN` reader - RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
pub type RxevpulsegenR = crate::BitReader<Rxevpulsegen>;
impl RxevpulsegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxevpulsegen {
        match self.bits {
            false => Rxevpulsegen::NoEffect,
            true => Rxevpulsegen::PulseRxevHigh,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Rxevpulsegen::NoEffect
    }
    #[doc = "Pulse RXEV High for one PSCLK cycle."]
    #[inline(always)]
    pub fn is_pulse_rxev_high(&self) -> bool {
        *self == Rxevpulsegen::PulseRxevHigh
    }
}
#[doc = "Field `RXEVPULSEGEN` writer - RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
pub type RxevpulsegenW<'a, REG> = crate::BitWriter<'a, REG, Rxevpulsegen>;
impl<'a, REG> RxevpulsegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxevpulsegen::NoEffect)
    }
    #[doc = "Pulse RXEV High for one PSCLK cycle."]
    #[inline(always)]
    pub fn pulse_rxev_high(self) -> &'a mut crate::W<REG> {
        self.variant(Rxevpulsegen::PulseRxevHigh)
    }
}
impl R {
    #[doc = "Bit 0 - RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
    #[inline(always)]
    pub fn rxevpulsegen(&self) -> RxevpulsegenR {
        RxevpulsegenR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXEVPULSEGEN")
            .field("rxevpulsegen", &self.rxevpulsegen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
    #[inline(always)]
    pub fn rxevpulsegen(&mut self) -> RxevpulsegenW<RxevpulsegenSpec> {
        RxevpulsegenW::new(self, 0)
    }
}
#[doc = "RX Event Pulse Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`rxevpulsegen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxevpulsegen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxevpulsegenSpec;
impl crate::RegisterSpec for RxevpulsegenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxevpulsegen::R`](R) reader structure"]
impl crate::Readable for RxevpulsegenSpec {}
#[doc = "`write(|w| ..)` method takes [`rxevpulsegen::W`](W) writer structure"]
impl crate::Writable for RxevpulsegenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXEVPULSEGEN to value 0"]
impl crate::Resettable for RxevpulsegenSpec {
    const RESET_VALUE: u32 = 0;
}
