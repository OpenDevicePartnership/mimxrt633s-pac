#[doc = "Register `EV_STATE` reader"]
pub type R = crate::R<EvStateSpec>;
#[doc = "Register `EV_STATE` writer"]
pub type W = crate::W<EvStateSpec>;
#[doc = "Field `STATEMSKn` reader - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
pub type StatemsknR = crate::FieldReader<u16>;
#[doc = "Field `STATEMSKn` writer - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
pub type StatemsknW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&self) -> StatemsknR {
        StatemsknR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EV_STATE")
            .field("statemskn", &self.statemskn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn statemskn(&mut self) -> StatemsknW<EvStateSpec> {
        StatemsknW::new(self, 0)
    }
}
#[doc = "SCT event state register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStateSpec;
impl crate::RegisterSpec for EvStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_state::R`](R) reader structure"]
impl crate::Readable for EvStateSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_state::W`](W) writer structure"]
impl crate::Writable for EvStateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EV_STATE to value 0"]
impl crate::Resettable for EvStateSpec {
    const RESET_VALUE: u32 = 0;
}
