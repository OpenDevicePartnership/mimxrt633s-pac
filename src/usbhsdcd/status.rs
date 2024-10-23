#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Charger Detection Sequence Results\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SeqRes {
    #[doc = "0: No results to report."]
    NoResult = 0,
    #[doc = "1: Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    ConnSdp = 1,
    #[doc = "2: Attached to a charging port. The exact meaning depends on bit 18 (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)"]
    ConnCp = 2,
    #[doc = "3: Attached to a DCP."]
    ConnDcp = 3,
}
impl From<SeqRes> for u8 {
    #[inline(always)]
    fn from(variant: SeqRes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SeqRes {
    type Ux = u8;
}
impl crate::IsEnum for SeqRes {}
#[doc = "Field `SEQ_RES` reader - Charger Detection Sequence Results"]
pub type SeqResR = crate::FieldReader<SeqRes>;
impl SeqResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SeqRes {
        match self.bits {
            0 => SeqRes::NoResult,
            1 => SeqRes::ConnSdp,
            2 => SeqRes::ConnCp,
            3 => SeqRes::ConnDcp,
            _ => unreachable!(),
        }
    }
    #[doc = "No results to report."]
    #[inline(always)]
    pub fn is_no_result(&self) -> bool {
        *self == SeqRes::NoResult
    }
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    #[inline(always)]
    pub fn is_conn_sdp(&self) -> bool {
        *self == SeqRes::ConnSdp
    }
    #[doc = "Attached to a charging port. The exact meaning depends on bit 18 (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)"]
    #[inline(always)]
    pub fn is_conn_cp(&self) -> bool {
        *self == SeqRes::ConnCp
    }
    #[doc = "Attached to a DCP."]
    #[inline(always)]
    pub fn is_conn_dcp(&self) -> bool {
        *self == SeqRes::ConnDcp
    }
}
#[doc = "Charger Detection Sequence Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SeqStat {
    #[doc = "0: The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    NoDataPinConn = 0,
    #[doc = "1: Data pin contact detection is complete."]
    DataPinConn = 1,
    #[doc = "2: Charging port detection is complete."]
    CpDetDone = 2,
    #[doc = "3: Charger type detection is complete."]
    CtDetDone = 3,
}
impl From<SeqStat> for u8 {
    #[inline(always)]
    fn from(variant: SeqStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SeqStat {
    type Ux = u8;
}
impl crate::IsEnum for SeqStat {}
#[doc = "Field `SEQ_STAT` reader - Charger Detection Sequence Status"]
pub type SeqStatR = crate::FieldReader<SeqStat>;
impl SeqStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SeqStat {
        match self.bits {
            0 => SeqStat::NoDataPinConn,
            1 => SeqStat::DataPinConn,
            2 => SeqStat::CpDetDone,
            3 => SeqStat::CtDetDone,
            _ => unreachable!(),
        }
    }
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    #[inline(always)]
    pub fn is_no_data_pin_conn(&self) -> bool {
        *self == SeqStat::NoDataPinConn
    }
    #[doc = "Data pin contact detection is complete."]
    #[inline(always)]
    pub fn is_data_pin_conn(&self) -> bool {
        *self == SeqStat::DataPinConn
    }
    #[doc = "Charging port detection is complete."]
    #[inline(always)]
    pub fn is_cp_det_done(&self) -> bool {
        *self == SeqStat::CpDetDone
    }
    #[doc = "Charger type detection is complete."]
    #[inline(always)]
    pub fn is_ct_det_done(&self) -> bool {
        *self == SeqStat::CtDetDone
    }
}
#[doc = "Error Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err {
    #[doc = "0: No sequence errors."]
    NoSeqErr = 0,
    #[doc = "1: Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred."]
    SeqErr = 1,
}
impl From<Err> for bool {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Error Flag"]
pub type ErrR = crate::BitReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err {
        match self.bits {
            false => Err::NoSeqErr,
            true => Err::SeqErr,
        }
    }
    #[doc = "No sequence errors."]
    #[inline(always)]
    pub fn is_no_seq_err(&self) -> bool {
        *self == Err::NoSeqErr
    }
    #[doc = "Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred."]
    #[inline(always)]
    pub fn is_seq_err(&self) -> bool {
        *self == Err::SeqErr
    }
}
#[doc = "Timeout Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum To {
    #[doc = "0: The detection sequence has not been running for over 1 s."]
    NoTimeout = 0,
    #[doc = "1: It has been over 1 s since the data pin contact was detected and debounced."]
    Timeout = 1,
}
impl From<To> for bool {
    #[inline(always)]
    fn from(variant: To) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO` reader - Timeout Flag"]
pub type ToR = crate::BitReader<To>;
impl ToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> To {
        match self.bits {
            false => To::NoTimeout,
            true => To::Timeout,
        }
    }
    #[doc = "The detection sequence has not been running for over 1 s."]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == To::NoTimeout
    }
    #[doc = "It has been over 1 s since the data pin contact was detected and debounced."]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == To::Timeout
    }
}
#[doc = "Active Status Indicator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: The sequence is not running."]
    SeqNotRunning = 0,
    #[doc = "1: The sequence is running."]
    SeqRunning = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Active Status Indicator"]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::SeqNotRunning,
            true => Active::SeqRunning,
        }
    }
    #[doc = "The sequence is not running."]
    #[inline(always)]
    pub fn is_seq_not_running(&self) -> bool {
        *self == Active::SeqNotRunning
    }
    #[doc = "The sequence is running."]
    #[inline(always)]
    pub fn is_seq_running(&self) -> bool {
        *self == Active::SeqRunning
    }
}
impl R {
    #[doc = "Bits 16:17 - Charger Detection Sequence Results"]
    #[inline(always)]
    pub fn seq_res(&self) -> SeqResR {
        SeqResR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Charger Detection Sequence Status"]
    #[inline(always)]
    pub fn seq_stat(&self) -> SeqStatR {
        SeqStatR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Error Flag"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timeout Flag"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Active Status Indicator"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("seq_res", &self.seq_res())
            .field("seq_stat", &self.seq_stat())
            .field("err", &self.err())
            .field("to", &self.to())
            .field("active", &self.active())
            .finish()
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
