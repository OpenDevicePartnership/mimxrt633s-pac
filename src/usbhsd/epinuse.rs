#[doc = "Register `EPINUSE` reader"]
pub type R = crate::R<EpinuseSpec>;
#[doc = "Register `EPINUSE` writer"]
pub type W = crate::W<EpinuseSpec>;
#[doc = "Field `BUF` reader - Buffer in use: This register has one bit per physical endpoint."]
pub type BufR = crate::FieldReader<u16>;
#[doc = "Field `BUF` writer - Buffer in use: This register has one bit per physical endpoint."]
pub type BufW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf(&self) -> BufR {
        BufR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPINUSE").field("buf", &self.buf()).finish()
    }
}
impl W {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn buf(&mut self) -> BufW<EpinuseSpec> {
        BufW::new(self, 2)
    }
}
#[doc = "USB Endpoint Buffer in use\n\nYou can [`read`](crate::Reg::read) this register and get [`epinuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpinuseSpec;
impl crate::RegisterSpec for EpinuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epinuse::R`](R) reader structure"]
impl crate::Readable for EpinuseSpec {}
#[doc = "`write(|w| ..)` method takes [`epinuse::W`](W) writer structure"]
impl crate::Writable for EpinuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPINUSE to value 0"]
impl crate::Resettable for EpinuseSpec {
    const RESET_VALUE: u32 = 0;
}
