#[doc = "Register `HWVADTHGN` reader"]
pub type R = crate::R<HwvadthgnSpec>;
#[doc = "Register `HWVADTHGN` writer"]
pub type W = crate::W<HwvadthgnSpec>;
#[doc = "Field `THGN` reader - Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
pub type ThgnR = crate::FieldReader;
#[doc = "Field `THGN` writer - Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
pub type ThgnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
    #[inline(always)]
    pub fn thgn(&self) -> ThgnR {
        ThgnR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADTHGN")
            .field("thgn", &self.thgn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
    #[inline(always)]
    pub fn thgn(&mut self) -> ThgnW<HwvadthgnSpec> {
        ThgnW::new(self, 0)
    }
}
#[doc = "HWVAD noise estimator gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadthgn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadthgn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadthgnSpec;
impl crate::RegisterSpec for HwvadthgnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadthgn::R`](R) reader structure"]
impl crate::Readable for HwvadthgnSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvadthgn::W`](W) writer structure"]
impl crate::Writable for HwvadthgnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADTHGN to value 0"]
impl crate::Resettable for HwvadthgnSpec {
    const RESET_VALUE: u32 = 0;
}
