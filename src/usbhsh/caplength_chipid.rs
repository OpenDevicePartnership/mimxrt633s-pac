#[doc = "Register `CAPLENGTH_CHIPID` reader"]
pub type R = crate::R<CaplengthChipidSpec>;
#[doc = "Field `CAPLENGTH` reader - Capability Length: This is used as an offset."]
pub type CaplengthR = crate::FieldReader;
#[doc = "Field `CHIPID` reader - Chip identification: indicates major and minor revision of the IP: \\[31:24\\]
= Major revision \\[23:16\\]
= Minor revision Major revisions used: 0x01: USB2."]
pub type ChipidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Capability Length: This is used as an offset."]
    #[inline(always)]
    pub fn caplength(&self) -> CaplengthR {
        CaplengthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Chip identification: indicates major and minor revision of the IP: \\[31:24\\]
= Major revision \\[23:16\\]
= Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub fn chipid(&self) -> ChipidR {
        ChipidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPLENGTH_CHIPID")
            .field("caplength", &self.caplength())
            .field("chipid", &self.chipid())
            .finish()
    }
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block\n\nYou can [`read`](crate::Reg::read) this register and get [`caplength_chipid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaplengthChipidSpec;
impl crate::RegisterSpec for CaplengthChipidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caplength_chipid::R`](R) reader structure"]
impl crate::Readable for CaplengthChipidSpec {}
#[doc = "`reset()` method sets CAPLENGTH_CHIPID to value 0x0101_0010"]
impl crate::Resettable for CaplengthChipidSpec {
    const RESET_VALUE: u32 = 0x0101_0010;
}
