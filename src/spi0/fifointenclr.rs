#[doc = "Register `FIFOINTENCLR` reader"]
pub type R = crate::R<FifointenclrSpec>;
#[doc = "Register `FIFOINTENCLR` writer"]
pub type W = crate::W<FifointenclrSpec>;
#[doc = "Field `TXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TxlvlR = crate::BitReader;
#[doc = "Field `TXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TxlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RxlvlR = crate::BitReader;
#[doc = "Field `RXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RxlvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TxlvlR {
        TxlvlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RxlvlR {
        RxlvlR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTENCLR")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TxerrW<FifointenclrSpec> {
        TxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RxerrW<FifointenclrSpec> {
        RxerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TxlvlW<FifointenclrSpec> {
        TxlvlW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RxlvlW<FifointenclrSpec> {
        RxlvlW::new(self, 3)
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifointenclrSpec;
impl crate::RegisterSpec for FifointenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifointenclr::R`](R) reader structure"]
impl crate::Readable for FifointenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifointenclr::W`](W) writer structure"]
impl crate::Writable for FifointenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOINTENCLR to value 0"]
impl crate::Resettable for FifointenclrSpec {
    const RESET_VALUE: u32 = 0;
}
