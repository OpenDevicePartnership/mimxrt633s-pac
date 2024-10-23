#[doc = "Register `JTAG_ID` reader"]
pub type R = crate::R<JtagIdSpec>;
#[doc = "Field `FIXBIT` reader - JTAG IDCODE fix bit"]
pub type FixbitR = crate::BitReader;
#[doc = "Field `MANU` reader - JTAG IDCODE manufacturer identity"]
pub type ManuR = crate::FieldReader<u16>;
#[doc = "Field `PARTNUM` reader - JTAG IDCODE part number"]
pub type PartnumR = crate::FieldReader<u16>;
#[doc = "Field `VERNUM` reader - JTAG IDCODE version number"]
pub type VernumR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - JTAG IDCODE fix bit"]
    #[inline(always)]
    pub fn fixbit(&self) -> FixbitR {
        FixbitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - JTAG IDCODE manufacturer identity"]
    #[inline(always)]
    pub fn manu(&self) -> ManuR {
        ManuR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - JTAG IDCODE part number"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - JTAG IDCODE version number"]
    #[inline(always)]
    pub fn vernum(&self) -> VernumR {
        VernumR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_ID")
            .field("fixbit", &self.fixbit())
            .field("manu", &self.manu())
            .field("partnum", &self.partnum())
            .field("vernum", &self.vernum())
            .finish()
    }
}
#[doc = "jtag ID\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagIdSpec;
impl crate::RegisterSpec for JtagIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag_id::R`](R) reader structure"]
impl crate::Readable for JtagIdSpec {}
#[doc = "`reset()` method sets JTAG_ID to value 0"]
impl crate::Resettable for JtagIdSpec {
    const RESET_VALUE: u32 = 0;
}
