#[doc = "Register `SEED` reader"]
pub type R = crate::R<SeedSpec>;
#[doc = "Register `SEED` writer"]
pub type W = crate::W<SeedSpec>;
#[doc = "Field `CRC_SEED` reader - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub type CrcSeedR = crate::FieldReader<u32>;
#[doc = "Field `CRC_SEED` writer - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub type CrcSeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub fn crc_seed(&self) -> CrcSeedR {
        CrcSeedR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEED")
            .field("crc_seed", &self.crc_seed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    #[must_use]
    pub fn crc_seed(&mut self) -> CrcSeedW<SeedSpec> {
        CrcSeedW::new(self, 0)
    }
}
#[doc = "CRC seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`seed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeedSpec;
impl crate::RegisterSpec for SeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seed::R`](R) reader structure"]
impl crate::Readable for SeedSpec {}
#[doc = "`write(|w| ..)` method takes [`seed::W`](W) writer structure"]
impl crate::Writable for SeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEED to value 0xffff"]
impl crate::Resettable for SeedSpec {
    const RESET_VALUE: u32 = 0xffff;
}
