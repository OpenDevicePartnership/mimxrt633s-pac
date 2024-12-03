#[doc = "Register `PCFG2` reader"]
pub type R = crate::R<Pcfg2Spec>;
#[doc = "Register `PCFG2` writer"]
pub type W = crate::W<Pcfg2Spec>;
#[doc = "Field `POSITION` reader - Data Position."]
pub type PositionR = crate::FieldReader<u16>;
#[doc = "Field `POSITION` writer - Data Position."]
pub type PositionW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&self) -> PositionR {
        PositionR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFG2")
            .field("position", &self.position())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&mut self) -> PositionW<Pcfg2Spec> {
        PositionW::new(self, 16)
    }
}
#[doc = "Configuration register 2 for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcfg2Spec;
impl crate::RegisterSpec for Pcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfg2::R`](R) reader structure"]
impl crate::Readable for Pcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcfg2::W`](W) writer structure"]
impl crate::Writable for Pcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCFG2 to value 0"]
impl crate::Resettable for Pcfg2Spec {
    const RESET_VALUE: u32 = 0;
}
