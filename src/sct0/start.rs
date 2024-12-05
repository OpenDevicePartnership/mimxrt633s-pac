#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `STARTMSK_L` reader - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type StartmskLR = crate::FieldReader<u16>;
#[doc = "Field `STARTMSK_L` writer - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type StartmskLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STARTMSK_H` reader - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type StartmskHR = crate::FieldReader<u16>;
#[doc = "Field `STARTMSK_H` writer - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type StartmskHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_l(&self) -> StartmskLR {
        StartmskLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_h(&self) -> StartmskHR {
        StartmskHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START")
            .field("startmsk_l", &self.startmsk_l())
            .field("startmsk_h", &self.startmsk_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_l(&mut self) -> StartmskLW<StartSpec> {
        StartmskLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_h(&mut self) -> StartmskHW<StartSpec> {
        StartmskHW::new(self, 16)
    }
}
#[doc = "SCT start event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {
    const RESET_VALUE: u32 = 0;
}
