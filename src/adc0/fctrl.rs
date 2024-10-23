#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FctrlSpec>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FctrlSpec>;
#[doc = "Field `FCOUNT` reader - Result FIFO counter"]
pub type FcountR = crate::FieldReader;
#[doc = "Field `FWMARK` reader - Watermark level selection"]
pub type FwmarkR = crate::FieldReader;
#[doc = "Field `FWMARK` writer - Watermark level selection"]
pub type FwmarkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Result FIFO counter"]
    #[inline(always)]
    pub fn fcount(&self) -> FcountR {
        FcountR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    pub fn fwmark(&self) -> FwmarkR {
        FwmarkR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCTRL")
            .field("fcount", &self.fcount())
            .field("fwmark", &self.fwmark())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    #[must_use]
    pub fn fwmark(&mut self) -> FwmarkW<FctrlSpec> {
        FwmarkW::new(self, 16)
    }
}
#[doc = "ADC FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlSpec;
impl crate::RegisterSpec for FctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTRL to value 0"]
impl crate::Resettable for FctrlSpec {
    const RESET_VALUE: u32 = 0;
}
