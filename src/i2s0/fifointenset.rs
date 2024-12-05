#[doc = "Register `FIFOINTENSET` reader"]
pub type R = crate::R<FifointensetSpec>;
#[doc = "Register `FIFOINTENSET` writer"]
pub type W = crate::W<FifointensetSpec>;
#[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txerr {
    #[doc = "0: No interrupt will be generated for a transmit error."]
    Disabled = 0,
    #[doc = "1: An interrupt will be generated when a transmit error occurs."]
    Enabled = 1,
}
impl From<Txerr> for bool {
    #[inline(always)]
    fn from(variant: Txerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXERR` reader - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
pub type TxerrR = crate::BitReader<Txerr>;
impl TxerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txerr {
        match self.bits {
            false => Txerr::Disabled,
            true => Txerr::Enabled,
        }
    }
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txerr::Disabled
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txerr::Enabled
    }
}
#[doc = "Field `TXERR` writer - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG, Txerr>;
impl<'a, REG> TxerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txerr::Disabled)
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txerr::Enabled)
    }
}
#[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxerr {
    #[doc = "0: No interrupt will be generated for a receive error."]
    Disabled = 0,
    #[doc = "1: An interrupt will be generated when a receive error occurs."]
    Enabled = 1,
}
impl From<Rxerr> for bool {
    #[inline(always)]
    fn from(variant: Rxerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXERR` reader - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
pub type RxerrR = crate::BitReader<Rxerr>;
impl RxerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxerr {
        match self.bits {
            false => Rxerr::Disabled,
            true => Rxerr::Enabled,
        }
    }
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxerr::Disabled
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxerr::Enabled
    }
}
#[doc = "Field `RXERR` writer - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG, Rxerr>;
impl<'a, REG> RxerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxerr::Disabled)
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxerr::Enabled)
    }
}
#[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txlvl {
    #[doc = "0: No interrupt will be generated based on the TX FIFO level."]
    Disabled = 0,
    #[doc = "1: If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    Enabled = 1,
}
impl From<Txlvl> for bool {
    #[inline(always)]
    fn from(variant: Txlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXLVL` reader - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
pub type TxlvlR = crate::BitReader<Txlvl>;
impl TxlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txlvl {
        match self.bits {
            false => Txlvl::Disabled,
            true => Txlvl::Enabled,
        }
    }
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txlvl::Disabled
    }
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txlvl::Enabled
    }
}
#[doc = "Field `TXLVL` writer - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
pub type TxlvlW<'a, REG> = crate::BitWriter<'a, REG, Txlvl>;
impl<'a, REG> TxlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txlvl::Disabled)
    }
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txlvl::Enabled)
    }
}
#[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxlvl {
    #[doc = "0: No interrupt will be generated based on the RX FIFO level."]
    Disabled = 0,
    #[doc = "1: If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    Enabled = 1,
}
impl From<Rxlvl> for bool {
    #[inline(always)]
    fn from(variant: Rxlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLVL` reader - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
pub type RxlvlR = crate::BitReader<Rxlvl>;
impl RxlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxlvl {
        match self.bits {
            false => Rxlvl::Disabled,
            true => Rxlvl::Enabled,
        }
    }
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxlvl::Disabled
    }
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxlvl::Enabled
    }
}
#[doc = "Field `RXLVL` writer - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
pub type RxlvlW<'a, REG> = crate::BitWriter<'a, REG, Rxlvl>;
impl<'a, REG> RxlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlvl::Disabled)
    }
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxlvl::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TxlvlR {
        TxlvlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RxlvlR {
        RxlvlR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOINTENSET")
            .field("txerr", &self.txerr())
            .field("rxerr", &self.rxerr())
            .field("txlvl", &self.txlvl())
            .field("rxlvl", &self.rxlvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<FifointensetSpec> {
        TxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<FifointensetSpec> {
        RxerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TxlvlW<FifointensetSpec> {
        TxlvlW::new(self, 2)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RxlvlW<FifointensetSpec> {
        RxlvlW::new(self, 3)
    }
}
#[doc = "FIFO interrupt enable set (enable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifointensetSpec;
impl crate::RegisterSpec for FifointensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifointenset::R`](R) reader structure"]
impl crate::Readable for FifointensetSpec {}
#[doc = "`write(|w| ..)` method takes [`fifointenset::W`](W) writer structure"]
impl crate::Writable for FifointensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOINTENSET to value 0"]
impl crate::Resettable for FifointensetSpec {
    const RESET_VALUE: u32 = 0;
}
