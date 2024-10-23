#[doc = "Register `TIMER1` reader"]
pub type R = crate::R<Timer1Spec>;
#[doc = "Register `TIMER1` writer"]
pub type W = crate::W<Timer1Spec>;
#[doc = "Field `TVDPSRC_ON` reader - Time Period Comparator Enabled"]
pub type TvdpsrcOnR = crate::FieldReader<u16>;
#[doc = "Field `TVDPSRC_ON` writer - Time Period Comparator Enabled"]
pub type TvdpsrcOnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TDCD_DBNC` reader - Time Period to Debounce D+ Signal"]
pub type TdcdDbncR = crate::FieldReader<u16>;
#[doc = "Field `TDCD_DBNC` writer - Time Period to Debounce D+ Signal"]
pub type TdcdDbncW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn tvdpsrc_on(&self) -> TvdpsrcOnR {
        TvdpsrcOnR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn tdcd_dbnc(&self) -> TdcdDbncR {
        TdcdDbncR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1")
            .field("tvdpsrc_on", &self.tvdpsrc_on())
            .field("tdcd_dbnc", &self.tdcd_dbnc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_on(&mut self) -> TvdpsrcOnW<Timer1Spec> {
        TvdpsrcOnW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    #[must_use]
    pub fn tdcd_dbnc(&mut self) -> TdcdDbncW<Timer1Spec> {
        TdcdDbncW::new(self, 16)
    }
}
#[doc = "TIMER1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1Spec;
impl crate::RegisterSpec for Timer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1::R`](R) reader structure"]
impl crate::Readable for Timer1Spec {}
#[doc = "`write(|w| ..)` method takes [`timer1::W`](W) writer structure"]
impl crate::Writable for Timer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0x000a_0028"]
impl crate::Resettable for Timer1Spec {
    const RESET_VALUE: u32 = 0x000a_0028;
}
