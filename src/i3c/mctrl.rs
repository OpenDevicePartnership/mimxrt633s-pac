#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MctrlSpec>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "Request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Request {
    #[doc = "0: NONE: Returns to this when finished with any request. The MSTATUS register indicates the master's state. See also AutoIBI mode. NONE is only written as 0: when setting RDTERM to 1 (to stop a read in progress) or when setting IBI reponse field (IBIRESP) for MSG use"]
    None = 0,
    #[doc = "1: EMITSTARTADDR: Emit START with address and direction from a stopped state or in the middle of a Single Data Rate (SDR) message. If from a stopped state (IDLE), then emit start may be prevented by an event (like IBI, MR, HJ), in which case the appropriate interrupt is signaled; note that Emit START can be resubmitted."]
    Emitstartaddr = 1,
    #[doc = "2: EMITSTOP: Emit a STOP on bus. Must be in Single Data Rate (SDR) mode. If in Dynamic Address Assignment (DAA) mode, Emit stop will exit DAA mode."]
    Emitstop = 2,
    #[doc = "3: IBIACKNACK: Manual In-Band Interrupt (IBI) Acknowledge (ACK) or Not Acknowledge (NACK). When IBIRESP has indicated a hold on an In-Band Interrupt to allow a manual decision, this request completes it. Uses IBIRESP to provide the information."]
    Ibiacknack = 3,
    #[doc = "4: PROCESSDAA: If not in Dynamic Address Assignment (DAA) mode now, will issue START, 7E, ENTDAA, and then will emit 7E/R to process each slave. Will stop just before the new Dynamic Address (DA) is to be emitted. The next Process DAA request will use the Addr field as the new DA to assign. If NACKed on the 7E/R, then the interrupt will indicate this situation, and a STOP will be emitted."]
    Processdaa = 4,
    #[doc = "6: FORCEEXIT and IBHR: Emit an Exit Pattern from any state, but end Double Data Rate (DDR) (including MSGDDR), if in DDR mode now. Includes a STOP afterward. If TYPE != 0, then it will perform an IBHR (In-Band Hardware Reset). If TYPE=2, then it does a normal reset (DEFRST can prevent the reset). If TYPE=3, it does a forced reset (will always reset)."]
    Forceexit = 6,
    #[doc = "7: AUTOIBI: Hold in a stopped state, but auto-emit START,7E when the slave is holding down SDA to get an In-Band Interrupt (IBI). Actual In-Band Interrupt handling is defined by IBIRESP."]
    Autoibi = 7,
}
impl From<Request> for u8 {
    #[inline(always)]
    fn from(variant: Request) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Request {
    type Ux = u8;
}
impl crate::IsEnum for Request {}
#[doc = "Field `REQUEST` reader - Request"]
pub type RequestR = crate::FieldReader<Request>;
impl RequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Request> {
        match self.bits {
            0 => Some(Request::None),
            1 => Some(Request::Emitstartaddr),
            2 => Some(Request::Emitstop),
            3 => Some(Request::Ibiacknack),
            4 => Some(Request::Processdaa),
            6 => Some(Request::Forceexit),
            7 => Some(Request::Autoibi),
            _ => None,
        }
    }
    #[doc = "NONE: Returns to this when finished with any request. The MSTATUS register indicates the master's state. See also AutoIBI mode. NONE is only written as 0: when setting RDTERM to 1 (to stop a read in progress) or when setting IBI reponse field (IBIRESP) for MSG use"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Request::None
    }
    #[doc = "EMITSTARTADDR: Emit START with address and direction from a stopped state or in the middle of a Single Data Rate (SDR) message. If from a stopped state (IDLE), then emit start may be prevented by an event (like IBI, MR, HJ), in which case the appropriate interrupt is signaled; note that Emit START can be resubmitted."]
    #[inline(always)]
    pub fn is_emitstartaddr(&self) -> bool {
        *self == Request::Emitstartaddr
    }
    #[doc = "EMITSTOP: Emit a STOP on bus. Must be in Single Data Rate (SDR) mode. If in Dynamic Address Assignment (DAA) mode, Emit stop will exit DAA mode."]
    #[inline(always)]
    pub fn is_emitstop(&self) -> bool {
        *self == Request::Emitstop
    }
    #[doc = "IBIACKNACK: Manual In-Band Interrupt (IBI) Acknowledge (ACK) or Not Acknowledge (NACK). When IBIRESP has indicated a hold on an In-Band Interrupt to allow a manual decision, this request completes it. Uses IBIRESP to provide the information."]
    #[inline(always)]
    pub fn is_ibiacknack(&self) -> bool {
        *self == Request::Ibiacknack
    }
    #[doc = "PROCESSDAA: If not in Dynamic Address Assignment (DAA) mode now, will issue START, 7E, ENTDAA, and then will emit 7E/R to process each slave. Will stop just before the new Dynamic Address (DA) is to be emitted. The next Process DAA request will use the Addr field as the new DA to assign. If NACKed on the 7E/R, then the interrupt will indicate this situation, and a STOP will be emitted."]
    #[inline(always)]
    pub fn is_processdaa(&self) -> bool {
        *self == Request::Processdaa
    }
    #[doc = "FORCEEXIT and IBHR: Emit an Exit Pattern from any state, but end Double Data Rate (DDR) (including MSGDDR), if in DDR mode now. Includes a STOP afterward. If TYPE != 0, then it will perform an IBHR (In-Band Hardware Reset). If TYPE=2, then it does a normal reset (DEFRST can prevent the reset). If TYPE=3, it does a forced reset (will always reset)."]
    #[inline(always)]
    pub fn is_forceexit(&self) -> bool {
        *self == Request::Forceexit
    }
    #[doc = "AUTOIBI: Hold in a stopped state, but auto-emit START,7E when the slave is holding down SDA to get an In-Band Interrupt (IBI). Actual In-Band Interrupt handling is defined by IBIRESP."]
    #[inline(always)]
    pub fn is_autoibi(&self) -> bool {
        *self == Request::Autoibi
    }
}
#[doc = "Field `REQUEST` writer - Request"]
pub type RequestW<'a, REG> = crate::FieldWriter<'a, REG, 3, Request>;
impl<'a, REG> RequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE: Returns to this when finished with any request. The MSTATUS register indicates the master's state. See also AutoIBI mode. NONE is only written as 0: when setting RDTERM to 1 (to stop a read in progress) or when setting IBI reponse field (IBIRESP) for MSG use"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Request::None)
    }
    #[doc = "EMITSTARTADDR: Emit START with address and direction from a stopped state or in the middle of a Single Data Rate (SDR) message. If from a stopped state (IDLE), then emit start may be prevented by an event (like IBI, MR, HJ), in which case the appropriate interrupt is signaled; note that Emit START can be resubmitted."]
    #[inline(always)]
    pub fn emitstartaddr(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Emitstartaddr)
    }
    #[doc = "EMITSTOP: Emit a STOP on bus. Must be in Single Data Rate (SDR) mode. If in Dynamic Address Assignment (DAA) mode, Emit stop will exit DAA mode."]
    #[inline(always)]
    pub fn emitstop(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Emitstop)
    }
    #[doc = "IBIACKNACK: Manual In-Band Interrupt (IBI) Acknowledge (ACK) or Not Acknowledge (NACK). When IBIRESP has indicated a hold on an In-Band Interrupt to allow a manual decision, this request completes it. Uses IBIRESP to provide the information."]
    #[inline(always)]
    pub fn ibiacknack(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Ibiacknack)
    }
    #[doc = "PROCESSDAA: If not in Dynamic Address Assignment (DAA) mode now, will issue START, 7E, ENTDAA, and then will emit 7E/R to process each slave. Will stop just before the new Dynamic Address (DA) is to be emitted. The next Process DAA request will use the Addr field as the new DA to assign. If NACKed on the 7E/R, then the interrupt will indicate this situation, and a STOP will be emitted."]
    #[inline(always)]
    pub fn processdaa(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Processdaa)
    }
    #[doc = "FORCEEXIT and IBHR: Emit an Exit Pattern from any state, but end Double Data Rate (DDR) (including MSGDDR), if in DDR mode now. Includes a STOP afterward. If TYPE != 0, then it will perform an IBHR (In-Band Hardware Reset). If TYPE=2, then it does a normal reset (DEFRST can prevent the reset). If TYPE=3, it does a forced reset (will always reset)."]
    #[inline(always)]
    pub fn forceexit(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Forceexit)
    }
    #[doc = "AUTOIBI: Hold in a stopped state, but auto-emit START,7E when the slave is holding down SDA to get an In-Band Interrupt (IBI). Actual In-Band Interrupt handling is defined by IBIRESP."]
    #[inline(always)]
    pub fn autoibi(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Autoibi)
    }
}
#[doc = "Bus type with START\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: I3C: Normally the SDR mode of I3C. For ForceExit, the Exit pattern."]
    I3c = 0,
    #[doc = "1: I2C: Normally the Standard I2C protocol."]
    I2c = 1,
    #[doc = "2: DDR: (Double Data Rate): Normally the HDR-DDR mode of I3C. Enter DDR mode (7E and then ENTHDR0), if the module is not already in DDR mode. The 1st byte written to the TX FIFO should be a command, and should already be in the FIFO. To end DDR mode, use ForceExit. For ForceExit, the normal IBHR (In-Band Hardware Reset)."]
    Ddr = 2,
    #[doc = "3: For ForcedExit, this is forced IBHR."]
    Forcedibhr = 3,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - Bus type with START"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Type {
        match self.bits {
            0 => Type::I3c,
            1 => Type::I2c,
            2 => Type::Ddr,
            3 => Type::Forcedibhr,
            _ => unreachable!(),
        }
    }
    #[doc = "I3C: Normally the SDR mode of I3C. For ForceExit, the Exit pattern."]
    #[inline(always)]
    pub fn is_i3c(&self) -> bool {
        *self == Type::I3c
    }
    #[doc = "I2C: Normally the Standard I2C protocol."]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Type::I2c
    }
    #[doc = "DDR: (Double Data Rate): Normally the HDR-DDR mode of I3C. Enter DDR mode (7E and then ENTHDR0), if the module is not already in DDR mode. The 1st byte written to the TX FIFO should be a command, and should already be in the FIFO. To end DDR mode, use ForceExit. For ForceExit, the normal IBHR (In-Band Hardware Reset)."]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == Type::Ddr
    }
    #[doc = "For ForcedExit, this is forced IBHR."]
    #[inline(always)]
    pub fn is_forcedibhr(&self) -> bool {
        *self == Type::Forcedibhr
    }
}
#[doc = "Field `TYPE` writer - Bus type with START"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Type, crate::Safe>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I3C: Normally the SDR mode of I3C. For ForceExit, the Exit pattern."]
    #[inline(always)]
    pub fn i3c(self) -> &'a mut crate::W<REG> {
        self.variant(Type::I3c)
    }
    #[doc = "I2C: Normally the Standard I2C protocol."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Type::I2c)
    }
    #[doc = "DDR: (Double Data Rate): Normally the HDR-DDR mode of I3C. Enter DDR mode (7E and then ENTHDR0), if the module is not already in DDR mode. The 1st byte written to the TX FIFO should be a command, and should already be in the FIFO. To end DDR mode, use ForceExit. For ForceExit, the normal IBHR (In-Band Hardware Reset)."]
    #[inline(always)]
    pub fn ddr(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Ddr)
    }
    #[doc = "For ForcedExit, this is forced IBHR."]
    #[inline(always)]
    pub fn forcedibhr(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Forcedibhr)
    }
}
#[doc = "In-Band Interrupt (IBI) response\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibiresp {
    #[doc = "0: ACK: Acknowledge. A mandatory byte (or not) is decided by the Master In-band Interrupt Registry and Rules Register (MIBIRULES). To limit the maximum number of IBI bytes, configure the Read Termination field (MCTRL.RDTERM)."]
    Ack = 0,
    #[doc = "1: NACK: Not acknowledge"]
    Nack = 1,
    #[doc = "2: ACK_WITH_MANDATORY: Acknowledge with mandatory byte (ignores the MIBIRULES register). Acknowledge with mandatory byte should not be used, unless only slaves with a mandatory byte can cause an In-Band Interrupt."]
    AckWithMandatory = 2,
    #[doc = "3: MANUAL: stop and wait for a decision using the IBIAckNack request"]
    Manual = 3,
}
impl From<Ibiresp> for u8 {
    #[inline(always)]
    fn from(variant: Ibiresp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ibiresp {
    type Ux = u8;
}
impl crate::IsEnum for Ibiresp {}
#[doc = "Field `IBIRESP` reader - In-Band Interrupt (IBI) response"]
pub type IbirespR = crate::FieldReader<Ibiresp>;
impl IbirespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibiresp {
        match self.bits {
            0 => Ibiresp::Ack,
            1 => Ibiresp::Nack,
            2 => Ibiresp::AckWithMandatory,
            3 => Ibiresp::Manual,
            _ => unreachable!(),
        }
    }
    #[doc = "ACK: Acknowledge. A mandatory byte (or not) is decided by the Master In-band Interrupt Registry and Rules Register (MIBIRULES). To limit the maximum number of IBI bytes, configure the Read Termination field (MCTRL.RDTERM)."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Ibiresp::Ack
    }
    #[doc = "NACK: Not acknowledge"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Ibiresp::Nack
    }
    #[doc = "ACK_WITH_MANDATORY: Acknowledge with mandatory byte (ignores the MIBIRULES register). Acknowledge with mandatory byte should not be used, unless only slaves with a mandatory byte can cause an In-Band Interrupt."]
    #[inline(always)]
    pub fn is_ack_with_mandatory(&self) -> bool {
        *self == Ibiresp::AckWithMandatory
    }
    #[doc = "MANUAL: stop and wait for a decision using the IBIAckNack request"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Ibiresp::Manual
    }
}
#[doc = "Field `IBIRESP` writer - In-Band Interrupt (IBI) response"]
pub type IbirespW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ibiresp, crate::Safe>;
impl<'a, REG> IbirespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACK: Acknowledge. A mandatory byte (or not) is decided by the Master In-band Interrupt Registry and Rules Register (MIBIRULES). To limit the maximum number of IBI bytes, configure the Read Termination field (MCTRL.RDTERM)."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Ibiresp::Ack)
    }
    #[doc = "NACK: Not acknowledge"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Ibiresp::Nack)
    }
    #[doc = "ACK_WITH_MANDATORY: Acknowledge with mandatory byte (ignores the MIBIRULES register). Acknowledge with mandatory byte should not be used, unless only slaves with a mandatory byte can cause an In-Band Interrupt."]
    #[inline(always)]
    pub fn ack_with_mandatory(self) -> &'a mut crate::W<REG> {
        self.variant(Ibiresp::AckWithMandatory)
    }
    #[doc = "MANUAL: stop and wait for a decision using the IBIAckNack request"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Ibiresp::Manual)
    }
}
#[doc = "DIR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: DIRWRITE: Write"]
    Dirwrite = 0,
    #[doc = "1: DIRREAD: Read"]
    Dirread = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - DIR"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Dirwrite,
            true => Dir::Dirread,
        }
    }
    #[doc = "DIRWRITE: Write"]
    #[inline(always)]
    pub fn is_dirwrite(&self) -> bool {
        *self == Dir::Dirwrite
    }
    #[doc = "DIRREAD: Read"]
    #[inline(always)]
    pub fn is_dirread(&self) -> bool {
        *self == Dir::Dirread
    }
}
#[doc = "Field `DIR` writer - DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DIRWRITE: Write"]
    #[inline(always)]
    pub fn dirwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Dirwrite)
    }
    #[doc = "DIRREAD: Read"]
    #[inline(always)]
    pub fn dirread(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Dirread)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - ADDR"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RDTERM` reader - Read terminate"]
pub type RdtermR = crate::FieldReader;
#[doc = "Field `RDTERM` writer - Read terminate"]
pub type RdtermW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Request"]
    #[inline(always)]
    pub fn request(&self) -> RequestR {
        RequestR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Bus type with START"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) response"]
    #[inline(always)]
    pub fn ibiresp(&self) -> IbirespR {
        IbirespR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Read terminate"]
    #[inline(always)]
    pub fn rdterm(&self) -> RdtermR {
        RdtermR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("request", &self.request())
            .field("type_", &self.type_())
            .field("ibiresp", &self.ibiresp())
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("rdterm", &self.rdterm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Request"]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> RequestW<MctrlSpec> {
        RequestW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Bus type with START"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<MctrlSpec> {
        TypeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) response"]
    #[inline(always)]
    #[must_use]
    pub fn ibiresp(&mut self) -> IbirespW<MctrlSpec> {
        IbirespW::new(self, 6)
    }
    #[doc = "Bit 8 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<MctrlSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bits 9:15 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<MctrlSpec> {
        AddrW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Read terminate"]
    #[inline(always)]
    #[must_use]
    pub fn rdterm(&mut self) -> RdtermW<MctrlSpec> {
        RdtermW::new(self, 16)
    }
}
#[doc = "Master Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0;
}
