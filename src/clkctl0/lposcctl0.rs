#[doc = "Register `LPOSCCTL0` reader"]
pub type R = crate::R<Lposcctl0Spec>;
#[doc = "Register `LPOSCCTL0` writer"]
pub type W = crate::W<Lposcctl0Spec>;
#[doc = "Field `CLKRDY` reader - Clock ready flag status. LPOSC clock ready takes 64uS."]
pub type ClkrdyR = crate::BitReader;
#[doc = "Field `CLKRDY` writer - Clock ready flag status. LPOSC clock ready takes 64uS."]
pub type ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPOSCCTL0")
            .field("clkrdy", &self.clkrdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[inline(always)]
    pub fn clkrdy(&mut self) -> ClkrdyW<Lposcctl0Spec> {
        ClkrdyW::new(self, 31)
    }
}
#[doc = "low power oscillator control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lposcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lposcctl0Spec;
impl crate::RegisterSpec for Lposcctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposcctl0::R`](R) reader structure"]
impl crate::Readable for Lposcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lposcctl0::W`](W) writer structure"]
impl crate::Writable for Lposcctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOSCCTL0 to value 0x807b_c4d4"]
impl crate::Resettable for Lposcctl0Spec {
    const RESET_VALUE: u32 = 0x807b_c4d4;
}
