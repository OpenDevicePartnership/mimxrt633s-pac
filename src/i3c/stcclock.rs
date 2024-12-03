#[doc = "Register `STCCLOCK` reader"]
pub type R = crate::R<StcclockSpec>;
#[doc = "Register `STCCLOCK` writer"]
pub type W = crate::W<StcclockSpec>;
#[doc = "Field `ACCURACY` reader - Clock accuracy"]
pub type AccuracyR = crate::FieldReader;
#[doc = "Field `ACCURACY` writer - Clock accuracy"]
pub type AccuracyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREQ` reader - Clock frequency"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - Clock frequency"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock accuracy"]
    #[inline(always)]
    pub fn accuracy(&self) -> AccuracyR {
        AccuracyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCCLOCK")
            .field("accuracy", &self.accuracy())
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock accuracy"]
    #[inline(always)]
    pub fn accuracy(&mut self) -> AccuracyW<StcclockSpec> {
        AccuracyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<StcclockSpec> {
        FreqW::new(self, 8)
    }
}
#[doc = "Slave Time Control Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcclockSpec;
impl crate::RegisterSpec for StcclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcclock::R`](R) reader structure"]
impl crate::Readable for StcclockSpec {}
#[doc = "`write(|w| ..)` method takes [`stcclock::W`](W) writer structure"]
impl crate::Writable for StcclockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCCLOCK to value 0x0214"]
impl crate::Resettable for StcclockSpec {
    const RESET_VALUE: u32 = 0x0214;
}
