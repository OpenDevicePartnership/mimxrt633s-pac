#[doc = "Register `MINTMASKED` reader"]
pub type R = crate::R<MintmaskedSpec>;
#[doc = "Field `SLVSTART` reader - SLVSTART interrupt mask"]
pub type SlvstartR = crate::BitReader;
#[doc = "Field `MCTRLDONE` reader - MCTRLDONE interrupt mask"]
pub type MctrldoneR = crate::BitReader;
#[doc = "Field `COMPLETE` reader - COMPLETE interrupt mask"]
pub type CompleteR = crate::BitReader;
#[doc = "Field `RXPEND` reader - RXPEND interrupt mask"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `TXNOTFULL` reader - TXNOTFULL interrupt mask"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `IBIWON` reader - IBIWON interrupt mask"]
pub type IbiwonR = crate::BitReader;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt mask"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `NOWMASTER` reader - NOWMASTER interrupt mask"]
pub type NowmasterR = crate::BitReader;
impl R {
    #[doc = "Bit 8 - SLVSTART interrupt mask"]
    #[inline(always)]
    pub fn slvstart(&self) -> SlvstartR {
        SlvstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTRLDONE interrupt mask"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MctrldoneR {
        MctrldoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMPLETE interrupt mask"]
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt mask"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXNOTFULL interrupt mask"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IBIWON interrupt mask"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IbiwonR {
        IbiwonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - NOWMASTER interrupt mask"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NowmasterR {
        NowmasterR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MINTMASKED")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[doc = "Master Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintmasked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintmaskedSpec;
impl crate::RegisterSpec for MintmaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintmasked::R`](R) reader structure"]
impl crate::Readable for MintmaskedSpec {}
#[doc = "`reset()` method sets MINTMASKED to value 0x1000"]
impl crate::Resettable for MintmaskedSpec {
    const RESET_VALUE: u32 = 0x1000;
}
