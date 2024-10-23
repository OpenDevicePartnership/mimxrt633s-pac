#[doc = "Register `SVENDORID` reader"]
pub type R = crate::R<SvendoridSpec>;
#[doc = "Register `SVENDORID` writer"]
pub type W = crate::W<SvendoridSpec>;
#[doc = "Field `VID` reader - Vendor ID"]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `VID` writer - Vendor ID"]
pub type VidW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Vendor ID"]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVENDORID")
            .field("vid", &self.vid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Vendor ID"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VidW<SvendoridSpec> {
        VidW::new(self, 0)
    }
}
#[doc = "Slave Vendor ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`svendorid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svendorid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SvendoridSpec;
impl crate::RegisterSpec for SvendoridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`svendorid::R`](R) reader structure"]
impl crate::Readable for SvendoridSpec {}
#[doc = "`write(|w| ..)` method takes [`svendorid::W`](W) writer structure"]
impl crate::Writable for SvendoridSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SVENDORID to value 0x011b"]
impl crate::Resettable for SvendoridSpec {
    const RESET_VALUE: u32 = 0x011b;
}
