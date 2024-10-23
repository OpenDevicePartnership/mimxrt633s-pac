#[doc = "Register `USBPHYPLL0LOCKTIMEDIV2` reader"]
pub type R = crate::R<Usbphypll0locktimediv2Spec>;
#[doc = "Register `USBPHYPLL0LOCKTIMEDIV2` writer"]
pub type W = crate::W<Usbphypll0locktimediv2Spec>;
#[doc = "Field `LOCKTIMEDIV2` reader - USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
pub type Locktimediv2R = crate::FieldReader<u16>;
#[doc = "Field `LOCKTIMEDIV2` writer - USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
pub type Locktimediv2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
    #[inline(always)]
    pub fn locktimediv2(&self) -> Locktimediv2R {
        Locktimediv2R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBPHYPLL0LOCKTIMEDIV2")
            .field("locktimediv2", &self.locktimediv2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
    #[inline(always)]
    #[must_use]
    pub fn locktimediv2(&mut self) -> Locktimediv2W<Usbphypll0locktimediv2Spec> {
        Locktimediv2W::new(self, 0)
    }
}
#[doc = "USB PHY PLL0 lock time division 2\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphypll0locktimediv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphypll0locktimediv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usbphypll0locktimediv2Spec;
impl crate::RegisterSpec for Usbphypll0locktimediv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphypll0locktimediv2::R`](R) reader structure"]
impl crate::Readable for Usbphypll0locktimediv2Spec {}
#[doc = "`write(|w| ..)` method takes [`usbphypll0locktimediv2::W`](W) writer structure"]
impl crate::Writable for Usbphypll0locktimediv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHYPLL0LOCKTIMEDIV2 to value 0xcafe"]
impl crate::Resettable for Usbphypll0locktimediv2Spec {
    const RESET_VALUE: u32 = 0xcafe;
}
