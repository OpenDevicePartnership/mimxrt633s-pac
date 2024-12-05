#[doc = "Register `BOOTSTATEHMAC%s` reader"]
pub type R = crate::R<BootstatehmacSpec>;
#[doc = "Register `BOOTSTATEHMAC%s` writer"]
pub type W = crate::W<BootstatehmacSpec>;
#[doc = "Field `BOOTSTATEHMAC` reader - HMAC of boot state used for attestation"]
pub type BootstatehmacR = crate::FieldReader<u32>;
#[doc = "Field `BOOTSTATEHMAC` writer - HMAC of boot state used for attestation"]
pub type BootstatehmacW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HMAC of boot state used for attestation"]
    #[inline(always)]
    pub fn bootstatehmac(&self) -> BootstatehmacR {
        BootstatehmacR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTSTATEHMAC")
            .field("bootstatehmac", &self.bootstatehmac())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - HMAC of boot state used for attestation"]
    #[inline(always)]
    pub fn bootstatehmac(&mut self) -> BootstatehmacW<BootstatehmacSpec> {
        BootstatehmacW::new(self, 0)
    }
}
#[doc = "boot state hmac register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootstatehmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootstatehmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootstatehmacSpec;
impl crate::RegisterSpec for BootstatehmacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootstatehmac::R`](R) reader structure"]
impl crate::Readable for BootstatehmacSpec {}
#[doc = "`write(|w| ..)` method takes [`bootstatehmac::W`](W) writer structure"]
impl crate::Writable for BootstatehmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTSTATEHMAC%s to value 0"]
impl crate::Resettable for BootstatehmacSpec {
    const RESET_VALUE: u32 = 0;
}
