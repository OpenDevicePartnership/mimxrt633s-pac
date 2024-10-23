#[doc = "Register `TEMPSENSORCTL` reader"]
pub type R = crate::R<TempsensorctlSpec>;
#[doc = "Register `TEMPSENSORCTL` writer"]
pub type W = crate::W<TempsensorctlSpec>;
#[doc = "Temperature Sensor Source. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssrc {
    #[doc = "0: ADC Built-in Temperature Sensor."]
    AdcTs = 0,
}
impl From<Tssrc> for bool {
    #[inline(always)]
    fn from(variant: Tssrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSSRC` reader - Temperature Sensor Source. . ."]
pub type TssrcR = crate::BitReader<Tssrc>;
impl TssrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tssrc> {
        match self.bits {
            false => Some(Tssrc::AdcTs),
            _ => None,
        }
    }
    #[doc = "ADC Built-in Temperature Sensor."]
    #[inline(always)]
    pub fn is_adc_ts(&self) -> bool {
        *self == Tssrc::AdcTs
    }
}
#[doc = "Field `TSSRC` writer - Temperature Sensor Source. . ."]
pub type TssrcW<'a, REG> = crate::BitWriter<'a, REG, Tssrc>;
impl<'a, REG> TssrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC Built-in Temperature Sensor."]
    #[inline(always)]
    pub fn adc_ts(self) -> &'a mut crate::W<REG> {
        self.variant(Tssrc::AdcTs)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor Source. . ."]
    #[inline(always)]
    pub fn tssrc(&self) -> TssrcR {
        TssrcR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPSENSORCTL")
            .field("tssrc", &self.tssrc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor Source. . ."]
    #[inline(always)]
    #[must_use]
    pub fn tssrc(&mut self) -> TssrcW<TempsensorctlSpec> {
        TssrcW::new(self, 0)
    }
}
#[doc = "tempsensor ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`tempsensorctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempsensorctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempsensorctlSpec;
impl crate::RegisterSpec for TempsensorctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempsensorctl::R`](R) reader structure"]
impl crate::Readable for TempsensorctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tempsensorctl::W`](W) writer structure"]
impl crate::Writable for TempsensorctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEMPSENSORCTL to value 0"]
impl crate::Resettable for TempsensorctlSpec {
    const RESET_VALUE: u32 = 0;
}
