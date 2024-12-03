#[doc = "Register `FLSHB2CR0` reader"]
pub type R = crate::R<Flshb2cr0Spec>;
#[doc = "Register `FLSHB2CR0` writer"]
pub type W = crate::W<Flshb2cr0Spec>;
#[doc = "Field `FLSHSZ` reader - Flash Size in KByte."]
pub type FlshszR = crate::FieldReader<u32>;
#[doc = "Field `FLSHSZ` writer - Flash Size in KByte."]
pub type FlshszW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&self) -> FlshszR {
        FlshszR::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLSHB2CR0")
            .field("flshsz", &self.flshsz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&mut self) -> FlshszW<Flshb2cr0Spec> {
        FlshszW::new(self, 0)
    }
}
#[doc = "Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flshb2cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshb2cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flshb2cr0Spec;
impl crate::RegisterSpec for Flshb2cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flshb2cr0::R`](R) reader structure"]
impl crate::Readable for Flshb2cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`flshb2cr0::W`](W) writer structure"]
impl crate::Writable for Flshb2cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLSHB2CR0 to value 0x0001_0000"]
impl crate::Resettable for Flshb2cr0Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
