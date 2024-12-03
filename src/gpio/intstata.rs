#[doc = "Register `INTSTATA[%s]` reader"]
pub type R = crate::R<IntstataSpec>;
#[doc = "Register `INTSTATA[%s]` writer"]
pub type W = crate::W<IntstataSpec>;
#[doc = "Field `STATUS` reader - interrupt status"]
pub type StatusR = crate::FieldReader<u32>;
#[doc = "Field `STATUS` writer - interrupt status"]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - interrupt status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATA")
            .field("status", &self.status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt status"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<IntstataSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "interrupt status for interrupt A\n\nYou can [`read`](crate::Reg::read) this register and get [`intstata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstataSpec;
impl crate::RegisterSpec for IntstataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstata::R`](R) reader structure"]
impl crate::Readable for IntstataSpec {}
#[doc = "`write(|w| ..)` method takes [`intstata::W`](W) writer structure"]
impl crate::Writable for IntstataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTATA[%s]
to value 0"]
impl crate::Resettable for IntstataSpec {
    const RESET_VALUE: u32 = 0;
}
