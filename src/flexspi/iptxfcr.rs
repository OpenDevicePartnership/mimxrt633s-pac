#[doc = "Register `IPTXFCR` reader"]
pub type R = crate::R<IptxfcrSpec>;
#[doc = "Register `IPTXFCR` writer"]
pub type W = crate::W<IptxfcrSpec>;
#[doc = "Field `CLRIPTXF` reader - Clear all valid data entries in IP TX FIFO."]
pub type ClriptxfR = crate::BitReader;
#[doc = "Field `CLRIPTXF` writer - Clear all valid data entries in IP TX FIFO."]
pub type ClriptxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IP TX FIFO filling by DMA enabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: IP TX FIFO would be filled by processor."]
    Txdmaen0 = 0,
    #[doc = "1: IP TX FIFO would be filled by DMA."]
    Txdmaen1 = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - IP TX FIFO filling by DMA enabled."]
pub type TxdmaenR = crate::BitReader<Txdmaen>;
impl TxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmaen {
        match self.bits {
            false => Txdmaen::Txdmaen0,
            true => Txdmaen::Txdmaen1,
        }
    }
    #[doc = "IP TX FIFO would be filled by processor."]
    #[inline(always)]
    pub fn is_txdmaen_0(&self) -> bool {
        *self == Txdmaen::Txdmaen0
    }
    #[doc = "IP TX FIFO would be filled by DMA."]
    #[inline(always)]
    pub fn is_txdmaen_1(&self) -> bool {
        *self == Txdmaen::Txdmaen1
    }
}
#[doc = "Field `TXDMAEN` writer - IP TX FIFO filling by DMA enabled."]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP TX FIFO would be filled by processor."]
    #[inline(always)]
    pub fn txdmaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::Txdmaen0)
    }
    #[doc = "IP TX FIFO would be filled by DMA."]
    #[inline(always)]
    pub fn txdmaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::Txdmaen1)
    }
}
#[doc = "Field `TXWMRK` reader - Watermark level is (TXWMRK+1)*64 Bits."]
pub type TxwmrkR = crate::FieldReader;
#[doc = "Field `TXWMRK` writer - Watermark level is (TXWMRK+1)*64 Bits."]
pub type TxwmrkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn clriptxf(&self) -> ClriptxfR {
        ClriptxfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn txwmrk(&self) -> TxwmrkR {
        TxwmrkR::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPTXFCR")
            .field("clriptxf", &self.clriptxf())
            .field("txdmaen", &self.txdmaen())
            .field("txwmrk", &self.txwmrk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn clriptxf(&mut self) -> ClriptxfW<IptxfcrSpec> {
        ClriptxfW::new(self, 0)
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<IptxfcrSpec> {
        TxdmaenW::new(self, 1)
    }
    #[doc = "Bits 2:8 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn txwmrk(&mut self) -> TxwmrkW<IptxfcrSpec> {
        TxwmrkW::new(self, 2)
    }
}
#[doc = "IP TX FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iptxfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptxfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IptxfcrSpec;
impl crate::RegisterSpec for IptxfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iptxfcr::R`](R) reader structure"]
impl crate::Readable for IptxfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`iptxfcr::W`](W) writer structure"]
impl crate::Writable for IptxfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPTXFCR to value 0"]
impl crate::Resettable for IptxfcrSpec {
    const RESET_VALUE: u32 = 0;
}
