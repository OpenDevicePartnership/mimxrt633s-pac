#[doc = "Register `FRQMAX` reader"]
pub type R = crate::R<FrqmaxSpec>;
#[doc = "Register `FRQMAX` writer"]
pub type W = crate::W<FrqmaxSpec>;
#[doc = "Field `FRQ_MAX` reader - Frequency Counter Maximum Limit"]
pub type FrqMaxR = crate::FieldReader<u32>;
#[doc = "Field `FRQ_MAX` writer - Frequency Counter Maximum Limit"]
pub type FrqMaxW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&self) -> FrqMaxR {
        FrqMaxR::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRQMAX")
            .field("frq_max", &self.frq_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn frq_max(&mut self) -> FrqMaxW<FrqmaxSpec> {
        FrqMaxW::new(self, 0)
    }
}
#[doc = "Frequency Count Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frqmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frqmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrqmaxSpec;
impl crate::RegisterSpec for FrqmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frqmax::R`](R) reader structure"]
impl crate::Readable for FrqmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`frqmax::W`](W) writer structure"]
impl crate::Writable for FrqmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRQMAX to value 0x6400"]
impl crate::Resettable for FrqmaxSpec {
    const RESET_VALUE: u32 = 0x6400;
}
