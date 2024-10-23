#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `TXIDLE` reader - Transmitter Idle status."]
pub type TxidleR = crate::BitReader;
#[doc = "Field `DELTACTS` reader - This bit is set when a change in the state of the CTS input is detected."]
pub type DeltactsR = crate::BitReader;
#[doc = "Field `TXDISINT` reader - Transmitter Disabled Interrupt flag."]
pub type TxdisintR = crate::BitReader;
#[doc = "Field `DELTARXBRK` reader - This bit is set when a change in the state of receiver break detection occurs."]
pub type DeltarxbrkR = crate::BitReader;
#[doc = "Field `START` reader - This bit is set when a start is detected on the receiver input."]
pub type StartR = crate::BitReader;
#[doc = "Field `FRAMERRINT` reader - Framing Error interrupt flag."]
pub type FramerrintR = crate::BitReader;
#[doc = "Field `PARITYERRINT` reader - Parity Error interrupt flag."]
pub type ParityerrintR = crate::BitReader;
#[doc = "Field `RXNOISEINT` reader - Received Noise interrupt flag."]
pub type RxnoiseintR = crate::BitReader;
#[doc = "Field `ABERRINT` reader - Auto baud Error Interrupt flag."]
pub type AberrintR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Transmitter Idle status."]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub fn deltacts(&self) -> DeltactsR {
        DeltactsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub fn txdisint(&self) -> TxdisintR {
        TxdisintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DeltarxbrkR {
        DeltarxbrkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag."]
    #[inline(always)]
    pub fn framerrint(&self) -> FramerrintR {
        FramerrintR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag."]
    #[inline(always)]
    pub fn parityerrint(&self) -> ParityerrintR {
        ParityerrintR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag."]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RxnoiseintR {
        RxnoiseintR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto baud Error Interrupt flag."]
    #[inline(always)]
    pub fn aberrint(&self) -> AberrintR {
        AberrintR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("txidle", &self.txidle())
            .field("deltacts", &self.deltacts())
            .field("txdisint", &self.txdisint())
            .field("deltarxbrk", &self.deltarxbrk())
            .field("start", &self.start())
            .field("framerrint", &self.framerrint())
            .field("parityerrint", &self.parityerrint())
            .field("rxnoiseint", &self.rxnoiseint())
            .field("aberrint", &self.aberrint())
            .finish()
    }
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
