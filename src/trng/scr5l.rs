#[doc = "Register `SCR5L` reader"]
pub type R = crate::R<Scr5lSpec>;
#[doc = "Register `SCR5L` writer"]
pub type W = crate::W<Scr5lSpec>;
#[doc = "Field `RUN5_MAX` reader - Run Length 5 Maximum Limit"]
pub type Run5MaxR = crate::FieldReader<u16>;
#[doc = "Field `RUN5_MAX` writer - Run Length 5 Maximum Limit"]
pub type Run5MaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RUN5_RNG` reader - Run Length 5 Range"]
pub type Run5RngR = crate::FieldReader<u16>;
#[doc = "Field `RUN5_RNG` writer - Run Length 5 Range"]
pub type Run5RngW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub fn run5_max(&self) -> Run5MaxR {
        Run5MaxR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    pub fn run5_rng(&self) -> Run5RngR {
        Run5RngR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR5L")
            .field("run5_max", &self.run5_max())
            .field("run5_rng", &self.run5_rng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub fn run5_max(&mut self) -> Run5MaxW<Scr5lSpec> {
        Run5MaxW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    pub fn run5_rng(&mut self) -> Run5RngW<Scr5lSpec> {
        Run5RngW::new(self, 16)
    }
}
#[doc = "Statistical Check Run Length 5 Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr5l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr5l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr5lSpec;
impl crate::RegisterSpec for Scr5lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr5l::R`](R) reader structure"]
impl crate::Readable for Scr5lSpec {}
#[doc = "`write(|w| ..)` method takes [`scr5l::W`](W) writer structure"]
impl crate::Writable for Scr5lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR5L to value 0x002e_002f"]
impl crate::Resettable for Scr5lSpec {
    const RESET_VALUE: u32 = 0x002e_002f;
}
