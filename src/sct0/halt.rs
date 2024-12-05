#[doc = "Register `HALT` reader"]
pub type R = crate::R<HaltSpec>;
#[doc = "Register `HALT` writer"]
pub type W = crate::W<HaltSpec>;
#[doc = "Field `HALTMSK_L` reader - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type HaltmskLR = crate::FieldReader<u16>;
#[doc = "Field `HALTMSK_L` writer - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type HaltmskLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HALTMSK_H` reader - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type HaltmskHR = crate::FieldReader<u16>;
#[doc = "Field `HALTMSK_H` writer - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type HaltmskHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&self) -> HaltmskLR {
        HaltmskLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&self) -> HaltmskHR {
        HaltmskHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HALT")
            .field("haltmsk_l", &self.haltmsk_l())
            .field("haltmsk_h", &self.haltmsk_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&mut self) -> HaltmskLW<HaltSpec> {
        HaltmskLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&mut self) -> HaltmskHW<HaltSpec> {
        HaltmskHW::new(self, 16)
    }
}
#[doc = "SCT halt event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaltSpec;
impl crate::RegisterSpec for HaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halt::R`](R) reader structure"]
impl crate::Readable for HaltSpec {}
#[doc = "`write(|w| ..)` method takes [`halt::W`](W) writer structure"]
impl crate::Writable for HaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALT to value 0"]
impl crate::Resettable for HaltSpec {
    const RESET_VALUE: u32 = 0;
}
