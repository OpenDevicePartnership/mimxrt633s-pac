#[doc = "Register `IPTXFSTS` reader"]
pub type R = crate::R<IptxfstsSpec>;
#[doc = "Field `FILL` reader - Fill level of IP TX FIFO."]
pub type FillR = crate::FieldReader;
#[doc = "Field `WRCNTR` reader - Total Write Data Counter: WRCNTR * 64 Bits."]
pub type WrcntrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP TX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FillR {
        FillR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub fn wrcntr(&self) -> WrcntrR {
        WrcntrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPTXFSTS")
            .field("fill", &self.fill())
            .field("wrcntr", &self.wrcntr())
            .finish()
    }
}
#[doc = "IP TX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iptxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IptxfstsSpec;
impl crate::RegisterSpec for IptxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iptxfsts::R`](R) reader structure"]
impl crate::Readable for IptxfstsSpec {}
#[doc = "`reset()` method sets IPTXFSTS to value 0"]
impl crate::Resettable for IptxfstsSpec {
    const RESET_VALUE: u32 = 0;
}
