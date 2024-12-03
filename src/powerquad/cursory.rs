#[doc = "Register `CURSORY` reader"]
pub type R = crate::R<CursorySpec>;
#[doc = "Register `CURSORY` writer"]
pub type W = crate::W<CursorySpec>;
#[doc = "Field `cursory` reader - 1 : Enable cursory mode"]
pub type CursoryR = crate::BitReader;
#[doc = "Field `cursory` writer - 1 : Enable cursory mode"]
pub type CursoryW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&self) -> CursoryR {
        CursoryR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CURSORY")
            .field("cursory", &self.cursory())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&mut self) -> CursoryW<CursorySpec> {
        CursoryW::new(self, 0)
    }
}
#[doc = "Cursory register\n\nYou can [`read`](crate::Reg::read) this register and get [`cursory::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursory::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CursorySpec;
impl crate::RegisterSpec for CursorySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cursory::R`](R) reader structure"]
impl crate::Readable for CursorySpec {}
#[doc = "`write(|w| ..)` method takes [`cursory::W`](W) writer structure"]
impl crate::Writable for CursorySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURSORY to value 0"]
impl crate::Resettable for CursorySpec {
    const RESET_VALUE: u32 = 0;
}
