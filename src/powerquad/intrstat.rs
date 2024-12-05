#[doc = "Register `INTRSTAT` reader"]
pub type R = crate::R<IntrstatSpec>;
#[doc = "Register `INTRSTAT` writer"]
pub type W = crate::W<IntrstatSpec>;
#[doc = "Field `intr_stat` reader - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
pub type IntrStatR = crate::BitReader;
#[doc = "Field `intr_stat` writer - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
pub type IntrStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&self) -> IntrStatR {
        IntrStatR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTRSTAT")
            .field("intr_stat", &self.intr_stat())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&mut self) -> IntrStatW<IntrstatSpec> {
        IntrStatW::new(self, 0)
    }
}
#[doc = "INTERRUPT STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`intrstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrstatSpec;
impl crate::RegisterSpec for IntrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrstat::R`](R) reader structure"]
impl crate::Readable for IntrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intrstat::W`](W) writer structure"]
impl crate::Writable for IntrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRSTAT to value 0"]
impl crate::Resettable for IntrstatSpec {
    const RESET_VALUE: u32 = 0;
}
