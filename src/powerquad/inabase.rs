#[doc = "Register `INABASE` reader"]
pub type R = crate::R<InabaseSpec>;
#[doc = "Register `INABASE` writer"]
pub type W = crate::W<InabaseSpec>;
#[doc = "Field `inabase` reader - Base address register for the input A region"]
pub type InabaseR = crate::FieldReader<u32>;
#[doc = "Field `inabase` writer - Base address register for the input A region"]
pub type InabaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    pub fn inabase(&self) -> InabaseR {
        InabaseR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INABASE")
            .field("inabase", &self.inabase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the input A region"]
    #[inline(always)]
    #[must_use]
    pub fn inabase(&mut self) -> InabaseW<InabaseSpec> {
        InabaseW::new(self, 0)
    }
}
#[doc = "Base address register for input A region\n\nYou can [`read`](crate::Reg::read) this register and get [`inabase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inabase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InabaseSpec;
impl crate::RegisterSpec for InabaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inabase::R`](R) reader structure"]
impl crate::Readable for InabaseSpec {}
#[doc = "`write(|w| ..)` method takes [`inabase::W`](W) writer structure"]
impl crate::Writable for InabaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INABASE to value 0"]
impl crate::Resettable for InabaseSpec {
    const RESET_VALUE: u32 = 0;
}
