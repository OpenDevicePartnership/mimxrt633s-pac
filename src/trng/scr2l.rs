#[doc = "Register `SCR2L` reader"]
pub type R = crate::R<Scr2lSpec>;
#[doc = "Register `SCR2L` writer"]
pub type W = crate::W<Scr2lSpec>;
#[doc = "Field `RUN2_MAX` reader - Run Length 2 Maximum Limit"]
pub type Run2MaxR = crate::FieldReader<u16>;
#[doc = "Field `RUN2_MAX` writer - Run Length 2 Maximum Limit"]
pub type Run2MaxW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RUN2_RNG` reader - Run Length 2 Range"]
pub type Run2RngR = crate::FieldReader<u16>;
#[doc = "Field `RUN2_RNG` writer - Run Length 2 Range"]
pub type Run2RngW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub fn run2_max(&self) -> Run2MaxR {
        Run2MaxR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    pub fn run2_rng(&self) -> Run2RngR {
        Run2RngR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR2L")
            .field("run2_max", &self.run2_max())
            .field("run2_rng", &self.run2_rng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run2_max(&mut self) -> Run2MaxW<Scr2lSpec> {
        Run2MaxW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run2_rng(&mut self) -> Run2RngW<Scr2lSpec> {
        Run2RngW::new(self, 16)
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr2lSpec;
impl crate::RegisterSpec for Scr2lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2l::R`](R) reader structure"]
impl crate::Readable for Scr2lSpec {}
#[doc = "`write(|w| ..)` method takes [`scr2l::W`](W) writer structure"]
impl crate::Writable for Scr2lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR2L to value 0x007a_00dc"]
impl crate::Resettable for Scr2lSpec {
    const RESET_VALUE: u32 = 0x007a_00dc;
}
