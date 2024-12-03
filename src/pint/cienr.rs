#[doc = "Register `CIENR` writer"]
pub type W = crate::W<CienrSpec>;
#[doc = "Field `CENRL` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub type CenrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CienrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    pub fn cenrl(&mut self) -> CenrlW<CienrSpec> {
        CenrlW::new(self, 0)
    }
}
#[doc = "Pin interrupt level (rising edge interrupt) clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CienrSpec;
impl crate::RegisterSpec for CienrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cienr::W`](W) writer structure"]
impl crate::Writable for CienrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIENR to value 0"]
impl crate::Resettable for CienrSpec {
    const RESET_VALUE: u32 = 0;
}
