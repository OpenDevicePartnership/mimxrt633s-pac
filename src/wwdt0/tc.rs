#[doc = "Register `TC` reader"]
pub type R = crate::R<TcSpec>;
#[doc = "Register `TC` writer"]
pub type W = crate::W<TcSpec>;
#[doc = "Field `COUNT` reader - Watchdog time-out value."]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Watchdog time-out value."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Watchdog time-out value."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC").field("count", &self.count()).finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog time-out value."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<TcSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value.\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcSpec;
impl crate::RegisterSpec for TcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TcSpec {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TC to value 0xff"]
impl crate::Resettable for TcSpec {
    const RESET_VALUE: u32 = 0xff;
}
