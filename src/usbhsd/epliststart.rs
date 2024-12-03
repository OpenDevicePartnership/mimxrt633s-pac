#[doc = "Register `EPLISTSTART` reader"]
pub type R = crate::R<EpliststartSpec>;
#[doc = "Register `EPLISTSTART` writer"]
pub type W = crate::W<EpliststartSpec>;
#[doc = "Field `EP_LIST_PRG` reader - Programmable portion of the USB EP Command/Status List address."]
pub type EpListPrgR = crate::FieldReader<u16>;
#[doc = "Field `EP_LIST_PRG` writer - Programmable portion of the USB EP Command/Status List address."]
pub type EpListPrgW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EP_LIST_FIXED` reader - Fixed portion of USB EP Command/Status List address."]
pub type EpListFixedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&self) -> EpListPrgR {
        EpListPrgR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_fixed(&self) -> EpListFixedR {
        EpListFixedR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPLISTSTART")
            .field("ep_list_prg", &self.ep_list_prg())
            .field("ep_list_fixed", &self.ep_list_fixed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&mut self) -> EpListPrgW<EpliststartSpec> {
        EpListPrgW::new(self, 8)
    }
}
#[doc = "USB EP Command/Status List start address\n\nYou can [`read`](crate::Reg::read) this register and get [`epliststart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epliststart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpliststartSpec;
impl crate::RegisterSpec for EpliststartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epliststart::R`](R) reader structure"]
impl crate::Readable for EpliststartSpec {}
#[doc = "`write(|w| ..)` method takes [`epliststart::W`](W) writer structure"]
impl crate::Writable for EpliststartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPLISTSTART to value 0"]
impl crate::Resettable for EpliststartSpec {
    const RESET_VALUE: u32 = 0;
}
