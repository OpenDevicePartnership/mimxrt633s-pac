#[doc = "Register `SDIOPADCTL` reader"]
pub type R = crate::R<SdiopadctlSpec>;
#[doc = "Register `SDIOPADCTL` writer"]
pub type W = crate::W<SdiopadctlSpec>;
#[doc = "Field `RASRCN` reader - Drives SDIO Pad Compensation Circuit"]
pub type RasrcnR = crate::FieldReader;
#[doc = "Field `RASRCN` writer - Drives SDIO Pad Compensation Circuit"]
pub type RasrcnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RASRCP` reader - Drives SDIO Pad Compensation Circuit"]
pub type RasrcpR = crate::FieldReader;
#[doc = "Field `RASRCP` writer - Drives SDIO Pad Compensation Circuit"]
pub type RasrcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FASTFRZ` reader - Drives SDIO Pad Compensation Circuit"]
pub type FastfrzR = crate::BitReader;
#[doc = "Field `FASTFRZ` writer - Drives SDIO Pad Compensation Circuit"]
pub type FastfrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - Drives SDIO Pad Compensation Circuit"]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - Drives SDIO Pad Compensation Circuit"]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPTQ` reader - Drives SDIO Pad Compensation Circuit"]
pub type ComptqR = crate::BitReader;
#[doc = "Field `COMPTQ` writer - Drives SDIO Pad Compensation Circuit"]
pub type ComptqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - Drives SDIO Pad Compensation Circuit"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - Drives SDIO Pad Compensation Circuit"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NASRCN` reader - SDIO Pad Compensation Circuit Status"]
pub type NasrcnR = crate::FieldReader;
#[doc = "Field `NASRCP` reader - SDIO Pad Compensation Circuit Status"]
pub type NasrcpR = crate::FieldReader;
#[doc = "Field `COMPOK` reader - SDIO Pad Compensation Circuit Status"]
pub type CompokR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcn(&self) -> RasrcnR {
        RasrcnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcp(&self) -> RasrcpR {
        RasrcpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn fastfrz(&self) -> FastfrzR {
        FastfrzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn comptq(&self) -> ComptqR {
        ComptqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn nasrcn(&self) -> NasrcnR {
        NasrcnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn nasrcp(&self) -> NasrcpR {
        NasrcpR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn compok(&self) -> CompokR {
        CompokR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIOPADCTL")
            .field("rasrcn", &self.rasrcn())
            .field("rasrcp", &self.rasrcp())
            .field("fastfrz", &self.fastfrz())
            .field("freeze", &self.freeze())
            .field("comptq", &self.comptq())
            .field("compen", &self.compen())
            .field("nasrcn", &self.nasrcn())
            .field("nasrcp", &self.nasrcp())
            .field("compok", &self.compok())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcn(&mut self) -> RasrcnW<SdiopadctlSpec> {
        RasrcnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcp(&mut self) -> RasrcpW<SdiopadctlSpec> {
        RasrcpW::new(self, 4)
    }
    #[doc = "Bit 8 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn fastfrz(&mut self) -> FastfrzW<SdiopadctlSpec> {
        FastfrzW::new(self, 8)
    }
    #[doc = "Bit 9 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn freeze(&mut self) -> FreezeW<SdiopadctlSpec> {
        FreezeW::new(self, 9)
    }
    #[doc = "Bit 10 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn comptq(&mut self) -> ComptqW<SdiopadctlSpec> {
        ComptqW::new(self, 10)
    }
    #[doc = "Bit 11 - Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<SdiopadctlSpec> {
        CompenW::new(self, 11)
    }
}
#[doc = "sdio pad ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`sdiopadctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdiopadctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdiopadctlSpec;
impl crate::RegisterSpec for SdiopadctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdiopadctl::R`](R) reader structure"]
impl crate::Readable for SdiopadctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdiopadctl::W`](W) writer structure"]
impl crate::Writable for SdiopadctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIOPADCTL to value 0x01a5_0000"]
impl crate::Resettable for SdiopadctlSpec {
    const RESET_VALUE: u32 = 0x01a5_0000;
}
