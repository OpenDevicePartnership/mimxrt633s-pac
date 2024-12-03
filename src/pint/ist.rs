#[doc = "Register `IST` reader"]
pub type R = crate::R<IstSpec>;
#[doc = "Register `IST` writer"]
pub type W = crate::W<IstSpec>;
#[doc = "Field `PSTAT` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
pub type PstatR = crate::FieldReader;
#[doc = "Field `PSTAT` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
pub type PstatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat(&self) -> PstatR {
        PstatR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IST").field("pstat", &self.pstat()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat(&mut self) -> PstatW<IstSpec> {
        PstatW::new(self, 0)
    }
}
#[doc = "Pin interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ist::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ist::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstSpec;
impl crate::RegisterSpec for IstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ist::R`](R) reader structure"]
impl crate::Readable for IstSpec {}
#[doc = "`write(|w| ..)` method takes [`ist::W`](W) writer structure"]
impl crate::Writable for IstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IST to value 0"]
impl crate::Resettable for IstSpec {
    const RESET_VALUE: u32 = 0;
}
