#[doc = "Register `INTPTD` reader"]
pub type R = crate::R<IntptdSpec>;
#[doc = "Register `INTPTD` writer"]
pub type W = crate::W<IntptdSpec>;
#[doc = "Field `INT_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub type IntFirstR = crate::FieldReader;
#[doc = "Field `INT_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub type IntFirstW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT_BASE` reader - Base address to be used by the hardware to find the start of the INT list."]
pub type IntBaseR = crate::FieldReader<u32>;
#[doc = "Field `INT_BASE` writer - Base address to be used by the hardware to find the start of the INT list."]
pub type IntBaseW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub fn int_first(&self) -> IntFirstR {
        IntFirstR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub fn int_base(&self) -> IntBaseR {
        IntBaseR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPTD")
            .field("int_first", &self.int_first())
            .field("int_base", &self.int_base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    #[must_use]
    pub fn int_first(&mut self) -> IntFirstW<IntptdSpec> {
        IntFirstW::new(self, 5)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    #[must_use]
    pub fn int_base(&mut self) -> IntBaseW<IntptdSpec> {
        IntBaseW::new(self, 10)
    }
}
#[doc = "Memory base address where INT PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`intptd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intptd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntptdSpec;
impl crate::RegisterSpec for IntptdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intptd::R`](R) reader structure"]
impl crate::Readable for IntptdSpec {}
#[doc = "`write(|w| ..)` method takes [`intptd::W`](W) writer structure"]
impl crate::Writable for IntptdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPTD to value 0"]
impl crate::Resettable for IntptdSpec {
    const RESET_VALUE: u32 = 0;
}
