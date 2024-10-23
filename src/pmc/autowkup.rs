#[doc = "Register `AUTOWKUP` reader"]
pub type R = crate::R<AutowkupSpec>;
#[doc = "Register `AUTOWKUP` writer"]
pub type W = crate::W<AutowkupSpec>;
#[doc = "Field `AUTOWKTIME` reader - Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
pub type AutowktimeR = crate::FieldReader<u16>;
#[doc = "Field `AUTOWKTIME` writer - Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
pub type AutowktimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
    #[inline(always)]
    pub fn autowktime(&self) -> AutowktimeR {
        AutowktimeR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOWKUP")
            .field("autowktime", &self.autowktime())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
    #[inline(always)]
    #[must_use]
    pub fn autowktime(&mut self) -> AutowktimeW<AutowkupSpec> {
        AutowktimeW::new(self, 0)
    }
}
#[doc = "Automatic wakeup from deepsleep / deep powerdown modes\n\nYou can [`read`](crate::Reg::read) this register and get [`autowkup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autowkup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutowkupSpec;
impl crate::RegisterSpec for AutowkupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autowkup::R`](R) reader structure"]
impl crate::Readable for AutowkupSpec {}
#[doc = "`write(|w| ..)` method takes [`autowkup::W`](W) writer structure"]
impl crate::Writable for AutowkupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOWKUP to value 0"]
impl crate::Resettable for AutowkupSpec {
    const RESET_VALUE: u32 = 0;
}
