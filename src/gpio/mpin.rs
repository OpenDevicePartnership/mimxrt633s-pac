#[doc = "Register `MPIN[%s]` reader"]
pub type R = crate::R<MpinSpec>;
#[doc = "Register `MPIN[%s]` writer"]
pub type W = crate::W<MpinSpec>;
#[doc = "Field `MPORTP` reader - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MportpR = crate::FieldReader<u32>;
#[doc = "Field `MPORTP` writer - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
pub type MportpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp(&self) -> MportpR {
        MportpR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPIN")
            .field("mportp", &self.mportp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    #[must_use]
    pub fn mportp(&mut self) -> MportpW<MpinSpec> {
        MportpW::new(self, 0)
    }
}
#[doc = "Masked port register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpinSpec;
impl crate::RegisterSpec for MpinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpin::R`](R) reader structure"]
impl crate::Readable for MpinSpec {}
#[doc = "`write(|w| ..)` method takes [`mpin::W`](W) writer structure"]
impl crate::Writable for MpinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPIN[%s]
to value 0"]
impl crate::Resettable for MpinSpec {
    const RESET_VALUE: u32 = 0;
}
