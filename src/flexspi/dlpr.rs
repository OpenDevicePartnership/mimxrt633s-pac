#[doc = "Register `DLPR` reader"]
pub type R = crate::R<DlprSpec>;
#[doc = "Register `DLPR` writer"]
pub type W = crate::W<DlprSpec>;
#[doc = "Field `DLP` reader - Data Learning Pattern."]
pub type DlpR = crate::FieldReader<u32>;
#[doc = "Field `DLP` writer - Data Learning Pattern."]
pub type DlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Learning Pattern."]
    #[inline(always)]
    pub fn dlp(&self) -> DlpR {
        DlpR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLPR").field("dlp", &self.dlp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Learning Pattern."]
    #[inline(always)]
    #[must_use]
    pub fn dlp(&mut self) -> DlpW<DlprSpec> {
        DlpW::new(self, 0)
    }
}
#[doc = "Data Learn Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlprSpec;
impl crate::RegisterSpec for DlprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlpr::R`](R) reader structure"]
impl crate::Readable for DlprSpec {}
#[doc = "`write(|w| ..)` method takes [`dlpr::W`](W) writer structure"]
impl crate::Writable for DlprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLPR to value 0"]
impl crate::Resettable for DlprSpec {
    const RESET_VALUE: u32 = 0;
}
