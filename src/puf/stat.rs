#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `SUCCESS` reader - Success"]
pub type SuccessR = crate::BitReader;
#[doc = "Field `ERROR` reader - Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `KEYINREQ` reader - Key In Request"]
pub type KeyinreqR = crate::BitReader;
#[doc = "Field `KEYOUTAVAIL` reader - Key Out Available"]
pub type KeyoutavailR = crate::BitReader;
#[doc = "Field `CODEINREQ` reader - Code In Request"]
pub type CodeinreqR = crate::BitReader;
#[doc = "Field `CODEOUTAVAIL` reader - Code Out Available"]
pub type CodeoutavailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    pub fn success(&self) -> SuccessR {
        SuccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KeyinreqR {
        KeyinreqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KeyoutavailR {
        KeyoutavailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CodeinreqR {
        CodeinreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CodeoutavailR {
        CodeoutavailR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("busy", &self.busy())
            .field("success", &self.success())
            .field("error", &self.error())
            .field("keyinreq", &self.keyinreq())
            .field("keyoutavail", &self.keyoutavail())
            .field("codeinreq", &self.codeinreq())
            .field("codeoutavail", &self.codeoutavail())
            .finish()
    }
}
#[doc = "PUF Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x01;
}
