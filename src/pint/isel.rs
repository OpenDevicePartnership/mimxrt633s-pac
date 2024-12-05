#[doc = "Register `ISEL` reader"]
pub type R = crate::R<IselSpec>;
#[doc = "Register `ISEL` writer"]
pub type W = crate::W<IselSpec>;
#[doc = "Field `PMODE` reader - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
pub type PmodeR = crate::FieldReader;
#[doc = "Field `PMODE` writer - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
pub type PmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISEL")
            .field("pmode", &self.pmode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PmodeW<IselSpec> {
        PmodeW::new(self, 0)
    }
}
#[doc = "Pin Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`isel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IselSpec;
impl crate::RegisterSpec for IselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isel::R`](R) reader structure"]
impl crate::Readable for IselSpec {}
#[doc = "`write(|w| ..)` method takes [`isel::W`](W) writer structure"]
impl crate::Writable for IselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISEL to value 0"]
impl crate::Resettable for IselSpec {
    const RESET_VALUE: u32 = 0;
}
