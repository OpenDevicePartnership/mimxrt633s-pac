#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `RXIDLE` reader - Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
pub type RxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` reader - Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
pub type TxidleR = crate::BitReader;
#[doc = "Field `CTS` reader - This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
pub type CtsR = crate::BitReader;
#[doc = "Field `DELTACTS` writer - This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
pub type DeltactsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISSTAT` reader - Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
pub type TxdisstatR = crate::BitReader;
#[doc = "Field `RXBRK` reader - Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `DELTARXBRK` reader - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
pub type DeltarxbrkR = crate::BitReader;
#[doc = "Field `DELTARXBRK` writer - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
pub type DeltarxbrkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `START` reader - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
pub type StartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRAMERRINT` reader - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
pub type FramerrintR = crate::BitReader;
#[doc = "Field `FRAMERRINT` writer - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
pub type FramerrintW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PARITYERRINT` reader - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
pub type ParityerrintR = crate::BitReader;
#[doc = "Field `PARITYERRINT` writer - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
pub type ParityerrintW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXNOISEINT` reader - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
pub type RxnoiseintR = crate::BitReader;
#[doc = "Field `RXNOISEINT` writer - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
pub type RxnoiseintW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ABERR` reader - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
pub type AberrR = crate::BitReader;
#[doc = "Field `ABERR` writer - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
pub type AberrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 1 - Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[inline(always)]
    pub fn rxidle(&self) -> RxidleR {
        RxidleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[inline(always)]
    pub fn txdisstat(&self) -> TxdisstatR {
        TxdisstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DeltarxbrkR {
        DeltarxbrkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub fn framerrint(&self) -> FramerrintR {
        FramerrintR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[inline(always)]
    pub fn parityerrint(&self) -> ParityerrintR {
        ParityerrintR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RxnoiseintR {
        RxnoiseintR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[inline(always)]
    pub fn aberr(&self) -> AberrR {
        AberrR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("rxidle", &self.rxidle())
            .field("txidle", &self.txidle())
            .field("cts", &self.cts())
            .field("txdisstat", &self.txdisstat())
            .field("rxbrk", &self.rxbrk())
            .field("deltarxbrk", &self.deltarxbrk())
            .field("start", &self.start())
            .field("framerrint", &self.framerrint())
            .field("parityerrint", &self.parityerrint())
            .field("rxnoiseint", &self.rxnoiseint())
            .field("aberr", &self.aberr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn deltacts(&mut self) -> DeltactsW<StatSpec> {
        DeltactsW::new(self, 5)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrk(&mut self) -> DeltarxbrkW<StatSpec> {
        DeltarxbrkW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<StatSpec> {
        StartW::new(self, 12)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    #[must_use]
    pub fn framerrint(&mut self) -> FramerrintW<StatSpec> {
        FramerrintW::new(self, 13)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[inline(always)]
    #[must_use]
    pub fn parityerrint(&mut self) -> ParityerrintW<StatSpec> {
        ParityerrintW::new(self, 14)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseint(&mut self) -> RxnoiseintW<StatSpec> {
        RxnoiseintW::new(self, 15)
    }
    #[doc = "Bit 16 - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[inline(always)]
    #[must_use]
    pub fn aberr(&mut self) -> AberrW<StatSpec> {
        AberrW::new(self, 16)
    }
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_f800;
}
#[doc = "`reset()` method sets STAT to value 0x0a"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0a;
}
