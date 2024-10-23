#[doc = "Register `TIMER0` reader"]
pub type R = crate::R<Timer0Spec>;
#[doc = "Register `TIMER0` writer"]
pub type W = crate::W<Timer0Spec>;
#[doc = "Field `TUNITCON` reader - Unit Connection Timer Elapse (in ms)"]
pub type TunitconR = crate::FieldReader<u16>;
#[doc = "Field `TSEQ_INIT` reader - Sequence Initiation Time"]
pub type TseqInitR = crate::FieldReader<u16>;
#[doc = "Field `TSEQ_INIT` writer - Sequence Initiation Time"]
pub type TseqInitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub fn tunitcon(&self) -> TunitconR {
        TunitconR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    pub fn tseq_init(&self) -> TseqInitR {
        TseqInitR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0")
            .field("tunitcon", &self.tunitcon())
            .field("tseq_init", &self.tseq_init())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    #[must_use]
    pub fn tseq_init(&mut self) -> TseqInitW<Timer0Spec> {
        TseqInitW::new(self, 16)
    }
}
#[doc = "TIMER0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0Spec;
impl crate::RegisterSpec for Timer0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0::R`](R) reader structure"]
impl crate::Readable for Timer0Spec {}
#[doc = "`write(|w| ..)` method takes [`timer0::W`](W) writer structure"]
impl crate::Writable for Timer0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER0 to value 0x0010_0000"]
impl crate::Resettable for Timer0Spec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
