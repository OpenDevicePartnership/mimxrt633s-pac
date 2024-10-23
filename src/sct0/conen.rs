#[doc = "Register `CONEN` reader"]
pub type R = crate::R<ConenSpec>;
#[doc = "Register `CONEN` writer"]
pub type W = crate::W<ConenSpec>;
#[doc = "Field `NCEN` reader - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NcenR = crate::FieldReader<u16>;
#[doc = "Field `NCEN` writer - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NcenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&self) -> NcenR {
        NcenR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONEN").field("ncen", &self.ncen()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn ncen(&mut self) -> NcenW<ConenSpec> {
        NcenW::new(self, 0)
    }
}
#[doc = "SCT conflict interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`conen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConenSpec;
impl crate::RegisterSpec for ConenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conen::R`](R) reader structure"]
impl crate::Readable for ConenSpec {}
#[doc = "`write(|w| ..)` method takes [`conen::W`](W) writer structure"]
impl crate::Writable for ConenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONEN to value 0"]
impl crate::Resettable for ConenSpec {
    const RESET_VALUE: u32 = 0;
}
