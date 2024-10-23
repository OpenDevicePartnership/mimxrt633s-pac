#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `MSTPENDINGCLR` writer - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
pub type MstpendingclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTARBLOSSCLR` writer - Master Arbitration Loss interrupt clear."]
pub type MstarblossclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTSTSTPERRCLR` writer - Master Start/Stop Error interrupt clear."]
pub type MstststperrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVPENDINGCLR` writer - Slave Pending interrupt clear."]
pub type SlvpendingclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVNOTSTRCLR` writer - Slave Not Stretching interrupt clear."]
pub type SlvnotstrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVDESELCLR` writer - Slave Deselect interrupt clear."]
pub type SlvdeselclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDYCLR` writer - Monitor data Ready interrupt clear."]
pub type MonrdyclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONOVCLR` writer - Monitor Overrun interrupt clear."]
pub type MonovclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONIDLECLR` writer - Monitor Idle interrupt clear."]
pub type MonidleclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTTIMEOUTCLR` writer - Event time-out interrupt clear."]
pub type EventtimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLTIMEOUTCLR` writer - SCL time-out interrupt clear."]
pub type ScltimeoutclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IntenclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    #[must_use]
    pub fn mstpendingclr(&mut self) -> MstpendingclrW<IntenclrSpec> {
        MstpendingclrW::new(self, 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn mstarblossclr(&mut self) -> MstarblossclrW<IntenclrSpec> {
        MstarblossclrW::new(self, 4)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn mstststperrclr(&mut self) -> MstststperrclrW<IntenclrSpec> {
        MstststperrclrW::new(self, 6)
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn slvpendingclr(&mut self) -> SlvpendingclrW<IntenclrSpec> {
        SlvpendingclrW::new(self, 8)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn slvnotstrclr(&mut self) -> SlvnotstrclrW<IntenclrSpec> {
        SlvnotstrclrW::new(self, 11)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn slvdeselclr(&mut self) -> SlvdeselclrW<IntenclrSpec> {
        SlvdeselclrW::new(self, 15)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn monrdyclr(&mut self) -> MonrdyclrW<IntenclrSpec> {
        MonrdyclrW::new(self, 16)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn monovclr(&mut self) -> MonovclrW<IntenclrSpec> {
        MonovclrW::new(self, 17)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn monidleclr(&mut self) -> MonidleclrW<IntenclrSpec> {
        MonidleclrW::new(self, 19)
    }
    #[doc = "Bit 24 - Event time-out interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn eventtimeoutclr(&mut self) -> EventtimeoutclrW<IntenclrSpec> {
        EventtimeoutclrW::new(self, 24)
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn scltimeoutclr(&mut self) -> ScltimeoutclrW<IntenclrSpec> {
        ScltimeoutclrW::new(self, 25)
    }
}
#[doc = "Interrupt Enable Clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
