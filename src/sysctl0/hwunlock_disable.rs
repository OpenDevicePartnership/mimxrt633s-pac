#[doc = "Register `HWUNLOCK_DISABLE` reader"]
pub type R = crate::R<HwunlockDisableSpec>;
#[doc = "Register `HWUNLOCK_DISABLE` writer"]
pub type W = crate::W<HwunlockDisableSpec>;
#[doc = "Field `HWUNLOCK_DISABLE` reader - HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
pub type HwunlockDisableR = crate::FieldReader;
#[doc = "Field `HWUNLOCK_DISABLE` writer - HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
pub type HwunlockDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
    #[inline(always)]
    pub fn hwunlock_disable(&self) -> HwunlockDisableR {
        HwunlockDisableR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWUNLOCK_DISABLE")
            .field("hwunlock_disable", &self.hwunlock_disable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn hwunlock_disable(&mut self) -> HwunlockDisableW<HwunlockDisableSpec> {
        HwunlockDisableW::new(self, 0)
    }
}
#[doc = "HW unlock disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hwunlock_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwunlock_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwunlockDisableSpec;
impl crate::RegisterSpec for HwunlockDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwunlock_disable::R`](R) reader structure"]
impl crate::Readable for HwunlockDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`hwunlock_disable::W`](W) writer structure"]
impl crate::Writable for HwunlockDisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWUNLOCK_DISABLE to value 0"]
impl crate::Resettable for HwunlockDisableSpec {
    const RESET_VALUE: u32 = 0;
}
