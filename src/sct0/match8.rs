#[doc = "Register `MATCH8` reader"]
pub type R = crate::R<Match8Spec>;
#[doc = "Register `MATCH8` writer"]
pub type W = crate::W<Match8Spec>;
#[doc = "Field `MATCHn_L` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MatchnLR = crate::FieldReader<u16>;
#[doc = "Field `MATCHn_L` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MatchnLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MATCHn_H` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MatchnHR = crate::FieldReader<u16>;
#[doc = "Field `MATCHn_H` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub type MatchnHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&self) -> MatchnLR {
        MatchnLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&self) -> MatchnHR {
        MatchnHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH8")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&mut self) -> MatchnLW<Match8Spec> {
        MatchnLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&mut self) -> MatchnHW<Match8Spec> {
        MatchnHW::new(self, 16)
    }
}
#[doc = "SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`match8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Match8Spec;
impl crate::RegisterSpec for Match8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match8::R`](R) reader structure"]
impl crate::Readable for Match8Spec {}
#[doc = "`write(|w| ..)` method takes [`match8::W`](W) writer structure"]
impl crate::Writable for Match8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCH8 to value 0"]
impl crate::Resettable for Match8Spec {
    const RESET_VALUE: u32 = 0;
}
