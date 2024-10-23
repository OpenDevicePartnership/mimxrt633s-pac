#[doc = "Register `SCR6PC` reader"]
pub type R = crate::R<Scr6pcSpec>;
#[doc = "Field `R6P_0_CT` reader - Runs of Zero, Length 6+ Count"]
pub type R6p0CtR = crate::FieldReader<u16>;
#[doc = "Field `R6P_1_CT` reader - Runs of One, Length 6+ Count"]
pub type R6p1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_0_ct(&self) -> R6p0CtR {
        R6p0CtR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_1_ct(&self) -> R6p1CtR {
        R6p1CtR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR6PC")
            .field("r6p_0_ct", &self.r6p_0_ct())
            .field("r6p_1_ct", &self.r6p_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 6+ Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr6pc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr6pcSpec;
impl crate::RegisterSpec for Scr6pcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr6pc::R`](R) reader structure"]
impl crate::Readable for Scr6pcSpec {}
#[doc = "`reset()` method sets SCR6PC to value 0"]
impl crate::Resettable for Scr6pcSpec {
    const RESET_VALUE: u32 = 0;
}
