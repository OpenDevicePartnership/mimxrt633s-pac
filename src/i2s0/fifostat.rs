#[doc = "Register `FIFOSTAT` reader"]
pub type R = crate::R<FifostatSpec>;
#[doc = "Register `FIFOSTAT` writer"]
pub type W = crate::W<FifostatSpec>;
#[doc = "Field `TXERR` reader - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERINT` reader - Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
pub type PerintR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXNOTFULL` reader - Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `RXNOTEMPTY` reader - Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
pub type RxnotemptyR = crate::BitReader;
#[doc = "Field `RXFULL` reader - Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
pub type RxfullR = crate::BitReader;
#[doc = "Field `TXLVL` reader - Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
pub type TxlvlR = crate::FieldReader;
#[doc = "Field `RXLVL` reader - Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
pub type RxlvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub fn perint(&self) -> PerintR {
        PerintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxnotempty(&self) -> RxnotemptyR {
        RxnotemptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub fn txlvl(&self) -> TxlvlR {
        TxlvlR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RxlvlR {
        RxlvlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSTAT")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("perint", &self.perint())
            .field("txempty", &self.txempty())
            .field("txnotfull", &self.txnotfull())
            .field("rxnotempty", &self.rxnotempty())
            .field("rxfull", &self.rxfull())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TxerrW<FifostatSpec> {
        TxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RxerrW<FifostatSpec> {
        RxerrW::new(self, 1)
    }
}
#[doc = "FIFO status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifostat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifostatSpec;
impl crate::RegisterSpec for FifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifostat::R`](R) reader structure"]
impl crate::Readable for FifostatSpec {}
#[doc = "`write(|w| ..)` method takes [`fifostat::W`](W) writer structure"]
impl crate::Writable for FifostatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOSTAT to value 0x30"]
impl crate::Resettable for FifostatSpec {
    const RESET_VALUE: u32 = 0x30;
}
