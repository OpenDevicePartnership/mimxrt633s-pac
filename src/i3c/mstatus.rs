#[doc = "Register `MSTATUS` reader"]
pub type R = crate::R<MstatusSpec>;
#[doc = "Register `MSTATUS` writer"]
pub type W = crate::W<MstatusSpec>;
#[doc = "State of the master\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: IDLE: the bus has STOPped."]
    Idle = 0,
    #[doc = "1: SLVREQ: (Slave Request state) the bus has STOPped but a slave is holding SDA low. If using auto-emit IBI (MCTRL.AutoIBI), then the master will not stay in the Slave Request state."]
    Slvreq = 1,
    #[doc = "2: MSGSDR: in Single Data Rate (SDR) Message state (from using MWMSG_SDR)"]
    Msgsdr = 2,
    #[doc = "3: NORMACT: normal active Single Data Rate (SDR) state (from using MCTRL and MWDATAn and MRDATAn registers). The master will stay in the NORMACT state until a STOP is issued."]
    Normact = 3,
    #[doc = "4: MSGDDR: in Double Data Rate (DDR) Message mode (from using MWMSG_DDR or using the normal method with DDR). The master will stay in the DDR state, until the master exits using EXIT (emits the Exit pattern)."]
    Ddr = 4,
    #[doc = "5: DAA: in Enter Dynamic Address Assignment (ENTDAA) mode"]
    Daa = 5,
    #[doc = "6: IBIACK: waiting for an In-Band Interrupt (IBI) ACK/NACK decision"]
    Ibiack = 6,
    #[doc = "7: IBIRCV: Receiving an In-Band Interrupt (IBI); this IBIRCV state is used after IBI/MR/HJ has won the arbitration, and IBIRCV state is also used for IBI mandatory byte (if any) and any bytes that follow."]
    Ibircv = 7,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State of the master"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            0 => State::Idle,
            1 => State::Slvreq,
            2 => State::Msgsdr,
            3 => State::Normact,
            4 => State::Ddr,
            5 => State::Daa,
            6 => State::Ibiack,
            7 => State::Ibircv,
            _ => unreachable!(),
        }
    }
    #[doc = "IDLE: the bus has STOPped."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
    #[doc = "SLVREQ: (Slave Request state) the bus has STOPped but a slave is holding SDA low. If using auto-emit IBI (MCTRL.AutoIBI), then the master will not stay in the Slave Request state."]
    #[inline(always)]
    pub fn is_slvreq(&self) -> bool {
        *self == State::Slvreq
    }
    #[doc = "MSGSDR: in Single Data Rate (SDR) Message state (from using MWMSG_SDR)"]
    #[inline(always)]
    pub fn is_msgsdr(&self) -> bool {
        *self == State::Msgsdr
    }
    #[doc = "NORMACT: normal active Single Data Rate (SDR) state (from using MCTRL and MWDATAn and MRDATAn registers). The master will stay in the NORMACT state until a STOP is issued."]
    #[inline(always)]
    pub fn is_normact(&self) -> bool {
        *self == State::Normact
    }
    #[doc = "MSGDDR: in Double Data Rate (DDR) Message mode (from using MWMSG_DDR or using the normal method with DDR). The master will stay in the DDR state, until the master exits using EXIT (emits the Exit pattern)."]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == State::Ddr
    }
    #[doc = "DAA: in Enter Dynamic Address Assignment (ENTDAA) mode"]
    #[inline(always)]
    pub fn is_daa(&self) -> bool {
        *self == State::Daa
    }
    #[doc = "IBIACK: waiting for an In-Band Interrupt (IBI) ACK/NACK decision"]
    #[inline(always)]
    pub fn is_ibiack(&self) -> bool {
        *self == State::Ibiack
    }
    #[doc = "IBIRCV: Receiving an In-Band Interrupt (IBI); this IBIRCV state is used after IBI/MR/HJ has won the arbitration, and IBIRCV state is also used for IBI mandatory byte (if any) and any bytes that follow."]
    #[inline(always)]
    pub fn is_ibircv(&self) -> bool {
        *self == State::Ibircv
    }
}
#[doc = "Field `BETWEEN` reader - Between messages or Dynamic Address Assignments (DAA)"]
pub type BetweenR = crate::BitReader;
#[doc = "Field `NACKED` reader - Not acknowledged"]
pub type NackedR = crate::BitReader;
#[doc = "In-Band Interrupt (IBI) type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibitype {
    #[doc = "0: NONE: cleared when IBI Won bit (MSTATUS.IBIWON) is cleared"]
    None = 0,
    #[doc = "1: IBI: In-Band Interrupt"]
    Ibi = 1,
    #[doc = "2: MR: Master Request"]
    Mr = 2,
    #[doc = "3: HJ: Hot-Join"]
    Hj = 3,
}
impl From<Ibitype> for u8 {
    #[inline(always)]
    fn from(variant: Ibitype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ibitype {
    type Ux = u8;
}
impl crate::IsEnum for Ibitype {}
#[doc = "Field `IBITYPE` reader - In-Band Interrupt (IBI) type"]
pub type IbitypeR = crate::FieldReader<Ibitype>;
impl IbitypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibitype {
        match self.bits {
            0 => Ibitype::None,
            1 => Ibitype::Ibi,
            2 => Ibitype::Mr,
            3 => Ibitype::Hj,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE: cleared when IBI Won bit (MSTATUS.IBIWON) is cleared"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ibitype::None
    }
    #[doc = "IBI: In-Band Interrupt"]
    #[inline(always)]
    pub fn is_ibi(&self) -> bool {
        *self == Ibitype::Ibi
    }
    #[doc = "MR: Master Request"]
    #[inline(always)]
    pub fn is_mr(&self) -> bool {
        *self == Ibitype::Mr
    }
    #[doc = "HJ: Hot-Join"]
    #[inline(always)]
    pub fn is_hj(&self) -> bool {
        *self == Ibitype::Hj
    }
}
#[doc = "Field `SLVSTART` reader - Slave start"]
pub type SlvstartR = crate::BitReader;
#[doc = "Field `SLVSTART` writer - Slave start"]
pub type SlvstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCTRLDONE` reader - Master control done"]
pub type MctrldoneR = crate::BitReader;
#[doc = "Field `MCTRLDONE` writer - Master control done"]
pub type MctrldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLETE` reader - COMPLETE"]
pub type CompleteR = crate::BitReader;
#[doc = "Field `COMPLETE` writer - COMPLETE"]
pub type CompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - RXPEND"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `TXNOTFULL` reader - TX buffer/FIFO not yet full"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `IBIWON` reader - In-Band Interrupt (IBI) won"]
pub type IbiwonR = crate::BitReader;
#[doc = "Field `IBIWON` writer - In-Band Interrupt (IBI) won"]
pub type IbiwonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error or warning"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `NOWMASTER` reader - Now master (now this module is a master)"]
pub type NowmasterR = crate::BitReader;
#[doc = "Field `NOWMASTER` writer - Now master (now this module is a master)"]
pub type NowmasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIADDR` reader - IBI address"]
pub type IbiaddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - State of the master"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Between messages or Dynamic Address Assignments (DAA)"]
    #[inline(always)]
    pub fn between(&self) -> BetweenR {
        BetweenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not acknowledged"]
    #[inline(always)]
    pub fn nacked(&self) -> NackedR {
        NackedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) type"]
    #[inline(always)]
    pub fn ibitype(&self) -> IbitypeR {
        IbitypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Slave start"]
    #[inline(always)]
    pub fn slvstart(&self) -> SlvstartR {
        SlvstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master control done"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MctrldoneR {
        MctrldoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMPLETE"]
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX buffer/FIFO not yet full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IbiwonR {
        IbiwonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error or warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Now master (now this module is a master)"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NowmasterR {
        NowmasterR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:30 - IBI address"]
    #[inline(always)]
    pub fn ibiaddr(&self) -> IbiaddrR {
        IbiaddrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTATUS")
            .field("state", &self.state())
            .field("between", &self.between())
            .field("nacked", &self.nacked())
            .field("ibitype", &self.ibitype())
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .field("ibiaddr", &self.ibiaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Slave start"]
    #[inline(always)]
    pub fn slvstart(&mut self) -> SlvstartW<MstatusSpec> {
        SlvstartW::new(self, 8)
    }
    #[doc = "Bit 9 - Master control done"]
    #[inline(always)]
    pub fn mctrldone(&mut self) -> MctrldoneW<MstatusSpec> {
        MctrldoneW::new(self, 9)
    }
    #[doc = "Bit 10 - COMPLETE"]
    #[inline(always)]
    pub fn complete(&mut self) -> CompleteW<MstatusSpec> {
        CompleteW::new(self, 10)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won"]
    #[inline(always)]
    pub fn ibiwon(&mut self) -> IbiwonW<MstatusSpec> {
        IbiwonW::new(self, 13)
    }
    #[doc = "Bit 19 - Now master (now this module is a master)"]
    #[inline(always)]
    pub fn nowmaster(&mut self) -> NowmasterW<MstatusSpec> {
        NowmasterW::new(self, 19)
    }
}
#[doc = "Master Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstatusSpec;
impl crate::RegisterSpec for MstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstatus::R`](R) reader structure"]
impl crate::Readable for MstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mstatus::W`](W) writer structure"]
impl crate::Writable for MstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTATUS to value 0x1000"]
impl crate::Resettable for MstatusSpec {
    const RESET_VALUE: u32 = 0x1000;
}
