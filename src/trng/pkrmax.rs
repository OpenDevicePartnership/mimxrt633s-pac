#[doc = "Register `PKRMAX` reader"]
pub type R = crate::R<PkrmaxSpec>;
#[doc = "Register `PKRMAX` writer"]
pub type W = crate::W<PkrmaxSpec>;
#[doc = "Field `PKR_MAX` reader - Poker Maximum Limit."]
pub type PkrMaxR = crate::FieldReader<u32>;
#[doc = "Field `PKR_MAX` writer - Poker Maximum Limit."]
pub type PkrMaxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    pub fn pkr_max(&self) -> PkrMaxR {
        PkrMaxR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKRMAX")
            .field("pkr_max", &self.pkr_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    #[must_use]
    pub fn pkr_max(&mut self) -> PkrMaxW<PkrmaxSpec> {
        PkrMaxW::new(self, 0)
    }
}
#[doc = "Poker Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkrmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkrmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkrmaxSpec;
impl crate::RegisterSpec for PkrmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkrmax::R`](R) reader structure"]
impl crate::Readable for PkrmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`pkrmax::W`](W) writer structure"]
impl crate::Writable for PkrmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKRMAX to value 0x6920"]
impl crate::Resettable for PkrmaxSpec {
    const RESET_VALUE: u32 = 0x6920;
}
