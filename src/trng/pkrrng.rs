#[doc = "Register `PKRRNG` reader"]
pub type R = crate::R<PkrrngSpec>;
#[doc = "Register `PKRRNG` writer"]
pub type W = crate::W<PkrrngSpec>;
#[doc = "Field `PKR_RNG` reader - Poker Range"]
pub type PkrRngR = crate::FieldReader<u16>;
#[doc = "Field `PKR_RNG` writer - Poker Range"]
pub type PkrRngW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&self) -> PkrRngR {
        PkrRngR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKRRNG")
            .field("pkr_rng", &self.pkr_rng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    #[must_use]
    pub fn pkr_rng(&mut self) -> PkrRngW<PkrrngSpec> {
        PkrRngW::new(self, 0)
    }
}
#[doc = "Poker Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrrng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkrrng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkrrngSpec;
impl crate::RegisterSpec for PkrrngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkrrng::R`](R) reader structure"]
impl crate::Readable for PkrrngSpec {}
#[doc = "`write(|w| ..)` method takes [`pkrrng::W`](W) writer structure"]
impl crate::Writable for PkrrngSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKRRNG to value 0x09a3"]
impl crate::Resettable for PkrrngSpec {
    const RESET_VALUE: u32 = 0x09a3;
}
