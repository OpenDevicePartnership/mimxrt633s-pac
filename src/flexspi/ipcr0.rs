#[doc = "Register `IPCR0` reader"]
pub type R = crate::R<Ipcr0Spec>;
#[doc = "Register `IPCR0` writer"]
pub type W = crate::W<Ipcr0Spec>;
#[doc = "Field `SFAR` reader - Serial Flash Address for IP command."]
pub type SfarR = crate::FieldReader<u32>;
#[doc = "Field `SFAR` writer - Serial Flash Address for IP command."]
pub type SfarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    pub fn sfar(&self) -> SfarR {
        SfarR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCR0").field("sfar", &self.sfar()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn sfar(&mut self) -> SfarW<Ipcr0Spec> {
        SfarW::new(self, 0)
    }
}
#[doc = "IP Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipcr0Spec;
impl crate::RegisterSpec for Ipcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcr0::R`](R) reader structure"]
impl crate::Readable for Ipcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ipcr0::W`](W) writer structure"]
impl crate::Writable for Ipcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCR0 to value 0"]
impl crate::Resettable for Ipcr0Spec {
    const RESET_VALUE: u32 = 0;
}
