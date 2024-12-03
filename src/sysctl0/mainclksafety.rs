#[doc = "Register `MAINCLKSAFETY` reader"]
pub type R = crate::R<MainclksafetySpec>;
#[doc = "Register `MAINCLKSAFETY` writer"]
pub type W = crate::W<MainclksafetySpec>;
#[doc = "Field `DELAY` reader - Main Clock turn on delay for Deep Sleep wake up"]
pub type DelayR = crate::FieldReader<u16>;
#[doc = "Field `DELAY` writer - Main Clock turn on delay for Deep Sleep wake up"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Main Clock turn on delay for Deep Sleep wake up"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAINCLKSAFETY")
            .field("delay", &self.delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock turn on delay for Deep Sleep wake up"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<MainclksafetySpec> {
        DelayW::new(self, 0)
    }
}
#[doc = "Main Clock Safety\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksafety::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksafety::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclksafetySpec;
impl crate::RegisterSpec for MainclksafetySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclksafety::R`](R) reader structure"]
impl crate::Readable for MainclksafetySpec {}
#[doc = "`write(|w| ..)` method takes [`mainclksafety::W`](W) writer structure"]
impl crate::Writable for MainclksafetySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAINCLKSAFETY to value 0"]
impl crate::Resettable for MainclksafetySpec {
    const RESET_VALUE: u32 = 0;
}
