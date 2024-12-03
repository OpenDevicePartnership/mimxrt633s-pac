#[doc = "Register `FLADJ_FRINDEX` reader"]
pub type R = crate::R<FladjFrindexSpec>;
#[doc = "Register `FLADJ_FRINDEX` writer"]
pub type W = crate::W<FladjFrindexSpec>;
#[doc = "Field `FLADJ` reader - Frame Length Timing Value."]
pub type FladjR = crate::FieldReader;
#[doc = "Field `FLADJ` writer - Frame Length Timing Value."]
pub type FladjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRINDEX` reader - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub type FrindexR = crate::FieldReader<u16>;
#[doc = "Field `FRINDEX` writer - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub type FrindexW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&self) -> FladjR {
        FladjR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&self) -> FrindexR {
        FrindexR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLADJ_FRINDEX")
            .field("fladj", &self.fladj())
            .field("frindex", &self.frindex())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&mut self) -> FladjW<FladjFrindexSpec> {
        FladjW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&mut self) -> FrindexW<FladjFrindexSpec> {
        FrindexW::new(self, 16)
    }
}
#[doc = "Frame Length Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`fladj_frindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fladj_frindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FladjFrindexSpec;
impl crate::RegisterSpec for FladjFrindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fladj_frindex::R`](R) reader structure"]
impl crate::Readable for FladjFrindexSpec {}
#[doc = "`write(|w| ..)` method takes [`fladj_frindex::W`](W) writer structure"]
impl crate::Writable for FladjFrindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLADJ_FRINDEX to value 0x20"]
impl crate::Resettable for FladjFrindexSpec {
    const RESET_VALUE: u32 = 0x20;
}
