#[doc = "Register `B_[%s]` reader"]
pub type R = crate::R<B_Spec>;
#[doc = "Register `B_[%s]` writer"]
pub type W = crate::W<B_Spec>;
#[doc = "Field `PBYTE` reader - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PbyteR = crate::BitReader;
#[doc = "Field `PBYTE` writer - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PbyteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&self) -> PbyteR {
        PbyteR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_").field("pbyte", &self.pbyte()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&mut self) -> PbyteW<B_Spec> {
        PbyteW::new(self, 0)
    }
}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_Spec;
impl crate::RegisterSpec for B_Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`b_::R`](R) reader structure"]
impl crate::Readable for B_Spec {}
#[doc = "`write(|w| ..)` method takes [`b_::W`](W) writer structure"]
impl crate::Writable for B_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets B_[%s]
to value 0"]
impl crate::Resettable for B_Spec {
    const RESET_VALUE: u8 = 0;
}
