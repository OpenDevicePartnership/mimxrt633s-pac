#[doc = "Register `SYSPLL0DENOM` reader"]
pub type R = crate::R<Syspll0denomSpec>;
#[doc = "Register `SYSPLL0DENOM` writer"]
pub type W = crate::W<Syspll0denomSpec>;
#[doc = "Field `DENOM` reader - This field contains the denominator of the SYSPLL0 fractional loop divider."]
pub type DenomR = crate::FieldReader<u32>;
#[doc = "Field `DENOM` writer - This field contains the denominator of the SYSPLL0 fractional loop divider."]
pub type DenomW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn denom(&self) -> DenomR {
        DenomR::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSPLL0DENOM")
            .field("denom", &self.denom())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn denom(&mut self) -> DenomW<Syspll0denomSpec> {
        DenomW::new(self, 0)
    }
}
#[doc = "system pll0 denom\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0denom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0denom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspll0denomSpec;
impl crate::RegisterSpec for Syspll0denomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspll0denom::R`](R) reader structure"]
impl crate::Readable for Syspll0denomSpec {}
#[doc = "`write(|w| ..)` method takes [`syspll0denom::W`](W) writer structure"]
impl crate::Writable for Syspll0denomSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLL0DENOM to value 0x1fff_ffdb"]
impl crate::Resettable for Syspll0denomSpec {
    const RESET_VALUE: u32 = 0x1fff_ffdb;
}
