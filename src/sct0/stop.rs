#[doc = "Register `STOP` reader"]
pub type R = crate::R<StopSpec>;
#[doc = "Register `STOP` writer"]
pub type W = crate::W<StopSpec>;
#[doc = "Field `STOPMSK_L` reader - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type StopmskLR = crate::FieldReader<u16>;
#[doc = "Field `STOPMSK_L` writer - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type StopmskLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STOPMSK_H` reader - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type StopmskHR = crate::FieldReader<u16>;
#[doc = "Field `STOPMSK_H` writer - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type StopmskHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&self) -> StopmskLR {
        StopmskLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&self) -> StopmskHR {
        StopmskHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOP")
            .field("stopmsk_l", &self.stopmsk_l())
            .field("stopmsk_h", &self.stopmsk_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&mut self) -> StopmskLW<StopSpec> {
        StopmskLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&mut self) -> StopmskHW<StopSpec> {
        StopmskHW::new(self, 16)
    }
}
#[doc = "SCT stop event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopSpec;
impl crate::RegisterSpec for StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop::R`](R) reader structure"]
impl crate::Readable for StopSpec {}
#[doc = "`write(|w| ..)` method takes [`stop::W`](W) writer structure"]
impl crate::Writable for StopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for StopSpec {
    const RESET_VALUE: u32 = 0;
}
