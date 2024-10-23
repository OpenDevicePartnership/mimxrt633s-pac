#[doc = "Register `SRDATAH` reader"]
pub type R = crate::R<SrdatahSpec>;
#[doc = "Field `LSB` reader - The 1st byte read from the slave"]
pub type LsbR = crate::FieldReader;
#[doc = "Field `MSB` reader - The 2nd byte read from the slave"]
pub type MsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The 1st byte read from the slave"]
    #[inline(always)]
    pub fn lsb(&self) -> LsbR {
        LsbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The 2nd byte read from the slave"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRDATAH")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[doc = "Slave Read Data Half-word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srdatah::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrdatahSpec;
impl crate::RegisterSpec for SrdatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdatah::R`](R) reader structure"]
impl crate::Readable for SrdatahSpec {}
#[doc = "`reset()` method sets SRDATAH to value 0"]
impl crate::Resettable for SrdatahSpec {
    const RESET_VALUE: u32 = 0;
}
