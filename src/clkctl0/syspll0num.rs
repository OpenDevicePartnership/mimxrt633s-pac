#[doc = "Register `SYSPLL0NUM` reader"]
pub type R = crate::R<Syspll0numSpec>;
#[doc = "Register `SYSPLL0NUM` writer"]
pub type W = crate::W<Syspll0numSpec>;
#[doc = "Field `NUM` reader - This field contains the numerator of the SYSPLL0 fractional loop divider."]
pub type NumR = crate::FieldReader<u32>;
#[doc = "Field `NUM` writer - This field contains the numerator of the SYSPLL0 fractional loop divider."]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLL0NUM")
            .field("num", &self.num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn num(&mut self) -> NumW<Syspll0numSpec> {
        NumW::new(self, 0)
    }
}
#[doc = "system pll0 number\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspll0numSpec;
impl crate::RegisterSpec for Syspll0numSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspll0num::R`](R) reader structure"]
impl crate::Readable for Syspll0numSpec {}
#[doc = "`write(|w| ..)` method takes [`syspll0num::W`](W) writer structure"]
impl crate::Writable for Syspll0numSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLL0NUM to value 0x04dd_2f15"]
impl crate::Resettable for Syspll0numSpec {
    const RESET_VALUE: u32 = 0x04dd_2f15;
}
