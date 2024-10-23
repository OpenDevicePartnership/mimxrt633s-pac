#[doc = "Register `DICEHWREG[%s]` reader"]
pub type R = crate::R<DicehwregSpec>;
#[doc = "Register `DICEHWREG[%s]` writer"]
pub type W = crate::W<DicehwregSpec>;
#[doc = "Field `DICEHWREG` reader - DICE General Purpose 32-Bit Data Register"]
pub type DicehwregR = crate::FieldReader<u32>;
#[doc = "Field `DICEHWREG` writer - DICE General Purpose 32-Bit Data Register"]
pub type DicehwregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub fn dicehwreg(&self) -> DicehwregR {
        DicehwregR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DICEHWREG")
            .field("dicehwreg", &self.dicehwreg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn dicehwreg(&mut self) -> DicehwregW<DicehwregSpec> {
        DicehwregW::new(self, 0)
    }
}
#[doc = "DICE General Purpose 32-Bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dicehwreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dicehwreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DicehwregSpec;
impl crate::RegisterSpec for DicehwregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dicehwreg::R`](R) reader structure"]
impl crate::Readable for DicehwregSpec {}
#[doc = "`write(|w| ..)` method takes [`dicehwreg::W`](W) writer structure"]
impl crate::Writable for DicehwregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DICEHWREG[%s]
to value 0"]
impl crate::Resettable for DicehwregSpec {
    const RESET_VALUE: u32 = 0;
}
