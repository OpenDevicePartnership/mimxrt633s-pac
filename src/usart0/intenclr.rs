#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `TXIDLECLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type TxidleclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELTACTSCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type DeltactsclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type TxdisclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELTARXBRKCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type DeltarxbrkclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type StartclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type FramerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type ParityerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNOISECLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type RxnoiseclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABERRCLR` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type AberrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IntenclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txidleclr(&mut self) -> TxidleclrW<IntenclrSpec> {
        TxidleclrW::new(self, 3)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn deltactsclr(&mut self) -> DeltactsclrW<IntenclrSpec> {
        DeltactsclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txdisclr(&mut self) -> TxdisclrW<IntenclrSpec> {
        TxdisclrW::new(self, 6)
    }
    #[doc = "Bit 11 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrkclr(&mut self) -> DeltarxbrkclrW<IntenclrSpec> {
        DeltarxbrkclrW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn startclr(&mut self) -> StartclrW<IntenclrSpec> {
        StartclrW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn framerrclr(&mut self) -> FramerrclrW<IntenclrSpec> {
        FramerrclrW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn parityerrclr(&mut self) -> ParityerrclrW<IntenclrSpec> {
        ParityerrclrW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseclr(&mut self) -> RxnoiseclrW<IntenclrSpec> {
        RxnoiseclrW::new(self, 15)
    }
    #[doc = "Bit 16 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn aberrclr(&mut self) -> AberrclrW<IntenclrSpec> {
        AberrclrW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
