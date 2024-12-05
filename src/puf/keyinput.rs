#[doc = "Register `KEYINPUT` writer"]
pub type W = crate::W<KeyinputSpec>;
#[doc = "Field `KEYIN` writer - Key Input Data"]
pub type KeyinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<KeyinputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Input Data"]
    #[inline(always)]
    pub fn keyin(&mut self) -> KeyinW<KeyinputSpec> {
        KeyinW::new(self, 0)
    }
}
#[doc = "PUF Key Input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyinput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyinputSpec;
impl crate::RegisterSpec for KeyinputSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyinput::W`](W) writer structure"]
impl crate::Writable for KeyinputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYINPUT to value 0"]
impl crate::Resettable for KeyinputSpec {
    const RESET_VALUE: u32 = 0;
}
