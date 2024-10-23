#[doc = "Register `SCR2C` reader"]
pub type R = crate::R<Scr2cSpec>;
#[doc = "Field `R2_0_CT` reader - Runs of Zero, Length 2 Count"]
pub type R2_0CtR = crate::FieldReader<u16>;
#[doc = "Field `R2_1_CT` reader - Runs of One, Length 2 Count"]
pub type R2_1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Runs of Zero, Length 2 Count"]
    #[inline(always)]
    pub fn r2_0_ct(&self) -> R2_0CtR {
        R2_0CtR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Runs of One, Length 2 Count"]
    #[inline(always)]
    pub fn r2_1_ct(&self) -> R2_1CtR {
        R2_1CtR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR2C")
            .field("r2_0_ct", &self.r2_0_ct())
            .field("r2_1_ct", &self.r2_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 2 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr2cSpec;
impl crate::RegisterSpec for Scr2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2c::R`](R) reader structure"]
impl crate::Readable for Scr2cSpec {}
#[doc = "`reset()` method sets SCR2C to value 0"]
impl crate::Resettable for Scr2cSpec {
    const RESET_VALUE: u32 = 0;
}
