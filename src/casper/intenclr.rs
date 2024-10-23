#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Written to clear an interrupt set with INTENSET.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: If written 0, ignored"]
    Ignored = 0,
    #[doc = "1: If written 1, do not Interrupt when done"]
    NoInterrupt = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Written to clear an interrupt set with INTENSET."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Ignored,
            true => Done::NoInterrupt,
        }
    }
    #[doc = "If written 0, ignored"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == Done::Ignored
    }
    #[doc = "If written 1, do not Interrupt when done"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Done::NoInterrupt
    }
}
#[doc = "Field `DONE` writer - Written to clear an interrupt set with INTENSET."]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If written 0, ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Ignored)
    }
    #[doc = "If written 1, do not Interrupt when done"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Done::NoInterrupt)
    }
}
impl R {
    #[doc = "Bit 0 - Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IntenclrSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Clears interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
