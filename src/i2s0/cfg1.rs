#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Main enable for I 2S function in this Flexcomm\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mainenable {
    #[doc = "0: All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    Disabled = 0,
    #[doc = "1: This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    Enabled = 1,
}
impl From<Mainenable> for bool {
    #[inline(always)]
    fn from(variant: Mainenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAINENABLE` reader - Main enable for I 2S function in this Flexcomm"]
pub type MainenableR = crate::BitReader<Mainenable>;
impl MainenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mainenable {
        match self.bits {
            false => Mainenable::Disabled,
            true => Mainenable::Enabled,
        }
    }
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mainenable::Disabled
    }
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mainenable::Enabled
    }
}
#[doc = "Field `MAINENABLE` writer - Main enable for I 2S function in this Flexcomm"]
pub type MainenableW<'a, REG> = crate::BitWriter<'a, REG, Mainenable>;
impl<'a, REG> MainenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mainenable::Disabled)
    }
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mainenable::Enabled)
    }
}
#[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datapause {
    #[doc = "0: Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    Normal = 0,
    #[doc = "1: A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    Pause = 1,
}
impl From<Datapause> for bool {
    #[inline(always)]
    fn from(variant: Datapause) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPAUSE` reader - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
pub type DatapauseR = crate::BitReader<Datapause>;
impl DatapauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datapause {
        match self.bits {
            false => Datapause::Normal,
            true => Datapause::Pause,
        }
    }
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Datapause::Normal
    }
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == Datapause::Pause
    }
}
#[doc = "Field `DATAPAUSE` writer - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
pub type DatapauseW<'a, REG> = crate::BitWriter<'a, REG, Datapause>;
impl<'a, REG> DatapauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Datapause::Normal)
    }
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(Datapause::Pause)
    }
}
#[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Paircount {
    #[doc = "0: 1 I2S channel pairs in this flexcomm"]
    Pairs1 = 0,
    #[doc = "1: 2 I2S channel pairs in this flexcomm"]
    Pairs2 = 1,
    #[doc = "2: 3 I2S channel pairs in this flexcomm"]
    Pairs3 = 2,
    #[doc = "3: 4 I2S channel pairs in this flexcomm"]
    Pairs4 = 3,
}
impl From<Paircount> for u8 {
    #[inline(always)]
    fn from(variant: Paircount) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Paircount {
    type Ux = u8;
}
impl crate::IsEnum for Paircount {}
#[doc = "Field `PAIRCOUNT` reader - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
pub type PaircountR = crate::FieldReader<Paircount>;
impl PaircountR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paircount {
        match self.bits {
            0 => Paircount::Pairs1,
            1 => Paircount::Pairs2,
            2 => Paircount::Pairs3,
            3 => Paircount::Pairs4,
            _ => unreachable!(),
        }
    }
    #[doc = "1 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn is_pairs_1(&self) -> bool {
        *self == Paircount::Pairs1
    }
    #[doc = "2 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn is_pairs_2(&self) -> bool {
        *self == Paircount::Pairs2
    }
    #[doc = "3 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn is_pairs_3(&self) -> bool {
        *self == Paircount::Pairs3
    }
    #[doc = "4 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn is_pairs_4(&self) -> bool {
        *self == Paircount::Pairs4
    }
}
#[doc = "Field `PAIRCOUNT` writer - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
pub type PaircountW<'a, REG> = crate::FieldWriter<'a, REG, 2, Paircount, crate::Safe>;
impl<'a, REG> PaircountW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Paircount::Pairs1)
    }
    #[doc = "2 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Paircount::Pairs2)
    }
    #[doc = "3 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Paircount::Pairs3)
    }
    #[doc = "4 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Paircount::Pairs4)
    }
}
#[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mstslvcfg {
    #[doc = "0: Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NormalSlaveMode = 0,
    #[doc = "1: WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WsSyncMaster = 1,
    #[doc = "2: Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MasterUsingSck = 2,
    #[doc = "3: Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NormalMaster = 3,
}
impl From<Mstslvcfg> for u8 {
    #[inline(always)]
    fn from(variant: Mstslvcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mstslvcfg {
    type Ux = u8;
}
impl crate::IsEnum for Mstslvcfg {}
#[doc = "Field `MSTSLVCFG` reader - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
pub type MstslvcfgR = crate::FieldReader<Mstslvcfg>;
impl MstslvcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstslvcfg {
        match self.bits {
            0 => Mstslvcfg::NormalSlaveMode,
            1 => Mstslvcfg::WsSyncMaster,
            2 => Mstslvcfg::MasterUsingSck,
            3 => Mstslvcfg::NormalMaster,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    #[inline(always)]
    pub fn is_normal_slave_mode(&self) -> bool {
        *self == Mstslvcfg::NormalSlaveMode
    }
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    #[inline(always)]
    pub fn is_ws_sync_master(&self) -> bool {
        *self == Mstslvcfg::WsSyncMaster
    }
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    #[inline(always)]
    pub fn is_master_using_sck(&self) -> bool {
        *self == Mstslvcfg::MasterUsingSck
    }
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    #[inline(always)]
    pub fn is_normal_master(&self) -> bool {
        *self == Mstslvcfg::NormalMaster
    }
}
#[doc = "Field `MSTSLVCFG` writer - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
pub type MstslvcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mstslvcfg, crate::Safe>;
impl<'a, REG> MstslvcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    #[inline(always)]
    pub fn normal_slave_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Mstslvcfg::NormalSlaveMode)
    }
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    #[inline(always)]
    pub fn ws_sync_master(self) -> &'a mut crate::W<REG> {
        self.variant(Mstslvcfg::WsSyncMaster)
    }
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    #[inline(always)]
    pub fn master_using_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Mstslvcfg::MasterUsingSck)
    }
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    #[inline(always)]
    pub fn normal_master(self) -> &'a mut crate::W<REG> {
        self.variant(Mstslvcfg::NormalMaster)
    }
}
#[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    ClassicMode = 0,
    #[doc = "1: DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DspModeWs50Dutycycle = 1,
    #[doc = "2: DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DspModeWs1Clock = 2,
    #[doc = "3: DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DspModeWs1Data = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::ClassicMode,
            1 => Mode::DspModeWs50Dutycycle,
            2 => Mode::DspModeWs1Clock,
            3 => Mode::DspModeWs1Data,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    #[inline(always)]
    pub fn is_classic_mode(&self) -> bool {
        *self == Mode::ClassicMode
    }
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    #[inline(always)]
    pub fn is_dsp_mode_ws_50_dutycycle(&self) -> bool {
        *self == Mode::DspModeWs50Dutycycle
    }
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_clock(&self) -> bool {
        *self == Mode::DspModeWs1Clock
    }
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_data(&self) -> bool {
        *self == Mode::DspModeWs1Data
    }
}
#[doc = "Field `MODE` writer - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    #[inline(always)]
    pub fn classic_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ClassicMode)
    }
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    #[inline(always)]
    pub fn dsp_mode_ws_50_dutycycle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::DspModeWs50Dutycycle)
    }
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn dsp_mode_ws_1_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::DspModeWs1Clock)
    }
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn dsp_mode_ws_1_data(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::DspModeWs1Data)
    }
}
#[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rightlow {
    #[doc = "0: The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RightHigh = 0,
    #[doc = "1: The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RightLow = 1,
}
impl From<Rightlow> for bool {
    #[inline(always)]
    fn from(variant: Rightlow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIGHTLOW` reader - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
pub type RightlowR = crate::BitReader<Rightlow>;
impl RightlowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rightlow {
        match self.bits {
            false => Rightlow::RightHigh,
            true => Rightlow::RightLow,
        }
    }
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    #[inline(always)]
    pub fn is_right_high(&self) -> bool {
        *self == Rightlow::RightHigh
    }
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    #[inline(always)]
    pub fn is_right_low(&self) -> bool {
        *self == Rightlow::RightLow
    }
}
#[doc = "Field `RIGHTLOW` writer - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
pub type RightlowW<'a, REG> = crate::BitWriter<'a, REG, Rightlow>;
impl<'a, REG> RightlowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    #[inline(always)]
    pub fn right_high(self) -> &'a mut crate::W<REG> {
        self.variant(Rightlow::RightHigh)
    }
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    #[inline(always)]
    pub fn right_low(self) -> &'a mut crate::W<REG> {
        self.variant(Rightlow::RightLow)
    }
}
#[doc = "Left Justify data.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leftjust {
    #[doc = "0: Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RightJustified = 0,
    #[doc = "1: Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LeftJustified = 1,
}
impl From<Leftjust> for bool {
    #[inline(always)]
    fn from(variant: Leftjust) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEFTJUST` reader - Left Justify data."]
pub type LeftjustR = crate::BitReader<Leftjust>;
impl LeftjustR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leftjust {
        match self.bits {
            false => Leftjust::RightJustified,
            true => Leftjust::LeftJustified,
        }
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn is_right_justified(&self) -> bool {
        *self == Leftjust::RightJustified
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn is_left_justified(&self) -> bool {
        *self == Leftjust::LeftJustified
    }
}
#[doc = "Field `LEFTJUST` writer - Left Justify data."]
pub type LeftjustW<'a, REG> = crate::BitWriter<'a, REG, Leftjust>;
impl<'a, REG> LeftjustW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn right_justified(self) -> &'a mut crate::W<REG> {
        self.variant(Leftjust::RightJustified)
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn left_justified(self) -> &'a mut crate::W<REG> {
        self.variant(Leftjust::LeftJustified)
    }
}
#[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onechannel {
    #[doc = "0: I2S data for this channel pair is treated as left and right channels."]
    DualChannel = 0,
    #[doc = "1: I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SingleChannel = 1,
}
impl From<Onechannel> for bool {
    #[inline(always)]
    fn from(variant: Onechannel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONECHANNEL` reader - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
pub type OnechannelR = crate::BitReader<Onechannel>;
impl OnechannelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onechannel {
        match self.bits {
            false => Onechannel::DualChannel,
            true => Onechannel::SingleChannel,
        }
    }
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    #[inline(always)]
    pub fn is_dual_channel(&self) -> bool {
        *self == Onechannel::DualChannel
    }
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == Onechannel::SingleChannel
    }
}
#[doc = "Field `ONECHANNEL` writer - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
pub type OnechannelW<'a, REG> = crate::BitWriter<'a, REG, Onechannel>;
impl<'a, REG> OnechannelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    #[inline(always)]
    pub fn dual_channel(self) -> &'a mut crate::W<REG> {
        self.variant(Onechannel::DualChannel)
    }
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(Onechannel::SingleChannel)
    }
}
#[doc = "PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdmdata {
    #[doc = "0: Normal operation, data is transferred to or from the Flexcomm FIFO."]
    Normal = 0,
    #[doc = "1: The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    DmicSubsystem = 1,
}
impl From<Pdmdata> for bool {
    #[inline(always)]
    fn from(variant: Pdmdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMDATA` reader - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
pub type PdmdataR = crate::BitReader<Pdmdata>;
impl PdmdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdmdata {
        match self.bits {
            false => Pdmdata::Normal,
            true => Pdmdata::DmicSubsystem,
        }
    }
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Pdmdata::Normal
    }
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    #[inline(always)]
    pub fn is_dmic_subsystem(&self) -> bool {
        *self == Pdmdata::DmicSubsystem
    }
}
#[doc = "Field `PDMDATA` writer - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
pub type PdmdataW<'a, REG> = crate::BitWriter<'a, REG, Pdmdata>;
impl<'a, REG> PdmdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Pdmdata::Normal)
    }
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    #[inline(always)]
    pub fn dmic_subsystem(self) -> &'a mut crate::W<REG> {
        self.variant(Pdmdata::DmicSubsystem)
    }
}
#[doc = "SCK polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SckPol {
    #[doc = "0: Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FallingEdge = 0,
    #[doc = "1: Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RisingEdge = 1,
}
impl From<SckPol> for bool {
    #[inline(always)]
    fn from(variant: SckPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCK_POL` reader - SCK polarity."]
pub type SckPolR = crate::BitReader<SckPol>;
impl SckPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SckPol {
        match self.bits {
            false => SckPol::FallingEdge,
            true => SckPol::RisingEdge,
        }
    }
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SckPol::FallingEdge
    }
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SckPol::RisingEdge
    }
}
#[doc = "Field `SCK_POL` writer - SCK polarity."]
pub type SckPolW<'a, REG> = crate::BitWriter<'a, REG, SckPol>;
impl<'a, REG> SckPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SckPol::FallingEdge)
    }
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SckPol::RisingEdge)
    }
}
#[doc = "WS polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WsPol {
    #[doc = "0: Data frames begin at a falling edge of WS (standard for classic I2S)."]
    NotInverted = 0,
    #[doc = "1: WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    Inverted = 1,
}
impl From<WsPol> for bool {
    #[inline(always)]
    fn from(variant: WsPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WS_POL` reader - WS polarity."]
pub type WsPolR = crate::BitReader<WsPol>;
impl WsPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WsPol {
        match self.bits {
            false => WsPol::NotInverted,
            true => WsPol::Inverted,
        }
    }
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == WsPol::NotInverted
    }
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == WsPol::Inverted
    }
}
#[doc = "Field `WS_POL` writer - WS polarity."]
pub type WsPolW<'a, REG> = crate::BitWriter<'a, REG, WsPol>;
impl<'a, REG> WsPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(WsPol::NotInverted)
    }
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(WsPol::Inverted)
    }
}
#[doc = "Field `DATALEN` reader - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
pub type DatalenR = crate::FieldReader;
#[doc = "Field `DATALEN` writer - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline(always)]
    pub fn mainenable(&self) -> MainenableR {
        MainenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub fn datapause(&self) -> DatapauseR {
        DatapauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn paircount(&self) -> PaircountR {
        PaircountR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn mstslvcfg(&self) -> MstslvcfgR {
        MstslvcfgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub fn rightlow(&self) -> RightlowR {
        RightlowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline(always)]
    pub fn leftjust(&self) -> LeftjustR {
        LeftjustR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub fn onechannel(&self) -> OnechannelR {
        OnechannelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
    #[inline(always)]
    pub fn pdmdata(&self) -> PdmdataR {
        PdmdataR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline(always)]
    pub fn sck_pol(&self) -> SckPolR {
        SckPolR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline(always)]
    pub fn ws_pol(&self) -> WsPolR {
        WsPolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("mainenable", &self.mainenable())
            .field("datapause", &self.datapause())
            .field("paircount", &self.paircount())
            .field("mstslvcfg", &self.mstslvcfg())
            .field("mode", &self.mode())
            .field("rightlow", &self.rightlow())
            .field("leftjust", &self.leftjust())
            .field("onechannel", &self.onechannel())
            .field("pdmdata", &self.pdmdata())
            .field("sck_pol", &self.sck_pol())
            .field("ws_pol", &self.ws_pol())
            .field("datalen", &self.datalen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline(always)]
    pub fn mainenable(&mut self) -> MainenableW<Cfg1Spec> {
        MainenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub fn datapause(&mut self) -> DatapauseW<Cfg1Spec> {
        DatapauseW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn paircount(&mut self) -> PaircountW<Cfg1Spec> {
        PaircountW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn mstslvcfg(&mut self) -> MstslvcfgW<Cfg1Spec> {
        MstslvcfgW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<Cfg1Spec> {
        ModeW::new(self, 6)
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub fn rightlow(&mut self) -> RightlowW<Cfg1Spec> {
        RightlowW::new(self, 8)
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline(always)]
    pub fn leftjust(&mut self) -> LeftjustW<Cfg1Spec> {
        LeftjustW::new(self, 9)
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> OnechannelW<Cfg1Spec> {
        OnechannelW::new(self, 10)
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
    #[inline(always)]
    pub fn pdmdata(&mut self) -> PdmdataW<Cfg1Spec> {
        PdmdataW::new(self, 11)
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline(always)]
    pub fn sck_pol(&mut self) -> SckPolW<Cfg1Spec> {
        SckPolW::new(self, 12)
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline(always)]
    pub fn ws_pol(&mut self) -> WsPolW<Cfg1Spec> {
        WsPolW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DatalenW<Cfg1Spec> {
        DatalenW::new(self, 16)
    }
}
#[doc = "Configuration register 1 for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
