#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Set if the accelerator should interrupt when done.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Do not interrupt when done"]
    NoInterrupt = 0,
    #[doc = "1: Interrupt when done"]
    Interrupt = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Set if the accelerator should interrupt when done."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::NoInterrupt,
            true => Done::Interrupt,
        }
    }
    #[doc = "Do not interrupt when done"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Done::NoInterrupt
    }
    #[doc = "Interrupt when done"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Done::Interrupt
    }
}
#[doc = "Field `DONE` writer - Set if the accelerator should interrupt when done."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not interrupt when done"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Done::NoInterrupt)
    }
    #[doc = "Interrupt when done"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Interrupt)
    }
}
impl R {
    #[doc = "Bit 0 - Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntensetSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Sets interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
