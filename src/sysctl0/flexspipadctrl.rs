#[doc = "Register `FLEXSPIPADCTRL` reader"]
pub type R = crate::R<FlexspipadctrlSpec>;
#[doc = "Register `FLEXSPIPADCTRL` writer"]
pub type W = crate::W<FlexspipadctrlSpec>;
#[doc = "Field `RASRCN` reader - Drive FLEXSPI pad compensation circuit"]
pub type RasrcnR = crate::FieldReader;
#[doc = "Field `RASRCN` writer - Drive FLEXSPI pad compensation circuit"]
pub type RasrcnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RASRCP` reader - Drive FLEXSPI pad compensation circuit"]
pub type RasrcpR = crate::FieldReader;
#[doc = "Field `RASRCP` writer - Drive FLEXSPI pad compensation circuit"]
pub type RasrcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FASTFRZ` reader - Drive FLEXSPI pad compensation circuit"]
pub type FastfrzR = crate::BitReader;
#[doc = "Field `FASTFRZ` writer - Drive FLEXSPI pad compensation circuit"]
pub type FastfrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - Drive FLEXSPI pad compensation circuit"]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - Drive FLEXSPI pad compensation circuit"]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPTQ` reader - Drive FLEXSPI pad compensation circuit"]
pub type ComptqR = crate::BitReader;
#[doc = "Field `COMPTQ` writer - Drive FLEXSPI pad compensation circuit"]
pub type ComptqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - Drive FLEXSPI pad compensation circuit"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - Drive FLEXSPI pad compensation circuit"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NASRCN` reader - FLEXSPI pad compensation circuit status"]
pub type NasrcnR = crate::FieldReader;
#[doc = "Field `NASRCP` reader - FLEXSPI pad compensation circuit status"]
pub type NasrcpR = crate::FieldReader;
#[doc = "Field `COMPOK` reader - FLEXSPI pad compensation circuit status"]
pub type CompokR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn rasrcn(&self) -> RasrcnR {
        RasrcnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn rasrcp(&self) -> RasrcpR {
        RasrcpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn fastfrz(&self) -> FastfrzR {
        FastfrzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn comptq(&self) -> ComptqR {
        ComptqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub fn nasrcn(&self) -> NasrcnR {
        NasrcnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub fn nasrcp(&self) -> NasrcpR {
        NasrcpR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub fn compok(&self) -> CompokR {
        CompokR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPIPADCTRL")
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
    #[doc = "Bits 0:3 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn rasrcn(&mut self) -> RasrcnW<FlexspipadctrlSpec> {
        RasrcnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn rasrcp(&mut self) -> RasrcpW<FlexspipadctrlSpec> {
        RasrcpW::new(self, 4)
    }
    #[doc = "Bit 8 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn fastfrz(&mut self) -> FastfrzW<FlexspipadctrlSpec> {
        FastfrzW::new(self, 8)
    }
    #[doc = "Bit 9 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<FlexspipadctrlSpec> {
        FreezeW::new(self, 9)
    }
    #[doc = "Bit 10 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn comptq(&mut self) -> ComptqW<FlexspipadctrlSpec> {
        ComptqW::new(self, 10)
    }
    #[doc = "Bit 11 - Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> CompenW<FlexspipadctrlSpec> {
        CompenW::new(self, 11)
    }
}
#[doc = "FLEXSPI IO pads ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspipadctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspipadctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexspipadctrlSpec;
impl crate::RegisterSpec for FlexspipadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexspipadctrl::R`](R) reader structure"]
impl crate::Readable for FlexspipadctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flexspipadctrl::W`](W) writer structure"]
impl crate::Writable for FlexspipadctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLEXSPIPADCTRL to value 0x01a5_0000"]
impl crate::Resettable for FlexspipadctrlSpec {
    const RESET_VALUE: u32 = 0x01a5_0000;
}
