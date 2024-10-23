#[doc = "Register `SEC_VIO_ADDR[%s]` reader"]
pub type R = crate::R<SecVioAddrSpec>;
#[doc = "Field `SEC_VIO_ADDR` reader - security violation address for AHB layer"]
pub type SecVioAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - security violation address for AHB layer"]
    #[inline(always)]
    pub fn sec_vio_addr(&self) -> SecVioAddrR {
        SecVioAddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_VIO_ADDR")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[doc = "most recent security violation address for AHB layer n\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_vio_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecVioAddrSpec;
impl crate::RegisterSpec for SecVioAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_vio_addr::R`](R) reader structure"]
impl crate::Readable for SecVioAddrSpec {}
#[doc = "`reset()` method sets SEC_VIO_ADDR[%s]
to value 0"]
impl crate::Resettable for SecVioAddrSpec {
    const RESET_VALUE: u32 = 0;
}
