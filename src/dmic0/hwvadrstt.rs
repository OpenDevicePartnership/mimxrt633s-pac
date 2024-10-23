#[doc = "Register `HWVADRSTT` reader"]
pub type R = crate::R<HwvadrsttSpec>;
#[doc = "Register `HWVADRSTT` writer"]
pub type W = crate::W<HwvadrsttSpec>;
#[doc = "Field `RSTT` reader - Reset HWVAD. Write back to 0 to release reset."]
pub type RsttR = crate::BitReader;
#[doc = "Field `RSTT` writer - Reset HWVAD. Write back to 0 to release reset."]
pub type RsttW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset HWVAD. Write back to 0 to release reset."]
    #[inline(always)]
    pub fn rstt(&self) -> RsttR {
        RsttR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADRSTT")
            .field("rstt", &self.rstt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset HWVAD. Write back to 0 to release reset."]
    #[inline(always)]
    #[must_use]
    pub fn rstt(&mut self) -> RsttW<HwvadrsttSpec> {
        RsttW::new(self, 0)
    }
}
#[doc = "HWVAD filter reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadrstt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadrstt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadrsttSpec;
impl crate::RegisterSpec for HwvadrsttSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadrstt::R`](R) reader structure"]
impl crate::Readable for HwvadrsttSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvadrstt::W`](W) writer structure"]
impl crate::Writable for HwvadrsttSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADRSTT to value 0"]
impl crate::Resettable for HwvadrsttSpec {
    const RESET_VALUE: u32 = 0;
}
