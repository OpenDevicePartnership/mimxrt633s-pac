#[doc = "Register `MSTDAT` reader"]
pub type R = crate::R<MstdatSpec>;
#[doc = "Register `MSTDAT` writer"]
pub type W = crate::W<MstdatSpec>;
#[doc = "Field `DATA` reader - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTDAT")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MstdatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Combined Master receiver and transmitter data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstdatSpec;
impl crate::RegisterSpec for MstdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstdat::R`](R) reader structure"]
impl crate::Readable for MstdatSpec {}
#[doc = "`write(|w| ..)` method takes [`mstdat::W`](W) writer structure"]
impl crate::Writable for MstdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTDAT to value 0"]
impl crate::Resettable for MstdatSpec {
    const RESET_VALUE: u32 = 0;
}
