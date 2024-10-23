#[doc = "Register `DIRSET[%s]` writer"]
pub type W = crate::W<DirsetSpec>;
#[doc = "Field `DIRSETP` writer - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
pub type DirsetpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DirsetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp(&mut self) -> DirsetpW<DirsetSpec> {
        DirsetpW::new(self, 0)
    }
}
#[doc = "Set pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirsetSpec;
impl crate::RegisterSpec for DirsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirset::W`](W) writer structure"]
impl crate::Writable for DirsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRSET[%s]
to value 0"]
impl crate::Resettable for DirsetSpec {
    const RESET_VALUE: u32 = 0;
}
