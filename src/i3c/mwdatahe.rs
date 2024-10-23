#[doc = "Register `MWDATAHE` writer"]
pub type W = crate::W<MwdataheSpec>;
#[doc = "Field `DATA0` writer - DATA 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - DATA 1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MwdataheSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<MwdataheSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<MwdataheSpec> {
        Data1W::new(self, 8)
    }
}
#[doc = "Master Write Data Byte End Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatahe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdataheSpec;
impl crate::RegisterSpec for MwdataheSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatahe::W`](W) writer structure"]
impl crate::Writable for MwdataheSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATAHE to value 0"]
impl crate::Resettable for MwdataheSpec {
    const RESET_VALUE: u32 = 0;
}
