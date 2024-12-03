#[doc = "Register `RISE` reader"]
pub type R = crate::R<RiseSpec>;
#[doc = "Register `RISE` writer"]
pub type W = crate::W<RiseSpec>;
#[doc = "Field `RDET` reader - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub type RdetR = crate::FieldReader;
#[doc = "Field `RDET` writer - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub type RdetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&self) -> RdetR {
        RdetR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RISE").field("rdet", &self.rdet()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&mut self) -> RdetW<RiseSpec> {
        RdetW::new(self, 0)
    }
}
#[doc = "Pin interrupt rising edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RiseSpec;
impl crate::RegisterSpec for RiseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rise::R`](R) reader structure"]
impl crate::Readable for RiseSpec {}
#[doc = "`write(|w| ..)` method takes [`rise::W`](W) writer structure"]
impl crate::Writable for RiseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RISE to value 0"]
impl crate::Resettable for RiseSpec {
    const RESET_VALUE: u32 = 0;
}
