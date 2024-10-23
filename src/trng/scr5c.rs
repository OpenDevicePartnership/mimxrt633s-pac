#[doc = "Register `SCR5C` reader"]
pub type R = crate::R<Scr5cSpec>;
#[doc = "Field `R5_0_CT` reader - Runs of Zero, Length 5 Count"]
pub type R5_0CtR = crate::FieldReader<u16>;
#[doc = "Field `R5_1_CT` reader - Runs of One, Length 5 Count"]
pub type R5_1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 5 Count"]
    #[inline(always)]
    pub fn r5_0_ct(&self) -> R5_0CtR {
        R5_0CtR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 5 Count"]
    #[inline(always)]
    pub fn r5_1_ct(&self) -> R5_1CtR {
        R5_1CtR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR5C")
            .field("r5_0_ct", &self.r5_0_ct())
            .field("r5_1_ct", &self.r5_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 5 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr5c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr5cSpec;
impl crate::RegisterSpec for Scr5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr5c::R`](R) reader structure"]
impl crate::Readable for Scr5cSpec {}
#[doc = "`reset()` method sets SCR5C to value 0"]
impl crate::Resettable for Scr5cSpec {
    const RESET_VALUE: u32 = 0;
}
