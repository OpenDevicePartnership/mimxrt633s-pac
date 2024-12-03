#[doc = "Register `EVFLAG` reader"]
pub type R = crate::R<EvflagSpec>;
#[doc = "Register `EVFLAG` writer"]
pub type W = crate::W<EvflagSpec>;
#[doc = "Field `FLAG` reader - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type FlagR = crate::FieldReader<u16>;
#[doc = "Field `FLAG` writer - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVFLAG")
            .field("flag", &self.flag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<EvflagSpec> {
        FlagW::new(self, 0)
    }
}
#[doc = "SCT event flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`evflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvflagSpec;
impl crate::RegisterSpec for EvflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evflag::R`](R) reader structure"]
impl crate::Readable for EvflagSpec {}
#[doc = "`write(|w| ..)` method takes [`evflag::W`](W) writer structure"]
impl crate::Writable for EvflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EvflagSpec {
    const RESET_VALUE: u32 = 0;
}
