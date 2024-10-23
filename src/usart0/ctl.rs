#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Break Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txbrken {
    #[doc = "0: Normal operation."]
    Normal = 0,
    #[doc = "1: Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    Continous = 1,
}
impl From<Txbrken> for bool {
    #[inline(always)]
    fn from(variant: Txbrken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBRKEN` reader - Break Enable."]
pub type TxbrkenR = crate::BitReader<Txbrken>;
impl TxbrkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txbrken {
        match self.bits {
            false => Txbrken::Normal,
            true => Txbrken::Continous,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Txbrken::Normal
    }
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        *self == Txbrken::Continous
    }
}
#[doc = "Field `TXBRKEN` writer - Break Enable."]
pub type TxbrkenW<'a, REG> = crate::BitWriter<'a, REG, Txbrken>;
impl<'a, REG> TxbrkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Txbrken::Normal)
    }
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline(always)]
    pub fn continous(self) -> &'a mut crate::W<REG> {
        self.variant(Txbrken::Continous)
    }
}
#[doc = "Enable address detect mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrdet {
    #[doc = "0: Disabled. The USART presents all incoming data."]
    Disabled = 0,
    #[doc = "1: Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    Enabled = 1,
}
impl From<Addrdet> for bool {
    #[inline(always)]
    fn from(variant: Addrdet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRDET` reader - Enable address detect mode."]
pub type AddrdetR = crate::BitReader<Addrdet>;
impl AddrdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrdet {
        match self.bits {
            false => Addrdet::Disabled,
            true => Addrdet::Enabled,
        }
    }
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Addrdet::Disabled
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Addrdet::Enabled
    }
}
#[doc = "Field `ADDRDET` writer - Enable address detect mode."]
pub type AddrdetW<'a, REG> = crate::BitWriter<'a, REG, Addrdet>;
impl<'a, REG> AddrdetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addrdet::Disabled)
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Addrdet::Enabled)
    }
}
#[doc = "Transmit Disable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdis {
    #[doc = "0: Not disabled. USART transmitter is not disabled."]
    Enabled = 0,
    #[doc = "1: Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    Disabled = 1,
}
impl From<Txdis> for bool {
    #[inline(always)]
    fn from(variant: Txdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIS` reader - Transmit Disable."]
pub type TxdisR = crate::BitReader<Txdis>;
impl TxdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdis {
        match self.bits {
            false => Txdis::Enabled,
            true => Txdis::Disabled,
        }
    }
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdis::Enabled
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdis::Disabled
    }
}
#[doc = "Field `TXDIS` writer - Transmit Disable."]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG, Txdis>;
impl<'a, REG> TxdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdis::Enabled)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdis::Disabled)
    }
}
#[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc {
    #[doc = "0: Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    ClockOnCharacter = 0,
    #[doc = "1: Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    ContinousClock = 1,
}
impl From<Cc> for bool {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC` reader - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
pub type CcR = crate::BitReader<Cc>;
impl CcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            false => Cc::ClockOnCharacter,
            true => Cc::ContinousClock,
        }
    }
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline(always)]
    pub fn is_clock_on_character(&self) -> bool {
        *self == Cc::ClockOnCharacter
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline(always)]
    pub fn is_continous_clock(&self) -> bool {
        *self == Cc::ContinousClock
    }
}
#[doc = "Field `CC` writer - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG, Cc>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline(always)]
    pub fn clock_on_character(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::ClockOnCharacter)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline(always)]
    pub fn continous_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::ContinousClock)
    }
}
#[doc = "Clear Continuous Clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrcconrx {
    #[doc = "0: No effect. No effect on the CC bit."]
    NoEffect = 0,
    #[doc = "1: Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AutoClear = 1,
}
impl From<Clrcconrx> for bool {
    #[inline(always)]
    fn from(variant: Clrcconrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRCCONRX` reader - Clear Continuous Clock."]
pub type ClrcconrxR = crate::BitReader<Clrcconrx>;
impl ClrcconrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clrcconrx {
        match self.bits {
            false => Clrcconrx::NoEffect,
            true => Clrcconrx::AutoClear,
        }
    }
    #[doc = "No effect. No effect on the CC bit."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Clrcconrx::NoEffect
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline(always)]
    pub fn is_auto_clear(&self) -> bool {
        *self == Clrcconrx::AutoClear
    }
}
#[doc = "Field `CLRCCONRX` writer - Clear Continuous Clock."]
pub type ClrcconrxW<'a, REG> = crate::BitWriter<'a, REG, Clrcconrx>;
impl<'a, REG> ClrcconrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. No effect on the CC bit."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Clrcconrx::NoEffect)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline(always)]
    pub fn auto_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clrcconrx::AutoClear)
    }
}
#[doc = "Autobaud enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autobaud {
    #[doc = "0: Disabled. USART is in normal operating mode."]
    Disabled = 0,
    #[doc = "1: Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    Enabled = 1,
}
impl From<Autobaud> for bool {
    #[inline(always)]
    fn from(variant: Autobaud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOBAUD` reader - Autobaud enable."]
pub type AutobaudR = crate::BitReader<Autobaud>;
impl AutobaudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autobaud {
        match self.bits {
            false => Autobaud::Disabled,
            true => Autobaud::Enabled,
        }
    }
    #[doc = "Disabled. USART is in normal operating mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Autobaud::Disabled
    }
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Autobaud::Enabled
    }
}
#[doc = "Field `AUTOBAUD` writer - Autobaud enable."]
pub type AutobaudW<'a, REG> = crate::BitWriter<'a, REG, Autobaud>;
impl<'a, REG> AutobaudW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. USART is in normal operating mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autobaud::Disabled)
    }
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autobaud::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&self) -> TxbrkenR {
        TxbrkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&self) -> AddrdetR {
        AddrdetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&self) -> TxdisR {
        TxdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&self) -> ClrcconrxR {
        ClrcconrxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&self) -> AutobaudR {
        AutobaudR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("txbrken", &self.txbrken())
            .field("addrdet", &self.addrdet())
            .field("txdis", &self.txdis())
            .field("cc", &self.cc())
            .field("clrcconrx", &self.clrcconrx())
            .field("autobaud", &self.autobaud())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    #[must_use]
    pub fn txbrken(&mut self) -> TxbrkenW<CtlSpec> {
        TxbrkenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    #[must_use]
    pub fn addrdet(&mut self) -> AddrdetW<CtlSpec> {
        AddrdetW::new(self, 2)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<CtlSpec> {
        TxdisW::new(self, 6)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<CtlSpec> {
        CcW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    #[must_use]
    pub fn clrcconrx(&mut self) -> ClrcconrxW<CtlSpec> {
        ClrcconrxW::new(self, 9)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    #[must_use]
    pub fn autobaud(&mut self) -> AutobaudW<CtlSpec> {
        AutobaudW::new(self, 16)
    }
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
