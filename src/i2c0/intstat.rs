#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `MSTPENDING` reader - Master Pending."]
pub type MstpendingR = crate::BitReader;
#[doc = "Field `MSTARBLOSS` reader - Master Arbitration Loss flag."]
pub type MstarblossR = crate::BitReader;
#[doc = "Field `MSTSTSTPERR` reader - Master Start/Stop Error flag."]
pub type MstststperrR = crate::BitReader;
#[doc = "Field `SLVPENDING` reader - Slave Pending."]
pub type SlvpendingR = crate::BitReader;
#[doc = "Field `SLVNOTSTR` reader - Slave Not Stretching status."]
pub type SlvnotstrR = crate::BitReader;
#[doc = "Field `SLVDESEL` reader - Slave Deselected flag."]
pub type SlvdeselR = crate::BitReader;
#[doc = "Field `MONRDY` reader - Monitor Ready."]
pub type MonrdyR = crate::BitReader;
#[doc = "Field `MONOV` reader - Monitor Overflow flag."]
pub type MonovR = crate::BitReader;
#[doc = "Field `MONIDLE` reader - Monitor Idle flag."]
pub type MonidleR = crate::BitReader;
#[doc = "Field `EVENTTIMEOUT` reader - Event time-out Interrupt flag."]
pub type EventtimeoutR = crate::BitReader;
#[doc = "Field `SCLTIMEOUT` reader - SCL time-out Interrupt flag."]
pub type ScltimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master Pending."]
    #[inline(always)]
    pub fn mstpending(&self) -> MstpendingR {
        MstpendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MstarblossR {
        MstarblossR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MstststperrR {
        MstststperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending."]
    #[inline(always)]
    pub fn slvpending(&self) -> SlvpendingR {
        SlvpendingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SlvnotstrR {
        SlvnotstrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SlvdeselR {
        SlvdeselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready."]
    #[inline(always)]
    pub fn monrdy(&self) -> MonrdyR {
        MonrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MonovR {
        MonovR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag."]
    #[inline(always)]
    pub fn monidle(&self) -> MonidleR {
        MonidleR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EventtimeoutR {
        EventtimeoutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn scltimeout(&self) -> ScltimeoutR {
        ScltimeoutR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("mstpending", &self.mstpending())
            .field("mstarbloss", &self.mstarbloss())
            .field("mstststperr", &self.mstststperr())
            .field("slvpending", &self.slvpending())
            .field("slvnotstr", &self.slvnotstr())
            .field("slvdesel", &self.slvdesel())
            .field("monrdy", &self.monrdy())
            .field("monov", &self.monov())
            .field("monidle", &self.monidle())
            .field("eventtimeout", &self.eventtimeout())
            .field("scltimeout", &self.scltimeout())
            .finish()
    }
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0x0801"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0x0801;
}
