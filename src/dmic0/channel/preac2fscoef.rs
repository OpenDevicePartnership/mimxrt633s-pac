#[doc = "Register `PREAC2FSCOEF` reader"]
pub type R = crate::R<Preac2fscoefSpec>;
#[doc = "Register `PREAC2FSCOEF` writer"]
pub type W = crate::W<Preac2fscoefSpec>;
#[doc = "Field `COMP` reader - Co-efficient choice for CIC droop compensation droop filter"]
pub type CompR = crate::FieldReader;
#[doc = "Field `COMP` writer - Co-efficient choice for CIC droop compensation droop filter"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PREAC2FSCOEF")
            .field("comp", &self.comp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<Preac2fscoefSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "Compensation filter for 2FS\n\nYou can [`read`](crate::Reg::read) this register and get [`preac2fscoef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preac2fscoef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preac2fscoefSpec;
impl crate::RegisterSpec for Preac2fscoefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preac2fscoef::R`](R) reader structure"]
impl crate::Readable for Preac2fscoefSpec {}
#[doc = "`write(|w| ..)` method takes [`preac2fscoef::W`](W) writer structure"]
impl crate::Writable for Preac2fscoefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREAC2FSCOEF to value 0"]
impl crate::Resettable for Preac2fscoefSpec {
    const RESET_VALUE: u32 = 0;
}
