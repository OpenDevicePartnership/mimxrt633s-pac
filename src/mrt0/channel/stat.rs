#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Monitors the interrupt flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intflag {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NoPendingInterrupt = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PendingInterrupt = 1,
}
impl From<Intflag> for bool {
    #[inline(always)]
    fn from(variant: Intflag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTFLAG` reader - Monitors the interrupt flag."]
pub type IntflagR = crate::BitReader<Intflag>;
impl IntflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intflag {
        match self.bits {
            false => Intflag::NoPendingInterrupt,
            true => Intflag::PendingInterrupt,
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == Intflag::NoPendingInterrupt
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == Intflag::PendingInterrupt
    }
}
#[doc = "Field `INTFLAG` writer - Monitors the interrupt flag."]
pub type IntflagW<'a, REG> = crate::BitWriter<'a, REG, Intflag>;
impl<'a, REG> IntflagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intflag::NoPendingInterrupt)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intflag::PendingInterrupt)
    }
}
#[doc = "Indicates the state of TIMERn. This bit is read-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "0: Idle state. TIMERn is stopped."]
    IdleState = 0,
    #[doc = "1: Running. TIMERn is running."]
    Running = 1,
}
impl From<Run> for bool {
    #[inline(always)]
    fn from(variant: Run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - Indicates the state of TIMERn. This bit is read-only."]
pub type RunR = crate::BitReader<Run>;
impl RunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run {
        match self.bits {
            false => Run::IdleState,
            true => Run::Running,
        }
    }
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == Run::IdleState
    }
    #[doc = "Running. TIMERn is running."]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Run::Running
    }
}
#[doc = "Field `RUN` writer - Indicates the state of TIMERn. This bit is read-only."]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG, Run>;
impl<'a, REG> RunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline(always)]
    pub fn idle_state(self) -> &'a mut crate::W<REG> {
        self.variant(Run::IdleState)
    }
    #[doc = "Running. TIMERn is running."]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(Run::Running)
    }
}
#[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inuse {
    #[doc = "0: This channel is not in use."]
    No = 0,
    #[doc = "1: This channel is in use."]
    Yes = 1,
}
impl From<Inuse> for bool {
    #[inline(always)]
    fn from(variant: Inuse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INUSE` reader - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
pub type InuseR = crate::BitReader<Inuse>;
impl InuseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inuse {
        match self.bits {
            false => Inuse::No,
            true => Inuse::Yes,
        }
    }
    #[doc = "This channel is not in use."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Inuse::No
    }
    #[doc = "This channel is in use."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Inuse::Yes
    }
}
#[doc = "Field `INUSE` writer - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
pub type InuseW<'a, REG> = crate::BitWriter<'a, REG, Inuse>;
impl<'a, REG> InuseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This channel is not in use."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Inuse::No)
    }
    #[doc = "This channel is in use."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Inuse::Yes)
    }
}
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&self) -> IntflagR {
        IntflagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn inuse(&self) -> InuseR {
        InuseR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("intflag", &self.intflag())
            .field("run", &self.run())
            .field("inuse", &self.inuse())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn intflag(&mut self) -> IntflagW<StatSpec> {
        IntflagW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RunW<StatSpec> {
        RunW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    #[must_use]
    pub fn inuse(&mut self) -> InuseW<StatSpec> {
        InuseW::new(self, 2)
    }
}
#[doc = "MRT Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
