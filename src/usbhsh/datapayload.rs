#[doc = "Register `DATAPAYLOAD` reader"]
pub type R = crate::R<DatapayloadSpec>;
#[doc = "Register `DATAPAYLOAD` writer"]
pub type W = crate::W<DatapayloadSpec>;
#[doc = "Field `DAT_BASE` reader - Base address to be used by the hardware to find the start of the data payload section."]
pub type DatBaseR = crate::FieldReader<u16>;
#[doc = "Field `DAT_BASE` writer - Base address to be used by the hardware to find the start of the data payload section."]
pub type DatBaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&self) -> DatBaseR {
        DatBaseR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAPAYLOAD")
            .field("dat_base", &self.dat_base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&mut self) -> DatBaseW<DatapayloadSpec> {
        DatBaseW::new(self, 16)
    }
}
#[doc = "Memory base address that indicates the start of the data payload buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`datapayload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datapayload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatapayloadSpec;
impl crate::RegisterSpec for DatapayloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datapayload::R`](R) reader structure"]
impl crate::Readable for DatapayloadSpec {}
#[doc = "`write(|w| ..)` method takes [`datapayload::W`](W) writer structure"]
impl crate::Writable for DatapayloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAPAYLOAD to value 0"]
impl crate::Resettable for DatapayloadSpec {
    const RESET_VALUE: u32 = 0;
}
