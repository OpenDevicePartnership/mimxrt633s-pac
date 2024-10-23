#[doc = "Register `SCR4C` reader"]
pub type R = crate::R<Scr4cSpec>;
#[doc = "Field `R4_0_CT` reader - Runs of Zero, Length 4 Count"]
pub type R4_0CtR = crate::FieldReader<u16>;
#[doc = "Field `R4_1_CT` reader - Runs of One, Length 4 Count"]
pub type R4_1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Runs of Zero, Length 4 Count"]
    #[inline(always)]
    pub fn r4_0_ct(&self) -> R4_0CtR {
        R4_0CtR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Runs of One, Length 4 Count"]
    #[inline(always)]
    pub fn r4_1_ct(&self) -> R4_1CtR {
        R4_1CtR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR4C")
            .field("r4_0_ct", &self.r4_0_ct())
            .field("r4_1_ct", &self.r4_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 4 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr4c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr4cSpec;
impl crate::RegisterSpec for Scr4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr4c::R`](R) reader structure"]
impl crate::Readable for Scr4cSpec {}
#[doc = "`reset()` method sets SCR4C to value 0"]
impl crate::Resettable for Scr4cSpec {
    const RESET_VALUE: u32 = 0;
}
