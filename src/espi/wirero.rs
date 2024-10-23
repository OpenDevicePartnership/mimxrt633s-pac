#[doc = "Register `WIRERO` reader"]
pub type R = crate::R<WireroSpec>;
#[doc = "Field `WIRES` reader - eSPI: Indicates the latest states from the Host via VWire"]
pub type WiresR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - eSPI: Indicates the latest states from the Host via VWire"]
    #[inline(always)]
    pub fn wires(&self) -> WiresR {
        WiresR::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIRERO")
            .field("wires", &self.wires())
            .finish()
    }
}
#[doc = "Wire states from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`wirero::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WireroSpec;
impl crate::RegisterSpec for WireroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wirero::R`](R) reader structure"]
impl crate::Readable for WireroSpec {}
#[doc = "`reset()` method sets WIRERO to value 0"]
impl crate::Resettable for WireroSpec {
    const RESET_VALUE: u32 = 0;
}
