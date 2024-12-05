#[doc = "Register `BRG` reader"]
pub type R = crate::R<BrgSpec>;
#[doc = "Register `BRG` writer"]
pub type W = crate::W<BrgSpec>;
#[doc = "Field `BRGVAL` reader - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
pub type BrgvalR = crate::FieldReader<u16>;
#[doc = "Field `BRGVAL` writer - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
pub type BrgvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub fn brgval(&self) -> BrgvalR {
        BrgvalR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRG")
            .field("brgval", &self.brgval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub fn brgval(&mut self) -> BrgvalW<BrgSpec> {
        BrgvalW::new(self, 0)
    }
}
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value.\n\nYou can [`read`](crate::Reg::read) this register and get [`brg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgSpec;
impl crate::RegisterSpec for BrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brg::R`](R) reader structure"]
impl crate::Readable for BrgSpec {}
#[doc = "`write(|w| ..)` method takes [`brg::W`](W) writer structure"]
impl crate::Writable for BrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BrgSpec {
    const RESET_VALUE: u32 = 0;
}
