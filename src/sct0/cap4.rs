#[doc = "Register `CAP4` reader"]
pub type R = crate::R<Cap4Spec>;
#[doc = "Register `CAP4` writer"]
pub type W = crate::W<Cap4Spec>;
#[doc = "Field `CAPn_L` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub type CapnLR = crate::FieldReader<u16>;
#[doc = "Field `CAPn_L` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub type CapnLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CAPn_H` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub type CapnHR = crate::FieldReader<u16>;
#[doc = "Field `CAPn_H` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub type CapnHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&self) -> CapnLR {
        CapnLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&self) -> CapnHR {
        CapnHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP4")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&mut self) -> CapnLW<Cap4Spec> {
        CapnLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&mut self) -> CapnHW<Cap4Spec> {
        CapnHW::new(self, 16)
    }
}
#[doc = "SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap4Spec;
impl crate::RegisterSpec for Cap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap4::R`](R) reader structure"]
impl crate::Readable for Cap4Spec {}
#[doc = "`write(|w| ..)` method takes [`cap4::W`](W) writer structure"]
impl crate::Writable for Cap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP4 to value 0"]
impl crate::Resettable for Cap4Spec {
    const RESET_VALUE: u32 = 0;
}
