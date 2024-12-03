#[doc = "Register `MINTSET` reader"]
pub type R = crate::R<MintsetSpec>;
#[doc = "Register `MINTSET` writer"]
pub type W = crate::W<MintsetSpec>;
#[doc = "Field `SLVSTART` reader - Slave start interrupt enable"]
pub type SlvstartR = crate::BitReader;
#[doc = "Field `SLVSTART` writer - Slave start interrupt enable"]
pub type SlvstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCTRLDONE` reader - Master control done interrupt enable"]
pub type MctrldoneR = crate::BitReader;
#[doc = "Field `MCTRLDONE` writer - Master control done interrupt enable"]
pub type MctrldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLETE` reader - Completed message interrupt enable"]
pub type CompleteR = crate::BitReader;
#[doc = "Field `COMPLETE` writer - Completed message interrupt enable"]
pub type CompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - RX pending interrupt enable"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `RXPEND` writer - RX pending interrupt enable"]
pub type RxpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNOTFULL` reader - TX buffer/FIFO is not full interrupt enable"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `TXNOTFULL` writer - TX buffer/FIFO is not full interrupt enable"]
pub type TxnotfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIWON` reader - In-Band Interrupt (IBI) won interrupt enable"]
pub type IbiwonR = crate::BitReader;
#[doc = "Field `IBIWON` writer - In-Band Interrupt (IBI) won interrupt enable"]
pub type IbiwonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error or warning (ERRWARN) interrupt enable"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `ERRWARN` writer - Error or warning (ERRWARN) interrupt enable"]
pub type ErrwarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOWMASTER` reader - Now master (now this I3C module is a master) interrupt enable"]
pub type NowmasterR = crate::BitReader;
#[doc = "Field `NOWMASTER` writer - Now master (now this I3C module is a master) interrupt enable"]
pub type NowmasterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Slave start interrupt enable"]
    #[inline(always)]
    pub fn slvstart(&self) -> SlvstartR {
        SlvstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master control done interrupt enable"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MctrldoneR {
        MctrldoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Completed message interrupt enable"]
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX pending interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IbiwonR {
        IbiwonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NowmasterR {
        NowmasterR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MINTSET")
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
impl W {
    #[doc = "Bit 8 - Slave start interrupt enable"]
    #[inline(always)]
    pub fn slvstart(&mut self) -> SlvstartW<MintsetSpec> {
        SlvstartW::new(self, 8)
    }
    #[doc = "Bit 9 - Master control done interrupt enable"]
    #[inline(always)]
    pub fn mctrldone(&mut self) -> MctrldoneW<MintsetSpec> {
        MctrldoneW::new(self, 9)
    }
    #[doc = "Bit 10 - Completed message interrupt enable"]
    #[inline(always)]
    pub fn complete(&mut self) -> CompleteW<MintsetSpec> {
        CompleteW::new(self, 10)
    }
    #[doc = "Bit 11 - RX pending interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&mut self) -> RxpendW<MintsetSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    pub fn txnotfull(&mut self) -> TxnotfullW<MintsetSpec> {
        TxnotfullW::new(self, 12)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    pub fn ibiwon(&mut self) -> IbiwonW<MintsetSpec> {
        IbiwonW::new(self, 13)
    }
    #[doc = "Bit 15 - Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&mut self) -> ErrwarnW<MintsetSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 19 - Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    pub fn nowmaster(&mut self) -> NowmasterW<MintsetSpec> {
        NowmasterW::new(self, 19)
    }
}
#[doc = "Master Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintsetSpec;
impl crate::RegisterSpec for MintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintset::R`](R) reader structure"]
impl crate::Readable for MintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`mintset::W`](W) writer structure"]
impl crate::Writable for MintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MINTSET to value 0"]
impl crate::Resettable for MintsetSpec {
    const RESET_VALUE: u32 = 0;
}
