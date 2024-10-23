#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `SSA` reader - Slave Select Assert."]
pub type SsaR = crate::BitReader;
#[doc = "Field `SSD` reader - Slave Select Deassert."]
pub type SsdR = crate::BitReader;
#[doc = "Field `MSTIDLE` reader - Master Idle status flag."]
pub type MstidleR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Slave Select Assert."]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deassert."]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Idle status flag."]
    #[inline(always)]
    pub fn mstidle(&self) -> MstidleR {
        MstidleR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("ssa", &self.ssa())
            .field("ssd", &self.ssd())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
#[doc = "SPI Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
