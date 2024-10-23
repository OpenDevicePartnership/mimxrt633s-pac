#[doc = "Register `SMAXLIMITS` reader"]
pub type R = crate::R<SmaxlimitsSpec>;
#[doc = "Register `SMAXLIMITS` writer"]
pub type W = crate::W<SmaxlimitsSpec>;
#[doc = "Field `MAXRD` reader - Maximum read length"]
pub type MaxrdR = crate::FieldReader<u16>;
#[doc = "Field `MAXRD` writer - Maximum read length"]
pub type MaxrdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MAXWR` reader - Maximum write length"]
pub type MaxwrR = crate::FieldReader<u16>;
#[doc = "Field `MAXWR` writer - Maximum write length"]
pub type MaxwrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Maximum read length"]
    #[inline(always)]
    pub fn maxrd(&self) -> MaxrdR {
        MaxrdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Maximum write length"]
    #[inline(always)]
    pub fn maxwr(&self) -> MaxwrR {
        MaxwrR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMAXLIMITS")
            .field("maxrd", &self.maxrd())
            .field("maxwr", &self.maxwr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Maximum read length"]
    #[inline(always)]
    #[must_use]
    pub fn maxrd(&mut self) -> MaxrdW<SmaxlimitsSpec> {
        MaxrdW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Maximum write length"]
    #[inline(always)]
    #[must_use]
    pub fn maxwr(&mut self) -> MaxwrW<SmaxlimitsSpec> {
        MaxwrW::new(self, 16)
    }
}
#[doc = "Slave Maximum Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smaxlimits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smaxlimits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmaxlimitsSpec;
impl crate::RegisterSpec for SmaxlimitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smaxlimits::R`](R) reader structure"]
impl crate::Readable for SmaxlimitsSpec {}
#[doc = "`write(|w| ..)` method takes [`smaxlimits::W`](W) writer structure"]
impl crate::Writable for SmaxlimitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMAXLIMITS to value 0"]
impl crate::Resettable for SmaxlimitsSpec {
    const RESET_VALUE: u32 = 0;
}
