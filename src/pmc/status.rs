#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "General sequencer and finite state machine status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activefsm {
    #[doc = "0: All PMC finite state machines are idle. OK to set APPLYCFG to trigger the PMC state machines."]
    Activefsm0 = 0,
    #[doc = "1: One or more PMC finite state machines are active, do not set APPLYCFG or write to any PDRUNCFG or CTRL register values that are used by the PMC state machines."]
    Activefsm1 = 1,
}
impl From<Activefsm> for bool {
    #[inline(always)]
    fn from(variant: Activefsm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVEFSM` reader - General sequencer and finite state machine status"]
pub type ActivefsmR = crate::BitReader<Activefsm>;
impl ActivefsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Activefsm {
        match self.bits {
            false => Activefsm::Activefsm0,
            true => Activefsm::Activefsm1,
        }
    }
    #[doc = "All PMC finite state machines are idle. OK to set APPLYCFG to trigger the PMC state machines."]
    #[inline(always)]
    pub fn is_activefsm_0(&self) -> bool {
        *self == Activefsm::Activefsm0
    }
    #[doc = "One or more PMC finite state machines are active, do not set APPLYCFG or write to any PDRUNCFG or CTRL register values that are used by the PMC state machines."]
    #[inline(always)]
    pub fn is_activefsm_1(&self) -> bool {
        *self == Activefsm::Activefsm1
    }
}
impl R {
    #[doc = "Bit 0 - General sequencer and finite state machine status"]
    #[inline(always)]
    pub fn activefsm(&self) -> ActivefsmR {
        ActivefsmR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("activefsm", &self.activefsm())
            .finish()
    }
}
#[doc = "PMC status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
