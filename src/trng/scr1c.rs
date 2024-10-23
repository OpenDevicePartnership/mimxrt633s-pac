#[doc = "Register `SCR1C` reader"]
pub type R = crate::R<Scr1cSpec>;
#[doc = "Field `R1_0_CT` reader - Runs of Zero, Length 1 Count"]
pub type R1_0CtR = crate::FieldReader<u16>;
#[doc = "Field `R1_1_CT` reader - Runs of One, Length 1 Count"]
pub type R1_1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub fn r1_0_ct(&self) -> R1_0CtR {
        R1_0CtR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Runs of One, Length 1 Count"]
    #[inline(always)]
    pub fn r1_1_ct(&self) -> R1_1CtR {
        R1_1CtR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR1C")
            .field("r1_0_ct", &self.r1_0_ct())
            .field("r1_1_ct", &self.r1_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 1 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr1cSpec;
impl crate::RegisterSpec for Scr1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr1c::R`](R) reader structure"]
impl crate::Readable for Scr1cSpec {}
#[doc = "`reset()` method sets SCR1C to value 0"]
impl crate::Resettable for Scr1cSpec {
    const RESET_VALUE: u32 = 0;
}
