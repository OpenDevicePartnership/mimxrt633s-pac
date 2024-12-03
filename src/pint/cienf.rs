#[doc = "Register `CIENF` writer"]
pub type W = crate::W<CienfSpec>;
#[doc = "Field `CENAF` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub type CenafW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CienfSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    pub fn cenaf(&mut self) -> CenafW<CienfSpec> {
        CenafW::new(self, 0)
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CienfSpec;
impl crate::RegisterSpec for CienfSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cienf::W`](W) writer structure"]
impl crate::Writable for CienfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIENF to value 0"]
impl crate::Resettable for CienfSpec {
    const RESET_VALUE: u32 = 0;
}
