#[doc = "Register `DS_ADDR` reader"]
pub type R = crate::R<DsAddrSpec>;
#[doc = "Register `DS_ADDR` writer"]
pub type W = crate::W<DsAddrSpec>;
#[doc = "Field `DS_ADDR` reader - DS_ADDR"]
pub type DsAddrR = crate::FieldReader<u32>;
#[doc = "Field `DS_ADDR` writer - DS_ADDR"]
pub type DsAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DS_ADDR"]
    #[inline(always)]
    pub fn ds_addr(&self) -> DsAddrR {
        DsAddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS_ADDR")
            .field("ds_addr", &self.ds_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DS_ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn ds_addr(&mut self) -> DsAddrW<DsAddrSpec> {
        DsAddrW::new(self, 0)
    }
}
#[doc = "DMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsAddrSpec;
impl crate::RegisterSpec for DsAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds_addr::R`](R) reader structure"]
impl crate::Readable for DsAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ds_addr::W`](W) writer structure"]
impl crate::Writable for DsAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DS_ADDR to value 0"]
impl crate::Resettable for DsAddrSpec {
    const RESET_VALUE: u32 = 0;
}
