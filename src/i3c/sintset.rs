#[doc = "Register `SINTSET` reader"]
pub type R = crate::R<SintsetSpec>;
#[doc = "Register `SINTSET` writer"]
pub type W = crate::W<SintsetSpec>;
#[doc = "Field `START` reader - Start interrupt enable"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start interrupt enable"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHED` reader - Match interrupt enable"]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - Match interrupt enable"]
pub type MatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop interrupt enable"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop interrupt enable"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - Receive interrupt enable"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `RXPEND` writer - Receive interrupt enable"]
pub type RxpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEND` reader - Transmit interrupt enable"]
pub type TxsendR = crate::BitReader;
#[doc = "Field `TXSEND` writer - Transmit interrupt enable"]
pub type TxsendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACHG` reader - Dynamic address change interrupt enable"]
pub type DachgR = crate::BitReader;
#[doc = "Field `DACHG` writer - Dynamic address change interrupt enable"]
pub type DachgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC` reader - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
pub type CccR = crate::BitReader;
#[doc = "Field `CCC` writer - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
pub type CccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error/warning interrupt enable"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `ERRWARN` writer - Error/warning interrupt enable"]
pub type ErrwarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRMATCHED` reader - Double Data Rate (DDR) interrupt enable"]
pub type DdrmatchedR = crate::BitReader;
#[doc = "Field `DDRMATCHED` writer - Double Data Rate (DDR) interrupt enable"]
pub type DdrmatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANDLED` reader - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
pub type ChandledR = crate::BitReader;
#[doc = "Field `CHANDLED` writer - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
pub type ChandledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT` reader - Event interrupt enable"]
pub type EventR = crate::BitReader;
#[doc = "Field `EVENT` writer - Event interrupt enable"]
pub type EventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Start interrupt enable"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt enable"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop interrupt enable"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn txsend(&self) -> TxsendR {
        TxsendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Dynamic address change interrupt enable"]
    #[inline(always)]
    pub fn dachg(&self) -> DachgR {
        DachgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error/warning interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DdrmatchedR {
        DdrmatchedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn chandled(&self) -> ChandledR {
        ChandledR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event interrupt enable"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTSET")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Start interrupt enable"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<SintsetSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - Match interrupt enable"]
    #[inline(always)]
    pub fn matched(&mut self) -> MatchedW<SintsetSpec> {
        MatchedW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop interrupt enable"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<SintsetSpec> {
        StopW::new(self, 10)
    }
    #[doc = "Bit 11 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&mut self) -> RxpendW<SintsetSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn txsend(&mut self) -> TxsendW<SintsetSpec> {
        TxsendW::new(self, 12)
    }
    #[doc = "Bit 13 - Dynamic address change interrupt enable"]
    #[inline(always)]
    pub fn dachg(&mut self) -> DachgW<SintsetSpec> {
        DachgW::new(self, 13)
    }
    #[doc = "Bit 14 - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn ccc(&mut self) -> CccW<SintsetSpec> {
        CccW::new(self, 14)
    }
    #[doc = "Bit 15 - Error/warning interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&mut self) -> ErrwarnW<SintsetSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 16 - Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    pub fn ddrmatched(&mut self) -> DdrmatchedW<SintsetSpec> {
        DdrmatchedW::new(self, 16)
    }
    #[doc = "Bit 17 - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn chandled(&mut self) -> ChandledW<SintsetSpec> {
        ChandledW::new(self, 17)
    }
    #[doc = "Bit 18 - Event interrupt enable"]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<SintsetSpec> {
        EventW::new(self, 18)
    }
}
#[doc = "Slave Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SintsetSpec;
impl crate::RegisterSpec for SintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintset::R`](R) reader structure"]
impl crate::Readable for SintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sintset::W`](W) writer structure"]
impl crate::Writable for SintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINTSET to value 0"]
impl crate::Resettable for SintsetSpec {
    const RESET_VALUE: u32 = 0;
}
