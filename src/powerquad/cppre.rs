#[doc = "Register `CPPRE` reader"]
pub type R = crate::R<CppreSpec>;
#[doc = "Register `CPPRE` writer"]
pub type W = crate::W<CppreSpec>;
#[doc = "Field `cppre_in` reader - co-processor scaling of input"]
pub type CppreInR = crate::FieldReader;
#[doc = "Field `cppre_in` writer - co-processor scaling of input"]
pub type CppreInW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cppre_out` reader - co-processor fixed point output"]
pub type CppreOutR = crate::FieldReader;
#[doc = "Field `cppre_out` writer - co-processor fixed point output"]
pub type CppreOutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cppre_sat` reader - 1 : forces sub-32 bit saturation"]
pub type CppreSatR = crate::BitReader;
#[doc = "Field `cppre_sat` writer - 1 : forces sub-32 bit saturation"]
pub type CppreSatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cppre_sat8` reader - 0 = 8bits, 1 = 16bits"]
pub type CppreSat8R = crate::BitReader;
#[doc = "Field `cppre_sat8` writer - 0 = 8bits, 1 = 16bits"]
pub type CppreSat8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&self) -> CppreInR {
        CppreInR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&self) -> CppreOutR {
        CppreOutR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&self) -> CppreSatR {
        CppreSatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&self) -> CppreSat8R {
        CppreSat8R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPPRE")
            .field("cppre_in", &self.cppre_in())
            .field("cppre_out", &self.cppre_out())
            .field("cppre_sat", &self.cppre_sat())
            .field("cppre_sat8", &self.cppre_sat8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&mut self) -> CppreInW<CppreSpec> {
        CppreInW::new(self, 0)
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&mut self) -> CppreOutW<CppreSpec> {
        CppreOutW::new(self, 8)
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&mut self) -> CppreSatW<CppreSpec> {
        CppreSatW::new(self, 16)
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&mut self) -> CppreSat8W<CppreSpec> {
        CppreSat8W::new(self, 17)
    }
}
#[doc = "Pre-scale register\n\nYou can [`read`](crate::Reg::read) this register and get [`cppre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cppre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CppreSpec;
impl crate::RegisterSpec for CppreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cppre::R`](R) reader structure"]
impl crate::Readable for CppreSpec {}
#[doc = "`write(|w| ..)` method takes [`cppre::W`](W) writer structure"]
impl crate::Writable for CppreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPPRE to value 0"]
impl crate::Resettable for CppreSpec {
    const RESET_VALUE: u32 = 0;
}
