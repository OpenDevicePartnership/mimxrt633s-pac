#[doc = "Register `MWDATAH` writer"]
pub type W = crate::W<MwdatahSpec>;
#[doc = "Field `DATA0` writer - Data byte 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - Data byte 1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `END` writer - End of message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MwdatahSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<MwdatahSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<MwdatahSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bit 16 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<MwdatahSpec> {
        EndW::new(self, 16)
    }
}
#[doc = "Master Write Data Half-word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatah::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdatahSpec;
impl crate::RegisterSpec for MwdatahSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatah::W`](W) writer structure"]
impl crate::Writable for MwdatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATAH to value 0"]
impl crate::Resettable for MwdatahSpec {
    const RESET_VALUE: u32 = 0;
}
