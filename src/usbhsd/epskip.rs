#[doc = "Register `EPSKIP` reader"]
pub type R = crate::R<EpskipSpec>;
#[doc = "Register `EPSKIP` writer"]
pub type W = crate::W<EpskipSpec>;
#[doc = "Field `SKIP` reader - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
pub type SkipR = crate::FieldReader<u16>;
#[doc = "Field `SKIP` writer - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
pub type SkipW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn skip(&self) -> SkipR {
        SkipR::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPSKIP")
            .field("skip", &self.skip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    #[must_use]
    pub fn skip(&mut self) -> SkipW<EpskipSpec> {
        SkipW::new(self, 0)
    }
}
#[doc = "USB Endpoint skip\n\nYou can [`read`](crate::Reg::read) this register and get [`epskip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epskip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpskipSpec;
impl crate::RegisterSpec for EpskipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epskip::R`](R) reader structure"]
impl crate::Readable for EpskipSpec {}
#[doc = "`write(|w| ..)` method takes [`epskip::W`](W) writer structure"]
impl crate::Writable for EpskipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPSKIP to value 0"]
impl crate::Resettable for EpskipSpec {
    const RESET_VALUE: u32 = 0;
}
