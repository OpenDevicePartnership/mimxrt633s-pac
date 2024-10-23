#[doc = "Register `CREG` reader"]
pub type R = crate::R<CregSpec>;
#[doc = "Register `CREG` writer"]
pub type W = crate::W<CregSpec>;
#[doc = "Field `REG_VALUE` reader - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueR = crate::FieldReader<u32>;
#[doc = "Field `REG_VALUE` writer - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> RegValueR {
        RegValueR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CREG")
            .field("reg_value", &self.reg_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    #[must_use]
    pub fn reg_value(&mut self) -> RegValueW<CregSpec> {
        RegValueW::new(self, 0)
    }
}
#[doc = "C register\n\nYou can [`read`](crate::Reg::read) this register and get [`creg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`creg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CregSpec;
impl crate::RegisterSpec for CregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`creg::R`](R) reader structure"]
impl crate::Readable for CregSpec {}
#[doc = "`write(|w| ..)` method takes [`creg::W`](W) writer structure"]
impl crate::Writable for CregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CREG to value 0"]
impl crate::Resettable for CregSpec {
    const RESET_VALUE: u32 = 0;
}
