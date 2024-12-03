#[doc = "Register `ADMA_SYS_ADDR` reader"]
pub type R = crate::R<AdmaSysAddrSpec>;
#[doc = "Register `ADMA_SYS_ADDR` writer"]
pub type W = crate::W<AdmaSysAddrSpec>;
#[doc = "Field `ADS_ADDR` reader - ADMA System Address"]
pub type AdsAddrR = crate::FieldReader<u32>;
#[doc = "Field `ADS_ADDR` writer - ADMA System Address"]
pub type AdsAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn ads_addr(&self) -> AdsAddrR {
        AdsAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADMA_SYS_ADDR")
            .field("ads_addr", &self.ads_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn ads_addr(&mut self) -> AdsAddrW<AdmaSysAddrSpec> {
        AdsAddrW::new(self, 2)
    }
}
#[doc = "ADMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_sys_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_sys_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaSysAddrSpec;
impl crate::RegisterSpec for AdmaSysAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_sys_addr::R`](R) reader structure"]
impl crate::Readable for AdmaSysAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`adma_sys_addr::W`](W) writer structure"]
impl crate::Writable for AdmaSysAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADMA_SYS_ADDR to value 0"]
impl crate::Resettable for AdmaSysAddrSpec {
    const RESET_VALUE: u32 = 0;
}
