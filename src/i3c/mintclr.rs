#[doc = "Register `MINTCLR` writer"]
pub type W = crate::W<MintclrSpec>;
#[doc = "Field `SLVSTART` writer - SLVSTART interrupt enable clear"]
pub type SlvstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCTRLDONE` writer - MCTRLDONE interrupt enable clear"]
pub type MctrldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLETE` writer - COMPLETE interrupt enable clear"]
pub type CompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` writer - RXPEND interrupt enable clear"]
pub type RxpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNOTFULL` writer - TXNOTFULL interrupt enable clear"]
pub type TxnotfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIWON` writer - IBIWON interrupt enable clear"]
pub type IbiwonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` writer - ERRWARN interrupt enable clear"]
pub type ErrwarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOWMASTER` writer - NOWMASTER interrupt enable clear"]
pub type NowmasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MintclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 8 - SLVSTART interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn slvstart(&mut self) -> SlvstartW<MintclrSpec> {
        SlvstartW::new(self, 8)
    }
    #[doc = "Bit 9 - MCTRLDONE interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn mctrldone(&mut self) -> MctrldoneW<MintclrSpec> {
        MctrldoneW::new(self, 9)
    }
    #[doc = "Bit 10 - COMPLETE interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> CompleteW<MintclrSpec> {
        CompleteW::new(self, 10)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RxpendW<MintclrSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - TXNOTFULL interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn txnotfull(&mut self) -> TxnotfullW<MintclrSpec> {
        TxnotfullW::new(self, 12)
    }
    #[doc = "Bit 13 - IBIWON interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ibiwon(&mut self) -> IbiwonW<MintclrSpec> {
        IbiwonW::new(self, 13)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ErrwarnW<MintclrSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 19 - NOWMASTER interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn nowmaster(&mut self) -> NowmasterW<MintclrSpec> {
        NowmasterW::new(self, 19)
    }
}
#[doc = "Master Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintclrSpec;
impl crate::RegisterSpec for MintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mintclr::W`](W) writer structure"]
impl crate::Writable for MintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MINTCLR to value 0"]
impl crate::Resettable for MintclrSpec {
    const RESET_VALUE: u32 = 0;
}
