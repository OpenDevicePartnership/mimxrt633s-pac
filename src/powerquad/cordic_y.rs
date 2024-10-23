#[doc = "Register `CORDIC_Y` reader"]
pub type R = crate::R<CordicYSpec>;
#[doc = "Register `CORDIC_Y` writer"]
pub type W = crate::W<CordicYSpec>;
#[doc = "Field `cordic_y` reader - Cordic input y"]
pub type CordicYR = crate::FieldReader<u32>;
#[doc = "Field `cordic_y` writer - Cordic input y"]
pub type CordicYW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    pub fn cordic_y(&self) -> CordicYR {
        CordicYR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_Y")
            .field("cordic_y", &self.cordic_y())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    #[must_use]
    pub fn cordic_y(&mut self) -> CordicYW<CordicYSpec> {
        CordicYW::new(self, 0)
    }
}
#[doc = "Cordic input Y register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CordicYSpec;
impl crate::RegisterSpec for CordicYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordic_y::R`](R) reader structure"]
impl crate::Readable for CordicYSpec {}
#[doc = "`write(|w| ..)` method takes [`cordic_y::W`](W) writer structure"]
impl crate::Writable for CordicYSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORDIC_Y to value 0"]
impl crate::Resettable for CordicYSpec {
    const RESET_VALUE: u32 = 0;
}
