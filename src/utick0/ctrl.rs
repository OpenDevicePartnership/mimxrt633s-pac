#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DELAYVAL` reader - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
pub type DelayvalR = crate::FieldReader<u32>;
#[doc = "Field `DELAYVAL` writer - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
pub type DelayvalW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `REPEAT` reader - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
pub type RepeatR = crate::BitReader;
#[doc = "Field `REPEAT` writer - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
pub type RepeatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub fn delayval(&self) -> DelayvalR {
        DelayvalR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub fn repeat(&self) -> RepeatR {
        RepeatR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("delayval", &self.delayval())
            .field("repeat", &self.repeat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    #[must_use]
    pub fn delayval(&mut self) -> DelayvalW<CtrlSpec> {
        DelayvalW::new(self, 0)
    }
    #[doc = "Bit 31 - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> RepeatW<CtrlSpec> {
        RepeatW::new(self, 31)
    }
}
#[doc = "Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
