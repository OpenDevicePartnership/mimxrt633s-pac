#[doc = "Register `SINTCLR` reader"]
pub type R = crate::R<SintclrSpec>;
#[doc = "Register `SINTCLR` writer"]
pub type W = crate::W<SintclrSpec>;
#[doc = "Field `START` reader - START interrupt enable clear"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START interrupt enable clear"]
pub type StartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MATCHED` reader - MATCHED interrupt enable clear"]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - MATCHED interrupt enable clear"]
pub type MatchedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `STOP` reader - STOP interrupt enable clear"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - STOP interrupt enable clear"]
pub type StopW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXPEND` reader - RXPEND interrupt enable clear"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `RXPEND` writer - RXPEND interrupt enable clear"]
pub type RxpendW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXSEND` reader - TXSEND interrupt enable clear"]
pub type TxsendR = crate::BitReader;
#[doc = "Field `TXSEND` writer - TXSEND interrupt enable clear"]
pub type TxsendW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DACHG` reader - DACHG interrupt enable clear"]
pub type DachgR = crate::BitReader;
#[doc = "Field `DACHG` writer - DACHG interrupt enable clear"]
pub type DachgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CCC` reader - CCC interrupt enable clear"]
pub type CccR = crate::BitReader;
#[doc = "Field `CCC` writer - CCC interrupt enable clear"]
pub type CccW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt enable clear"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `ERRWARN` writer - ERRWARN interrupt enable clear"]
pub type ErrwarnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DDRMATCHED` reader - DDRMATCHED interrupt enable clear"]
pub type DdrmatchedR = crate::BitReader;
#[doc = "Field `DDRMATCHED` writer - DDRMATCHED interrupt enable clear"]
pub type DdrmatchedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHANDLED` reader - CHANDLED interrupt enable clear"]
pub type ChandledR = crate::BitReader;
#[doc = "Field `CHANDLED` writer - CHANDLED interrupt enable clear"]
pub type ChandledW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EVENT` reader - EVENT interrupt enable clear"]
pub type EventR = crate::BitReader;
#[doc = "Field `EVENT` writer - EVENT interrupt enable clear"]
pub type EventW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 8 - START interrupt enable clear"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP interrupt enable clear"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXSEND interrupt enable clear"]
    #[inline(always)]
    pub fn txsend(&self) -> TxsendR {
        TxsendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG interrupt enable clear"]
    #[inline(always)]
    pub fn dachg(&self) -> DachgR {
        DachgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCC interrupt enable clear"]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DdrmatchedR {
        DdrmatchedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CHANDLED interrupt enable clear"]
    #[inline(always)]
    pub fn chandled(&self) -> ChandledR {
        ChandledR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EVENT interrupt enable clear"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTCLR")
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
    #[doc = "Bit 8 - START interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<SintclrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - MATCHED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MatchedW<SintclrSpec> {
        MatchedW::new(self, 9)
    }
    #[doc = "Bit 10 - STOP interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<SintclrSpec> {
        StopW::new(self, 10)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RxpendW<SintclrSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - TXSEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TxsendW<SintclrSpec> {
        TxsendW::new(self, 12)
    }
    #[doc = "Bit 13 - DACHG interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn dachg(&mut self) -> DachgW<SintclrSpec> {
        DachgW::new(self, 13)
    }
    #[doc = "Bit 14 - CCC interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CccW<SintclrSpec> {
        CccW::new(self, 14)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ErrwarnW<SintclrSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddrmatched(&mut self) -> DdrmatchedW<SintclrSpec> {
        DdrmatchedW::new(self, 16)
    }
    #[doc = "Bit 17 - CHANDLED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn chandled(&mut self) -> ChandledW<SintclrSpec> {
        ChandledW::new(self, 17)
    }
    #[doc = "Bit 18 - EVENT interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EventW<SintclrSpec> {
        EventW::new(self, 18)
    }
}
#[doc = "Slave Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SintclrSpec;
impl crate::RegisterSpec for SintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintclr::R`](R) reader structure"]
impl crate::Readable for SintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`sintclr::W`](W) writer structure"]
impl crate::Writable for SintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0007_ff00;
}
#[doc = "`reset()` method sets SINTCLR to value 0"]
impl crate::Resettable for SintclrSpec {
    const RESET_VALUE: u32 = 0;
}
