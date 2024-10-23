#[doc = "Register `HASHHWKEYDISABLE` reader"]
pub type R = crate::R<HashhwkeydisableSpec>;
#[doc = "Register `HASHHWKEYDISABLE` writer"]
pub type W = crate::W<HashhwkeydisableSpec>;
#[doc = "Field `HASHHWKEYDISABLE` reader - This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
pub type HashhwkeydisableR = crate::FieldReader<u32>;
#[doc = "Field `HASHHWKEYDISABLE` writer - This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
pub type HashhwkeydisableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
    #[inline(always)]
    pub fn hashhwkeydisable(&self) -> HashhwkeydisableR {
        HashhwkeydisableR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASHHWKEYDISABLE")
            .field("hashhwkeydisable", &self.hashhwkeydisable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
    #[inline(always)]
    #[must_use]
    pub fn hashhwkeydisable(&mut self) -> HashhwkeydisableW<HashhwkeydisableSpec> {
        HashhwkeydisableW::new(self, 0)
    }
}
#[doc = "Hash hardware key disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hashhwkeydisable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashhwkeydisable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashhwkeydisableSpec;
impl crate::RegisterSpec for HashhwkeydisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashhwkeydisable::R`](R) reader structure"]
impl crate::Readable for HashhwkeydisableSpec {}
#[doc = "`write(|w| ..)` method takes [`hashhwkeydisable::W`](W) writer structure"]
impl crate::Writable for HashhwkeydisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHHWKEYDISABLE to value 0"]
impl crate::Resettable for HashhwkeydisableSpec {
    const RESET_VALUE: u32 = 0;
}
