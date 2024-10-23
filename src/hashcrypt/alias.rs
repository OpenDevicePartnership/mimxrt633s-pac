#[doc = "Register `ALIAS[%s]` writer"]
pub type W = crate::W<AliasSpec>;
#[doc = "Field `DATA` writer - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AliasSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write next word in little-endian form. The hash requires big endian word data, but this block swaps the bytes automatically. That is, SHA assumes the data coming in is treated as bytes (e.g. \"abcd\") and since the ARM core will treat \"abcd\" as a word as 0x64636261, the block will swap the word to restore into big endian."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<AliasSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alias::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AliasSpec;
impl crate::RegisterSpec for AliasSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alias::W`](W) writer structure"]
impl crate::Writable for AliasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALIAS[%s]
to value 0"]
impl crate::Resettable for AliasSpec {
    const RESET_VALUE: u32 = 0;
}
