#[doc = "Register `SDATACTRL` reader"]
pub type R = crate::R<SdatactrlSpec>;
#[doc = "Register `SDATACTRL` writer"]
pub type W = crate::W<SdatactrlSpec>;
#[doc = "Field `FLUSHTB` writer - Flush the to-bus buffer/FIFO"]
pub type FlushtbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFB` writer - Flushes the from-bus buffer/FIFO"]
pub type FlushfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` writer - Unlock"]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger level for TX FIFO emptiness\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txtrig {
    #[doc = "0: Trigger on empty"]
    Triggrempty = 0,
    #[doc = "1: Trigger on full or less"]
    Triggronefourth = 1,
    #[doc = "2: Trigger on .5 full or less"]
    Triggronehalf = 2,
    #[doc = "3: Trigger on 1 less than full or less (Default)"]
    Triggroneless = 3,
}
impl From<Txtrig> for u8 {
    #[inline(always)]
    fn from(variant: Txtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txtrig {
    type Ux = u8;
}
impl crate::IsEnum for Txtrig {}
#[doc = "Field `TXTRIG` reader - Trigger level for TX FIFO emptiness"]
pub type TxtrigR = crate::FieldReader<Txtrig>;
impl TxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txtrig {
        match self.bits {
            0 => Txtrig::Triggrempty,
            1 => Txtrig::Triggronefourth,
            2 => Txtrig::Triggronehalf,
            3 => Txtrig::Triggroneless,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger on empty"]
    #[inline(always)]
    pub fn is_triggrempty(&self) -> bool {
        *self == Txtrig::Triggrempty
    }
    #[doc = "Trigger on full or less"]
    #[inline(always)]
    pub fn is_triggronefourth(&self) -> bool {
        *self == Txtrig::Triggronefourth
    }
    #[doc = "Trigger on .5 full or less"]
    #[inline(always)]
    pub fn is_triggronehalf(&self) -> bool {
        *self == Txtrig::Triggronehalf
    }
    #[doc = "Trigger on 1 less than full or less (Default)"]
    #[inline(always)]
    pub fn is_triggroneless(&self) -> bool {
        *self == Txtrig::Triggroneless
    }
}
#[doc = "Field `TXTRIG` writer - Trigger level for TX FIFO emptiness"]
pub type TxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txtrig, crate::Safe>;
impl<'a, REG> TxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger on empty"]
    #[inline(always)]
    pub fn triggrempty(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Triggrempty)
    }
    #[doc = "Trigger on full or less"]
    #[inline(always)]
    pub fn triggronefourth(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Triggronefourth)
    }
    #[doc = "Trigger on .5 full or less"]
    #[inline(always)]
    pub fn triggronehalf(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Triggronehalf)
    }
    #[doc = "Trigger on 1 less than full or less (Default)"]
    #[inline(always)]
    pub fn triggroneless(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrig::Triggroneless)
    }
}
#[doc = "Trigger level for RX FIFO fullness\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtrig {
    #[doc = "0: Trigger on not empty"]
    Triggrnotempty = 0,
    #[doc = "1: Trigger on or more full"]
    Triggronefourth = 1,
    #[doc = "2: Trigger on .5 or more full"]
    Triggronehalf = 2,
    #[doc = "3: Trigger on 3/4 or more full"]
    Triggrthreefourths = 3,
}
impl From<Rxtrig> for u8 {
    #[inline(always)]
    fn from(variant: Rxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtrig {
    type Ux = u8;
}
impl crate::IsEnum for Rxtrig {}
#[doc = "Field `RXTRIG` reader - Trigger level for RX FIFO fullness"]
pub type RxtrigR = crate::FieldReader<Rxtrig>;
impl RxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxtrig {
        match self.bits {
            0 => Rxtrig::Triggrnotempty,
            1 => Rxtrig::Triggronefourth,
            2 => Rxtrig::Triggronehalf,
            3 => Rxtrig::Triggrthreefourths,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger on not empty"]
    #[inline(always)]
    pub fn is_triggrnotempty(&self) -> bool {
        *self == Rxtrig::Triggrnotempty
    }
    #[doc = "Trigger on or more full"]
    #[inline(always)]
    pub fn is_triggronefourth(&self) -> bool {
        *self == Rxtrig::Triggronefourth
    }
    #[doc = "Trigger on .5 or more full"]
    #[inline(always)]
    pub fn is_triggronehalf(&self) -> bool {
        *self == Rxtrig::Triggronehalf
    }
    #[doc = "Trigger on 3/4 or more full"]
    #[inline(always)]
    pub fn is_triggrthreefourths(&self) -> bool {
        *self == Rxtrig::Triggrthreefourths
    }
}
#[doc = "Field `RXTRIG` writer - Trigger level for RX FIFO fullness"]
pub type RxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxtrig, crate::Safe>;
impl<'a, REG> RxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger on not empty"]
    #[inline(always)]
    pub fn triggrnotempty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Triggrnotempty)
    }
    #[doc = "Trigger on or more full"]
    #[inline(always)]
    pub fn triggronefourth(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Triggronefourth)
    }
    #[doc = "Trigger on .5 or more full"]
    #[inline(always)]
    pub fn triggronehalf(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Triggronehalf)
    }
    #[doc = "Trigger on 3/4 or more full"]
    #[inline(always)]
    pub fn triggrthreefourths(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrig::Triggrthreefourths)
    }
}
#[doc = "Field `TXCOUNT` reader - Count of bytes in TX"]
pub type TxcountR = crate::FieldReader;
#[doc = "Field `RXCOUNT` reader - Count of bytes in RX"]
pub type RxcountR = crate::FieldReader;
#[doc = "TX is full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfull {
    #[doc = "0: TX is not full"]
    Txisnotfull = 0,
    #[doc = "1: TX is full"]
    Txisfull = 1,
}
impl From<Txfull> for bool {
    #[inline(always)]
    fn from(variant: Txfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFULL` reader - TX is full"]
pub type TxfullR = crate::BitReader<Txfull>;
impl TxfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfull {
        match self.bits {
            false => Txfull::Txisnotfull,
            true => Txfull::Txisfull,
        }
    }
    #[doc = "TX is not full"]
    #[inline(always)]
    pub fn is_txisnotfull(&self) -> bool {
        *self == Txfull::Txisnotfull
    }
    #[doc = "TX is full"]
    #[inline(always)]
    pub fn is_txisfull(&self) -> bool {
        *self == Txfull::Txisfull
    }
}
#[doc = "RX is empty\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxempty {
    #[doc = "0: RX is not empty"]
    Rxisnotempty = 0,
    #[doc = "1: RX is empty"]
    Rxisempty = 1,
}
impl From<Rxempty> for bool {
    #[inline(always)]
    fn from(variant: Rxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPTY` reader - RX is empty"]
pub type RxemptyR = crate::BitReader<Rxempty>;
impl RxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxempty {
        match self.bits {
            false => Rxempty::Rxisnotempty,
            true => Rxempty::Rxisempty,
        }
    }
    #[doc = "RX is not empty"]
    #[inline(always)]
    pub fn is_rxisnotempty(&self) -> bool {
        *self == Rxempty::Rxisnotempty
    }
    #[doc = "RX is empty"]
    #[inline(always)]
    pub fn is_rxisempty(&self) -> bool {
        *self == Rxempty::Rxisempty
    }
}
impl R {
    #[doc = "Bits 4:5 - Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    pub fn txtrig(&self) -> TxtrigR {
        TxtrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Trigger level for RX FIFO fullness"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RxtrigR {
        RxtrigR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:20 - Count of bytes in TX"]
    #[inline(always)]
    pub fn txcount(&self) -> TxcountR {
        TxcountR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Count of bytes in RX"]
    #[inline(always)]
    pub fn rxcount(&self) -> RxcountR {
        RxcountR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TX is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDATACTRL")
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flush the to-bus buffer/FIFO"]
    #[inline(always)]
    pub fn flushtb(&mut self) -> FlushtbW<SdatactrlSpec> {
        FlushtbW::new(self, 0)
    }
    #[doc = "Bit 1 - Flushes the from-bus buffer/FIFO"]
    #[inline(always)]
    pub fn flushfb(&mut self) -> FlushfbW<SdatactrlSpec> {
        FlushfbW::new(self, 1)
    }
    #[doc = "Bit 3 - Unlock"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<SdatactrlSpec> {
        UnlockW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TxtrigW<SdatactrlSpec> {
        TxtrigW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Trigger level for RX FIFO fullness"]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RxtrigW<SdatactrlSpec> {
        RxtrigW::new(self, 6)
    }
}
#[doc = "Slave Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdatactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdatactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdatactrlSpec;
impl crate::RegisterSpec for SdatactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdatactrl::R`](R) reader structure"]
impl crate::Readable for SdatactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdatactrl::W`](W) writer structure"]
impl crate::Writable for SdatactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDATACTRL to value 0x8000_0030"]
impl crate::Resettable for SdatactrlSpec {
    const RESET_VALUE: u32 = 0x8000_0030;
}
