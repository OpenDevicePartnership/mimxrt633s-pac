#[doc = "Register `FIFORDNOPOP` reader"]
pub type R = crate::R<FifordnopopSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO."]
pub type RxdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFORDNOPOP")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[doc = "FIFO data read with no FIFO pop.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifordnopop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifordnopopSpec;
impl crate::RegisterSpec for FifordnopopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifordnopop::R`](R) reader structure"]
impl crate::Readable for FifordnopopSpec {}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FifordnopopSpec {
    const RESET_VALUE: u32 = 0;
}
