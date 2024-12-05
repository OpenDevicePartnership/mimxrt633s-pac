#[doc = "Register `SIENF` writer"]
pub type W = crate::W<SienfSpec>;
#[doc = "Field `SETENAF` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub type SetenafW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SienfSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    pub fn setenaf(&mut self) -> SetenafW<SienfSpec> {
        SetenafW::new(self, 0)
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sienf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SienfSpec;
impl crate::RegisterSpec for SienfSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sienf::W`](W) writer structure"]
impl crate::Writable for SienfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIENF to value 0"]
impl crate::Resettable for SienfSpec {
    const RESET_VALUE: u32 = 0;
}
