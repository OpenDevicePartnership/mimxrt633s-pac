#[doc = "Register `OUTPUT` reader"]
pub type R = crate::R<OutputSpec>;
#[doc = "Register `OUTPUT` writer"]
pub type W = crate::W<OutputSpec>;
#[doc = "Field `OUT` reader - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type OutR = crate::FieldReader<u16>;
#[doc = "Field `OUT` writer - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUT").field("out", &self.out()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<OutputSpec> {
        OutW::new(self, 0)
    }
}
#[doc = "SCT output register\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputSpec;
impl crate::RegisterSpec for OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`output::R`](R) reader structure"]
impl crate::Readable for OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OutputSpec {
    const RESET_VALUE: u32 = 0;
}
