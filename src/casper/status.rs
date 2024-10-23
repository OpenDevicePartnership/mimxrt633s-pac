#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Busy or just cleared"]
    Busy = 0,
    #[doc = "1: Completed last operation"]
    Completed = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Busy,
            true => Done::Completed,
        }
    }
    #[doc = "Busy or just cleared"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Done::Busy
    }
    #[doc = "Completed last operation"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Done::Completed
    }
}
#[doc = "Field `DONE` writer - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Busy or just cleared"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Busy)
    }
    #[doc = "Completed last operation"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Completed)
    }
}
#[doc = "Last carry value if operation produced a carry bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carry {
    #[doc = "0: Carry was 0 or no carry"]
    NoCarry = 0,
    #[doc = "1: Carry was 1"]
    Carry = 1,
}
impl From<Carry> for bool {
    #[inline(always)]
    fn from(variant: Carry) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARRY` reader - Last carry value if operation produced a carry bit"]
pub type CarryR = crate::BitReader<Carry>;
impl CarryR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carry {
        match self.bits {
            false => Carry::NoCarry,
            true => Carry::Carry,
        }
    }
    #[doc = "Carry was 0 or no carry"]
    #[inline(always)]
    pub fn is_no_carry(&self) -> bool {
        *self == Carry::NoCarry
    }
    #[doc = "Carry was 1"]
    #[inline(always)]
    pub fn is_carry(&self) -> bool {
        *self == Carry::Carry
    }
}
#[doc = "Indicates if the accelerator is busy performing an operation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Not busy - is idle"]
    Idle = 0,
    #[doc = "1: Is busy"]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Indicates if the accelerator is busy performing an operation"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Busy,
        }
    }
    #[doc = "Not busy - is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "Is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Last carry value if operation produced a carry bit"]
    #[inline(always)]
    pub fn carry(&self) -> CarryR {
        CarryR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the accelerator is busy performing an operation"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("done", &self.done())
            .field("carry", &self.carry())
            .field("busy", &self.busy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<StatusSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Indicates operational status and would contain the carry bit if used.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
