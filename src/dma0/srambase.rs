#[doc = "Register `SRAMBASE` reader"]
pub type R = crate::R<SrambaseSpec>;
#[doc = "Register `SRAMBASE` writer"]
pub type W = crate::W<SrambaseSpec>;
#[doc = "Field `OFFSET` reader - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
pub type OffsetR = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMBASE")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<SrambaseSpec> {
        OffsetW::new(self, 9)
    }
}
#[doc = "SRAM address of the channel configuration table.\n\nYou can [`read`](crate::Reg::read) this register and get [`srambase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srambase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrambaseSpec;
impl crate::RegisterSpec for SrambaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srambase::R`](R) reader structure"]
impl crate::Readable for SrambaseSpec {}
#[doc = "`write(|w| ..)` method takes [`srambase::W`](W) writer structure"]
impl crate::Writable for SrambaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMBASE to value 0"]
impl crate::Resettable for SrambaseSpec {
    const RESET_VALUE: u32 = 0;
}
