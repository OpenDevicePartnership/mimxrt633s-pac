#[doc = "Register `SINTMASKED` reader"]
pub type R = crate::R<SintmaskedSpec>;
#[doc = "Field `START` reader - START interrupt mask"]
pub type StartR = crate::BitReader;
#[doc = "Field `MATCHED` reader - MATCHED interrupt mask"]
pub type MatchedR = crate::BitReader;
#[doc = "Field `STOP` reader - STOP interrupt mask"]
pub type StopR = crate::BitReader;
#[doc = "Field `RXPEND` reader - RXPEND interrupt mask"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `TXSEND` reader - TXSEND interrupt mask"]
pub type TxsendR = crate::BitReader;
#[doc = "Field `DACHG` reader - DACHG interrupt mask"]
pub type DachgR = crate::BitReader;
#[doc = "Field `CCC` reader - CCC interrupt mask"]
pub type CccR = crate::BitReader;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt mask"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `DDRMATCHED` reader - DDRMATCHED interrupt mask"]
pub type DdrmatchedR = crate::BitReader;
#[doc = "Field `CHANDLED` reader - CHANDLED interrupt mask"]
pub type ChandledR = crate::BitReader;
#[doc = "Field `EVENT` reader - EVENT interrupt mask"]
pub type EventR = crate::BitReader;
impl R {
    #[doc = "Bit 8 - START interrupt mask"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MATCHED interrupt mask"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP interrupt mask"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt mask"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXSEND interrupt mask"]
    #[inline(always)]
    pub fn txsend(&self) -> TxsendR {
        TxsendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG interrupt mask"]
    #[inline(always)]
    pub fn dachg(&self) -> DachgR {
        DachgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCC interrupt mask"]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt mask"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DdrmatchedR {
        DdrmatchedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CHANDLED interrupt mask"]
    #[inline(always)]
    pub fn chandled(&self) -> ChandledR {
        ChandledR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EVENT interrupt mask"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTMASKED")
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
#[doc = "Slave Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintmasked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SintmaskedSpec;
impl crate::RegisterSpec for SintmaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintmasked::R`](R) reader structure"]
impl crate::Readable for SintmaskedSpec {}
#[doc = "`reset()` method sets SINTMASKED to value 0x1000"]
impl crate::Resettable for SintmaskedSpec {
    const RESET_VALUE: u32 = 0x1000;
}
