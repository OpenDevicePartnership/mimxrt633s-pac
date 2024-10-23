#[doc = "Register `LIMIT` reader"]
pub type R = crate::R<LimitSpec>;
#[doc = "Register `LIMIT` writer"]
pub type W = crate::W<LimitSpec>;
#[doc = "Field `LIMMSK_L` reader - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type LimmskLR = crate::FieldReader<u16>;
#[doc = "Field `LIMMSK_L` writer - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type LimmskLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LIMMSK_H` reader - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type LimmskHR = crate::FieldReader<u16>;
#[doc = "Field `LIMMSK_H` writer - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type LimmskHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_l(&self) -> LimmskLR {
        LimmskLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_h(&self) -> LimmskHR {
        LimmskHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIMIT")
            .field("limmsk_l", &self.limmsk_l())
            .field("limmsk_h", &self.limmsk_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn limmsk_l(&mut self) -> LimmskLW<LimitSpec> {
        LimmskLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn limmsk_h(&mut self) -> LimmskHW<LimitSpec> {
        LimmskHW::new(self, 16)
    }
}
#[doc = "SCT limit event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitSpec;
impl crate::RegisterSpec for LimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit::R`](R) reader structure"]
impl crate::Readable for LimitSpec {}
#[doc = "`write(|w| ..)` method takes [`limit::W`](W) writer structure"]
impl crate::Writable for LimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMIT to value 0"]
impl crate::Resettable for LimitSpec {
    const RESET_VALUE: u32 = 0;
}
