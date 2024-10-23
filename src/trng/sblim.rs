#[doc = "Register `SBLIM` reader"]
pub type R = crate::R<SblimSpec>;
#[doc = "Register `SBLIM` writer"]
pub type W = crate::W<SblimSpec>;
#[doc = "Field `SB_LIM` reader - Sparse Bit Limit"]
pub type SbLimR = crate::FieldReader<u16>;
#[doc = "Field `SB_LIM` writer - Sparse Bit Limit"]
pub type SbLimW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&self) -> SbLimR {
        SbLimR::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBLIM")
            .field("sb_lim", &self.sb_lim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    #[must_use]
    pub fn sb_lim(&mut self) -> SbLimW<SblimSpec> {
        SbLimW::new(self, 0)
    }
}
#[doc = "Sparse Bit Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sblim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sblim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SblimSpec;
impl crate::RegisterSpec for SblimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sblim::R`](R) reader structure"]
impl crate::Readable for SblimSpec {}
#[doc = "`write(|w| ..)` method takes [`sblim::W`](W) writer structure"]
impl crate::Writable for SblimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBLIM to value 0x3f"]
impl crate::Resettable for SblimSpec {
    const RESET_VALUE: u32 = 0x3f;
}
