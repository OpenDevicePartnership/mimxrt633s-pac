#[doc = "Register `FRQMIN` reader"]
pub type R = crate::R<FrqminSpec>;
#[doc = "Register `FRQMIN` writer"]
pub type W = crate::W<FrqminSpec>;
#[doc = "Field `FRQ_MIN` reader - Frequency Count Minimum Limit"]
pub type FrqMinR = crate::FieldReader<u32>;
#[doc = "Field `FRQ_MIN` writer - Frequency Count Minimum Limit"]
pub type FrqMinW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&self) -> FrqMinR {
        FrqMinR::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRQMIN")
            .field("frq_min", &self.frq_min())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&mut self) -> FrqMinW<FrqminSpec> {
        FrqMinW::new(self, 0)
    }
}
#[doc = "Frequency Count Minimum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frqmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frqmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrqminSpec;
impl crate::RegisterSpec for FrqminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frqmin::R`](R) reader structure"]
impl crate::Readable for FrqminSpec {}
#[doc = "`write(|w| ..)` method takes [`frqmin::W`](W) writer structure"]
impl crate::Writable for FrqminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRQMIN to value 0x0640"]
impl crate::Resettable for FrqminSpec {
    const RESET_VALUE: u32 = 0x0640;
}
