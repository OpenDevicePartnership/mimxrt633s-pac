#[doc = "Register `HWVADTHGS` reader"]
pub type R = crate::R<HwvadthgsSpec>;
#[doc = "Register `HWVADTHGS` writer"]
pub type W = crate::W<HwvadthgsSpec>;
#[doc = "Field `THGS` reader - Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
pub type ThgsR = crate::FieldReader;
#[doc = "Field `THGS` writer - Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
pub type ThgsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
    #[inline(always)]
    pub fn thgs(&self) -> ThgsR {
        ThgsR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADTHGS")
            .field("thgs", &self.thgs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
    #[inline(always)]
    #[must_use]
    pub fn thgs(&mut self) -> ThgsW<HwvadthgsSpec> {
        ThgsW::new(self, 0)
    }
}
#[doc = "HWVAD signal estimator gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadthgs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadthgs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadthgsSpec;
impl crate::RegisterSpec for HwvadthgsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadthgs::R`](R) reader structure"]
impl crate::Readable for HwvadthgsSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvadthgs::W`](W) writer structure"]
impl crate::Writable for HwvadthgsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADTHGS to value 0x04"]
impl crate::Resettable for HwvadthgsSpec {
    const RESET_VALUE: u32 = 0x04;
}
