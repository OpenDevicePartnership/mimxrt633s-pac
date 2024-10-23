#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `FRAMELEN` reader - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in."]
pub type FramelenR = crate::FieldReader<u16>;
#[doc = "Field `FRAMELEN` writer - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in."]
pub type FramelenW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `POSITION` reader - Data Position."]
pub type PositionR = crate::FieldReader<u16>;
#[doc = "Field `POSITION` writer - Data Position."]
pub type PositionW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in."]
    #[inline(always)]
    pub fn framelen(&self) -> FramelenR {
        FramelenR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Data Position."]
    #[inline(always)]
    pub fn position(&self) -> PositionR {
        PositionR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("framelen", &self.framelen())
            .field("position", &self.position())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in."]
    #[inline(always)]
    #[must_use]
    pub fn framelen(&mut self) -> FramelenW<Cfg2Spec> {
        FramelenW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Data Position."]
    #[inline(always)]
    #[must_use]
    pub fn position(&mut self) -> PositionW<Cfg2Spec> {
        PositionW::new(self, 16)
    }
}
#[doc = "Configuration register 2 for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
