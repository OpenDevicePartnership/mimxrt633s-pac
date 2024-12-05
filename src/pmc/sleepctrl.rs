#[doc = "Register `SLEEPCTRL` reader"]
pub type R = crate::R<SleepctrlSpec>;
#[doc = "Register `SLEEPCTRL` writer"]
pub type W = crate::W<SleepctrlSpec>;
#[doc = "Field `CORELVL` reader - Vddcore voltage value when SYSCTL is in sleep mode"]
pub type CorelvlR = crate::FieldReader;
#[doc = "Field `CORELVL` writer - Vddcore voltage value when SYSCTL is in sleep mode"]
pub type CorelvlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Vddcore voltage value when SYSCTL is in sleep mode"]
    #[inline(always)]
    pub fn corelvl(&self) -> CorelvlR {
        CorelvlR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEPCTRL")
            .field("corelvl", &self.corelvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Vddcore voltage value when SYSCTL is in sleep mode"]
    #[inline(always)]
    pub fn corelvl(&mut self) -> CorelvlW<SleepctrlSpec> {
        CorelvlW::new(self, 0)
    }
}
#[doc = "PMC controls used during deep sleep mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepctrlSpec;
impl crate::RegisterSpec for SleepctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepctrl::R`](R) reader structure"]
impl crate::Readable for SleepctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sleepctrl::W`](W) writer structure"]
impl crate::Writable for SleepctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCTRL to value 0x21"]
impl crate::Resettable for SleepctrlSpec {
    const RESET_VALUE: u32 = 0x21;
}
