#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "USART Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    Disabled = 0,
    #[doc = "1: Enabled. The USART is enabled for operation."]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - USART Enable."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - USART Enable."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "Selects the data size for the USART.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datalen {
    #[doc = "0: 7 bit Data length."]
    Bit7 = 0,
    #[doc = "1: 8 bit Data length."]
    Bit8 = 1,
    #[doc = "2: 9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    Bit9 = 2,
}
impl From<Datalen> for u8 {
    #[inline(always)]
    fn from(variant: Datalen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datalen {
    type Ux = u8;
}
impl crate::IsEnum for Datalen {}
#[doc = "Field `DATALEN` reader - Selects the data size for the USART."]
pub type DatalenR = crate::FieldReader<Datalen>;
impl DatalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datalen> {
        match self.bits {
            0 => Some(Datalen::Bit7),
            1 => Some(Datalen::Bit8),
            2 => Some(Datalen::Bit9),
            _ => None,
        }
    }
    #[doc = "7 bit Data length."]
    #[inline(always)]
    pub fn is_bit_7(&self) -> bool {
        *self == Datalen::Bit7
    }
    #[doc = "8 bit Data length."]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == Datalen::Bit8
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    #[inline(always)]
    pub fn is_bit_9(&self) -> bool {
        *self == Datalen::Bit9
    }
}
#[doc = "Field `DATALEN` writer - Selects the data size for the USART."]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datalen>;
impl<'a, REG> DatalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "7 bit Data length."]
    #[inline(always)]
    pub fn bit_7(self) -> &'a mut crate::W<REG> {
        self.variant(Datalen::Bit7)
    }
    #[doc = "8 bit Data length."]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Datalen::Bit8)
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    #[inline(always)]
    pub fn bit_9(self) -> &'a mut crate::W<REG> {
        self.variant(Datalen::Bit9)
    }
}
#[doc = "Selects what type of parity is used by the USART.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Paritysel {
    #[doc = "0: No parity."]
    NoParity = 0,
    #[doc = "2: Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EvenParity = 2,
    #[doc = "3: Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    OddParity = 3,
}
impl From<Paritysel> for u8 {
    #[inline(always)]
    fn from(variant: Paritysel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Paritysel {
    type Ux = u8;
}
impl crate::IsEnum for Paritysel {}
#[doc = "Field `PARITYSEL` reader - Selects what type of parity is used by the USART."]
pub type ParityselR = crate::FieldReader<Paritysel>;
impl ParityselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Paritysel> {
        match self.bits {
            0 => Some(Paritysel::NoParity),
            2 => Some(Paritysel::EvenParity),
            3 => Some(Paritysel::OddParity),
            _ => None,
        }
    }
    #[doc = "No parity."]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == Paritysel::NoParity
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == Paritysel::EvenParity
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == Paritysel::OddParity
    }
}
#[doc = "Field `PARITYSEL` writer - Selects what type of parity is used by the USART."]
pub type ParityselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Paritysel>;
impl<'a, REG> ParityselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No parity."]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Paritysel::NoParity)
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Paritysel::EvenParity)
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Paritysel::OddParity)
    }
}
#[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stoplen {
    #[doc = "0: 1 stop bit."]
    Bit1 = 0,
    #[doc = "1: 2 stop bits. This setting should only be used for asynchronous communication."]
    Bits2 = 1,
}
impl From<Stoplen> for bool {
    #[inline(always)]
    fn from(variant: Stoplen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPLEN` reader - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
pub type StoplenR = crate::BitReader<Stoplen>;
impl StoplenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stoplen {
        match self.bits {
            false => Stoplen::Bit1,
            true => Stoplen::Bits2,
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_bit_1(&self) -> bool {
        *self == Stoplen::Bit1
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline(always)]
    pub fn is_bits_2(&self) -> bool {
        *self == Stoplen::Bits2
    }
}
#[doc = "Field `STOPLEN` writer - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
pub type StoplenW<'a, REG> = crate::BitWriter<'a, REG, Stoplen>;
impl<'a, REG> StoplenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn bit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stoplen::Bit1)
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline(always)]
    pub fn bits_2(self) -> &'a mut crate::W<REG> {
        self.variant(Stoplen::Bits2)
    }
}
#[doc = "Selects standard or 32 kHz clocking mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode32k {
    #[doc = "0: Disabled. USART uses standard clocking."]
    Disabled = 0,
    #[doc = "1: Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    Enabled = 1,
}
impl From<Mode32k> for bool {
    #[inline(always)]
    fn from(variant: Mode32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE32K` reader - Selects standard or 32 kHz clocking mode."]
pub type Mode32kR = crate::BitReader<Mode32k>;
impl Mode32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode32k {
        match self.bits {
            false => Mode32k::Disabled,
            true => Mode32k::Enabled,
        }
    }
    #[doc = "Disabled. USART uses standard clocking."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode32k::Disabled
    }
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mode32k::Enabled
    }
}
#[doc = "Field `MODE32K` writer - Selects standard or 32 kHz clocking mode."]
pub type Mode32kW<'a, REG> = crate::BitWriter<'a, REG, Mode32k>;
impl<'a, REG> Mode32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. USART uses standard clocking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode32k::Disabled)
    }
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode32k::Enabled)
    }
}
#[doc = "LIN break mode enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linmode {
    #[doc = "0: Disabled. Break detect and generate is configured for normal operation."]
    Disabled = 0,
    #[doc = "1: Enabled. Break detect and generate is configured for LIN bus operation."]
    Enabled = 1,
}
impl From<Linmode> for bool {
    #[inline(always)]
    fn from(variant: Linmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINMODE` reader - LIN break mode enable."]
pub type LinmodeR = crate::BitReader<Linmode>;
impl LinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linmode {
        match self.bits {
            false => Linmode::Disabled,
            true => Linmode::Enabled,
        }
    }
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Linmode::Disabled
    }
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Linmode::Enabled
    }
}
#[doc = "Field `LINMODE` writer - LIN break mode enable."]
pub type LinmodeW<'a, REG> = crate::BitWriter<'a, REG, Linmode>;
impl<'a, REG> LinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Linmode::Disabled)
    }
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Linmode::Enabled)
    }
}
#[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: No flow control. The transmitter does not receive any automatic flow control signal."]
    Disabled = 0,
    #[doc = "1: Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    Enabled = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disabled,
            true => Ctsen::Enabled,
        }
    }
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctsen::Disabled
    }
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctsen::Enabled
    }
}
#[doc = "Field `CTSEN` writer - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disabled)
    }
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enabled)
    }
}
#[doc = "Selects synchronous or asynchronous operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncen {
    #[doc = "0: Asynchronous mode."]
    AsynchronousMode = 0,
    #[doc = "1: Synchronous mode."]
    SynchronousMode = 1,
}
impl From<Syncen> for bool {
    #[inline(always)]
    fn from(variant: Syncen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN` reader - Selects synchronous or asynchronous operation."]
pub type SyncenR = crate::BitReader<Syncen>;
impl SyncenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncen {
        match self.bits {
            false => Syncen::AsynchronousMode,
            true => Syncen::SynchronousMode,
        }
    }
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == Syncen::AsynchronousMode
    }
    #[doc = "Synchronous mode."]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == Syncen::SynchronousMode
    }
}
#[doc = "Field `SYNCEN` writer - Selects synchronous or asynchronous operation."]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG, Syncen>;
impl<'a, REG> SyncenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Syncen::AsynchronousMode)
    }
    #[doc = "Synchronous mode."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Syncen::SynchronousMode)
    }
}
#[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpol {
    #[doc = "0: Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FallingEdge = 0,
    #[doc = "1: Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RisingEdge = 1,
}
impl From<Clkpol> for bool {
    #[inline(always)]
    fn from(variant: Clkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - Selects the clock polarity and sampling edge of received data in synchronous mode."]
pub type ClkpolR = crate::BitReader<Clkpol>;
impl ClkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpol {
        match self.bits {
            false => Clkpol::FallingEdge,
            true => Clkpol::RisingEdge,
        }
    }
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Clkpol::FallingEdge
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Clkpol::RisingEdge
    }
}
#[doc = "Field `CLKPOL` writer - Selects the clock polarity and sampling edge of received data in synchronous mode."]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG, Clkpol>;
impl<'a, REG> ClkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::FallingEdge)
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::RisingEdge)
    }
}
#[doc = "Synchronous mode Master select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncmst {
    #[doc = "0: Slave. When synchronous mode is enabled, the USART is a slave."]
    Slave = 0,
    #[doc = "1: Master. When synchronous mode is enabled, the USART is a master."]
    Master = 1,
}
impl From<Syncmst> for bool {
    #[inline(always)]
    fn from(variant: Syncmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCMST` reader - Synchronous mode Master select."]
pub type SyncmstR = crate::BitReader<Syncmst>;
impl SyncmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncmst {
        match self.bits {
            false => Syncmst::Slave,
            true => Syncmst::Master,
        }
    }
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Syncmst::Slave
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Syncmst::Master
    }
}
#[doc = "Field `SYNCMST` writer - Synchronous mode Master select."]
pub type SyncmstW<'a, REG> = crate::BitWriter<'a, REG, Syncmst>;
impl<'a, REG> SyncmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Syncmst::Slave)
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Syncmst::Master)
    }
}
#[doc = "Selects data loopback mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loop {
    #[doc = "0: Normal operation."]
    Normal = 0,
    #[doc = "1: Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    Loopback = 1,
}
impl From<Loop> for bool {
    #[inline(always)]
    fn from(variant: Loop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOP` reader - Selects data loopback mode."]
pub type LoopR = crate::BitReader<Loop>;
impl LoopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loop {
        match self.bits {
            false => Loop::Normal,
            true => Loop::Loopback,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Loop::Normal
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == Loop::Loopback
    }
}
#[doc = "Field `LOOP` writer - Selects data loopback mode."]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG, Loop>;
impl<'a, REG> LoopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Normal)
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Loopback)
    }
}
#[doc = "Output Enable Turnaround time enable for RS-485 operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oeta {
    #[doc = "0: Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    Disabled = 0,
    #[doc = "1: Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    Enabled = 1,
}
impl From<Oeta> for bool {
    #[inline(always)]
    fn from(variant: Oeta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OETA` reader - Output Enable Turnaround time enable for RS-485 operation."]
pub type OetaR = crate::BitReader<Oeta>;
impl OetaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oeta {
        match self.bits {
            false => Oeta::Disabled,
            true => Oeta::Enabled,
        }
    }
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Oeta::Disabled
    }
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Oeta::Enabled
    }
}
#[doc = "Field `OETA` writer - Output Enable Turnaround time enable for RS-485 operation."]
pub type OetaW<'a, REG> = crate::BitWriter<'a, REG, Oeta>;
impl<'a, REG> OetaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oeta::Disabled)
    }
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oeta::Enabled)
    }
}
#[doc = "Automatic Address matching enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoaddr {
    #[doc = "0: Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    Disabled = 0,
    #[doc = "1: Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    Enabled = 1,
}
impl From<Autoaddr> for bool {
    #[inline(always)]
    fn from(variant: Autoaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOADDR` reader - Automatic Address matching enable."]
pub type AutoaddrR = crate::BitReader<Autoaddr>;
impl AutoaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoaddr {
        match self.bits {
            false => Autoaddr::Disabled,
            true => Autoaddr::Enabled,
        }
    }
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Autoaddr::Disabled
    }
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Autoaddr::Enabled
    }
}
#[doc = "Field `AUTOADDR` writer - Automatic Address matching enable."]
pub type AutoaddrW<'a, REG> = crate::BitWriter<'a, REG, Autoaddr>;
impl<'a, REG> AutoaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autoaddr::Disabled)
    }
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autoaddr::Enabled)
    }
}
#[doc = "Output Enable Select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oesel {
    #[doc = "0: Standard. The RTS signal is used as the standard flow control function."]
    Standard = 0,
    #[doc = "1: RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    Rs485 = 1,
}
impl From<Oesel> for bool {
    #[inline(always)]
    fn from(variant: Oesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OESEL` reader - Output Enable Select."]
pub type OeselR = crate::BitReader<Oesel>;
impl OeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oesel {
        match self.bits {
            false => Oesel::Standard,
            true => Oesel::Rs485,
        }
    }
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Oesel::Standard
    }
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    #[inline(always)]
    pub fn is_rs_485(&self) -> bool {
        *self == Oesel::Rs485
    }
}
#[doc = "Field `OESEL` writer - Output Enable Select."]
pub type OeselW<'a, REG> = crate::BitWriter<'a, REG, Oesel>;
impl<'a, REG> OeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Oesel::Standard)
    }
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    #[inline(always)]
    pub fn rs_485(self) -> &'a mut crate::W<REG> {
        self.variant(Oesel::Rs485)
    }
}
#[doc = "Output Enable Polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oepol {
    #[doc = "0: Low. If selected by OESEL, the output enable is active low."]
    Low = 0,
    #[doc = "1: High. If selected by OESEL, the output enable is active high."]
    High = 1,
}
impl From<Oepol> for bool {
    #[inline(always)]
    fn from(variant: Oepol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEPOL` reader - Output Enable Polarity."]
pub type OepolR = crate::BitReader<Oepol>;
impl OepolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oepol {
        match self.bits {
            false => Oepol::Low,
            true => Oepol::High,
        }
    }
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Oepol::Low
    }
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Oepol::High
    }
}
#[doc = "Field `OEPOL` writer - Output Enable Polarity."]
pub type OepolW<'a, REG> = crate::BitWriter<'a, REG, Oepol>;
impl<'a, REG> OepolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Oepol::Low)
    }
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Oepol::High)
    }
}
#[doc = "Receive data polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxpol {
    #[doc = "0: Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    Standard = 0,
    #[doc = "1: Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    Inverted = 1,
}
impl From<Rxpol> for bool {
    #[inline(always)]
    fn from(variant: Rxpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPOL` reader - Receive data polarity."]
pub type RxpolR = crate::BitReader<Rxpol>;
impl RxpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpol {
        match self.bits {
            false => Rxpol::Standard,
            true => Rxpol::Inverted,
        }
    }
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Rxpol::Standard
    }
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Rxpol::Inverted
    }
}
#[doc = "Field `RXPOL` writer - Receive data polarity."]
pub type RxpolW<'a, REG> = crate::BitWriter<'a, REG, Rxpol>;
impl<'a, REG> RxpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpol::Standard)
    }
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpol::Inverted)
    }
}
#[doc = "Transmit data polarity.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txpol {
    #[doc = "0: Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    Standard = 0,
    #[doc = "1: Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    Inverted = 1,
}
impl From<Txpol> for bool {
    #[inline(always)]
    fn from(variant: Txpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPOL` reader - Transmit data polarity."]
pub type TxpolR = crate::BitReader<Txpol>;
impl TxpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txpol {
        match self.bits {
            false => Txpol::Standard,
            true => Txpol::Inverted,
        }
    }
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Txpol::Standard
    }
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Txpol::Inverted
    }
}
#[doc = "Field `TXPOL` writer - Transmit data polarity."]
pub type TxpolW<'a, REG> = crate::BitWriter<'a, REG, Txpol>;
impl<'a, REG> TxpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Txpol::Standard)
    }
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Txpol::Inverted)
    }
}
impl R {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub fn paritysel(&self) -> ParityselR {
        ParityselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub fn stoplen(&self) -> StoplenR {
        StoplenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub fn mode32k(&self) -> Mode32kR {
        Mode32kR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    pub fn linmode(&self) -> LinmodeR {
        LinmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    pub fn syncmst(&self) -> SyncmstR {
        SyncmstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub fn oeta(&self) -> OetaR {
        OetaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    pub fn autoaddr(&self) -> AutoaddrR {
        AutoaddrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    pub fn oesel(&self) -> OeselR {
        OeselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    pub fn oepol(&self) -> OepolR {
        OepolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    pub fn rxpol(&self) -> RxpolR {
        RxpolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    pub fn txpol(&self) -> TxpolR {
        TxpolR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("enable", &self.enable())
            .field("datalen", &self.datalen())
            .field("paritysel", &self.paritysel())
            .field("stoplen", &self.stoplen())
            .field("mode32k", &self.mode32k())
            .field("linmode", &self.linmode())
            .field("ctsen", &self.ctsen())
            .field("syncen", &self.syncen())
            .field("clkpol", &self.clkpol())
            .field("syncmst", &self.syncmst())
            .field("loop_", &self.loop_())
            .field("oeta", &self.oeta())
            .field("autoaddr", &self.autoaddr())
            .field("oesel", &self.oesel())
            .field("oepol", &self.oepol())
            .field("rxpol", &self.rxpol())
            .field("txpol", &self.txpol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    pub fn datalen(&mut self) -> DatalenW<CfgSpec> {
        DatalenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub fn paritysel(&mut self) -> ParityselW<CfgSpec> {
        ParityselW::new(self, 4)
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub fn stoplen(&mut self) -> StoplenW<CfgSpec> {
        StoplenW::new(self, 6)
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub fn mode32k(&mut self) -> Mode32kW<CfgSpec> {
        Mode32kW::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    pub fn linmode(&mut self) -> LinmodeW<CfgSpec> {
        LinmodeW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<CfgSpec> {
        CtsenW::new(self, 9)
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<CfgSpec> {
        SyncenW::new(self, 11)
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub fn clkpol(&mut self) -> ClkpolW<CfgSpec> {
        ClkpolW::new(self, 12)
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    pub fn syncmst(&mut self) -> SyncmstW<CfgSpec> {
        SyncmstW::new(self, 14)
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<CfgSpec> {
        LoopW::new(self, 15)
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub fn oeta(&mut self) -> OetaW<CfgSpec> {
        OetaW::new(self, 18)
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    pub fn autoaddr(&mut self) -> AutoaddrW<CfgSpec> {
        AutoaddrW::new(self, 19)
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    pub fn oesel(&mut self) -> OeselW<CfgSpec> {
        OeselW::new(self, 20)
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    pub fn oepol(&mut self) -> OepolW<CfgSpec> {
        OepolW::new(self, 21)
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    pub fn rxpol(&mut self) -> RxpolW<CfgSpec> {
        RxpolW::new(self, 22)
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    pub fn txpol(&mut self) -> TxpolW<CfgSpec> {
        TxpolW::new(self, 23)
    }
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
