#[doc = "Register `INBBASE` reader"]
pub type R = crate::R<InbbaseSpec>;
#[doc = "Register `INBBASE` writer"]
pub type W = crate::W<InbbaseSpec>;
#[doc = "Field `inbbase` reader - Base address register for the input B region"]
pub type InbbaseR = crate::FieldReader<u32>;
#[doc = "Field `inbbase` writer - Base address register for the input B region"]
pub type InbbaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address register for the input B region"]
    #[inline(always)]
    pub fn inbbase(&self) -> InbbaseR {
        InbbaseR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INBBASE")
            .field("inbbase", &self.inbbase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the input B region"]
    #[inline(always)]
    pub fn inbbase(&mut self) -> InbbaseW<InbbaseSpec> {
        InbbaseW::new(self, 0)
    }
}
#[doc = "Base address register for input B region\n\nYou can [`read`](crate::Reg::read) this register and get [`inbbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inbbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InbbaseSpec;
impl crate::RegisterSpec for InbbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inbbase::R`](R) reader structure"]
impl crate::Readable for InbbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`inbbase::W`](W) writer structure"]
impl crate::Writable for InbbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INBBASE to value 0"]
impl crate::Resettable for InbbaseSpec {
    const RESET_VALUE: u32 = 0;
}
