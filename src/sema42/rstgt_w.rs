#[doc = "Register `RSTGT_W` writer"]
pub type W = crate::W<RstgtWSpec>;
#[doc = "Field `RSTGTN` writer - RSTGTN"]
pub type RstgtnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSTGDP` writer - RSTGDP"]
pub type RstgdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RstgtWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - RSTGTN"]
    #[inline(always)]
    pub fn rstgtn(&mut self) -> RstgtnW<RstgtWSpec> {
        RstgtnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RSTGDP"]
    #[inline(always)]
    pub fn rstgdp(&mut self) -> RstgdpW<RstgtWSpec> {
        RstgdpW::new(self, 8)
    }
}
#[doc = "Reset Gate Write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstgt_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstgtWSpec;
impl crate::RegisterSpec for RstgtWSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`rstgt_w::W`](W) writer structure"]
impl crate::Writable for RstgtWSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RSTGT_W to value 0"]
impl crate::Resettable for RstgtWSpec {
    const RESET_VALUE: u16 = 0;
}
