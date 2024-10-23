#[doc = "Register `SSTATUS` reader"]
pub type R = crate::R<SstatusSpec>;
#[doc = "Register `SSTATUS` writer"]
pub type W = crate::W<SstatusSpec>;
#[doc = "Field `STNOTSTOP` reader - Status not stop"]
pub type StnotstopR = crate::BitReader;
#[doc = "Field `STMSG` reader - Status message"]
pub type StmsgR = crate::BitReader;
#[doc = "Field `STCCCH` reader - Status Common Command Code Handler"]
pub type StccchR = crate::BitReader;
#[doc = "Field `STREQRD` reader - Status required"]
pub type StreqrdR = crate::BitReader;
#[doc = "Field `STREQWR` reader - Status request write"]
pub type StreqwrR = crate::BitReader;
#[doc = "Field `STDAA` reader - Status Dynamic Address Assignment"]
pub type StdaaR = crate::BitReader;
#[doc = "Field `STHDR` reader - Status High Data Rate"]
pub type SthdrR = crate::BitReader;
#[doc = "Field `START` reader - Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHED` reader - Matched"]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - Matched"]
pub type MatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PEND` reader - Received message pending"]
pub type RxPendR = crate::BitReader;
#[doc = "Field `TXNOTFULL` reader - Transmit buffer is not full"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `DACHG` reader - DACHG"]
pub type DachgR = crate::BitReader;
#[doc = "Field `DACHG` writer - DACHG"]
pub type DachgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC` reader - Common Command Code"]
pub type CccR = crate::BitReader;
#[doc = "Field `CCC` writer - Common Command Code"]
pub type CccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error warning"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `HDRMATCH` reader - High Data Rate command match"]
pub type HdrmatchR = crate::BitReader;
#[doc = "Field `HDRMATCH` writer - High Data Rate command match"]
pub type HdrmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANDLED` reader - Common-Command-Code handled"]
pub type ChandledR = crate::BitReader;
#[doc = "Field `CHANDLED` writer - Common-Command-Code handled"]
pub type ChandledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT` reader - Event"]
pub type EventR = crate::BitReader;
#[doc = "Field `EVENT` writer - Event"]
pub type EventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Event details\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evdet {
    #[doc = "0: NONE: no event or no pending event"]
    None = 0,
    #[doc = "1: NO_REQUEST: Request not sent yet. Either there was no START yet, or is waiting for Bus-Available or Bus-Idle (HJ)."]
    NoRequest = 1,
    #[doc = "2: NACKED: Not acknowledged(Request sent and NACKed); the module will try again."]
    Nacked = 2,
    #[doc = "3: ACKED: Acknowledged (Request sent and ACKed), so Done (unless the time control data is still being sent)."]
    Acked = 3,
}
impl From<Evdet> for u8 {
    #[inline(always)]
    fn from(variant: Evdet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evdet {
    type Ux = u8;
}
impl crate::IsEnum for Evdet {}
#[doc = "Field `EVDET` reader - Event details"]
pub type EvdetR = crate::FieldReader<Evdet>;
impl EvdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evdet {
        match self.bits {
            0 => Evdet::None,
            1 => Evdet::NoRequest,
            2 => Evdet::Nacked,
            3 => Evdet::Acked,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE: no event or no pending event"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Evdet::None
    }
    #[doc = "NO_REQUEST: Request not sent yet. Either there was no START yet, or is waiting for Bus-Available or Bus-Idle (HJ)."]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == Evdet::NoRequest
    }
    #[doc = "NACKED: Not acknowledged(Request sent and NACKed); the module will try again."]
    #[inline(always)]
    pub fn is_nacked(&self) -> bool {
        *self == Evdet::Nacked
    }
    #[doc = "ACKED: Acknowledged (Request sent and ACKed), so Done (unless the time control data is still being sent)."]
    #[inline(always)]
    pub fn is_acked(&self) -> bool {
        *self == Evdet::Acked
    }
}
#[doc = "Field `IBIDIS` reader - In-Band Interrupts are disabled"]
pub type IbidisR = crate::BitReader;
#[doc = "Field `MRDIS` reader - Master requests are disabled"]
pub type MrdisR = crate::BitReader;
#[doc = "Field `HJDIS` reader - Hot-Join is disabled"]
pub type HjdisR = crate::BitReader;
#[doc = "Activity state from Common Command Codes (CCC)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actstate {
    #[doc = "0: NO_LATENCY: normal bus operations"]
    NoLatency = 0,
    #[doc = "1: LATENCY_1MS: 1 ms of latency"]
    Latency1ms = 1,
    #[doc = "2: LATENCY_100MS: 100 ms of latency"]
    Latency100ms = 2,
    #[doc = "3: LATENCY_10S: 10 seconds of latency"]
    Latency10s = 3,
}
impl From<Actstate> for u8 {
    #[inline(always)]
    fn from(variant: Actstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actstate {
    type Ux = u8;
}
impl crate::IsEnum for Actstate {}
#[doc = "Field `ACTSTATE` reader - Activity state from Common Command Codes (CCC)"]
pub type ActstateR = crate::FieldReader<Actstate>;
impl ActstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actstate {
        match self.bits {
            0 => Actstate::NoLatency,
            1 => Actstate::Latency1ms,
            2 => Actstate::Latency100ms,
            3 => Actstate::Latency10s,
            _ => unreachable!(),
        }
    }
    #[doc = "NO_LATENCY: normal bus operations"]
    #[inline(always)]
    pub fn is_no_latency(&self) -> bool {
        *self == Actstate::NoLatency
    }
    #[doc = "LATENCY_1MS: 1 ms of latency"]
    #[inline(always)]
    pub fn is_latency_1ms(&self) -> bool {
        *self == Actstate::Latency1ms
    }
    #[doc = "LATENCY_100MS: 100 ms of latency"]
    #[inline(always)]
    pub fn is_latency_100ms(&self) -> bool {
        *self == Actstate::Latency100ms
    }
    #[doc = "LATENCY_10S: 10 seconds of latency"]
    #[inline(always)]
    pub fn is_latency_10s(&self) -> bool {
        *self == Actstate::Latency10s
    }
}
#[doc = "Time control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timectrl {
    #[doc = "0: NO_TIME_CONTROL: No time control is enabled"]
    NoTimeControl = 0,
    #[doc = "2: ASYNC_MODE: Asynchronous standard mode (0) is enabled"]
    AsyncMode = 2,
}
impl From<Timectrl> for u8 {
    #[inline(always)]
    fn from(variant: Timectrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timectrl {
    type Ux = u8;
}
impl crate::IsEnum for Timectrl {}
#[doc = "Field `TIMECTRL` reader - Time control"]
pub type TimectrlR = crate::FieldReader<Timectrl>;
impl TimectrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timectrl> {
        match self.bits {
            0 => Some(Timectrl::NoTimeControl),
            2 => Some(Timectrl::AsyncMode),
            _ => None,
        }
    }
    #[doc = "NO_TIME_CONTROL: No time control is enabled"]
    #[inline(always)]
    pub fn is_no_time_control(&self) -> bool {
        *self == Timectrl::NoTimeControl
    }
    #[doc = "ASYNC_MODE: Asynchronous standard mode (0) is enabled"]
    #[inline(always)]
    pub fn is_async_mode(&self) -> bool {
        *self == Timectrl::AsyncMode
    }
}
impl R {
    #[doc = "Bit 0 - Status not stop"]
    #[inline(always)]
    pub fn stnotstop(&self) -> StnotstopR {
        StnotstopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status message"]
    #[inline(always)]
    pub fn stmsg(&self) -> StmsgR {
        StmsgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Common Command Code Handler"]
    #[inline(always)]
    pub fn stccch(&self) -> StccchR {
        StccchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status required"]
    #[inline(always)]
    pub fn streqrd(&self) -> StreqrdR {
        StreqrdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status request write"]
    #[inline(always)]
    pub fn streqwr(&self) -> StreqwrR {
        StreqwrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Dynamic Address Assignment"]
    #[inline(always)]
    pub fn stdaa(&self) -> StdaaR {
        StdaaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status High Data Rate"]
    #[inline(always)]
    pub fn sthdr(&self) -> SthdrR {
        SthdrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Matched"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Received message pending"]
    #[inline(always)]
    pub fn rx_pend(&self) -> RxPendR {
        RxPendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit buffer is not full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG"]
    #[inline(always)]
    pub fn dachg(&self) -> DachgR {
        DachgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Common Command Code"]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High Data Rate command match"]
    #[inline(always)]
    pub fn hdrmatch(&self) -> HdrmatchR {
        HdrmatchR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Common-Command-Code handled"]
    #[inline(always)]
    pub fn chandled(&self) -> ChandledR {
        ChandledR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Event details"]
    #[inline(always)]
    pub fn evdet(&self) -> EvdetR {
        EvdetR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - In-Band Interrupts are disabled"]
    #[inline(always)]
    pub fn ibidis(&self) -> IbidisR {
        IbidisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master requests are disabled"]
    #[inline(always)]
    pub fn mrdis(&self) -> MrdisR {
        MrdisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Hot-Join is disabled"]
    #[inline(always)]
    pub fn hjdis(&self) -> HjdisR {
        HjdisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Activity state from Common Command Codes (CCC)"]
    #[inline(always)]
    pub fn actstate(&self) -> ActstateR {
        ActstateR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Time control"]
    #[inline(always)]
    pub fn timectrl(&self) -> TimectrlR {
        TimectrlR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTATUS")
            .field("stnotstop", &self.stnotstop())
            .field("stmsg", &self.stmsg())
            .field("stccch", &self.stccch())
            .field("streqrd", &self.streqrd())
            .field("streqwr", &self.streqwr())
            .field("stdaa", &self.stdaa())
            .field("sthdr", &self.sthdr())
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rx_pend", &self.rx_pend())
            .field("txnotfull", &self.txnotfull())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("hdrmatch", &self.hdrmatch())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .field("evdet", &self.evdet())
            .field("ibidis", &self.ibidis())
            .field("mrdis", &self.mrdis())
            .field("hjdis", &self.hjdis())
            .field("actstate", &self.actstate())
            .field("timectrl", &self.timectrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<SstatusSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - Matched"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MatchedW<SstatusSpec> {
        MatchedW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<SstatusSpec> {
        StopW::new(self, 10)
    }
    #[doc = "Bit 13 - DACHG"]
    #[inline(always)]
    #[must_use]
    pub fn dachg(&mut self) -> DachgW<SstatusSpec> {
        DachgW::new(self, 13)
    }
    #[doc = "Bit 14 - Common Command Code"]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CccW<SstatusSpec> {
        CccW::new(self, 14)
    }
    #[doc = "Bit 16 - High Data Rate command match"]
    #[inline(always)]
    #[must_use]
    pub fn hdrmatch(&mut self) -> HdrmatchW<SstatusSpec> {
        HdrmatchW::new(self, 16)
    }
    #[doc = "Bit 17 - Common-Command-Code handled"]
    #[inline(always)]
    #[must_use]
    pub fn chandled(&mut self) -> ChandledW<SstatusSpec> {
        ChandledW::new(self, 17)
    }
    #[doc = "Bit 18 - Event"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EventW<SstatusSpec> {
        EventW::new(self, 18)
    }
}
#[doc = "Slave Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatusSpec;
impl crate::RegisterSpec for SstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatus::R`](R) reader structure"]
impl crate::Readable for SstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sstatus::W`](W) writer structure"]
impl crate::Writable for SstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTATUS to value 0x1000"]
impl crate::Resettable for SstatusSpec {
    const RESET_VALUE: u32 = 0x1000;
}
