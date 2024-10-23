#[doc = "Register `REMASK` reader"]
pub type R = crate::R<RemaskSpec>;
#[doc = "Register `REMASK` writer"]
pub type W = crate::W<RemaskSpec>;
#[doc = "Field `MASK` reader - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMASK")
            .field("mask", &self.mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<RemaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Optional re-mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`remask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemaskSpec;
impl crate::RegisterSpec for RemaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remask::R`](R) reader structure"]
impl crate::Readable for RemaskSpec {}
#[doc = "`write(|w| ..)` method takes [`remask::W`](W) writer structure"]
impl crate::Writable for RemaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMASK to value 0"]
impl crate::Resettable for RemaskSpec {
    const RESET_VALUE: u32 = 0;
}
