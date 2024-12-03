#[doc = "Register `WARNINT` reader"]
pub type R = crate::R<WarnintSpec>;
#[doc = "Register `WARNINT` writer"]
pub type W = crate::W<WarnintSpec>;
#[doc = "Field `WARNINT` reader - Watchdog warning interrupt compare value."]
pub type WarnintR = crate::FieldReader<u16>;
#[doc = "Field `WARNINT` writer - Watchdog warning interrupt compare value."]
pub type WarnintW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&self) -> WarnintR {
        WarnintR::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WARNINT")
            .field("warnint", &self.warnint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&mut self) -> WarnintW<WarnintSpec> {
        WarnintW::new(self, 0)
    }
}
#[doc = "Watchdog Warning Interrupt compare value.\n\nYou can [`read`](crate::Reg::read) this register and get [`warnint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warnint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WarnintSpec;
impl crate::RegisterSpec for WarnintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`warnint::R`](R) reader structure"]
impl crate::Readable for WarnintSpec {}
#[doc = "`write(|w| ..)` method takes [`warnint::W`](W) writer structure"]
impl crate::Writable for WarnintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WARNINT to value 0"]
impl crate::Resettable for WarnintSpec {
    const RESET_VALUE: u32 = 0;
}
