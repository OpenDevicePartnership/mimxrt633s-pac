#[doc = "Register `EVENTEN` reader"]
pub type R = crate::R<EventenSpec>;
#[doc = "Register `EVENTEN` writer"]
pub type W = crate::W<EventenSpec>;
#[doc = "Field `event_oflow` reader - 1 : Enable event trigger on Floating point overflow"]
pub type EventOflowR = crate::BitReader;
#[doc = "Field `event_oflow` writer - 1 : Enable event trigger on Floating point overflow"]
pub type EventOflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_nan` reader - 1 : Enable event trigger on Floating point NaN"]
pub type EventNanR = crate::BitReader;
#[doc = "Field `event_nan` writer - 1 : Enable event trigger on Floating point NaN"]
pub type EventNanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_fixed` reader - 1: Enable event trigger on Fixed point Overflow"]
pub type EventFixedR = crate::BitReader;
#[doc = "Field `event_fixed` writer - 1: Enable event trigger on Fixed point Overflow"]
pub type EventFixedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_uflow` reader - 1 : Enable event trigger on Subnormal truncation"]
pub type EventUflowR = crate::BitReader;
#[doc = "Field `event_uflow` writer - 1 : Enable event trigger on Subnormal truncation"]
pub type EventUflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_berr` reader - 1: Enable event trigger on AHBM Buss Error"]
pub type EventBerrR = crate::BitReader;
#[doc = "Field `event_berr` writer - 1: Enable event trigger on AHBM Buss Error"]
pub type EventBerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_comp` reader - 1: Enable event trigger on instruction completion"]
pub type EventCompR = crate::BitReader;
#[doc = "Field `event_comp` writer - 1: Enable event trigger on instruction completion"]
pub type EventCompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&self) -> EventOflowR {
        EventOflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&self) -> EventNanR {
        EventNanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn event_fixed(&self) -> EventFixedR {
        EventFixedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&self) -> EventUflowR {
        EventUflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn event_berr(&self) -> EventBerrR {
        EventBerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&self) -> EventCompR {
        EventCompR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENTEN")
            .field("event_oflow", &self.event_oflow())
            .field("event_nan", &self.event_nan())
            .field("event_fixed", &self.event_fixed())
            .field("event_uflow", &self.event_uflow())
            .field("event_berr", &self.event_berr())
            .field("event_comp", &self.event_comp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn event_oflow(&mut self) -> EventOflowW<EventenSpec> {
        EventOflowW::new(self, 0)
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    #[must_use]
    pub fn event_nan(&mut self) -> EventNanW<EventenSpec> {
        EventNanW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn event_fixed(&mut self) -> EventFixedW<EventenSpec> {
        EventFixedW::new(self, 2)
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    #[must_use]
    pub fn event_uflow(&mut self) -> EventUflowW<EventenSpec> {
        EventUflowW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    #[must_use]
    pub fn event_berr(&mut self) -> EventBerrW<EventenSpec> {
        EventBerrW::new(self, 4)
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn event_comp(&mut self) -> EventCompW<EventenSpec> {
        EventCompW::new(self, 7)
    }
}
#[doc = "Event Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventenSpec;
impl crate::RegisterSpec for EventenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventen::R`](R) reader structure"]
impl crate::Readable for EventenSpec {}
#[doc = "`write(|w| ..)` method takes [`eventen::W`](W) writer structure"]
impl crate::Writable for EventenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EventenSpec {
    const RESET_VALUE: u32 = 0;
}
