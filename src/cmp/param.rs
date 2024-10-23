#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `PARAM` reader - Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
pub type ParamR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
    #[inline(always)]
    pub fn param(&self) -> ParamR {
        ParamR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("param", &self.param())
            .finish()
    }
}
#[doc = "Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0;
}
