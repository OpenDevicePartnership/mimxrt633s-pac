#[doc = "Register `KEYOUTPUT` reader"]
pub type R = crate::R<KeyoutputSpec>;
#[doc = "Field `KEYOUT` reader - Key Output Data"]
pub type KeyoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Key Output Data"]
    #[inline(always)]
    pub fn keyout(&self) -> KeyoutR {
        KeyoutR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYOUTPUT")
            .field("keyout", &self.keyout())
            .finish()
    }
}
#[doc = "PUF Key Output\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutput::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyoutputSpec;
impl crate::RegisterSpec for KeyoutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyoutput::R`](R) reader structure"]
impl crate::Readable for KeyoutputSpec {}
#[doc = "`reset()` method sets KEYOUTPUT to value 0"]
impl crate::Resettable for KeyoutputSpec {
    const RESET_VALUE: u32 = 0;
}
