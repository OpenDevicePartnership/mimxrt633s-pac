#[doc = "Register `FIFOTRIG` reader"]
pub type R = crate::R<FifotrigSpec>;
#[doc = "Register `FIFOTRIG` writer"]
pub type W = crate::W<FifotrigSpec>;
#[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txlvlena {
    #[doc = "0: Transmit FIFO level does not generate a FIFO level trigger."]
    Disabled = 0,
    #[doc = "1: An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    Enabled = 1,
}
impl From<Txlvlena> for bool {
    #[inline(always)]
    fn from(variant: Txlvlena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXLVLENA` reader - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub type TxlvlenaR = crate::BitReader<Txlvlena>;
impl TxlvlenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txlvlena {
        match self.bits {
            false => Txlvlena::Disabled,
            true => Txlvlena::Enabled,
        }
    }
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txlvlena::Disabled
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txlvlena::Enabled
    }
}
#[doc = "Field `TXLVLENA` writer - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub type TxlvlenaW<'a, REG> = crate::BitWriter<'a, REG, Txlvlena>;
impl<'a, REG> TxlvlenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txlvlena::Disabled)
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txlvlena::Enabled)
    }
}
#[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxlvlena {
    #[doc = "0: Receive FIFO level does not generate a FIFO level trigger."]
    Disabled = 0,
    #[doc = "1: An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    Enabled = 1,
}
impl From<Rxlvlena> for bool {
    #[inline(always)]
    fn from(variant: Rxlvlena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLVLENA` reader - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub type RxlvlenaR = crate::BitReader<Rxlvlena>;
impl RxlvlenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxlvlena {
        match self.bits {
            false => Rxlvlena::Disabled,
            true => Rxlvlena::Enabled,
        }
    }
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxlvlena::Disabled
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxlvlena::Enabled
    }
}
#[doc = "Field `RXLVLENA` writer - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub type RxlvlenaW<'a, REG> = crate::BitWriter<'a, REG, Rxlvlena>;
impl<'a, REG> RxlvlenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlvlena::Disabled)
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlvlena::Enabled)
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub type TxlvlR = crate::FieldReader;
#[doc = "Field `TXLVL` writer - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub type TxlvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXLVL` reader - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub type RxlvlR = crate::FieldReader;
#[doc = "Field `RXLVL` writer - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub type RxlvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&self) -> TxlvlenaR {
        TxlvlenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&self) -> RxlvlenaR {
        RxlvlenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&self) -> TxlvlR {
        TxlvlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RxlvlR {
        RxlvlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOTRIG")
            .field("txlvlena", &self.txlvlena())
            .field("rxlvlena", &self.rxlvlena())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    #[must_use]
    pub fn txlvlena(&mut self) -> TxlvlenaW<FifotrigSpec> {
        TxlvlenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvlena(&mut self) -> RxlvlenaW<FifotrigSpec> {
        RxlvlenaW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TxlvlW<FifotrigSpec> {
        TxlvlW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RxlvlW<FifotrigSpec> {
        RxlvlW::new(self, 16)
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifotrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifotrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifotrigSpec;
impl crate::RegisterSpec for FifotrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifotrig::R`](R) reader structure"]
impl crate::Readable for FifotrigSpec {}
#[doc = "`write(|w| ..)` method takes [`fifotrig::W`](W) writer structure"]
impl crate::Writable for FifotrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOTRIG to value 0"]
impl crate::Resettable for FifotrigSpec {
    const RESET_VALUE: u32 = 0;
}
