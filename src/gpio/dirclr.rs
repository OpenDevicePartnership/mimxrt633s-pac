#[doc = "Register `DIRCLR[%s]` writer"]
pub type W = crate::W<DirclrSpec>;
#[doc = "Field `DIRCLRP` writer - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
pub type DirclrpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DirclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub fn dirclrp(&mut self) -> DirclrpW<DirclrSpec> {
        DirclrpW::new(self, 0)
    }
}
#[doc = "Clear pin direction bits for port\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirclrSpec;
impl crate::RegisterSpec for DirclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DirclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRCLR[%s]
to value 0"]
impl crate::Resettable for DirclrSpec {
    const RESET_VALUE: u32 = 0;
}
