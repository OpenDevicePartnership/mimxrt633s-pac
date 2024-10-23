#[doc = "Register `INTENA[%s]` reader"]
pub type R = crate::R<IntenaSpec>;
#[doc = "Register `INTENA[%s]` writer"]
pub type W = crate::W<IntenaSpec>;
#[doc = "Field `INT_EN` reader - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
pub type IntEnR = crate::FieldReader<u32>;
#[doc = "Field `INT_EN` writer - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
pub type IntEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENA")
            .field("int_en", &self.int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<IntenaSpec> {
        IntEnW::new(self, 0)
    }
}
#[doc = "interrupt A enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenaSpec;
impl crate::RegisterSpec for IntenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intena::R`](R) reader structure"]
impl crate::Readable for IntenaSpec {}
#[doc = "`write(|w| ..)` method takes [`intena::W`](W) writer structure"]
impl crate::Writable for IntenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENA[%s]
to value 0"]
impl crate::Resettable for IntenaSpec {
    const RESET_VALUE: u32 = 0;
}
