#[doc = "Register `RUNCTRL` reader"]
pub type R = crate::R<RunctrlSpec>;
#[doc = "Register `RUNCTRL` writer"]
pub type W = crate::W<RunctrlSpec>;
#[doc = "Field `CORELVL` reader - Vddcore voltage value when SYSCTL is in run mode"]
pub type CorelvlR = crate::FieldReader;
#[doc = "Field `CORELVL` writer - Vddcore voltage value when SYSCTL is in run mode"]
pub type CorelvlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Vddcore voltage value when SYSCTL is in run mode"]
    #[inline(always)]
    pub fn corelvl(&self) -> CorelvlR {
        CorelvlR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RUNCTRL")
            .field("corelvl", &self.corelvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Vddcore voltage value when SYSCTL is in run mode"]
    #[inline(always)]
    pub fn corelvl(&mut self) -> CorelvlW<RunctrlSpec> {
        CorelvlW::new(self, 0)
    }
}
#[doc = "PMC controls used during run mode\n\nYou can [`read`](crate::Reg::read) this register and get [`runctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`runctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RunctrlSpec;
impl crate::RegisterSpec for RunctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`runctrl::R`](R) reader structure"]
impl crate::Readable for RunctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`runctrl::W`](W) writer structure"]
impl crate::Writable for RunctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RUNCTRL to value 0x21"]
impl crate::Resettable for RunctrlSpec {
    const RESET_VALUE: u32 = 0x21;
}
