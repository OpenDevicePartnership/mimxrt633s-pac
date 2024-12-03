#[doc = "Register `SIENR` writer"]
pub type W = crate::W<SienrSpec>;
#[doc = "Field `SETENRL` writer - Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
pub type SetenrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SienrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl(&mut self) -> SetenrlW<SienrSpec> {
        SetenrlW::new(self, 0)
    }
}
#[doc = "Pin interrupt level or rising edge interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sienr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SienrSpec;
impl crate::RegisterSpec for SienrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sienr::W`](W) writer structure"]
impl crate::Writable for SienrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIENR to value 0"]
impl crate::Resettable for SienrSpec {
    const RESET_VALUE: u32 = 0;
}
