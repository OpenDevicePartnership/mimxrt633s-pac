#[doc = "Register `KEYINDEX` reader"]
pub type R = crate::R<KeyindexSpec>;
#[doc = "Register `KEYINDEX` writer"]
pub type W = crate::W<KeyindexSpec>;
#[doc = "Field `KEYIDX` reader - Key index for Set Key operations"]
pub type KeyidxR = crate::FieldReader;
#[doc = "Field `KEYIDX` writer - Key index for Set Key operations"]
pub type KeyidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    pub fn keyidx(&self) -> KeyidxR {
        KeyidxR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYINDEX")
            .field("keyidx", &self.keyidx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    #[must_use]
    pub fn keyidx(&mut self) -> KeyidxW<KeyindexSpec> {
        KeyidxW::new(self, 0)
    }
}
#[doc = "PUF Key Index\n\nYou can [`read`](crate::Reg::read) this register and get [`keyindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyindexSpec;
impl crate::RegisterSpec for KeyindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyindex::R`](R) reader structure"]
impl crate::Readable for KeyindexSpec {}
#[doc = "`write(|w| ..)` method takes [`keyindex::W`](W) writer structure"]
impl crate::Writable for KeyindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYINDEX to value 0"]
impl crate::Resettable for KeyindexSpec {
    const RESET_VALUE: u32 = 0;
}
