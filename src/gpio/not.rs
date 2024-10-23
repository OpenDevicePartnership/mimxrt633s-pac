#[doc = "Register `NOT[%s]` writer"]
pub type W = crate::W<NotSpec>;
#[doc = "Field `NOTP` writer - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
pub type NotpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<NotSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    #[must_use]
    pub fn notp(&mut self) -> NotpW<NotSpec> {
        NotpW::new(self, 0)
    }
}
#[doc = "Toggle port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`not::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NotSpec;
impl crate::RegisterSpec for NotSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`not::W`](W) writer structure"]
impl crate::Writable for NotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOT[%s]
to value 0"]
impl crate::Resettable for NotSpec {
    const RESET_VALUE: u32 = 0;
}
