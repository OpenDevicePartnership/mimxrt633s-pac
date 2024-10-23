#[doc = "Register `HWVADGAIN` reader"]
pub type R = crate::R<HwvadgainSpec>;
#[doc = "Register `HWVADGAIN` writer"]
pub type W = crate::W<HwvadgainSpec>;
#[doc = "Field `INPUTGAIN` reader - Gain factor for input signal into HWVAD"]
pub type InputgainR = crate::FieldReader;
#[doc = "Field `INPUTGAIN` writer - Gain factor for input signal into HWVAD"]
pub type InputgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Gain factor for input signal into HWVAD"]
    #[inline(always)]
    pub fn inputgain(&self) -> InputgainR {
        InputgainR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADGAIN")
            .field("inputgain", &self.inputgain())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain factor for input signal into HWVAD"]
    #[inline(always)]
    #[must_use]
    pub fn inputgain(&mut self) -> InputgainW<HwvadgainSpec> {
        InputgainW::new(self, 0)
    }
}
#[doc = "HWVAD input gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadgain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadgain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadgainSpec;
impl crate::RegisterSpec for HwvadgainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadgain::R`](R) reader structure"]
impl crate::Readable for HwvadgainSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvadgain::W`](W) writer structure"]
impl crate::Writable for HwvadgainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADGAIN to value 0x05"]
impl crate::Resettable for HwvadgainSpec {
    const RESET_VALUE: u32 = 0x05;
}
