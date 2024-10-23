#[doc = "Register `FALL` reader"]
pub type R = crate::R<FallSpec>;
#[doc = "Register `FALL` writer"]
pub type W = crate::W<FallSpec>;
#[doc = "Field `FDET` reader - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub type FdetR = crate::FieldReader;
#[doc = "Field `FDET` writer - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub type FdetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&self) -> FdetR {
        FdetR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FALL").field("fdet", &self.fdet()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    #[must_use]
    pub fn fdet(&mut self) -> FdetW<FallSpec> {
        FdetW::new(self, 0)
    }
}
#[doc = "Pin interrupt falling edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FallSpec;
impl crate::RegisterSpec for FallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fall::R`](R) reader structure"]
impl crate::Readable for FallSpec {}
#[doc = "`write(|w| ..)` method takes [`fall::W`](W) writer structure"]
impl crate::Writable for FallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FALL to value 0"]
impl crate::Resettable for FallSpec {
    const RESET_VALUE: u32 = 0;
}
