#[doc = "Register `SCR3C` reader"]
pub type R = crate::R<Scr3cSpec>;
#[doc = "Field `R3_0_CT` reader - Runs of Zeroes, Length 3 Count"]
pub type R3_0CtR = crate::FieldReader<u16>;
#[doc = "Field `R3_1_CT` reader - Runs of Ones, Length 3 Count"]
pub type R3_1CtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Runs of Zeroes, Length 3 Count"]
    #[inline(always)]
    pub fn r3_0_ct(&self) -> R3_0CtR {
        R3_0CtR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Runs of Ones, Length 3 Count"]
    #[inline(always)]
    pub fn r3_1_ct(&self) -> R3_1CtR {
        R3_1CtR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR3C")
            .field("r3_0_ct", &self.r3_0_ct())
            .field("r3_1_ct", &self.r3_1_ct())
            .finish()
    }
}
#[doc = "Statistical Check Run Length 3 Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr3c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr3cSpec;
impl crate::RegisterSpec for Scr3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr3c::R`](R) reader structure"]
impl crate::Readable for Scr3cSpec {}
#[doc = "`reset()` method sets SCR3C to value 0"]
impl crate::Resettable for Scr3cSpec {
    const RESET_VALUE: u32 = 0;
}
