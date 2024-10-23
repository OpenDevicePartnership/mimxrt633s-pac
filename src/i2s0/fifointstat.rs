#[doc = "Register `FIFOINTSTAT` reader"]
pub type R = crate::R<FifointstatSpec>;
#[doc = "Field `TXERR` reader - TX FIFO error."]
pub type TxerrR = crate::BitReader;
#[doc = "Field `RXERR` reader - RX FIFO error."]
pub type RxerrR = crate::BitReader;
#[doc = "Field `TXLVL` reader - Transmit FIFO level interrupt."]
pub type TxlvlR = crate::BitReader;
#[doc = "Field `RXLVL` reader - Receive FIFO level interrupt."]
pub type RxlvlR = crate::BitReader;
#[doc = "Field `PERINT` reader - Peripheral interrupt."]
pub type PerintR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX FIFO error."]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error."]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO level interrupt."]
    #[inline(always)]
    pub fn txlvl(&self) -> TxlvlR {
        TxlvlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO level interrupt."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RxlvlR {
        RxlvlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral interrupt."]
    #[inline(always)]
    pub fn perint(&self) -> PerintR {
        PerintR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTSTAT")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .field("perint", &self.perint())
            .finish()
    }
}
#[doc = "FIFO interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifointstatSpec;
impl crate::RegisterSpec for FifointstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifointstat::R`](R) reader structure"]
impl crate::Readable for FifointstatSpec {}
#[doc = "`reset()` method sets FIFOINTSTAT to value 0"]
impl crate::Resettable for FifointstatSpec {
    const RESET_VALUE: u32 = 0;
}
