#[doc = "Register `MRDATAH` reader"]
pub type R = crate::R<MrdatahSpec>;
#[doc = "Field `LSB` reader - LSB"]
pub type LsbR = crate::FieldReader;
#[doc = "Field `MSB` reader - MSB"]
pub type MsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - LSB"]
    #[inline(always)]
    pub fn lsb(&self) -> LsbR {
        LsbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MSB"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRDATAH")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[doc = "Master Read Data Half-word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatah::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrdatahSpec;
impl crate::RegisterSpec for MrdatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdatah::R`](R) reader structure"]
impl crate::Readable for MrdatahSpec {}
#[doc = "`reset()` method sets MRDATAH to value 0"]
impl crate::Resettable for MrdatahSpec {
    const RESET_VALUE: u32 = 0;
}
