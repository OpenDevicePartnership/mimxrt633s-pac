#[doc = "Register `KEYOUTINDEX` reader"]
pub type R = crate::R<KeyoutindexSpec>;
#[doc = "Field `KEYOUTIDX` reader - Key Output Index"]
pub type KeyoutidxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Key Output Index"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KeyoutidxR {
        KeyoutidxR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYOUTINDEX")
            .field("keyoutidx", &self.keyoutidx())
            .finish()
    }
}
#[doc = "PUF Key Output Index\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutindex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyoutindexSpec;
impl crate::RegisterSpec for KeyoutindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyoutindex::R`](R) reader structure"]
impl crate::Readable for KeyoutindexSpec {}
#[doc = "`reset()` method sets KEYOUTINDEX to value 0"]
impl crate::Resettable for KeyoutindexSpec {
    const RESET_VALUE: u32 = 0;
}
