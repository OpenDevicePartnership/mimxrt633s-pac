#[doc = "Register `FLEXSPI_BOOTROM_SCRATCH0` reader"]
pub type R = crate::R<FlexspiBootromScratch0Spec>;
#[doc = "Register `FLEXSPI_BOOTROM_SCRATCH0` writer"]
pub type W = crate::W<FlexspiBootromScratch0Spec>;
#[doc = "Field `scratch` reader - no description available"]
pub type ScratchR = crate::FieldReader<u32>;
#[doc = "Field `scratch` writer - no description available"]
pub type ScratchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_BOOTROM_SCRATCH0")
            .field("scratch", &self.scratch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn scratch(&mut self) -> ScratchW<FlexspiBootromScratch0Spec> {
        ScratchW::new(self, 0)
    }
}
#[doc = "FLEXSPI NOR flash configure context register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi_bootrom_scratch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi_bootrom_scratch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexspiBootromScratch0Spec;
impl crate::RegisterSpec for FlexspiBootromScratch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexspi_bootrom_scratch0::R`](R) reader structure"]
impl crate::Readable for FlexspiBootromScratch0Spec {}
#[doc = "`write(|w| ..)` method takes [`flexspi_bootrom_scratch0::W`](W) writer structure"]
impl crate::Writable for FlexspiBootromScratch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLEXSPI_BOOTROM_SCRATCH0 to value 0"]
impl crate::Resettable for FlexspiBootromScratch0Spec {
    const RESET_VALUE: u32 = 0;
}
