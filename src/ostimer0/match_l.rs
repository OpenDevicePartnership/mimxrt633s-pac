#[doc = "Register `MATCH_L` reader"]
pub type R = crate::R<MatchLSpec>;
#[doc = "Register `MATCH_L` writer"]
pub type W = crate::W<MatchLSpec>;
#[doc = "Field `MATCH_VALUE` reader - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
pub type MatchValueR = crate::FieldReader<u32>;
#[doc = "Field `MATCH_VALUE` writer - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
pub type MatchValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
    #[inline(always)]
    pub fn match_value(&self) -> MatchValueR {
        MatchValueR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH_L")
            .field("match_value", &self.match_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
    #[inline(always)]
    pub fn match_value(&mut self) -> MatchValueW<MatchLSpec> {
        MatchValueW::new(self, 0)
    }
}
#[doc = "Local Match Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`match_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchLSpec;
impl crate::RegisterSpec for MatchLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_l::R`](R) reader structure"]
impl crate::Readable for MatchLSpec {}
#[doc = "`write(|w| ..)` method takes [`match_l::W`](W) writer structure"]
impl crate::Writable for MatchLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCH_L to value 0"]
impl crate::Resettable for MatchLSpec {
    const RESET_VALUE: u32 = 0;
}
