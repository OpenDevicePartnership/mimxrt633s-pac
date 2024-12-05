#[doc = "Register `CORDIC_X` reader"]
pub type R = crate::R<CordicXSpec>;
#[doc = "Register `CORDIC_X` writer"]
pub type W = crate::W<CordicXSpec>;
#[doc = "Field `cordic_x` reader - Cordic input x"]
pub type CordicXR = crate::FieldReader<u32>;
#[doc = "Field `cordic_x` writer - Cordic input x"]
pub type CordicXW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cordic input x"]
    #[inline(always)]
    pub fn cordic_x(&self) -> CordicXR {
        CordicXR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_X")
            .field("cordic_x", &self.cordic_x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input x"]
    #[inline(always)]
    pub fn cordic_x(&mut self) -> CordicXW<CordicXSpec> {
        CordicXW::new(self, 0)
    }
}
#[doc = "Cordic input X register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CordicXSpec;
impl crate::RegisterSpec for CordicXSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordic_x::R`](R) reader structure"]
impl crate::Readable for CordicXSpec {}
#[doc = "`write(|w| ..)` method takes [`cordic_x::W`](W) writer structure"]
impl crate::Writable for CordicXSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORDIC_X to value 0"]
impl crate::Resettable for CordicXSpec {
    const RESET_VALUE: u32 = 0;
}
