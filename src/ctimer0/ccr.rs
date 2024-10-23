#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `CAP0RE` reader - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap0reR = crate::BitReader;
#[doc = "Field `CAP0RE` writer - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap0reW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0FE` reader - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap0feR = crate::BitReader;
#[doc = "Field `CAP0FE` writer - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0I` reader - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub type Cap0iR = crate::BitReader;
#[doc = "Field `CAP0I` writer - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub type Cap0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1RE` reader - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap1reR = crate::BitReader;
#[doc = "Field `CAP1RE` writer - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap1reW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1FE` reader - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap1feR = crate::BitReader;
#[doc = "Field `CAP1FE` writer - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1I` reader - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub type Cap1iR = crate::BitReader;
#[doc = "Field `CAP1I` writer - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub type Cap1iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2RE` reader - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap2reR = crate::BitReader;
#[doc = "Field `CAP2RE` writer - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap2reW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2FE` reader - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap2feR = crate::BitReader;
#[doc = "Field `CAP2FE` writer - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2I` reader - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub type Cap2iR = crate::BitReader;
#[doc = "Field `CAP2I` writer - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub type Cap2iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3RE` reader - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap3reR = crate::BitReader;
#[doc = "Field `CAP3RE` writer - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap3reW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3FE` reader - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap3feR = crate::BitReader;
#[doc = "Field `CAP3FE` writer - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type Cap3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3I` reader - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub type Cap3iR = crate::BitReader;
#[doc = "Field `CAP3I` writer - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub type Cap3iW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0re(&self) -> Cap0reR {
        Cap0reR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0fe(&self) -> Cap0feR {
        Cap0feR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> Cap0iR {
        Cap0iR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1re(&self) -> Cap1reR {
        Cap1reR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1fe(&self) -> Cap1feR {
        Cap1feR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&self) -> Cap1iR {
        Cap1iR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2re(&self) -> Cap2reR {
        Cap2reR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2fe(&self) -> Cap2feR {
        Cap2feR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&self) -> Cap2iR {
        Cap2iR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3re(&self) -> Cap3reR {
        Cap3reR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3fe(&self) -> Cap3feR {
        Cap3feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub fn cap3i(&self) -> Cap3iR {
        Cap3iR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("cap0re", &self.cap0re())
            .field("cap0fe", &self.cap0fe())
            .field("cap0i", &self.cap0i())
            .field("cap1re", &self.cap1re())
            .field("cap1fe", &self.cap1fe())
            .field("cap1i", &self.cap1i())
            .field("cap2re", &self.cap2re())
            .field("cap2fe", &self.cap2fe())
            .field("cap2i", &self.cap2i())
            .field("cap3re", &self.cap3re())
            .field("cap3fe", &self.cap3fe())
            .field("cap3i", &self.cap3i())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap0re(&mut self) -> Cap0reW<CcrSpec> {
        Cap0reW::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap0fe(&mut self) -> Cap0feW<CcrSpec> {
        Cap0feW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap0i(&mut self) -> Cap0iW<CcrSpec> {
        Cap0iW::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap1re(&mut self) -> Cap1reW<CcrSpec> {
        Cap1reW::new(self, 3)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap1fe(&mut self) -> Cap1feW<CcrSpec> {
        Cap1feW::new(self, 4)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap1i(&mut self) -> Cap1iW<CcrSpec> {
        Cap1iW::new(self, 5)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap2re(&mut self) -> Cap2reW<CcrSpec> {
        Cap2reW::new(self, 6)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap2fe(&mut self) -> Cap2feW<CcrSpec> {
        Cap2feW::new(self, 7)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap2i(&mut self) -> Cap2iW<CcrSpec> {
        Cap2iW::new(self, 8)
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap3re(&mut self) -> Cap3reW<CcrSpec> {
        Cap3reW::new(self, 9)
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap3fe(&mut self) -> Cap3feW<CcrSpec> {
        Cap3feW::new(self, 10)
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap3i(&mut self) -> Cap3iW<CcrSpec> {
        Cap3iW::new(self, 11)
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
