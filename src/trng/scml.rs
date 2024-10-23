#[doc = "Register `SCML` reader"]
pub type R = crate::R<ScmlSpec>;
#[doc = "Register `SCML` writer"]
pub type W = crate::W<ScmlSpec>;
#[doc = "Field `MONO_MAX` reader - Monobit Maximum Limit"]
pub type MonoMaxR = crate::FieldReader<u16>;
#[doc = "Field `MONO_MAX` writer - Monobit Maximum Limit"]
pub type MonoMaxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MONO_RNG` reader - Monobit Range"]
pub type MonoRngR = crate::FieldReader<u16>;
#[doc = "Field `MONO_RNG` writer - Monobit Range"]
pub type MonoRngW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&self) -> MonoMaxR {
        MonoMaxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&self) -> MonoRngR {
        MonoRngR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCML")
            .field("mono_max", &self.mono_max())
            .field("mono_rng", &self.mono_rng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn mono_max(&mut self) -> MonoMaxW<ScmlSpec> {
        MonoMaxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    #[must_use]
    pub fn mono_rng(&mut self) -> MonoRngW<ScmlSpec> {
        MonoRngW::new(self, 16)
    }
}
#[doc = "Statistical Check Monobit Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmlSpec;
impl crate::RegisterSpec for ScmlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scml::R`](R) reader structure"]
impl crate::Readable for ScmlSpec {}
#[doc = "`write(|w| ..)` method takes [`scml::W`](W) writer structure"]
impl crate::Writable for ScmlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCML to value 0x010c_0568"]
impl crate::Resettable for ScmlSpec {
    const RESET_VALUE: u32 = 0x010c_0568;
}
