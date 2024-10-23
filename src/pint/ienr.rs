#[doc = "Register `IENR` reader"]
pub type R = crate::R<IenrSpec>;
#[doc = "Register `IENR` writer"]
pub type W = crate::W<IenrSpec>;
#[doc = "Field `ENRL` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type EnrlR = crate::FieldReader;
#[doc = "Field `ENRL` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub type EnrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl(&self) -> EnrlR {
        EnrlR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IENR").field("enrl", &self.enrl()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enrl(&mut self) -> EnrlW<IenrSpec> {
        EnrlW::new(self, 0)
    }
}
#[doc = "Pin interrupt level or rising edge interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenrSpec;
impl crate::RegisterSpec for IenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ienr::R`](R) reader structure"]
impl crate::Readable for IenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ienr::W`](W) writer structure"]
impl crate::Writable for IenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IENR to value 0"]
impl crate::Resettable for IenrSpec {
    const RESET_VALUE: u32 = 0;
}
