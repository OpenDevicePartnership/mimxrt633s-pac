#[doc = "Register `DIRNOT[%s]` writer"]
pub type W = crate::W<DirnotSpec>;
#[doc = "Field `DIRNOTP` writer - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
pub type DirnotpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DirnotSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirnotp(&mut self) -> DirnotpW<DirnotSpec> {
        DirnotpW::new(self, 0)
    }
}
#[doc = "Toggle pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirnot::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirnotSpec;
impl crate::RegisterSpec for DirnotSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirnot::W`](W) writer structure"]
impl crate::Writable for DirnotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRNOT[%s]
to value 0"]
impl crate::Resettable for DirnotSpec {
    const RESET_VALUE: u32 = 0;
}
