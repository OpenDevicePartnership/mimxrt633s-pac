#[doc = "Register `WAKE` reader"]
pub type R = crate::R<WakeSpec>;
#[doc = "Register `WAKE` writer"]
pub type W = crate::W<WakeSpec>;
#[doc = "Field `VAL` reader - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKE").field("val", &self.val()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<WakeSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "High-resolution/wake-up timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeSpec;
impl crate::RegisterSpec for WakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wake::R`](R) reader structure"]
impl crate::Readable for WakeSpec {}
#[doc = "`write(|w| ..)` method takes [`wake::W`](W) writer structure"]
impl crate::Writable for WakeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKE to value 0"]
impl crate::Resettable for WakeSpec {
    const RESET_VALUE: u32 = 0;
}
