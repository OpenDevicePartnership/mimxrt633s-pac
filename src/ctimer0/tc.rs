#[doc = "Register `TC` reader"]
pub type R = crate::R<TcSpec>;
#[doc = "Register `TC` writer"]
pub type W = crate::W<TcSpec>;
#[doc = "Field `TCVAL` reader - Timer counter value."]
pub type TcvalR = crate::FieldReader<u32>;
#[doc = "Field `TCVAL` writer - Timer counter value."]
pub type TcvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter value."]
    #[inline(always)]
    pub fn tcval(&self) -> TcvalR {
        TcvalR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC").field("tcval", &self.tcval()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter value."]
    #[inline(always)]
    pub fn tcval(&mut self) -> TcvalW<TcSpec> {
        TcvalW::new(self, 0)
    }
}
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcSpec;
impl crate::RegisterSpec for TcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TcSpec {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TC to value 0"]
impl crate::Resettable for TcSpec {
    const RESET_VALUE: u32 = 0;
}
