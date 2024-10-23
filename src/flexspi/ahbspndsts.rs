#[doc = "Register `AHBSPNDSTS` reader"]
pub type R = crate::R<AhbspndstsSpec>;
#[doc = "Field `ACTIVE` reader - Indicates if an AHB read prefetch command sequence has been suspended."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `BUFID` reader - AHB RX BUF ID for suspended command sequence."]
pub type BufidR = crate::FieldReader;
#[doc = "Field `DATLFT` reader - Left Data size for suspended command sequence (in byte)."]
pub type DatlftR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub fn bufid(&self) -> BufidR {
        BufidR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Left Data size for suspended command sequence (in byte)."]
    #[inline(always)]
    pub fn datlft(&self) -> DatlftR {
        DatlftR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSPNDSTS")
            .field("active", &self.active())
            .field("bufid", &self.bufid())
            .field("datlft", &self.datlft())
            .finish()
    }
}
#[doc = "AHB Suspend Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspndsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbspndstsSpec;
impl crate::RegisterSpec for AhbspndstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbspndsts::R`](R) reader structure"]
impl crate::Readable for AhbspndstsSpec {}
#[doc = "`reset()` method sets AHBSPNDSTS to value 0"]
impl crate::Resettable for AhbspndstsSpec {
    const RESET_VALUE: u32 = 0;
}
