#[doc = "Register `TMPBASE` reader"]
pub type R = crate::R<TmpbaseSpec>;
#[doc = "Register `TMPBASE` writer"]
pub type W = crate::W<TmpbaseSpec>;
#[doc = "Field `tmpbase` reader - Base address register for the temporary region"]
pub type TmpbaseR = crate::FieldReader<u32>;
#[doc = "Field `tmpbase` writer - Base address register for the temporary region"]
pub type TmpbaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&self) -> TmpbaseR {
        TmpbaseR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMPBASE")
            .field("tmpbase", &self.tmpbase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the temporary region"]
    #[inline(always)]
    pub fn tmpbase(&mut self) -> TmpbaseW<TmpbaseSpec> {
        TmpbaseW::new(self, 0)
    }
}
#[doc = "Base address register for temp region\n\nYou can [`read`](crate::Reg::read) this register and get [`tmpbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmpbaseSpec;
impl crate::RegisterSpec for TmpbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmpbase::R`](R) reader structure"]
impl crate::Readable for TmpbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`tmpbase::W`](W) writer structure"]
impl crate::Writable for TmpbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMPBASE to value 0"]
impl crate::Resettable for TmpbaseSpec {
    const RESET_VALUE: u32 = 0;
}
