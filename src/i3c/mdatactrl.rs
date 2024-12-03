#[doc = "Register `MDATACTRL` reader"]
pub type R = crate::R<MdatactrlSpec>;
#[doc = "Register `MDATACTRL` writer"]
pub type W = crate::W<MdatactrlSpec>;
#[doc = "Field `FLUSHTB` writer - Flush to-bus buffer/FIFO"]
pub type FlushtbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFB` writer - Flush from-bus buffer/FIFO"]
pub type FlushfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` writer - Unlock"]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIG` reader - TX trigger level"]
pub type TxtrigR = crate::FieldReader;
#[doc = "Field `TXTRIG` writer - TX trigger level"]
pub type TxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTRIG` reader - RX trigger level"]
pub type RxtrigR = crate::FieldReader;
#[doc = "Field `RXTRIG` writer - RX trigger level"]
pub type RxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXCOUNT` reader - TX byte count"]
pub type TxcountR = crate::FieldReader;
#[doc = "Field `RXCOUNT` reader - RX byte count"]
pub type RxcountR = crate::FieldReader;
#[doc = "Field `TXFULL` reader - TX is full"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `RXEMPTY` reader - RX is empty"]
pub type RxemptyR = crate::BitReader;
impl R {
    #[doc = "Bits 4:5 - TX trigger level"]
    #[inline(always)]
    pub fn txtrig(&self) -> TxtrigR {
        TxtrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RX trigger level"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RxtrigR {
        RxtrigR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:20 - TX byte count"]
    #[inline(always)]
    pub fn txcount(&self) -> TxcountR {
        TxcountR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RX byte count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RxcountR {
        RxcountR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TX is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDATACTRL")
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flush to-bus buffer/FIFO"]
    #[inline(always)]
    pub fn flushtb(&mut self) -> FlushtbW<MdatactrlSpec> {
        FlushtbW::new(self, 0)
    }
    #[doc = "Bit 1 - Flush from-bus buffer/FIFO"]
    #[inline(always)]
    pub fn flushfb(&mut self) -> FlushfbW<MdatactrlSpec> {
        FlushfbW::new(self, 1)
    }
    #[doc = "Bit 3 - Unlock"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<MdatactrlSpec> {
        UnlockW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TX trigger level"]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TxtrigW<MdatactrlSpec> {
        TxtrigW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RX trigger level"]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RxtrigW<MdatactrlSpec> {
        RxtrigW::new(self, 6)
    }
}
#[doc = "Master Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdatactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdatactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdatactrlSpec;
impl crate::RegisterSpec for MdatactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdatactrl::R`](R) reader structure"]
impl crate::Readable for MdatactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdatactrl::W`](W) writer structure"]
impl crate::Writable for MdatactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDATACTRL to value 0x8000_0030"]
impl crate::Resettable for MdatactrlSpec {
    const RESET_VALUE: u32 = 0x8000_0030;
}
