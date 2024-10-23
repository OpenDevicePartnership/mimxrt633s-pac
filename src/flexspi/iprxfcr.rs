#[doc = "Register `IPRXFCR` reader"]
pub type R = crate::R<IprxfcrSpec>;
#[doc = "Register `IPRXFCR` writer"]
pub type W = crate::W<IprxfcrSpec>;
#[doc = "Field `CLRIPRXF` reader - Clear all valid data entries in IP RX FIFO."]
pub type ClriprxfR = crate::BitReader;
#[doc = "Field `CLRIPRXF` writer - Clear all valid data entries in IP RX FIFO."]
pub type ClriprxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IP RX FIFO reading by DMA enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: IP RX FIFO would be read by processor."]
    Rxdmaen0 = 0,
    #[doc = "1: IP RX FIFO would be read by DMA."]
    Rxdmaen1 = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - IP RX FIFO reading by DMA enabled."]
pub type RxdmaenR = crate::BitReader<Rxdmaen>;
impl RxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmaen {
        match self.bits {
            false => Rxdmaen::Rxdmaen0,
            true => Rxdmaen::Rxdmaen1,
        }
    }
    #[doc = "IP RX FIFO would be read by processor."]
    #[inline(always)]
    pub fn is_rxdmaen_0(&self) -> bool {
        *self == Rxdmaen::Rxdmaen0
    }
    #[doc = "IP RX FIFO would be read by DMA."]
    #[inline(always)]
    pub fn is_rxdmaen_1(&self) -> bool {
        *self == Rxdmaen::Rxdmaen1
    }
}
#[doc = "Field `RXDMAEN` writer - IP RX FIFO reading by DMA enabled."]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP RX FIFO would be read by processor."]
    #[inline(always)]
    pub fn rxdmaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::Rxdmaen0)
    }
    #[doc = "IP RX FIFO would be read by DMA."]
    #[inline(always)]
    pub fn rxdmaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::Rxdmaen1)
    }
}
#[doc = "Field `RXWMRK` reader - Watermark level is (RXWMRK+1)*64 Bits."]
pub type RxwmrkR = crate::FieldReader;
#[doc = "Field `RXWMRK` writer - Watermark level is (RXWMRK+1)*64 Bits."]
pub type RxwmrkW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn clriprxf(&self) -> ClriprxfR {
        ClriprxfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn rxwmrk(&self) -> RxwmrkR {
        RxwmrkR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPRXFCR")
            .field("clriprxf", &self.clriprxf())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxwmrk", &self.rxwmrk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn clriprxf(&mut self) -> ClriprxfW<IprxfcrSpec> {
        ClriprxfW::new(self, 0)
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<IprxfcrSpec> {
        RxdmaenW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    #[must_use]
    pub fn rxwmrk(&mut self) -> RxwmrkW<IprxfcrSpec> {
        RxwmrkW::new(self, 2)
    }
}
#[doc = "IP RX FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprxfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprxfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IprxfcrSpec;
impl crate::RegisterSpec for IprxfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprxfcr::R`](R) reader structure"]
impl crate::Readable for IprxfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`iprxfcr::W`](W) writer structure"]
impl crate::Writable for IprxfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPRXFCR to value 0"]
impl crate::Resettable for IprxfcrSpec {
    const RESET_VALUE: u32 = 0;
}
