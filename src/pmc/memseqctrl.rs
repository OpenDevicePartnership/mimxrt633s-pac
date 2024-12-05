#[doc = "Register `MEMSEQCTRL` reader"]
pub type R = crate::R<MemseqctrlSpec>;
#[doc = "Register `MEMSEQCTRL` writer"]
pub type W = crate::W<MemseqctrlSpec>;
#[doc = "Field `MEMSEQNUM` reader - Number of memories to turn on/off at a time."]
pub type MemseqnumR = crate::FieldReader;
#[doc = "Field `MEMSEQNUM` writer - Number of memories to turn on/off at a time."]
pub type MemseqnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of memories to turn on/off at a time."]
    #[inline(always)]
    pub fn memseqnum(&self) -> MemseqnumR {
        MemseqnumR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMSEQCTRL")
            .field("memseqnum", &self.memseqnum())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of memories to turn on/off at a time."]
    #[inline(always)]
    pub fn memseqnum(&mut self) -> MemseqnumW<MemseqctrlSpec> {
        MemseqnumW::new(self, 0)
    }
}
#[doc = "Memory Sequencer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memseqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memseqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemseqctrlSpec;
impl crate::RegisterSpec for MemseqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memseqctrl::R`](R) reader structure"]
impl crate::Readable for MemseqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`memseqctrl::W`](W) writer structure"]
impl crate::Writable for MemseqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMSEQCTRL to value 0x3f"]
impl crate::Resettable for MemseqctrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
