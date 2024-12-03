#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `TXIDLEEN` reader - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
pub type TxidleenR = crate::BitReader;
#[doc = "Field `TXIDLEEN` writer - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
pub type TxidleenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELTACTSEN` reader - When 1, enables an interrupt when there is a change in the state of the CTS input."]
pub type DeltactsenR = crate::BitReader;
#[doc = "Field `DELTACTSEN` writer - When 1, enables an interrupt when there is a change in the state of the CTS input."]
pub type DeltactsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISEN` reader - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
pub type TxdisenR = crate::BitReader;
#[doc = "Field `TXDISEN` writer - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
pub type TxdisenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELTARXBRKEN` reader - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
pub type DeltarxbrkenR = crate::BitReader;
#[doc = "Field `DELTARXBRKEN` writer - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
pub type DeltarxbrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTEN` reader - When 1, enables an interrupt when a received start bit has been detected."]
pub type StartenR = crate::BitReader;
#[doc = "Field `STARTEN` writer - When 1, enables an interrupt when a received start bit has been detected."]
pub type StartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMERREN` reader - When 1, enables an interrupt when a framing error has been detected."]
pub type FramerrenR = crate::BitReader;
#[doc = "Field `FRAMERREN` writer - When 1, enables an interrupt when a framing error has been detected."]
pub type FramerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYERREN` reader - When 1, enables an interrupt when a parity error has been detected."]
pub type ParityerrenR = crate::BitReader;
#[doc = "Field `PARITYERREN` writer - When 1, enables an interrupt when a parity error has been detected."]
pub type ParityerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNOISEEN` reader - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
pub type RxnoiseenR = crate::BitReader;
#[doc = "Field `RXNOISEEN` writer - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
pub type RxnoiseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABERREN` reader - When 1, enables an interrupt when an auto baud error occurs."]
pub type AberrenR = crate::BitReader;
#[doc = "Field `ABERREN` writer - When 1, enables an interrupt when an auto baud error occurs."]
pub type AberrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&self) -> TxidleenR {
        TxidleenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&self) -> DeltactsenR {
        DeltactsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&self) -> TxdisenR {
        TxdisenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DeltarxbrkenR {
        DeltarxbrkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&self) -> StartenR {
        StartenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&self) -> FramerrenR {
        FramerrenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&self) -> ParityerrenR {
        ParityerrenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RxnoiseenR {
        RxnoiseenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub fn aberren(&self) -> AberrenR {
        AberrenR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("txidleen", &self.txidleen())
            .field("deltactsen", &self.deltactsen())
            .field("txdisen", &self.txdisen())
            .field("deltarxbrken", &self.deltarxbrken())
            .field("starten", &self.starten())
            .field("framerren", &self.framerren())
            .field("parityerren", &self.parityerren())
            .field("rxnoiseen", &self.rxnoiseen())
            .field("aberren", &self.aberren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&mut self) -> TxidleenW<IntensetSpec> {
        TxidleenW::new(self, 3)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&mut self) -> DeltactsenW<IntensetSpec> {
        DeltactsenW::new(self, 5)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&mut self) -> TxdisenW<IntensetSpec> {
        TxdisenW::new(self, 6)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&mut self) -> DeltarxbrkenW<IntensetSpec> {
        DeltarxbrkenW::new(self, 11)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&mut self) -> StartenW<IntensetSpec> {
        StartenW::new(self, 12)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&mut self) -> FramerrenW<IntensetSpec> {
        FramerrenW::new(self, 13)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&mut self) -> ParityerrenW<IntensetSpec> {
        ParityerrenW::new(self, 14)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub fn rxnoiseen(&mut self) -> RxnoiseenW<IntensetSpec> {
        RxnoiseenW::new(self, 15)
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub fn aberren(&mut self) -> AberrenW<IntensetSpec> {
        AberrenW::new(self, 16)
    }
}
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
