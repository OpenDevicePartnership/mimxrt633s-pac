#[doc = "Register `AESKEY_SRCSEL` reader"]
pub type R = crate::R<AeskeySrcselSpec>;
#[doc = "Register `AESKEY_SRCSEL` writer"]
pub type W = crate::W<AeskeySrcselSpec>;
#[doc = "Field `AESKEY_SRCSEL` reader - AES Key Source Select:"]
pub type AeskeySrcselR = crate::FieldReader;
#[doc = "Field `AESKEY_SRCSEL` writer - AES Key Source Select:"]
pub type AeskeySrcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - AES Key Source Select:"]
    #[inline(always)]
    pub fn aeskey_srcsel(&self) -> AeskeySrcselR {
        AeskeySrcselR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESKEY_SRCSEL")
            .field("aeskey_srcsel", &self.aeskey_srcsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - AES Key Source Select:"]
    #[inline(always)]
    #[must_use]
    pub fn aeskey_srcsel(&mut self) -> AeskeySrcselW<AeskeySrcselSpec> {
        AeskeySrcselW::new(self, 0)
    }
}
#[doc = "AES key source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey_srcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey_srcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeskeySrcselSpec;
impl crate::RegisterSpec for AeskeySrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey_srcsel::R`](R) reader structure"]
impl crate::Readable for AeskeySrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`aeskey_srcsel::W`](W) writer structure"]
impl crate::Writable for AeskeySrcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY_SRCSEL to value 0"]
impl crate::Resettable for AeskeySrcselSpec {
    const RESET_VALUE: u32 = 0;
}
