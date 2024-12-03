#[doc = "Register `MATCH_H` reader"]
pub type R = crate::R<MatchHSpec>;
#[doc = "Register `MATCH_H` writer"]
pub type W = crate::W<MatchHSpec>;
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
        f.debug_struct("MATCH_H")
            .field("match_value", &self.match_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
    #[inline(always)]
    pub fn match_value(&mut self) -> MatchValueW<MatchHSpec> {
        MatchValueW::new(self, 0)
    }
}
#[doc = "Match High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`match_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchHSpec;
impl crate::RegisterSpec for MatchHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_h::R`](R) reader structure"]
impl crate::Readable for MatchHSpec {}
#[doc = "`write(|w| ..)` method takes [`match_h::W`](W) writer structure"]
impl crate::Writable for MatchHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCH_H to value 0"]
impl crate::Resettable for MatchHSpec {
    const RESET_VALUE: u32 = 0;
}
